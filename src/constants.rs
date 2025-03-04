use bevy::math::Vec2;

// Window Settings
pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

// Copter Settings
pub const COPTER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const LIFT: f32 = 2250.0;
pub const GRAVITY: f32 = 1000.0;
pub const NEGATE_DELAY: f32 = 2.0;

// Obstacle Settings
pub const OBSTACLE_SPAWN_TIME: f32 = 1.25;
pub const OBSTACLE_HEIGHT: f32 = 250.0;
pub const OBSTACLE_WIDTH: f32 = 100.0;
pub const OBSTACLE_SPEED: f32 = 600.0;

// BorderTile Obstacle Settings
pub const DEFAULT_BORDERTILE_HEIGHT: f32 = 75.0;
pub const MIN_BORDERTILE_HEIGHT: f32 = 50.0;
pub const MAX_BORDERTILE_HEIGHT: f32 = 125.0;
pub const BORDERTILE_WIDTH: f32 = 64.0; // Perfect multiple of WINDOW_WIDTH = 1024.0
pub const BORDERTILE_SPEED: f32 = OBSTACLE_SPEED; // IDK just to match
pub const BORDERTILE_SPAWN_TIME: f32 = 0.3;

// BorderTile Fluctuation Settings
pub const FLUCTUATION_PER_FRAME: f32 = 5.0;
