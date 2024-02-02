use rand::seq::SliceRandom;

use crate::game::card::Card;

use super::card::STANARD_DECK;

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

    pub fn shuffle_in(&mut self, new_cards: Vec<Card>) {
        let mut rng = rand::thread_rng();
        self.cards.extend(new_cards);
        self.cards.shuffle(&mut rng);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
