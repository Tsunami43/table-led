// src/display.rs
use crate::model::MatchConfig;
use anyhow::Result;

pub fn init() -> Result<()> {
    println!("🖥️ Инициализация дисплея (заглушка)");
    Ok(())
}

pub fn show_game_info(config: &MatchConfig) {
    println!("🏟️ Матч: {} vs {}", config.team_a, config.team_b);
    println!("🎮 Вид спорта: {}", config.sport_type);
}
