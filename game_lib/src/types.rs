use std::collections::HashSet;

use crate::{config::Config, entitites::towers::Tower};

pub struct GameData {
    money: usize,
    available_towers: HashSet<Tower>,
}

impl From<&Config> for GameData {
    fn from(config: &Config) -> Self {
        Self {
            money: config.init_money,
            available_towers: config.init_available_towers.iter().cloned().collect(),
        }
    }
}

pub enum WaveRunningResult {
    ReadyToNextWave,
    GameOver,
}
