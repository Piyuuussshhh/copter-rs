use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Copter {
    pub velocity: f32,
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct BorderTile {
    pub height: f32,
}
