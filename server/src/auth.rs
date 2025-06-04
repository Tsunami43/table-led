use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
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
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Ожидаем JWT в заголовке Authorization: Bearer <token>
        if let Some(authen_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = authen_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let validation = Validation::new(Algorithm::HS256);
                    match decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &validation) {
                        Ok(token_data) => {
                            return ready(Ok(AuthenticatedUser {
                                user_id: token_data.claims.sub,
                            }));
                        }
                        Err(_) => {}
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
    }
}
