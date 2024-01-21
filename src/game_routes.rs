use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    card::CardDTO,
    game::{
        CreateGame, CreateGameResponse, CurrentPlayerGameState, CurrentPlayerGameStatePayload,
        Game, Opppnent, PlayCardPayload,
    },
};

pub async fn create_game(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateGame>,
) -> Response {
    let lobby_id = payload.lobby_id;
    let mut games = state.get_games();
    let mut players = state.get_players();
    let mut lobbies = state.get_lobbies();

    let lobby = lobbies.iter_mut().find(|lobby| lobby.id == lobby_id);

    if lobby.is_none() {
        return (StatusCode::NOT_FOUND, "lobby not found").into_response();
    }

    let lobby = lobby.unwrap();

    if lobby.player_ids.len() < 2 {
        return (StatusCode::BAD_REQUEST, "not enough players").into_response();
    }

    let mut random_id: u64 = rand::random();

    while games.iter().any(|game| game.id == random_id) {
        random_id = rand::random();
    }

    let mut game = Game::new(lobby.player_ids.clone(), lobby.id, random_id);
    game.give_cards(&mut players);
    game.turn_top_card();

    games.push(game);

    (
        StatusCode::CREATED,
        Json(CreateGameResponse { game_id: random_id }),
    )
        .into_response()
}

pub async fn get_game_state(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<u64>,
    Json(payload): Json<CurrentPlayerGameStatePayload>,
) -> Response {
    let player_id = payload.player_id;

    let games = state.get_games();
    let game = games.iter().find(|game| game.id == game_id);
    if game.is_none() {
        return (StatusCode::NOT_FOUND, "game not found").into_response();
    }
    let game = game.unwrap();

    let players = state.get_players();
    let player = players.iter().find(|player| player.id == player_id);
    if player.is_none() {
        return (StatusCode::BAD_REQUEST, "player not found").into_response();
    }
    if !game.player_ids.contains(&player_id) {
        return (StatusCode::BAD_REQUEST, "player not in game").into_response();
    }
    let player = player.unwrap();

    let opponents = game
        .player_ids
        .iter()
        .filter(|id| **id != player_id)
        .map(|id| {
            let player = players.iter().find(|player| player.id == *id).unwrap();
            Opppnent {
                id: player.id,
                name: player.name.clone(),
                hand_size: player.hand.len(),
            }
        })
        .collect::<Vec<_>>();

    let hand: Vec<CardDTO> = player.hand.iter().map(|card| card.to_dto()).collect();
    let played_cards: Vec<CardDTO> = game.played_cards.iter().map(|card| card.to_dto()).collect();

    let game_state = CurrentPlayerGameState {
        id: game.id,
        hand,
        current_player: game.current_player,
        played_cards,
        opponents,
    };

    Json(game_state).into_response()
}

pub async fn play_card(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<u64>,
    Json(payload): Json<PlayCardPayload>,
) -> Response {
    let player_id = payload.player_id;

    let mut games = state.get_games();
    let game = games.iter_mut().find(|game| game.id == game_id);
    if game.is_none() {
        return (StatusCode::NOT_FOUND, "game not found").into_response();
    }
    let game = game.unwrap();

    let mut players = state.get_players();
    let player = players.iter_mut().find(|player| player.id == player_id);
    if player.is_none() {
        return (StatusCode::BAD_REQUEST, "player not found").into_response();
    }
    if !game.player_ids.contains(&player_id) {
        return (StatusCode::BAD_REQUEST, "player not in game").into_response();
    }
    let player = player.unwrap();

    if game.current_player != player_id {
        return (StatusCode::BAD_REQUEST, "not your turn").into_response();
    }

    let card = match payload.card.to_card() {
        Ok(card) => card,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid card").into_response(),
    };

    if !game.can_play_card(&card) {
        return (StatusCode::BAD_REQUEST, "cannot play card").into_response();
    }

    match player.remove_card(&card) {
        Ok(_) => {}
        Err(_) => return (StatusCode::BAD_REQUEST, "card not in hand").into_response(),
    }
    game.play_card(card);

    if player.hand.is_empty() {
        game.winner = Some(player_id);
    }

    game.next_player();

    // todo: do i want to return more?
    // todo: do i want to make game copy and give easy acces methods to gamestate parts?
    // todo: add play card test
    (StatusCode::OK, "card played").into_response()
}
