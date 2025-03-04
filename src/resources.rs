use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::{BORDERTILE_SPAWN_TIME, DEFAULT_BORDERTILE_HEIGHT, OBSTACLE_SPAWN_TIME},
    enums::FluctuatingDirection,
};

// Resource for tracking game state
#[derive(Resource)]
pub struct GameState {
    pub score: f32,
    pub game_over: bool,
    pub obstacle_timer: Timer,
    pub bordertile_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0.0,
            game_over: false,
            obstacle_timer: Timer::new(
                Duration::from_secs_f32(OBSTACLE_SPAWN_TIME),
                TimerMode::Repeating,
            ),
            bordertile_timer: Timer::new(
                Duration::from_secs_f32(BORDERTILE_SPAWN_TIME),
                TimerMode::Repeating,
            ),
        }
    }
}

#[derive(Resource)]
pub struct BorderTileCurrentHeight {
    pub top_border: f32,
    pub bottom_border: f32,
}

impl Default for BorderTileCurrentHeight {
    fn default() -> Self {
        Self {
            top_border: DEFAULT_BORDERTILE_HEIGHT,
            bottom_border: DEFAULT_BORDERTILE_HEIGHT,
        }
    }
}

#[derive(Resource)]
pub struct BorderTileFluctuator {
    pub direction: FluctuatingDirection,
}

impl Default for BorderTileFluctuator {
    fn default() -> Self {
        Self {
            direction: FluctuatingDirection::Up,
        }
    }
}
