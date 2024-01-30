use serde::{Deserialize, Serialize};

use crate::{auth::user::User, game::card::Card};

pub struct Player {
    pub user: User,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(user: User) -> Self {
        Self { user, hand: vec![] }
    }

    pub fn to_dto(&self) -> PlayerDTO {
        PlayerDTO {
            user_id: self.user.id,
            username: self.user.username.clone(),
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

#[derive(Deserialize, Serialize, Clone)]
pub struct PlayerDTO {
    pub username: String,
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct CreatePlayer {
    pub name: String,
}

pub enum PlayerError {
    CardNotInHand,
}
