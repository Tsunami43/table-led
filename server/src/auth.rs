use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::FromRequest;
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"some_secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthenticatedUser {
    pub user_id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let validation = Validation::new(Algorithm::HS256);
                    if let Ok(token_data) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(SECRET),
                        &validation,
                    ) {
                        return ready(Ok(AuthenticatedUser {
                            user_id: token_data.claims.sub,
                        }));
                    }
                }
            }
        }
    ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
    }
}

// POST /auth
// {
//   "device_id": "DEVICE-001",
//   "token": "SECRET-TOKEN"
// }
//
// Response
// {
//   "ok": true,
//   "token": "jwt_token_example",
//   "config": {
//     "game_id": "game123",
//     "sport_type": "soccer",
//     "team_a": "Team A",
//     "team_b": "Team B"
//   }
// }
