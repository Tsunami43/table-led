use actix_web::{web, HttpResponse, Responder};
use crate::models::*;
use crate::auth::AuthenticatedUser;

pub async fn auth_handler(info: web::Json<AuthRequest>) -> impl Responder {
    if info.device_id == "DEVICE-001" && info.token == "SECRET-TOKEN" {
        let resp = AuthResponse {
            ok: true,
            token: "jwt_token_example".to_string(), // TODO: генерировать реальный JWT
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

pub async fn get_games(db: web::Data<sqlx::PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        Game,
        "SELECT id, team_a, team_b, sport_type, score_a, score_b, status FROM games"
    )
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_game(
    db: web::Data<sqlx::PgPool>,
    payload: web::Json<NewGame>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO games (team_a, team_b, sport_type, score_a, score_b, status)
         VALUES ($1, $2, $3, 0, 0, 'scheduled')",
        payload.team_a,
        payload.team_b,
        payload.sport_type
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_game(
    db: web::Data<sqlx::PgPool>,
    game_id: web::Path<i32>,
    data: web::Json<UpdateGame>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE games SET
            score_a = COALESCE($1, score_a),
            score_b = COALESCE($2, score_b),
            status = COALESCE($3, status)
         WHERE id = $4",
        data.score_a,
        data.score_b,
        data.status,
        *game_id
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

