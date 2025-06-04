use actix_web::{web, HttpResponse, Responder};
use crate::models::{AuthRequest, AuthResponse, MatchConfig};
use crate::auth::AuthenticatedUser;

pub async fn auth_handler(info: web::Json<AuthRequest>) -> impl Responder {
    // TODO: проверка device_id и token из БД
    if info.device_id == "DEVICE-001" && info.token == "SECRET-TOKEN" {
        let resp = AuthResponse {
            ok: true,
            token: "jwt_token_example".to_string(),
            config: MatchConfig {
                game_id: "game123".to_string(),
                sport_type: "soccer".to_string(),
                team_a: "Team A".to_string(),
                team_b: "Team B".to_string(),
            },
        };
        HttpResponse::Ok().json(resp)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub async fn protected_endpoint(user: AuthenticatedUser) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello user: {}", user.user_id))
}
