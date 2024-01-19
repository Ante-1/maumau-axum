use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::Clubs => "Clubs",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit)
    }
}

#[derive(Clone)]
pub enum Rank {
    Ace,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = match self {
            Rank::Ace => "Ace",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };
        write!(f, "{}", rank)
    }
}

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn _new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn to_dto(&self) -> CardDTO {
        CardDTO::new(self.clone())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardDTO {
    pub suit: String,
    pub rank: String,
}

impl CardDTO {
    pub fn new(card: Card) -> Self {
        Self {
            suit: card.suit.to_string(),
            rank: card.rank.to_string(),
        }
    }
}
