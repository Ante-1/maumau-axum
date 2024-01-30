use anyhow::Result;
use maumau_axum::game::{
    game::{CreateGameResponse, CurrentPlayerGameState},
    lobby::Lobby,
    player::PlayerDTO,
};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000/api")?;

    Ok(())
}
