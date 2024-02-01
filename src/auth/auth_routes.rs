use std::sync::Arc;

use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_messages::{Message, Messages};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    auth::user::{AuthSession, Credentials},
};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    messages: Vec<Message>,
    next: Option<String>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    messages: Vec<Message>,
    next: Option<String>,
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
        .route("/register", get(self::get::register))
        .route("/register", post(self::post::register))
}

mod post {
    use crate::auth::user::User;

    use super::*;

    pub async fn login(
        mut auth_session: AuthSession,
        messages: Messages,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                messages.error("Invalid credentials");

                let mut login_url = "/login".to_string();
                if let Some(next) = creds.next {
                    login_url = format!("{}?next={}", login_url, next);
                };

                return Redirect::to(&login_url).into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        messages.success(format!("Successfully logged in as {}", user.username));

        if let Some(ref next) = creds.next {
            Redirect::to(next)
        } else {
            Redirect::to("/")
        }
        .into_response()
    }

    pub async fn register(
        auth_session: AuthSession,
        messages: Messages,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        let users: Vec<User> = match sqlx::query_as("select * from users")
            .fetch_all(auth_session.backend.db())
            .await
        {
            Ok(users) => users,
            Err(_) => {
                messages.error("Failed to register");
                return Redirect::to("/register").into_response();
            }
        };
        if users.iter().any(|user| user.username == creds.username) {
            messages.error("Username already taken");
            return Redirect::to("/register").into_response();
        }

        let user = sqlx::query("insert into users (username, password) values (?, ?)")
            .bind(creds.username.clone())
            .bind(password_auth::generate_hash(creds.password))
            .execute(auth_session.backend.db())
            .await;
        match user {
            Ok(_) => {
                messages.success(format!("Successfully registered as {}", creds.username));
                Redirect::to("/login").into_response()
            }
            Err(_) => {
                messages.error("Failed to register");
                Redirect::to("/register").into_response()
            }
        }
    }
}

mod get {
    use super::*;

    pub async fn login(
        messages: Messages,
        Query(NextUrl { next }): Query<NextUrl>,
    ) -> LoginTemplate {
        LoginTemplate {
            messages: messages.into_iter().collect(),
            next,
        }
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn register(
        messages: Messages,
        Query(NextUrl { next }): Query<NextUrl>,
    ) -> RegisterTemplate {
        RegisterTemplate {
            messages: messages.into_iter().collect(),
            next,
        }
    }
}
