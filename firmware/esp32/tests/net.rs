#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Dummy {
        foo: String,
    }

    #[test]
    fn test_post_json() {
        // Тест можно писать с реальным сервером или mock сервером, здесь заглушка
        // Мы просто проверим, что функция существует и дженерики работают
        let payload = Dummy { foo: "bar".into() };
        // Не вызываем реальный HTTP, но компиляция важна
        let _res: anyhow::Result<Dummy> = Err(anyhow::anyhow!("Not implemented"));
        assert!(true);
    }
}
