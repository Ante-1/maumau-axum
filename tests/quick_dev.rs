use anyhow::Result;
use maumau_axum::{lobby::Lobby, player::PlayerDTO};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let player_ante: PlayerDTO = hc.post("/players", json!({"name": "ante"})).await?;
    println!("player_ante_id: {}", player_ante.id);

    let player_martin: PlayerDTO = hc.post("/players", json!({"name": "martin"})).await?;
    println!("player_martin_id: {}", player_martin.id);

    let lobby: Lobby = hc
        .post(
            "/lobbies",
            json!({
                "name": "lobby 1",
            }),
        )
        .await?;
    println!("lobby_id: {}", lobby.id);

    hc.do_post(
        "/lobbies/join",
        json!({
            "playerId": player_ante.id,
            "lobbyId": lobby.id,
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/lobbies/join",
        json!({
            "playerId": player_martin.id,
            "lobbyId": lobby.id,
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/games",
        json!({
            "lobbyId": lobby.id,
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
