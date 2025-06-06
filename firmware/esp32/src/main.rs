mod wifi;
mod http_client;
mod auth;
mod websocket;
mod display_controller;

use crate::auth::AuthManager;
use crate::websocket::WsClient;
use crate::display_controller::DisplayController;
use esp_idf_svc::log::EspLogger;
use smol::block_on;

fn main() -> anyhow::Result<()> {
    EspLogger::initialize_default();

    block_on(async {
        // Подключаемся к Wi-Fi
        wifi::connect("your_wifi_ssid", "your_wifi_password")?;

        // Авторизация
        let mut auth_manager = AuthManager::new();
        auth_manager.authenticate("DEVICE-001", "SECRET-TOKEN").await?;

        // Инициализация табло
        let mut display = DisplayController::new();

        // Подключение к WebSocket
        let ws_url = format!("ws://yourserver:3000/ws/esp/{}", auth_manager.game_id.as_ref().unwrap());
        let mut ws_client = WsClient::connect(&ws_url)?;

        // Запуск основного цикла WebSocket
        ws_client.run_event_loop(&mut display)?;

        Ok(())
    })
}

