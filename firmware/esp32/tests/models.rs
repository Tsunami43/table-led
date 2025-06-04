#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authpayload_serde() {
        let json = r#"{"device_id":"d1","token":"t1"}"#;
        let payload: AuthPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.device_id, "d1");
        assert_eq!(payload.token, "t1");
        let ser = serde_json::to_string(&payload).unwrap();
        assert!(ser.contains("device_id"));
    }
}
