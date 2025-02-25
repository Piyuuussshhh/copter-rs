use std::time::Duration;

use bevy::prelude::*;

use crate::constants;

// Resource for tracking game state
#[derive(Resource)]
pub struct GameState {
    pub score: f32,
    pub game_over: bool,
    pub obstacle_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0.0,
            game_over: false,
            obstacle_timer: Timer::new(
                Duration::from_secs_f32(constants::OBSTACLE_SPAWN_TIME),
                TimerMode::Repeating,
            )
        }
    }
}

