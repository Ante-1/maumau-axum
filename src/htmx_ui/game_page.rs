use askama::Template;
use serde::Deserialize;

use crate::game::{card::CardDTO, game::Action, player::PlayerDTO};

#[derive(Template, Debug)]
#[template(path = "game.html")]
pub struct GameTemplate {
    my_hand: Vec<CardDTO>,
    other_players: Vec<PlayerDTO>,
    current_turn_player: i64,
    is_my_turn: bool,
    winner: Option<PlayerDTO>,
    last_played_card: CardDTO,
    num_cards_in_deck: usize,
    num_cards_played: usize,
    viable_actions: ActionsDTO,
    handle_action_route: String,
}

#[derive(Debug)]
pub struct ActionsDTO {
    pub playable_cards: Vec<u8>,
    pub draw_cards: Option<u8>,
    pub decide_suit: bool,
    pub end_turn: bool,
}

impl From<Vec<Action>> for ActionsDTO {
    fn from(actions: Vec<Action>) -> Self {
        let mut playable_cards = vec![];
        let mut draw_cards = None;
        let mut decide_suit = false;
        let mut end_turn = false;
        for action in actions {
            match action {
                Action::PlayCard(card_id) => playable_cards.push(card_id),
                Action::DrawCards(n) => draw_cards = Some(n),
                Action::DecideSuit(_) => decide_suit = true,
                Action::CannotPlay => end_turn = true,
            }
        }
        Self {
            playable_cards,
            draw_cards,
            decide_suit,
            end_turn,
        }
    }
}

#[derive(Deserialize)]
pub struct StartGameParams {
    pub lobby_id: i64,
}

pub mod get {
    use std::sync::Arc;

    use askama_axum::IntoResponse;
    use axum::extract::{Path, State};
    use axum::response::Response;

    use crate::game::game_handler_helpers::get_game_state;
    use crate::{app_state::AppState, auth::user::AuthSession, game::player::PlayerDTO};

    use super::GameTemplate;

    pub async fn game_handler(
        auth_session: AuthSession,
        Path(game_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> Response {
        let current_player_game_state = match get_game_state(auth_session.clone(), state, game_id) {
            Ok(value) => value,
            Err(value) => return value,
        };
        let current_turn_player = current_player_game_state.current_player;
        let is_my_turn = current_turn_player == auth_session.user.unwrap().id;
        let winner: Option<PlayerDTO> = current_player_game_state.winner.map(|winner_id| {
            let winner = current_player_game_state
                .opponents
                .iter()
                .find(|player| player.user_id == winner_id)
                .unwrap();
            PlayerDTO {
                user_id: winner.user_id,
                username: winner.username.clone(),
                hand_size: 0,
            }
        });
        let num_cards_in_deck = current_player_game_state.deck_size;
        let num_cards_played = current_player_game_state.played_cards.len();

        let game_template = GameTemplate {
            my_hand: current_player_game_state.hand,
            other_players: current_player_game_state.opponents,
            current_turn_player,
            is_my_turn,
            winner,
            last_played_card: current_player_game_state
                .played_cards
                .last()
                .unwrap()
                .clone(),
            handle_action_route: format!("/games/{}/handle-action", game_id),
            num_cards_in_deck,
            num_cards_played,
            viable_actions: current_player_game_state.viable_actions.into(),
        };
        game_template.into_response()
    }
}

pub mod post {
    use std::sync::Arc;

    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::Form;

    use crate::app_state::AppState;
    use crate::game::game_handler_helpers::create_game;

    use super::StartGameParams;

    pub async fn create_game_handler(
        State(state): State<Arc<AppState>>,
        Form(params): Form<StartGameParams>,
    ) -> impl IntoResponse {
        let new_game_id = match create_game(state, params.lobby_id) {
            Ok(new_game_id) => new_game_id,
            Err(error_response) => return error_response,
        };
        (
            [("HX-Redirect", format!("/games/{}", new_game_id))],
            StatusCode::CREATED,
        )
            .into_response()
    }
}
