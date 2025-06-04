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
