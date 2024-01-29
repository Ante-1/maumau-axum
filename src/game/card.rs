use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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

    pub fn is_playable_on(&self, other: &Card) -> bool {
        self.suit == other.suit || self.rank == other.rank
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

    pub fn to_card(&self) -> Result<Card, CardError> {
        let suit = match self.suit.as_str() {
            "Clubs" => Suit::Clubs,
            "Diamonds" => Suit::Diamonds,
            "Hearts" => Suit::Hearts,
            "Spades" => Suit::Spades,
            _ => return Err(CardError::InvalidSuit),
        };

        let rank = match self.rank.as_str() {
            "Ace" => Rank::Ace,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "Jack" => Rank::Jack,
            "Queen" => Rank::Queen,
            "King" => Rank::King,
            _ => return Err(CardError::InvalidRank),
        };

        Ok(Card { suit, rank })
    }
}

pub enum CardError {
    InvalidSuit,
    InvalidRank,
}
