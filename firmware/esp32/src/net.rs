// src/net.rs
use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Serialize};

pub fn post_json<T: Serialize, R: DeserializeOwned>(url: &str, payload: &T) -> Result<R> {
    let res = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_json(payload)?;

    if res.status() == 200 {
        let parsed: R = res.into_json()?;
        Ok(parsed)
    } else {
        Err(anyhow!("HTTP error {}", res.status()))
    }
}
