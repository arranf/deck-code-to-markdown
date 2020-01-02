use std::fs::{create_dir_all, read_to_string, File};
use std::path::PathBuf;

use directories::ProjectDirs;
use indicatif::ProgressBar;

use crate::api_card::ApiCard;
use crate::error::AppError;

const LATEST_URL: &str = "https://api.hearthstonejson.com/v1/latest/";
const CARDS_URL: &str = "https://api.hearthstonejson.com/v1/latest/enUS/cards.json";

pub fn fetch() -> Result<Vec<ApiCard>, AppError> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_message("Fetching card data...");

    let version_res = reqwest::get(LATEST_URL)?;
    // Gets the version from the redirected URL, by splitting on slashes in reverse and ignoring the trailing slash
    let version = get_version_from_response(&version_res)?;

    // Attempt to get cached data
    if let Some(path) = get_data_filename(&version) {
        let file_contents_result = &read_to_string(path);
        match file_contents_result {
            Ok(file_contents) => {
                let api_cards: Vec<ApiCard> = serde_json::from_str(file_contents)?;
                pb.finish_and_clear();
                Ok(api_cards)
            }
            Err(_) => {
                let result = fetch_and_store_data();
                pb.finish_and_clear();
                result
            }
        }
    } else {
        let result = fetch_and_store_data();
        pb.finish_and_clear();
        result
    }
}

pub fn fetch_and_store_data() -> Result<Vec<ApiCard>, AppError> {
    let mut res = reqwest::get(CARDS_URL)?;
    debug!("Fetched HS data from {}", res.url().as_str());
    let detailed_cards: Vec<ApiCard> = res.json()?;
    let version_number = get_version_from_response(&res)?;
    match get_data_filename(&version_number) {
        Some(path) => {
            create_dir_all(&path.with_file_name(""))?;
            serde_json::to_writer(File::create(&path)?, &detailed_cards)?;
            debug!("Wrote cache to {}", &path.to_string_lossy());
        }
        None => Err(AppError::CacheError {})?,
    }
    Ok(detailed_cards)
}

fn get_version_from_response(res: &reqwest::Response) -> Result<String, AppError> {
    res.url()
        .as_str()
        .rsplit_terminator("/")
        .skip_while(|s| !s.trim().parse::<f64>().is_ok())
        .next()
        .map_or(Err(AppError::CacheError {}), |s| Ok(String::from(s)))
}

fn get_data_filename(version_number: &str) -> Option<PathBuf> {
    match ProjectDirs::from("com.arranfrance", "", "hearthstone-to-markdown") {
        Some(proj_dirs) => Some(
            proj_dirs
                .config_dir()
                .join(format!("{}.json", version_number)),
        ),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
