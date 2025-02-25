use std::time::Duration;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use rand::Rng;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const COPTER_SIZE: Vec2 = Vec2::new(50.0, 30.0);
const OBSTACLE_SPAWN_TIME: f32 = 1.5;
const GRAVITY: f32 = 800.0;
const LIFT: f32 = 15000.0;
const GAP_SIZE: f32 = 180.0;
const OBSTACLE_WIDTH: f32 = 30.0;
const OBSTACLE_SPEED: f32 = 200.0;

#[derive(Component)]
struct Copter {
    velocity: f32,
}

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct ScoreText;

// Resource for tracking game state
#[derive(Resource)]
struct GameState {
    score: u32,
    game_over: bool,
    obstacle_timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Copter".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(GameState {
            score: 0,
            game_over: false,
            obstacle_timer: Timer::new(
                Duration::from_secs_f32(OBSTACLE_SPAWN_TIME),
                TimerMode::Repeating,
            ),
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                copter_movement,
                spawn_obstacles,
                map_movement,
                collision_detection,
                update_score,
                restart,
            ),
        )
        .run();
}

// Setup the camera, background, helicopter and UI elements.
fn setup(mut commands: Commands) {
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

    // TODO Score text.
    commands.spawn((
        Text2d::new("Score: "),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
    ));
    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        ScoreText,
    ));

    // TODO Instruction text.
    commands.spawn((
        Text2d::new("Hold left click to fly up. Press R to restart when game over."),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Left),
    ));
}

fn copter_movement(
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut copter_query: Query<(&mut Copter, &mut Transform)>,
    game_state: Res<GameState>,
) {
    if game_state.game_over {
        return;
    }

    if let Ok((mut copter, mut transform)) = copter_query.get_single_mut() {
        copter.velocity -= GRAVITY * time.delta_secs();

        if mouse_input.pressed(MouseButton::Left) {
            copter.velocity = LIFT * time.delta_secs();
        }

        transform.translation.y += copter.velocity * time.delta_secs();
        // transform.translation.y = transform.translation.y.clamp(
        //     -WINDOW_HEIGHT / 2.0 + COPTER_SIZE.y / 2.0,
        //     WINDOW_HEIGHT / 2.0 - COPTER_SIZE.y / 2.0,
        // );
    }
}

fn spawn_obstacles(mut commands: Commands, time: Res<Time>, mut game_state: ResMut<GameState>) {
    if game_state.game_over {
        return;
    }

    game_state.obstacle_timer.tick(time.delta());

    if game_state.obstacle_timer.just_finished() {
        let mut rng = rand::rng();
        let gap_center = rng.random_range(-WINDOW_HEIGHT / 3.0..WINDOW_HEIGHT / 3.0);

        let top_height = (WINDOW_HEIGHT / 2.0) - (gap_center + GAP_SIZE / 2.0);
        if top_height > 0.0 {
            // Top border.
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.8, 0.3),
                    custom_size: Some(Vec2::new(OBSTACLE_WIDTH, top_height)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    WINDOW_WIDTH / 2.0 + OBSTACLE_WIDTH / 2.0,
                    WINDOW_HEIGHT / 2.0 - top_height / 2.0,
                    1.0,
                ),
                Obstacle,
            ));
        }

        let bottom_height = (WINDOW_HEIGHT / 2.0) - (GAP_SIZE / 2.0 - gap_center);
        if bottom_height > 0.0 {
            // Bottom border.
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.8, 0.3),
                    custom_size: Some(Vec2::new(OBSTACLE_WIDTH, bottom_height)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    WINDOW_WIDTH / 2.0 + OBSTACLE_WIDTH / 2.0,
                    -WINDOW_HEIGHT / 2.0 + bottom_height / 2.0,
                    1.0,
                ),
                Obstacle,
            ));
        }
    }
}

fn map_movement(
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

fn collide(
    copter_pos: &Vec3,
    copter_size: &Vec2,
    obstacle_pos: &Vec3,
    obstacle_size: &Vec2,
) -> bool {
    if obstacle_pos.y - obstacle_size.y / 2.0 <= copter_pos.y
        && copter_pos.y <= obstacle_size.y / 2.0 + obstacle_pos.y
        && copter_pos.x + copter_size.x >= obstacle_pos.x - obstacle_size.x / 2.0
    {
        return true;
    }
    false
}

fn collision_detection(
    mut game_state: ResMut<GameState>,
    copter_query: Query<&mut Transform, (With<Copter>, Without<Obstacle>)>,
    obstacle_query: Query<(&Transform, &Sprite), With<Obstacle>>,
) {
    if game_state.game_over {
        return;
    }

    if let Ok(copter_transform) = copter_query.get_single() {
        let copter_pos = copter_transform.translation;

        for (obstacle_transfom, obstacle_sprite) in obstacle_query.iter() {
            let obstacle_size = obstacle_sprite.custom_size.unwrap_or(Vec2::ONE);
            let obstacle_pos = obstacle_transfom.translation;

            if collide(&copter_pos, &COPTER_SIZE, &obstacle_pos, &obstacle_size) {
                game_state.game_over = true;
                return;
            }
        }

        if copter_pos.y > WINDOW_HEIGHT / 2.0 - COPTER_SIZE.y / 2.0
            || copter_pos.y < -WINDOW_HEIGHT / 2.0 + COPTER_SIZE.y / 2.0
        {
            game_state.game_over = true;
        }
    }
}

fn update_score(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut score_query: Query<&mut Text2d, With<ScoreText>>,
) {
    if game_state.game_over {
        return;
    }

    game_state.score += (time.delta_secs() * 10.0) as u32;

    // Highly inefficient I think but the only way I could come up w.
    if let Ok(mut score_text) = score_query.get_single_mut() {
        score_text.clear();
        score_text.push_str(game_state.score.to_string().as_str());
    }
}

fn restart(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    obstacle_query: Query<Entity, With<Obstacle>>,
    mut copter_query: Query<(&mut Copter, &mut Transform)>,
) {
    if game_state.game_over && keyboard_input.just_pressed(KeyCode::KeyR) {
        game_state.game_over = false;
        game_state.score = 0;

        for entity in obstacle_query.iter() {
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
