#![warn(clippy::all, clippy::pedantic)]

use anyhow::Error;
use deck_codes::decode_deck_code;
use log::{debug, info};
use std::io::{self};

mod api_card;
mod card_info_fetcher;
mod card_matcher;
mod deck_item;
mod detailed_deck;
mod error;

use crate::card_matcher::CardMatcher;
use crate::detailed_deck::DetailedDeck;

#[paw::main]
fn main(args: paw::Args) -> Result<(), Error> {
    env_logger::init();
    let mut args = args.skip(1);

    let deck_code = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "The deck code argument is missing"))?;
    debug!("Decoding deck");
    let deck = decode_deck_code(&deck_code)?;
    info!("Getting card info");
    let api_cards = card_info_fetcher::fetch()?;
    debug!("Matching cards against API response");
    let detailed_deck = CardMatcher::new(api_cards).do_match(&deck)?;
    println!("{}", format_as_markdown(&detailed_deck, &deck_code));
    Ok(())
}

fn format_as_markdown(detailed_deck: &DetailedDeck, deck_code: &str) -> String {
    let table_header = r"| Mana | Card Name                                                    | Qty  |                            Links                             |
| :--: | :----------------------------------------------------------- | :--: | :----------------------------------------------------------: |
";
    let mut table_lines = Vec::with_capacity(detailed_deck.deck_items.len() + 4);
    table_lines.push(table_header.to_owned());
    for card in &detailed_deck.deck_items {
        let formatted_line = format!(
            "|  {0}  | [{1}]({2}) |  {3}  | [HSReplay]({4}),[Wiki]({5}) |\n",
            card.cost, card.name, card.art_url, card.quantity, card.hs_replay_url, card.wiki_url
        );
        table_lines.push(formatted_line);
    }
    table_lines.push("\n".to_owned());
    table_lines.push(format!("**Total Dust:**: {}\n", detailed_deck.total_dust));
    table_lines.push(format!("**Deck Code:** `{deck_code}`\n"));
    table_lines.concat()
}
