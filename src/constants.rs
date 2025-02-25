use bevy::math::Vec2;

// Window Settings
pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

// Copter Settings
pub const COPTER_SIZE: Vec2 = Vec2::new(50.0, 50.0);
pub const LIFT: f32 = 2250.0;
pub const GRAVITY: f32 = 800.0;
pub const NEGATE_DELAY: f32 = 7.5;

// Obstacle Settings
pub const OBSTACLE_SPAWN_TIME: f32 = 1.5;
pub const OBSTACLE_HEIGHT: f32 = 250.0;
pub const OBSTACLE_WIDTH: f32 = 100.0;
pub const OBSTACLE_SPEED: f32 = 400.0;