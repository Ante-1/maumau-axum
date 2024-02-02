use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
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
    pub id: u8,
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        STANARD_DECK
            .iter()
            .find(|card| card.suit == suit && card.rank == rank)
            .unwrap()
            .clone()
    }

    pub fn to_dto(&self) -> CardDTO {
        CardDTO::new(self.clone())
    }

    pub fn is_playable_on(&self, other: &Card) -> bool {
        self.suit == other.suit || self.rank == other.rank
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardDTO {
    pub id: u8,
    pub suit: String,
    pub rank: String,
}

impl CardDTO {
    pub fn new(card: Card) -> Self {
        Self {
            id: card.id,
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

        Ok(Card::new(suit, rank))
    }
}

#[derive(Debug)]
pub enum CardError {
    InvalidSuit,
    InvalidRank,
}

pub const STANARD_DECK: [Card; 32] = [
    Card {
        id: 0,
        suit: Suit::Clubs,
        rank: Rank::Seven,
    },
    Card {
        id: 1,
        suit: Suit::Clubs,
        rank: Rank::Eight,
    },
    Card {
        id: 2,
        suit: Suit::Clubs,
        rank: Rank::Nine,
    },
    Card {
        id: 3,
        suit: Suit::Clubs,
        rank: Rank::Ten,
    },
    Card {
        id: 4,
        suit: Suit::Clubs,
        rank: Rank::Jack,
    },
    Card {
        id: 5,
        suit: Suit::Clubs,
        rank: Rank::Queen,
    },
    Card {
        id: 6,
        suit: Suit::Clubs,
        rank: Rank::King,
    },
    Card {
        id: 7,
        suit: Suit::Clubs,
        rank: Rank::Ace,
    },
    Card {
        id: 8,
        suit: Suit::Diamonds,
        rank: Rank::Seven,
    },
    Card {
        id: 9,
        suit: Suit::Diamonds,
        rank: Rank::Eight,
    },
    Card {
        id: 10,
        suit: Suit::Diamonds,
        rank: Rank::Nine,
    },
    Card {
        id: 11,
        suit: Suit::Diamonds,
        rank: Rank::Ten,
    },
    Card {
        id: 12,
        suit: Suit::Diamonds,
        rank: Rank::Jack,
    },
    Card {
        id: 13,
        suit: Suit::Diamonds,
        rank: Rank::Queen,
    },
    Card {
        id: 14,
        suit: Suit::Diamonds,
        rank: Rank::King,
    },
    Card {
        id: 15,
        suit: Suit::Diamonds,
        rank: Rank::Ace,
    },
    Card {
        id: 16,
        suit: Suit::Hearts,
        rank: Rank::Seven,
    },
    Card {
        id: 17,
        suit: Suit::Hearts,
        rank: Rank::Eight,
    },
    Card {
        id: 18,
        suit: Suit::Hearts,
        rank: Rank::Nine,
    },
    Card {
        id: 19,
        suit: Suit::Hearts,
        rank: Rank::Ten,
    },
    Card {
        id: 20,
        suit: Suit::Hearts,
        rank: Rank::Jack,
    },
    Card {
        id: 21,
        suit: Suit::Hearts,
        rank: Rank::Queen,
    },
    Card {
        id: 22,
        suit: Suit::Hearts,
        rank: Rank::King,
    },
    Card {
        id: 23,
        suit: Suit::Hearts,
        rank: Rank::Ace,
    },
    Card {
        id: 24,
        suit: Suit::Spades,
        rank: Rank::Seven,
    },
    Card {
        id: 25,
        suit: Suit::Spades,
        rank: Rank::Eight,
    },
    Card {
        id: 26,
        suit: Suit::Spades,
        rank: Rank::Nine,
    },
    Card {
        id: 27,
        suit: Suit::Spades,
        rank: Rank::Ten,
    },
    Card {
        id: 28,
        suit: Suit::Spades,
        rank: Rank::Jack,
    },
    Card {
        id: 29,
        suit: Suit::Spades,
        rank: Rank::Queen,
    },
    Card {
        id: 30,
        suit: Suit::Spades,
        rank: Rank::King,
    },
    Card {
        id: 31,
        suit: Suit::Spades,
        rank: Rank::Ace,
    },
];

pub const SEVENS_IDS: [u8; 4] = [0, 8, 16, 24];
pub const EIGHTS_IDS: [u8; 4] = [1, 9, 17, 25];
pub const JACK_IDS: [u8; 4] = [4, 12, 20, 28];
