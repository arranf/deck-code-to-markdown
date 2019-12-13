use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiCard {
    #[serde(rename = "cardClass")]
    pub card_class: Option<String>,
    #[serde(default)]
    pub rarity: Rarity,
    #[serde(rename = "dbfId")]
    pub id: u32,
    pub name: String,
    pub cost: Option<u8>,
    #[serde(rename = "id")]
    pub art_id: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum Rarity {
    #[serde(rename = "LEGENDARY")]
    Legendary,
    #[serde(rename = "EPIC")]
    Epic,
    #[serde(rename = "RARE")]
    Rare,
    #[serde(rename = "COMMON")]
    Common,
    #[serde(rename = "FREE")]
    Free,
    None,
}

impl Rarity {
    pub fn dust_cost(self) -> u16 {
        match self {
            Rarity::Legendary => 1600,
            Rarity::Epic => 400,
            Rarity::Rare => 100,
            Rarity::Common => 40,
            _ => 0,
        }
    }
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::None
    }
}
