use bevy::prelude::*;

use crate::{
    components::{BorderTile, Obstacle},
    constants::{
        BORDERTILE_SPEED, BORDERTILE_WIDTH, DEFAULT_BORDERTILE_HEIGHT, FLUCTUATION_PER_FRAME,
        MAX_BORDERTILE_HEIGHT, MIN_BORDERTILE_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH,
    },
    enums::FluctuatingDirection,
    resources::{BorderTileCurrentHeight, BorderTileFluctuator, GameState},
};

pub fn spawn_init_border(mut commands: Commands) {
    let num_border_rects = WINDOW_WIDTH / BORDERTILE_WIDTH;
    for i in 0..=num_border_rects as u32 {
        // Top border.
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(BORDERTILE_WIDTH, DEFAULT_BORDERTILE_HEIGHT)),
                ..Default::default()
            },
            Transform::from_xyz(
                if i == 0 {
                    -WINDOW_WIDTH * 0.5 + BORDERTILE_WIDTH * 0.5
                } else {
                    -WINDOW_WIDTH * 0.5 + BORDERTILE_WIDTH * 0.5 + (BORDERTILE_WIDTH * i as f32)
                },
                WINDOW_HEIGHT * 0.5 - DEFAULT_BORDERTILE_HEIGHT * 0.5,
                1.0,
            ),
            BorderTile {
                height: DEFAULT_BORDERTILE_HEIGHT,
            },
        ));
        // Bottom Border.
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(BORDERTILE_WIDTH, DEFAULT_BORDERTILE_HEIGHT)),
                ..Default::default()
            },
            Transform::from_xyz(
                if i == 0 {
                    -WINDOW_WIDTH * 0.5 + BORDERTILE_WIDTH * 0.5
                } else {
                    -WINDOW_WIDTH * 0.5 + BORDERTILE_WIDTH * 0.5 + (BORDERTILE_WIDTH * i as f32)
                },
                -WINDOW_HEIGHT * 0.5 + DEFAULT_BORDERTILE_HEIGHT * 0.5,
                1.0,
            ),
            BorderTile {
                height: DEFAULT_BORDERTILE_HEIGHT,
            },
        ));
    }
}

pub fn spawn_bordertiles(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut game_state: ResMut<GameState>,
    mut bordertile_fluctuator: ResMut<BorderTileFluctuator>,
    mut bordertile_cur_height: ResMut<BorderTileCurrentHeight>,
) {
    if game_state.game_over {
        return;
    }

    game_state.bordertile_timer.tick(time.delta());

    if game_state.bordertile_timer.just_finished() {
        // Fluctuate the borders.
        match bordertile_fluctuator.direction {
            FluctuatingDirection::Up => {
                bordertile_cur_height.top_border -= FLUCTUATION_PER_FRAME;
                bordertile_cur_height.bottom_border += FLUCTUATION_PER_FRAME;

                if bordertile_cur_height.top_border <= MIN_BORDERTILE_HEIGHT
                    && bordertile_cur_height.bottom_border >= MAX_BORDERTILE_HEIGHT
                {
                    bordertile_fluctuator.direction = FluctuatingDirection::Down;
                }
            }
            FluctuatingDirection::Down => {
                bordertile_cur_height.top_border += FLUCTUATION_PER_FRAME;
                bordertile_cur_height.bottom_border -= FLUCTUATION_PER_FRAME;

                if bordertile_cur_height.top_border >= MAX_BORDERTILE_HEIGHT
                    && bordertile_cur_height.bottom_border <= MIN_BORDERTILE_HEIGHT
                {
                    bordertile_fluctuator.direction = FluctuatingDirection::Up;
                }
            }
        }

        // Spawn the top bordertile.
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(
                    BORDERTILE_WIDTH,
                    bordertile_cur_height.top_border,
                )),
                ..Default::default()
            },
            Transform::from_xyz(
                WINDOW_WIDTH * 0.5 + BORDERTILE_WIDTH,
                WINDOW_HEIGHT * 0.5 - bordertile_cur_height.top_border * 0.5,
                1.0,
            ),
            BorderTile {
                height: bordertile_cur_height.top_border,
            },
        ));
        // Spawn the bottom bordertile.
        commands.spawn((
            Sprite {
                color: Color::srgb(0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(
                    BORDERTILE_WIDTH,
                    bordertile_cur_height.bottom_border,
                )),
                ..Default::default()
            },
            Transform::from_xyz(
                WINDOW_WIDTH * 0.5 + BORDERTILE_WIDTH,
                -WINDOW_HEIGHT * 0.5 + bordertile_cur_height.bottom_border * 0.5,
                1.0,
            ),
            BorderTile {
                height: bordertile_cur_height.bottom_border,
            },
        ));
    }
}

pub fn bordertile_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut bordertile_query: Query<(Entity, &mut Transform), (With<BorderTile>, Without<Obstacle>)>,
    game_state: Res<GameState>,
) {
    if game_state.game_over {
        return;
    }

    for (entity, mut transform) in bordertile_query.iter_mut() {
        transform.translation.x -= BORDERTILE_SPEED * time.delta_secs();

        if transform.translation.x < (-WINDOW_WIDTH * 0.5 - BORDERTILE_WIDTH * 0.5) {
            commands.entity(entity).despawn();
        }
    }
}
