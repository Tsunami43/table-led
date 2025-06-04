// src/model.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthPayload {
    pub device_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub ok: bool,
    pub token: String,
    pub config: MatchConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchConfig {
    pub game_id: String,
    pub sport_type: String,
    pub team_a: String,
    pub team_b: String,
}
