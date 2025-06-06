use crate::auth::AuthRequest;
use embedded_svc::http::client::{Client, Request, Response};
use serde::{Serialize, Deserialize};
use esp_idf_svc::http::client::EspHttpConnection;

pub async fn post_auth(auth_req: &AuthRequest) -> anyhow::Result<super::auth::AuthResponse> {
    let http_client = EspHttpConnection::new()?;
    let url = "http://yourserver:3000/api/esp/auth";

    let body = serde_json::to_string(auth_req)?;
    let mut req = http_client.post(url)?;
    req.header("Content-Type", "application/json")?;
    req.body(body)?;

    let mut resp = req.execute()?;
    let auth_resp = serde_json::from_reader(&mut resp)?;
    Ok(auth_resp)
}
