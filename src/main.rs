use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use axum_login::{
    login_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use axum_messages::MessagesManagerLayer;
use maumau_axum::{
    app_state::AppState,
    auth::auth_routes,
    auth::user::{Backend, User},
    db::db,
    game,
};
use time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tower_sessions_sqlx_store::SqliteStore;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let pool = db().await;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("could not run SQLx migrations");

    let app_state = Arc::new(AppState::new(pool.clone()));
    let session_store = SqliteStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("could not run Session SQLx migrations");

    tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = Backend::new(pool);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .nest("/api", game::router::game_router())
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/users", get(users))
        .route("/", get(root))
        .route("/db", get(using_connection_pool_extractor))
        .merge(auth_routes::router())
        .layer(MessagesManagerLayer)
        .layer(auth_layer)
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(app_state)
        // middlewares
        .layer(
            ServiceBuilder::new()
                // layer to log all incoming requests
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(5))),
        );

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn using_connection_pool_extractor(
    State(app_state): State<Arc<AppState>>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from sqlite'")
        .fetch_one(&app_state.db_conn_pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_string(),
            )
        })
}

async fn users(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let users: Vec<User> = sqlx::query_as("select * from users")
        .fetch_all(&app_state.db_conn_pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to execute query: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_string(),
            )
        })
        .expect("asd");
    Json(users)
}
