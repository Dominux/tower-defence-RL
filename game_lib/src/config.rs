use std::collections::HashSet;

use crate::entitites::towers::Tower;

/// Game config
pub struct Config {
    pub(crate) init_money: usize,
    pub(crate) init_available_towers: HashSet<Tower>,
    pub(crate) monsters_per_wave: usize,
}

impl Config {
    pub fn new(
        init_money: usize,
        init_available_towers: HashSet<Tower>,
        monsters_per_wave: usize,
    ) -> Self {
        Self {
            init_money,
            init_available_towers,
            monsters_per_wave,
        }
    }
}
