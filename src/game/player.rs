use serde::{Deserialize, Serialize};

use crate::game::card::Card;

use super::lobby::LobbyPlayer;

pub struct Player {
    pub lobby_player: LobbyPlayer,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(player: LobbyPlayer) -> Self {
        Self {
            lobby_player: player,
            hand: vec![],
        }
    }

    pub fn to_dto(&self) -> PlayerDTO {
        PlayerDTO {
            user_id: self.lobby_player.user_id,
            username: self.lobby_player.username.clone(),
            hand_size: self.hand.len(),
        }
    }

    pub fn remove_card(&mut self, card: &Card) -> Result<(), PlayerError> {
        let index = match self.hand.iter().position(|c| c == card) {
            Some(index) => index,
            None => return Err(PlayerError::CardNotInHand),
        };
        self.hand.remove(index);
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PlayerDTO {
    pub username: String,
    pub user_id: i64,
    pub hand_size: usize,
}

#[derive(Deserialize)]
pub struct CreatePlayer {
    pub name: String,
}

pub enum PlayerError {
    CardNotInHand,
}
