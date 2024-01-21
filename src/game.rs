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
    pub winner: Option<u64>,
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
            winner: None,
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

    pub fn turn_top_card(&mut self) {
        let card = self.deck.draw().unwrap();
        self.played_cards.push(card);
    }

    pub fn can_play_card(&self, card: &Card) -> bool {
        let top_card = self.played_cards.last().unwrap();
        card.is_playable_on(top_card)
    }

    pub fn play_card(&mut self, card: Card) {
        self.played_cards.push(card);
    }

    pub fn next_player(&mut self) {
        let index = self
            .player_ids
            .iter()
            .position(|id| *id == self.current_player)
            .unwrap();
        let next_index = (index + 1) % self.player_ids.len();
        self.current_player = self.player_ids[next_index];
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayCardPayload {
    pub player_id: u64,
    pub card: CardDTO,
}
