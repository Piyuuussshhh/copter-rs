use bevy::prelude::*;

use crate::{
    components::{BorderTile, Copter, Obstacle},
    constants::{COPTER_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH},
    resources::{BorderTileCurrentHeight, BorderTileFluctuator, GameState},
    systems::{
        collision::collision_detection,
        copter::copter_movement,
        obstacles::{
            bordertile_movement, obstacle_movement, spawn_bordertiles, spawn_init_border,
            spawn_obstacles,
        },
        ui::{setup_ui, update_score},
    },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default())
            .insert_resource(BorderTileCurrentHeight::default())
            .insert_resource(BorderTileFluctuator::default())
            .add_systems(Startup, (setup_game, setup_ui, spawn_init_border))
            .add_systems(
                Update,
                (
                    copter_movement,
                    spawn_obstacles,
                    spawn_bordertiles,
                    obstacle_movement,
                    bordertile_movement,
                    collision_detection,
                    update_score,
                    restart,
                ),
            )
            .add_systems(FixedUpdate, (spawn_bordertiles,));
    }
}

// Setup the camera, background, helicopter and UI elements.
fn setup_game(mut commands: Commands) {
    // Camera.
    commands.spawn(Camera2d::default());

    // Background.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.2),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Copter.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.3, 0.3),
            custom_size: Some(COPTER_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(-300.0, 0.0, 1.0),
        Copter { velocity: 0.0 },
    ));
}

fn restart(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    obstacle_query: Query<Entity, With<Obstacle>>,
    bordertile_query: Query<Entity, With<BorderTile>>,
    mut copter_query: Query<(&mut Copter, &mut Transform)>,
) {
    if game_state.game_over && keyboard_input.just_pressed(KeyCode::KeyR) {
        game_state.game_over = false;
        game_state.score = 0.0;

        for entity in obstacle_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in bordertile_query.iter() {
            commands.entity(entity).despawn();
        }


        // Important to respawn obstacles once gamer restarts.
        game_state.obstacle_timer.reset();

        if let Ok((mut copter, mut transform)) = copter_query.get_single_mut() {
            copter.velocity = 0.0;
            transform.translation = Vec3::new(-300.0, 0.0, 1.0);
        }
    }
}
