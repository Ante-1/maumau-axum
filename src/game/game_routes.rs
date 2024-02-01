use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    auth::user::AuthSession,
    game::card::CardDTO,
    game::game::{CreateGame, CreateGameResponse, CurrentPlayerGameState, Game, PlayCardPayload},
};

use super::player::PlayerDTO;

pub async fn create_game_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateGame>,
) -> Response {
    let lobby_id = payload.lobby_id;

    let new_game_id = match create_game(state, lobby_id) {
        Ok(new_game_id) => new_game_id,
        Err(error_response) => return error_response,
    };
    (
        StatusCode::CREATED,
        Json(CreateGameResponse {
            game_id: new_game_id,
        }),
    )
        .into_response()
}

pub fn create_game(state: Arc<AppState>, lobby_id: i64) -> Result<i64, Response> {
    let mut games = state.get_games();
    let lobbies = state.get_lobbies();

    let lobby = match lobbies.iter().find(|lobby| lobby.id == lobby_id) {
        Some(value) => value,
        None => return Err((StatusCode::NOT_FOUND, "lobby not found").into_response()),
    };

    if lobby.players.len() < 2 {
        return Err((StatusCode::BAD_REQUEST, "not enough players").into_response());
    }

    let mut new_game_id: i64 = rand::random();

    while games.iter().any(|game| game.id == new_game_id) {
        new_game_id = rand::random();
    }

    let mut game = Game::new(lobby.players.clone(), lobby.id, new_game_id);
    game.give_cards();
    game.turn_top_card();

    games.push(game);
    Ok(new_game_id)
}

pub async fn get_game_state_handler(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    auth_session: AuthSession,
) -> Response {
    let game_state = match get_game_state(auth_session, state, game_id) {
        Ok(value) => value,
        Err(value) => return value,
    };

    Json(game_state).into_response()
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
    let played_cards: Vec<CardDTO> = game.played_cards.iter().map(|card| card.to_dto()).collect();
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
    let game_state = CurrentPlayerGameState {
        game_id,
        hand,
        current_player: game.current_turn_player,
        played_cards,
        opponents,
        winner: game.winner,
    };
    Ok(game_state)
}

pub async fn play_card(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    auth_session: AuthSession,
    Json(payload): Json<PlayCardPayload>,
) -> Response {
    todo!()
    // if auth_session.user.is_none() {
    //     return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    // }
    // let user_id = auth_session.user.unwrap().id;
    // let card = match payload.card.to_card() {
    //     Ok(card) => card,
    //     Err(_) => return (StatusCode::BAD_REQUEST, "invalid card").into_response(),
    // };
    // {
    //     let mut games = state.get_games();
    //     let game = games.iter_mut().find(|game| game.id == game_id);
    //     if game.is_none() {
    //         return (StatusCode::NOT_FOUND, "game not found").into_response();
    //     }
    //     let game = game.unwrap();
    //     if game.current_turn_player != user_id {
    //         return (StatusCode::BAD_REQUEST, "not your turn").into_response();
    //     }

    //     if !game.can_play_card(&card) {
    //         return (StatusCode::BAD_REQUEST, "cannot play card").into_response();
    //     }

    //     let player = game
    //         .players
    //         .iter_mut()
    //         .find(|player| player.user.id == user_id);
    //     if player.is_none() {
    //         return (StatusCode::BAD_REQUEST, "player not in game").into_response();
    //     }
    //     let player = player.unwrap();

    //     match player.remove_card(&card) {
    //         Ok(_) => {}
    //         Err(_) => return (StatusCode::BAD_REQUEST, "card not in hand").into_response(),
    //     }
    // }

    // let mut games = state.get_games();
    // let game = games.iter_mut().find(|game| game.id == game_id).unwrap();

    // game.play_card(card);

    // let player = game
    //     .players
    //     .iter()
    //     .find(|player| player.user.id == user_id)
    //     .unwrap();

    // if player.hand.is_empty() {
    //     game.winner = Some(user_id);
    // }

    // game.next_player();

    // // // todo: do i want to return more?
    // // // todo: do i want to make game copy and give easy acces methods to gamestate parts?
    // // // todo: add play card test
    // (StatusCode::OK, "card played").into_response()
}
