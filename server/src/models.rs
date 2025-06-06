use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub device_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub ok: bool,
    pub token: String,
    pub config: MatchConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchConfig {
    pub game_id: String,
    pub sport_type: String,
    pub team_a: String,
    pub team_b: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Game {
    pub id: i32,
    pub team_a: String,
    pub team_b: String,
    pub sport_type: String,
    pub score_a: i32,
    pub score_b: i32,
    pub status: String,
}

#[derive(Deserialize)]
pub struct NewGame {
    pub team_a: String,
    pub team_b: String,
    pub sport_type: String,
}

#[derive(Deserialize)]
pub struct UpdateGame {
    pub score_a: Option<i32>,
    pub score_b: Option<i32>,
    pub status: Option<String>,
}

