use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use crate::api_card::ApiCard;

#[derive(Clone, Debug)]
pub struct DeckItem {
    pub name: String,
    pub id: u32,
    pub cost: u8,
    pub quantity: u8,
    pub art_url: String,
    pub hs_replay_url: String,
    pub wiki_url: String,
    pub dust: u16,
}

impl DeckItem {
    pub fn new(card: &ApiCard, quantity: u8) -> Self {
        Self {
            quantity: quantity,
            name: card.name.to_owned(),
            id: card.id,
            cost: card.cost.unwrap(),
            art_url: format!(
                "https://art.hearthstonejson.com/v1/render/latest/enUS/512x/{0}.png",
                card.art_id
            ),
            hs_replay_url: format!("https://hsreplay.net/cards/{0}/", card.id),
            wiki_url: format!(
                "https://hearthstone.gamepedia.com/{0}",
                utf8_percent_encode(&card.name, NON_ALPHANUMERIC).to_string()
            ),
            dust: card.rarity.dust_cost(),
        }
    }
}
