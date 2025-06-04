use actix_web::{App, HttpServer, web};
mod auth;
mod handlers;
mod models;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server started on http://localhost:3000");

    HttpServer::new(|| {
        App::new()
            .route("/api/esp/auth", web::post().to(handlers::auth_handler))
            .service(
                web::resource("/ws/esp/{game_id}")
                    .route(web::get().to(ws::ws_index))
            )
            .route("/api/protected", web::get().to(handlers::protected_endpoint))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
