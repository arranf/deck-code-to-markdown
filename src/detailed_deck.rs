use crate::deck_item::DeckItem;

#[derive(Clone, Debug)]
pub struct DetailedDeck {
    pub deck_items: Vec<DeckItem>,
    pub total_dust: u32,
}

impl DetailedDeck {
    pub fn new(mut deck_items: Vec<DeckItem>) -> Self {
        let total_dust: u32 = deck_items
            .iter()
            .map(|i| i.dust as u32 * i.quantity as u32)
            .sum();
        deck_items.sort_by(|a, b| {
            a.cost
                .partial_cmp(&b.cost)
                .unwrap()
                .then(a.name.cmp(&b.name))
        });
        Self {
            deck_items,
            total_dust,
        }
    }
}
