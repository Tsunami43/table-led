// src/ws.rs
use crate::model::AuthResponse;
use std::thread;
use ws::{connect, Handler, Handshake, Message, Result as WsResult, Sender};

pub fn start_ws_loop(auth: &AuthResponse) -> anyhow::Result<()> {
    let url = format!(
        "ws://192.168.0.100:3000/ws/esp/{}?token={}",
        auth.config.game_id, auth.token
    );

    connect(url, |out| WsClient { out })?;
    Ok(())
}

struct WsClient {
    out: Sender,
}

impl Handler for WsClient {
    fn on_message(&mut self, msg: Message) -> WsResult<()> {
        println!("📡 WebSocket message: {}", msg);
        // TODO: parse and call display::update_from_server()
        Ok(())
    }
}
