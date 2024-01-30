use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{
    auth::user::User,
    game::card::{Card, CardDTO},
    game::deck::Deck,
    game::player::Player,
};

pub struct Game {
    pub id: i64,
    pub user_ids: Vec<i64>,
    pub lobby_id: i64,
    pub deck: Deck,
    pub played_cards: Vec<Card>,
    pub current_turn_player: i64,
    pub winner: Option<i64>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(users: Vec<User>, lobby_id: i64, id: i64) -> Self {
        assert!(users.len() > 1);
        let random_player = users.choose(&mut thread_rng()).unwrap();
        let user_ids = users.iter().map(|user| user.id).collect();
        let players = users.iter().map(|user| Player::new(user.clone())).collect();

        Self {
            current_turn_player: random_player.id,
            user_ids,
            lobby_id,
            id,
            deck: Deck::new(),
            played_cards: vec![],
            winner: None,
            players,
        }
    }

    pub fn give_cards(&mut self) {
        for player in &mut self.players {
            let new_hand = self.deck.draw_many(5).unwrap();
            player.hand.extend(new_hand);
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
            .user_ids
            .iter()
            .position(|id| *id == self.current_turn_player)
            .unwrap();
        let next_index = (index + 1) % self.user_ids.len();
        self.current_turn_player = self.user_ids[next_index];
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGame {
    pub lobby_id: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameResponse {
    pub game_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPlayerGameState {
    pub game_id: i64,
    pub hand: Vec<CardDTO>,
    pub current_player: i64,
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
    pub id: i64,
    pub name: String,
    pub hand_size: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayCardPayload {
    pub card: CardDTO,
}
