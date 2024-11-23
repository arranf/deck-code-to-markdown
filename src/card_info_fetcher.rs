use std::fs::{create_dir_all, read_to_string, File};
use std::path::PathBuf;
use std::time::Duration;

use directories::ProjectDirs;
use indicatif::ProgressBar;
use log::debug;

use crate::api_card::ApiCard;
use crate::error::AppError;

const LATEST_URL: &str = "https://api.hearthstonejson.com/v1/latest/";
const CARDS_URL: &str = "https://api.hearthstonejson.com/v1/latest/enUS/cards.json";

pub fn fetch() -> Result<Vec<ApiCard>, AppError> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Fetching card data...");

    let version_res = reqwest::blocking::get(LATEST_URL)?;
    // Gets the version from the redirected URL, by splitting on slashes in reverse and ignoring the trailing slash
    let version = get_version_from_response(&version_res)?;

    // Attempt to get cached data
    if let Some(path) = get_data_filename(&version) {
        let file_contents_result = &read_to_string(path);
        if let Ok(file_contents) = file_contents_result {
            let api_cards: Vec<ApiCard> = serde_json::from_str(file_contents)?;
            pb.finish_and_clear();
            Ok(api_cards)
        } else {
            let result = fetch_and_store_data();
            pb.finish_and_clear();
            result
        }
    } else {
        let result = fetch_and_store_data();
        pb.finish_and_clear();
        result
    }
}

pub fn fetch_and_store_data() -> Result<Vec<ApiCard>, AppError> {
    let res = reqwest::blocking::get(CARDS_URL)?;
    debug!("Fetched HS data from {}", res.url().as_str());
    let version_number = get_version_from_response(&res)?;
    let detailed_cards: Vec<ApiCard> = res.json()?;
    match get_data_filename(&version_number) {
        Some(path) => {
            create_dir_all(path.with_file_name(""))?;
            serde_json::to_writer(File::create(&path)?, &detailed_cards)?;
            debug!("Wrote cache to {}", &path.to_string_lossy());
        }
        None => Err(AppError::CacheError {})?,
    }
    Ok(detailed_cards)
}

fn get_version_from_response(res: &reqwest::blocking::Response) -> Result<String, AppError> {
    res.url()
        .as_str()
        .rsplit_terminator('/')
        .find(|s| s.trim().parse::<f64>().is_ok())
        .map_or(Err(AppError::CacheError {}), |s| Ok(String::from(s)))
}

fn get_data_filename(version_number: &str) -> Option<PathBuf> {
    ProjectDirs::from("com.arranfrance", "", "hearthstone-to-markdown").map(|proj_dirs| {
        proj_dirs
            .config_dir()
            .join(format!("{version_number}.json"))
    })
}

#[cfg(test)]
mod tests {}
