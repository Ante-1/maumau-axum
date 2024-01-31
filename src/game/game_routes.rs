use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    auth::user::{AuthSession, User},
    game::card::CardDTO,
    game::game::{
        CreateGame, CreateGameResponse, CurrentPlayerGameState, Game, Opppnent, PlayCardPayload,
    },
};

#[debug_handler]
pub async fn create_game(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateGame>,
) -> Response {
    let users: Result<Vec<User>, _> = sqlx::query_as(
        r#"
        SELECT id, username
        FROM users
        WHERE id = IN($1)
        "#,
    )
    .fetch_all(&state.db_conn_pool)
    .await;
    if users.is_err() {
        tracing::error!("error fetching users from db {}", users.unwrap_err());
        return (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response();
    }
    let users = users.unwrap();

    let lobby_id = payload.lobby_id;
    let mut games = state.get_games();
    let mut lobbies = state.get_lobbies();

    let lobby = lobbies.iter_mut().find(|lobby| lobby.id == lobby_id);

    if lobby.is_none() {
        return (StatusCode::NOT_FOUND, "lobby not found").into_response();
    }

    let lobby = lobby.unwrap();

    todo!()
    // if lobby.user_ids.len() < 2 {
    //     return (StatusCode::BAD_REQUEST, "not enough players").into_response();
    // }

    // let mut new_game_id: i64 = rand::random();

    // while games.iter().any(|game| game.id == new_game_id) {
    //     new_game_id = rand::random();
    // }

    // let mut game = Game::new(users, lobby.id, new_game_id);
    // game.give_cards();
    // game.turn_top_card();

    // games.push(game);

    // (
    //     StatusCode::CREATED,
    //     Json(CreateGameResponse {
    //         game_id: new_game_id,
    //     }),
    // )
    //     .into_response()
}

pub async fn get_game_state(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    auth_session: AuthSession,
) -> Response {
    if auth_session.user.is_none() {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    }
    let user_id = auth_session.user.unwrap().id;
    {
        let mut games = state.get_games();
        let game = games.iter_mut().find(|game| game.id == game_id);
        if game.is_none() {
            return (StatusCode::NOT_FOUND, "game not found").into_response();
        }
        let game = game.unwrap();

        let player = game
            .players
            .iter_mut()
            .find(|player| player.user.id == user_id);
        if player.is_none() {
            return (StatusCode::BAD_REQUEST, "player not found").into_response();
        }
        if !game.user_ids.contains(&user_id) {
            return (StatusCode::BAD_REQUEST, "player not in game").into_response();
        }
    }
    let mut games = state.get_games();
    let game = games.iter_mut().find(|game| game.id == game_id).unwrap();
    let player = game
        .players
        .iter()
        .find(|player| player.user.id == user_id)
        .unwrap();

    let hand: Vec<CardDTO> = player.hand.iter().map(|card| card.to_dto()).collect();
    let played_cards: Vec<CardDTO> = game.played_cards.iter().map(|card| card.to_dto()).collect();
    let opponents = game
        .user_ids
        .iter()
        .filter(|id| **id != user_id)
        .map(|id| {
            let player = game
                .players
                .iter()
                .find(|player| player.user.id == *id)
                .unwrap();
            Opppnent {
                id: player.user.id,
                name: player.user.username.clone(),
                hand_size: player.hand.len(),
            }
        })
        .collect::<Vec<_>>();

    let game_state = CurrentPlayerGameState {
        game_id,
        hand,
        current_player: game.current_turn_player,
        played_cards,
        opponents,
    };

    Json(game_state).into_response()
}

pub async fn play_card(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    auth_session: AuthSession,
    Json(payload): Json<PlayCardPayload>,
) -> Response {
    if auth_session.user.is_none() {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    }
    let user_id = auth_session.user.unwrap().id;
    let card = match payload.card.to_card() {
        Ok(card) => card,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid card").into_response(),
    };
    {
        let mut games = state.get_games();
        let game = games.iter_mut().find(|game| game.id == game_id);
        if game.is_none() {
            return (StatusCode::NOT_FOUND, "game not found").into_response();
        }
        let game = game.unwrap();
        if game.current_turn_player != user_id {
            return (StatusCode::BAD_REQUEST, "not your turn").into_response();
        }

        if !game.can_play_card(&card) {
            return (StatusCode::BAD_REQUEST, "cannot play card").into_response();
        }

        let player = game
            .players
            .iter_mut()
            .find(|player| player.user.id == user_id);
        if player.is_none() {
            return (StatusCode::BAD_REQUEST, "player not in game").into_response();
        }
        let player = player.unwrap();

        match player.remove_card(&card) {
            Ok(_) => {}
            Err(_) => return (StatusCode::BAD_REQUEST, "card not in hand").into_response(),
        }
    }

    let mut games = state.get_games();
    let game = games.iter_mut().find(|game| game.id == game_id).unwrap();

    game.play_card(card);

    let player = game
        .players
        .iter()
        .find(|player| player.user.id == user_id)
        .unwrap();

    if player.hand.is_empty() {
        game.winner = Some(user_id);
    }

    game.next_player();

    // // todo: do i want to return more?
    // // todo: do i want to make game copy and give easy acces methods to gamestate parts?
    // // todo: add play card test
    (StatusCode::OK, "card played").into_response()
}
