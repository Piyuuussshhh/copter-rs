use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::Obstacle,
    constants::{OBSTACLE_HEIGHT, OBSTACLE_SPEED, OBSTACLE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
    resources::GameState,
};

pub fn spawn_obstacles(mut commands: Commands, time: Res<Time>, mut game_state: ResMut<GameState>) {
    if game_state.game_over {
        return;
    }

    game_state.obstacle_timer.tick(time.delta());

    if game_state.obstacle_timer.just_finished() {
        let mut rng = rand::rng();
        // Obstacle spawned outside the right wall due to this x-coordinate.
        let obstacle_x = WINDOW_WIDTH * 0.5 + OBSTACLE_WIDTH * 0.5;
        // Spawn at any legal y-coord where legal == within the window's height && doesn't clip the window egdes.
        let obstacle_y = rng.random_range(
            -WINDOW_HEIGHT * 0.5 + OBSTACLE_HEIGHT..WINDOW_HEIGHT * 0.5 - OBSTACLE_HEIGHT,
        );

        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(OBSTACLE_WIDTH, OBSTACLE_HEIGHT)),
                ..Default::default()
            },
            Transform::from_xyz(obstacle_x, obstacle_y, 1.0),
            Obstacle,
        ));
    }
}

pub fn map_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut obstacle_query: Query<(Entity, &mut Transform), With<Obstacle>>,
    game_state: Res<GameState>,
) {
    if game_state.game_over {
        return;
    }

    for (entity, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= OBSTACLE_SPEED * time.delta_secs();

        // If the obstacle has been surpassed and is now outside the screen, despawn it.
        if transform.translation.x < (-WINDOW_WIDTH / 2.0 - OBSTACLE_WIDTH) {
            commands.entity(entity).despawn();
        }
    }
}
