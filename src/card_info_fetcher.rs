use crate::api_card::ApiCard;

pub fn fetch() -> Result<Vec<ApiCard>, reqwest::Error> {
    println!("Fetching");
    let mut res = reqwest::get("https://api.hearthstonejson.com/v1/latest/enUS/cards.json")?;
    let detailed_cards: Vec<ApiCard> = res.json()?;
    Ok(detailed_cards)
}
