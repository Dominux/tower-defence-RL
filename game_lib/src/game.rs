use crate::{
    config::Config,
    types::{GameData, WaveRunningResult},
};

/// Higher-level game facade for interacting with the game
///
/// Provides all the methods you need
pub struct Game<'a> {
    config: &'a Config,
    game_data: GameData,
}

impl<'a> Game<'a> {
    pub fn new(config: &'a Config) -> Self {
        let game_data = config.into();
        Self { config, game_data }
    }

    /// Main method
    ///
    /// Submit it once you are ready for the next wave
    pub fn run_next_wave(&mut self) -> WaveRunningResult {
        todo!()
    }
}
