use std::{sync::Arc, time::Duration};

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use deck::Deck;
use lobby::{CreateLobby, JoinLobby, Lobby};
use player::{CreatePlayer, Player};
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;

mod app_state;
mod card;
mod deck;
mod game;
mod lobby;
mod player;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(root))
        .route("/players", post(create_player))
        .route("/players", get(get_players))
        .route("/deck", get(deck))
        .route("/lobbies", post(create_lobby))
        .route("/lobbies", get(get_lobbies))
        .route("/lobbies/join", post(join_lobby))
        .with_state(app_state)
        // middlewares
        .layer(
            ServiceBuilder::new()
                // layer to log all incoming requests
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        );

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn deck() -> impl IntoResponse {
    let deck = Deck::new()
        .cards
        .iter()
        .map(|card| card.to_dto())
        .collect::<Vec<_>>();
    Json(deck)
}

async fn create_player(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePlayer>,
) -> impl IntoResponse {
    let mut random_id: u64 = rand::random();
    while player_id_exists(&state, random_id) {
        random_id = rand::random();
    }

    let player = Player {
        id: random_id,
        name: payload.name,
    };

    let mut players = state.players.lock().expect("mutex was poisoned");

    players.push(player.clone());

    (StatusCode::CREATED, Json(player))
}

async fn get_players(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let players: Vec<Player> = state
        .players
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .cloned()
        .collect();

    Json(players)
}

#[debug_handler]
async fn create_lobby(
    State(state): State<Arc<AppState>>,
    Json(playload): Json<CreateLobby>,
) -> impl IntoResponse {
    let mut random_id: u64 = rand::random();
    while lobby_id_exists(&state, random_id) {
        random_id = rand::random();
    }

    let lobby = Lobby {
        id: random_id,
        name: playload.name,
        player_ids: vec![],
    };

    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    lobbies.push(lobby.clone());

    (StatusCode::CREATED, Json(lobby))
}

async fn get_lobbies(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lobbies: Vec<Lobby> = state
        .lobbies
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .cloned()
        .collect();

    Json(lobbies)
}

async fn join_lobby(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JoinLobby>,
) -> impl IntoResponse {
    let lobby_id = payload.lobby_id;
    let player_id = payload.player_id;
    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    let players = state.players.lock().expect("mutex was poisoned");

    let lobby = lobbies.iter_mut().find(|lobby| lobby.id == lobby_id);

    if lobby.is_none() {
        return (StatusCode::NOT_FOUND, "lobby not found");
    }

    let player = players.iter().find(|player| player.id == player_id);

    if player.is_none() {
        return (StatusCode::NOT_FOUND, "player not found");
    }

    lobby.unwrap().player_ids.push(player_id);
    (StatusCode::OK, "player joined lobby")
}

fn player_id_exists(state: &Arc<AppState>, random_id: u64) -> bool {
    state
        .players
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .any(|p| p.id == random_id)
}

fn lobby_id_exists(state: &Arc<AppState>, random_id: u64) -> bool {
    state
        .lobbies
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .any(|p| p.id == random_id)
}
