use serde::{Deserialize, Serialize};

use crate::card::Card;

pub struct Player {
    pub id: u64,
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            hand: vec![],
        }
    }

    pub fn to_dto(&self) -> PlayerDTO {
        PlayerDTO {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PlayerDTO {
    pub name: String,
    pub id: u64,
}

#[derive(Deserialize)]
pub struct CreatePlayer {
    pub name: String,
}
