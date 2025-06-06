use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use sqlx::postgres::PgPoolOptions;

mod auth;
mod handlers;
mod models;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL не задан");
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Ошибка подключения к базе данных");

    println!("Сервер запущен http://localhost:3000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .route("/api/esp/auth", web::post().to(handlers::auth_handler))
            .route("/api/protected", web::get().to(handlers::protected_endpoint))
            .route("/games", web::get().to(handlers::get_games))
            .route("/games", web::post().to(handlers::create_game))
            .route("/games/{id}", web::patch().to(handlers::update_game))
            .route("/status/ws", web::get().to(ws::ws_index))
            .service(web::resource("/ws/esp/{game_id}").route(web::get().to(ws::ws_index)))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
