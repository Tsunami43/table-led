// src/storage.rs
use crate::model::AuthPayload;
use anyhow::Result;

pub fn load_credentials() -> Result<AuthPayload> {
    // В будущем заменить на NVS / Flash
    Ok(AuthPayload {
        device_id: "DEVICE-001".to_string(),
        token: "SECRET-TOKEN".to_string(),
    })
}
