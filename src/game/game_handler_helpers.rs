use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::{app_state::AppState, auth::user::AuthSession, game::card::Suit};

use super::{
    card::{CardDTO, Rank, JACK_IDS, SEVENS_IDS},
    game::{Action, CurrentPlayerGameState, Game},
    player::{Player, PlayerDTO},
};

pub fn create_game(state: Arc<AppState>, lobby_id: i64) -> Result<i64, Response> {
    let mut games = state.get_games();
    let mut lobbies = state.get_lobbies();

    let lobby = match lobbies.iter_mut().find(|lobby| lobby.id == lobby_id) {
        Some(value) => value,
        None => return Err((StatusCode::NOT_FOUND, "lobby not found").into_response()),
    };

    if lobby.players.len() < 2 {
        return Err((StatusCode::BAD_REQUEST, "not enough players").into_response());
    }

    let mut new_game_id: i64 = rand::random();

    while new_game_id < 0 || games.iter().any(|game| game.id == new_game_id) {
        new_game_id = rand::random();
    }

    let mut game = Game::new(lobby.players.clone(), lobby.id, new_game_id);
    lobby.running_game = Some(new_game_id);
    game.give_cards();
    game.turn_top_card();

    games.push(game);
    Ok(new_game_id)
}

pub fn get_game_state(
    auth_session: AuthSession,
    state: Arc<AppState>,
    game_id: i64,
) -> Result<CurrentPlayerGameState, Response> {
    if auth_session.user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized").into_response());
    }
    let user_id = auth_session.user.unwrap().id;
    {
        let mut games = state.get_games();
        let game = match games.iter_mut().find(|game| game.id == game_id) {
            Some(value) => value,
            None => return Err((StatusCode::NOT_FOUND, "game not found").into_response()),
        };

        if !game
            .players
            .iter()
            .any(|player| player.lobby_player.user_id == user_id)
        {
            return Err((StatusCode::BAD_REQUEST, "player not in game").into_response());
        }
    }
    let mut games = state.get_games();
    let game = match games.iter_mut().find(|game| game.id == game_id) {
        Some(value) => value,
        None => return Err((StatusCode::NOT_FOUND, "game not found").into_response()),
    };
    let player = match game
        .players
        .iter()
        .find(|player| player.lobby_player.user_id == user_id)
    {
        Some(value) => value,
        None => return Err((StatusCode::BAD_REQUEST, "player not found").into_response()),
    };
    let hand: Vec<CardDTO> = player.hand.iter().map(|card| card.to_dto()).collect();
    let played_cards: Vec<CardDTO> = game.discard_pile.iter().map(|card| card.to_dto()).collect();
    let opponents = game
        .players
        .iter()
        .filter(|player| player.lobby_player.user_id != user_id)
        .map(|player| PlayerDTO {
            user_id: player.lobby_player.user_id,
            username: player.lobby_player.username.clone(),
            hand_size: player.hand.len(),
        })
        .collect::<Vec<_>>();

    let my_turn = game.current_turn_player == user_id;
    let viable_actions = if my_turn {
        calculate_viable_actions(player, game)
    } else {
        vec![]
    };

    let game_state = CurrentPlayerGameState {
        game_id,
        hand,
        current_player: game.current_turn_player,
        played_cards,
        opponents,
        winner: game.winner,
        deck_size: game.deck_size(),
        viable_actions,
    };
    Ok(game_state)
}

pub fn calculate_viable_actions(player: &Player, game: &Game) -> Vec<Action> {
    let playable_cards: Vec<u8> = player
        .hand
        .iter()
        .filter(|card| game.can_play_card(card))
        .map(|card| card.id)
        .collect();

    let last_action = game.actions.last().cloned();

    let viable_actions: Vec<Action> = match last_action
        .expect("there should always be a last action")
        .action
    {
        Action::PlayCard(card) => {
            if SEVENS_IDS.contains(&card) {
                let playable_sevens: Vec<u8> = player
                    .hand
                    .iter()
                    .filter(|card| card.rank == Rank::Seven)
                    .map(|card| card.id)
                    .collect();
                if !playable_sevens.is_empty() {
                    playable_sevens
                        .iter()
                        .map(|card_id| Action::PlayCard(*card_id))
                        .collect()
                } else {
                    let mut num_consecutive_sevens = 0;
                    for action in game.actions.iter().rev() {
                        if let Action::PlayCard(n) = action.action {
                            if SEVENS_IDS.contains(&n) {
                                num_consecutive_sevens += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    vec![Action::DrawCards(2 * num_consecutive_sevens)]
                }
            } else if JACK_IDS.contains(&card) {
                vec![
                    Action::DecideSuit(Suit::Hearts),
                    Action::DecideSuit(Suit::Diamonds),
                    Action::DecideSuit(Suit::Clubs),
                    Action::DecideSuit(Suit::Spades),
                ]
            }
            // eights are skipped auto matically
            else if !playable_cards.is_empty() {
                playable_cards
                    .iter()
                    .map(|card_id| Action::PlayCard(*card_id))
                    .collect()
            } else {
                vec![Action::DrawCards(1)]
            }
        }
        Action::DecideSuit(suit) => {
            let playable_cards: Vec<u8> = player
                .hand
                .iter()
                .filter(|card| card.suit == suit)
                .map(|card| card.id)
                .collect();
            if !playable_cards.is_empty() {
                playable_cards
                    .iter()
                    .map(|card_id| Action::PlayCard(*card_id))
                    .collect()
            } else {
                vec![Action::DrawCards(1)]
            }
        }
        Action::DrawCards(_) => {
            if !playable_cards.is_empty() {
                playable_cards
                    .iter()
                    .map(|card_id| Action::PlayCard(*card_id))
                    .collect()
            } else {
                vec![Action::CannotPlay]
            }
        }
        Action::CannotPlay => {
            if !playable_cards.is_empty() {
                playable_cards
                    .iter()
                    .map(|card_id| Action::PlayCard(*card_id))
                    .collect()
            } else {
                vec![Action::DrawCards(1)]
            }
        }
    };

    viable_actions
}
