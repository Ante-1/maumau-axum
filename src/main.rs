use std::{sync::Arc, time::Duration};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use deck::Deck;
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

    let mut players = state.players.lock().expect("mutex was poisoned");

    players.push(Player {
        id: random_id,
        name: payload.name.clone(),
    });

    let user = Player {
        id: random_id,
        name: payload.name,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
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

fn player_id_exists(state: &Arc<AppState>, random_id: u64) -> bool {
    state
        .players
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .any(|p| p.id == random_id)
}
