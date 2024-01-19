use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{
    card::{Card, CardDTO},
    deck::Deck,
    player::Player,
};

pub struct Game {
    pub id: u64,
    pub player_ids: Vec<u64>,
    pub lobby_id: u64,
    pub deck: Deck,
    pub played_cards: Vec<Card>,
    pub current_player: u64,
}

impl Game {
    pub fn new(players_ids: Vec<u64>, lobby_id: u64, id: u64) -> Self {
        assert!(players_ids.len() > 1);
        let random_player = players_ids.choose(&mut thread_rng()).unwrap();

        Self {
            current_player: *random_player,
            player_ids: players_ids,
            lobby_id,
            id,
            deck: Deck::new(),
            played_cards: vec![],
        }
    }

    pub fn give_cards(&mut self, players: &mut [Player]) {
        for player_id in &self.player_ids {
            let cards = self.deck.draw_many(5).unwrap();
            let player = players
                .iter_mut()
                .find(|player| player.id == *player_id)
                .unwrap();
            player.hand.extend(cards);
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGame {
    pub lobby_id: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameResponse {
    pub game_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPlayerGameState {
    pub id: u64,
    pub hand: Vec<CardDTO>,
    pub current_player: u64,
    pub played_cards: Vec<CardDTO>,
    pub opponents: Vec<Opppnent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPlayerGameStatePayload {
    pub player_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Opppnent {
    pub id: u64,
    pub name: String,
    pub hand_size: usize,
}
