use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::{Border, Copter, Obstacle},
    constants::{
        MIN_BORDER_HEIGHT, OBSTACLE_HEIGHT, OBSTACLE_SPEED, OBSTACLE_WIDTH, WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
    resources::GameState,
};

pub fn spawn_border(mut commands: Commands) {
    // Top border.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.8, 0.3),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, MIN_BORDER_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, WINDOW_HEIGHT * 0.5 - MIN_BORDER_HEIGHT * 0.5, 1.0),
        Border {
            height: MIN_BORDER_HEIGHT as u32,
        },
    ));
    // Bottom Border.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.8, 0.3),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, MIN_BORDER_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, -WINDOW_HEIGHT * 0.5 + MIN_BORDER_HEIGHT * 0.5, 1.0),
        Border {
            height: MIN_BORDER_HEIGHT as u32,
        },
    ));
}

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    border_query: Query<(&Transform, &Border), (Without<Obstacle>, Without<Copter>)>,
) {
    if game_state.game_over {
        return;
    }

    game_state.obstacle_timer.tick(time.delta());

    if game_state.obstacle_timer.just_finished() {
        let mut rng = rand::rng();
        // Obstacle spawned outside the right wall due to this x-coordinate.
        let obstacle_x = WINDOW_WIDTH * 0.5 + OBSTACLE_WIDTH * 0.5;
        // Spawn at any legal y-coord where legal == within the window's height && doesn't clip the window egdes.
        let obstacle_y = {
            let mut obstacle_region = [0.0; 2];
            let mut i = 0usize;
            for (border_transform, border) in border_query.iter() {
                if border_transform.translation.y > 0.0 {
                    obstacle_region[i] = border_transform.translation.y - border.height as f32 * 0.5;
                } else {
                    obstacle_region[i] = border.height as f32 * 0.5 + border_transform.translation.y;
                }
                i += 1;
            }
            obstacle_region.sort_by(|a, b| a.partial_cmp(&b).unwrap());
            // Final y-coord range.
            rng.random_range(
                obstacle_region[0] + OBSTACLE_HEIGHT * 0.5 ..= obstacle_region[1] - OBSTACLE_HEIGHT * 0.5
            )
        };

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
