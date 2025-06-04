#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::MatchConfig;

    #[test]
    fn test_show_game_info() {
        let cfg = MatchConfig {
            game_id: "g1".into(),
            sport_type: "hockey".into(),
            team_a: "TeamA".into(),
            team_b: "TeamB".into(),
        };
        show_game_info(&cfg); // Проверяем, что не падает и печатает
    }
}
