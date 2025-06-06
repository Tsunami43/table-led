use serde::{Serialize, Deserialize};
use crate::http_client::post_auth;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest<'a> {
    pub device_id: &'a str,
    pub token: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchConfig {
    pub game_id: String,
    pub sport_type: String,
    pub team_a: String,
    pub team_b: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub ok: bool,
    pub token: String,
    pub config: MatchConfig,
}

pub struct AuthManager {
    pub jwt_token: Option<String>,
    pub game_id: Option<String>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            jwt_token: None,
            game_id: None,
        }
    }

    pub async fn authenticate(&mut self, device_id: &str, device_token: &str) -> anyhow::Result<()> {
        let auth_req = AuthRequest {
            device_id,
            token: device_token,
        };
        let resp: AuthResponse = post_auth(&auth_req).await?;
        if resp.ok {
            self.jwt_token = Some(resp.token);
            self.game_id = Some(resp.config.game_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Auth failed"))
        }
    }
}

