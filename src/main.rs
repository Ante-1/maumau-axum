use std::{sync::Arc, time::Duration};

use axum::{
    routing::{get, post},
    Router,
};
use maumau_axum::{
    app_state::AppState,
    game_routes::{create_game, get_games},
    lobby_routes::{create_lobby, get_lobbies, join_lobby},
    player_routes::{create_player, get_players},
};
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

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
        .route("/lobbies", post(create_lobby))
        .route("/lobbies", get(get_lobbies))
        .route("/lobbies/join", post(join_lobby))
        .route("/games", post(create_game))
        .route("/games", get(get_games))
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
