use tungstenite::{connect, Message};
use url::Url;
use crate::display_controller::DisplayController;
use anyhow::Result;

pub struct WsClient {
    socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
}

impl WsClient {
    pub fn connect(ws_url: &str) -> Result<Self> {
        let (socket, _response) = connect(Url::parse(ws_url)?)?;
        Ok(Self { socket })
    }

    pub fn run_event_loop(&mut self, display: &mut DisplayController) -> Result<()> {
        loop {
            let msg = self.socket.read_message()?;
            if let Message::Text(text) = msg {
                // Пример обработки: обновляем табло
                display.update(&text)?;
            }
        }
    }
}

