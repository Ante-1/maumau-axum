use std::time::Duration;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use card::STANARD_DECK;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

mod card;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/deck", get(deck))
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
    let deck = STANARD_DECK
        .iter()
        .map(|card| card.to_dto())
        .collect::<Vec<_>>();
    Json(deck)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
