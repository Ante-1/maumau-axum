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

#[derive(Serialize, Deserialize)]
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

pub const STANARD_DECK: [Card; 32] = [
    Card {
        suit: Suit::Clubs,
        rank: Rank::Seven,
    },
    Card {
        suit: Suit::Clubs,
        rank: Rank::Eight,
    },
    Card {
        suit: Suit::Clubs,
        rank: Rank::Nine,
    },
    Card {
        suit: Suit::Clubs,
        rank: Rank::Ten,
    },
    Card {
        suit: Suit::Clubs,
        rank: Rank::Jack,
    },
    Card {
        suit: Suit::Clubs,
        rank: Rank::Queen,
    },
    Card {
        suit: Suit::Clubs,
        rank: Rank::King,
    },
    Card {
        suit: Suit::Clubs,
        rank: Rank::Ace,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::Seven,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::Eight,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::Nine,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::Ten,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::Jack,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::Queen,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::King,
    },
    Card {
        suit: Suit::Diamonds,
        rank: Rank::Ace,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::Seven,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::Eight,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::Nine,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::Ten,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::Jack,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::Queen,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::King,
    },
    Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::Seven,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::Eight,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::Nine,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::Ten,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::Jack,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::Queen,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::King,
    },
    Card {
        suit: Suit::Spades,
        rank: Rank::Ace,
    },
];
