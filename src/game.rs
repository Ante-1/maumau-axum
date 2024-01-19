use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;

use crate::{card::Card, deck::Deck};
pub struct Game {
    pub players_ids: Vec<u64>,
    pub lobby_id: u64,
    pub id: u64,
    pub deck: Deck,
    pub played_cards: Vec<Card>,
    pub current_player: u64,
}

impl Game {
    pub fn new(
        players_ids: Vec<u64>,
        lobby_id: u64,
        id: u64,
        deck: Deck,
        played_cards: Vec<Card>,
    ) -> Self {
        assert!(players_ids.len() > 1);
        let random_player = players_ids.choose(&mut thread_rng()).unwrap();

        Self {
            current_player: *random_player,
            players_ids,
            lobby_id,
            id,
            deck,
            played_cards,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGame {
    pub lobby_id: u64,
}
