use deck_codes::deck::Deck;
use std::collections::HashMap;

use crate::api_card::ApiCard;
use crate::deck_item::DeckItem;
use crate::detailed_deck::DetailedDeck;
use crate::error::AppError;

// Takes a set of possible HearthStone cards and matches against a decoded deck to produce a rich set of ordered cards
pub struct CardMatcher {
    pub cards: HashMap<u32, ApiCard>,
}

impl CardMatcher {
    // Creates a card matcher from a set of possible cards
    pub fn new(cards: Vec<ApiCard>) -> Self {
        let mut card_map: HashMap<u32, ApiCard> = HashMap::with_capacity(cards.len());
        for card in cards {
            card_map.insert(card.id, card);
        }

        Self { cards: card_map }
    }

    // Takes a deck and returns a set of ordered Cards
    pub fn do_match(self, deck: &Deck) -> Result<DetailedDeck, AppError> {
        let match_against = deck.cards();
        let mut deck_items: Vec<DeckItem> = Vec::new();

        for deck_card_and_quantity in match_against {
            if let Some(matching_card) = self.cards.get(&deck_card_and_quantity.1) {
                deck_items.push(DeckItem::new(matching_card, deck_card_and_quantity.0))
            }
        }
        Ok(DetailedDeck::new(deck_items))
    }
}
