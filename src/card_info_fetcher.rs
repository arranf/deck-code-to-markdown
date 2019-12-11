use crate::api_card::ApiCard;
use indicatif::ProgressBar;

pub fn fetch() -> Result<Vec<ApiCard>, reqwest::Error> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_message("Fetching card data...");
    let mut res = reqwest::get("https://api.hearthstonejson.com/v1/latest/enUS/cards.json")?;
    let detailed_cards: Vec<ApiCard> = res.json()?;
    debug!("Fetching HS data from {}", res.url().as_str());
    pb.finish_and_clear();
    Ok(detailed_cards)
}
