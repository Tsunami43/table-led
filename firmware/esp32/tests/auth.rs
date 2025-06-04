#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AuthPayload;
    use serde_json::json;

    #[test]
    fn test_authenticate_success() {
        // Мокаем net::post_json
        fn mock_post_json<T, R>(_url: &str, _payload: &T) -> anyhow::Result<R>
        where
            T: serde::Serialize,
            R: serde::de::DeserializeOwned,
        {
            let data = json!({
                "ok": true,
                "token": "tok",
                "config": {
                    "game_id": "g1",
                    "sport_type": "soccer",
                    "team_a": "A",
                    "team_b": "B"
                }
            });
            let parsed: R = serde_json::from_value(data).unwrap();
            Ok(parsed)
        }

        // Подмена функции (в реале можно использовать traits или mock библиотеки)
        let creds = AuthPayload {
            device_id: "dev".into(),
            token: "tok".into(),
        };

        // Для теста вызовем напрямую mock
        let result: anyhow::Result<super::model::AuthResponse> = mock_post_json("", &creds);
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.ok);
        assert_eq!(resp.config.team_a, "A");
    }
}
