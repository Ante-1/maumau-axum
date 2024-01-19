use crate::{card::Card, deck::Deck};
pub struct Game {
    pub players_ids: Vec<u64>,
    pub lobby_id: u64,
    pub id: u64,
    pub deck: Deck,
    pub played_cards: Vec<Card>,
    pub current_player: u64,
}
