use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{
    game::card::{Card, CardDTO},
    game::deck::Deck,
    game::player::Player,
};

use super::{lobby::LobbyPlayer, player::PlayerDTO};

pub struct Game {
    pub id: i64,
    pub lobby_id: i64,
    deck: Deck,
    pub played_cards: Vec<Card>,
    pub current_turn_player: i64,
    pub winner: Option<i64>,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(players: Vec<LobbyPlayer>, lobby_id: i64, id: i64) -> Self {
        assert!(players.len() > 1);
        let random_player = players.choose(&mut thread_rng()).unwrap();
        let players = players
            .iter()
            .map(|player| Player::new(player.clone()))
            .collect();

        Self {
            current_turn_player: random_player.user_id,
            lobby_id,
            id,
            deck: Deck::new(),
            played_cards: vec![],
            winner: None,
            players,
        }
    }

    pub fn deck_size(&self) -> usize {
        self.deck.len()
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

    fn can_play_card(&self, card: &Card) -> bool {
        let top_card = self.played_cards.last().unwrap();
        card.is_playable_on(top_card)
    }

    pub fn play_card(&mut self, card: Card) -> Result<(), PlayCardError> {
        if !self.can_play_card(&card) {
            return Err(PlayCardError::CouldNotPlayCard);
        }
        self.played_cards.push(card);
        Ok(())
    }

    pub fn next_player(&mut self) {
        let index = self
            .players
            .iter()
            .position(|player| player.lobby_player.user_id == self.current_turn_player)
            .unwrap();
        let next_index = (index + 1) % self.players.len();
        self.current_turn_player = self.players[next_index].lobby_player.user_id;
    }

    pub fn draw_card(&mut self, player_id: i64) -> Result<(), DrawCardError> {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.lobby_player.user_id == player_id);
        let player = match player {
            Some(player) => player,
            None => return Err(DrawCardError::PlayerNotFound),
        };
        if self.deck.is_empty() {
            // shuffle in all but the top card
            self.deck
                .shuffle_in(self.played_cards[0..self.played_cards.len() - 2].to_vec());
            if self.deck.is_empty() {
                return Err(DrawCardError::NoCardsLeft);
            }
        }
        let card = self.deck.draw().unwrap();
        player.hand.push(card);
        Ok(())
    }

    pub fn draw_many_cards(&mut self, player_id: i64, n: usize) -> Result<(), DrawCardError> {
        let player = self
            .players
            .iter_mut()
            .find(|player| player.lobby_player.user_id == player_id);
        let player = match player {
            Some(player) => player,
            None => return Err(DrawCardError::PlayerNotFound),
        };
        if self.deck.len() < n {
            // shuffle in all but the top card
            self.deck
                .shuffle_in(self.played_cards[0..self.played_cards.len() - 2].to_vec());
            if self.deck.len() < n {
                return Err(DrawCardError::NoCardsLeft);
            }
        }
        let cards = self.deck.draw_many(n).unwrap();
        player.hand.extend(cards);
        Ok(())
    }
}

pub enum PlayCardError {
    CouldNotPlayCard,
}

pub enum DrawCardError {
    PlayerNotFound,
    NoCardsLeft,
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
    pub opponents: Vec<PlayerDTO>,
    pub winner: Option<i64>,
    pub deck_size: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPlayerGameStatePayload {
    pub player_id: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayCardPayload {
    pub card: CardDTO,
}
