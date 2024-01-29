use std::{sync::Arc, time::Duration};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use maumau_axum::{
    app_state::AppState,
    db::db,
    game_routes::{create_game, get_game_state, play_card},
    lobby_routes::{create_lobby, get_lobbies, join_lobby},
    player_routes::{create_player, get_players},
};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();
    let pool = db().await;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("could not run SQLx migrations");

    let app_state = Arc::new(AppState::new(pool));

    let api_routes = Router::new()
        .route("/players", post(create_player))
        .route("/players", get(get_players))
        .route("/lobbies", post(create_lobby))
        .route("/lobbies", get(get_lobbies))
        .route("/lobbies/join", post(join_lobby))
        .route("/games", post(create_game))
        .route("/games/:game_id", post(get_game_state))
        .route("/games/:game_id/play-card", post(play_card));
    let app = Router::new()
        .route("/", get(root))
        .route("/db", get(using_connection_pool_extractor))
        .nest("/api", api_routes)
        .nest_service("/assets", ServeDir::new("assets"))
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

async fn using_connection_pool_extractor(
    State(app_state): State<Arc<AppState>>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from sqlite'")
        .fetch_one(&app_state.pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_string(),
            )
        })
}
