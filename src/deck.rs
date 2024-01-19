use rand::seq::SliceRandom;

use crate::card::{Card, Rank, Suit};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut deck = STANARD_DECK.clone();
        deck.shuffle(&mut rng);
        Self {
            cards: deck.to_vec(),
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_many(&mut self, n: usize) -> Option<Vec<Card>> {
        let mut cards = vec![];
        for _ in 0..n {
            match self.draw() {
                Some(card) => cards.push(card),
                None => return None,
            }
        }
        Some(cards)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
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
