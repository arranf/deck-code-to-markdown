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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legendary_dust_cost() {
        let card = ApiCard {
            rarity: Rarity::Legendary,
            id: 1,
            name: String::from("Test Card"),
            art_id: String::from("Unknown"),
            cost: None,
            card_class: None,
        };

        assert_eq!(card.rarity.dust_cost(), 1600);
    }

    #[test]
    fn test_epic_dust_cost() {
        let card = ApiCard {
            rarity: Rarity::Epic,
            id: 1,
            name: String::from("Test Card"),
            art_id: String::from("Unknown"),
            cost: None,
            card_class: None,
        };

        assert_eq!(card.rarity.dust_cost(), 400);
    }

    #[test]
    fn test_rare_dust_cost() {
        let card = ApiCard {
            rarity: Rarity::Rare,
            id: 1,
            name: String::from("Test Card"),
            art_id: String::from("Unknown"),
            cost: None,
            card_class: None,
        };

        assert_eq!(card.rarity.dust_cost(), 100);
    }

    #[test]
    fn test_common_dust_cost() {
        let card = ApiCard {
            rarity: Rarity::Common,
            id: 1,
            name: String::from("Test Card"),
            art_id: String::from("Unknown"),
            cost: None,
            card_class: None,
        };

        assert_eq!(card.rarity.dust_cost(), 40);
    }

    #[test]
    fn test_default_dust_cost() {
        let card = ApiCard {
            rarity: Rarity::default(),
            id: 1,
            name: String::from("Test Card"),
            art_id: String::from("Unknown"),
            cost: None,
            card_class: None,
        };

        assert_eq!(card.rarity.dust_cost(), 0);
    }
}
