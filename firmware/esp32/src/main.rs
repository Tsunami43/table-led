mod auth;
mod ws;
mod model;
mod display;
mod input;
mod storage;
mod net;

use esp_idf_sys as _;

#[no_mangle]
fn app_main() {
    esp_idf_sys::link_patches();

    println!("🚀 Загрузка устройства");

    let creds = storage::load_credentials().unwrap();
    let auth = auth::authenticate(&creds).expect("❌ Авторизация не прошла");

    display::init().unwrap();
    display::show_game_info(&auth.config);

    ws::start_ws_loop(&auth).expect("❌ WebSocket не работает");
}

