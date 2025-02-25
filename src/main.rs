use std::time::Duration;

use bevy::{
    prelude::*, window::{PresentMode, WindowResolution, WindowTheme}
};
use rand::Rng;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;

const COPTER_SIZE: Vec2 = Vec2::new(50.0, 30.0);
const OBSTACLE_SPAWN_TIME: f32 = 1.5;
const GRAVITY: f32 = 500.0;
const LIFT: f32 = 800.0;
const GAP_SIZE: f32 = 150.0;

#[derive(Component)]
struct Copter {
    velocity: f32,
}

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
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
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
            obstacle_timer: Timer::new(Duration::from_secs_f32(OBSTACLE_SPAWN_TIME), TimerMode::Repeating)
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (copter_movement))
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
        ScoreText
    ));
    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        ScoreText
    ));

    // TODO Instruction text.
    commands.spawn((
        Text2d::new("Hold SPACE to fly up. Press R to restart when game over."),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
    ));
}

fn copter_movement(
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut copter_query: Query<(&mut Copter, &mut Transform)>,
    game_state: Res<GameState>
) {
    if game_state.game_over {
        return;
    }

    if let Ok((mut copter, mut transform)) = copter_query.get_single_mut() {
        copter.velocity -= GRAVITY * time.delta_secs();

        if mouse_input.pressed(MouseButton::Left) {
            copter.velocity = LIFT * time.delta_secs();
        }

        transform.translation.y = copter.velocity * time.delta_secs();
        transform.translation.y = transform.translation.y.clamp(
            -WINDOW_HEIGHT / 2.0 + COPTER_SIZE.y / 2.0,
            WINDOW_HEIGHT / 2.0 - COPTER_SIZE.y / 2.0,
        )
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.game_over {
        return;
    }

    game_state.obstacle_timer.tick(time.delta());

    if game_state.obstacle_timer.just_finished() {
        let mut rng = rand::rng();
        let gap_center = rng.random_range(-WINDOW_HEIGHT / 3.0..WINDOW_HEIGHT / 3.0);

        let top_height = (WINDOW_HEIGHT / 2.0) - (gap_center + GAP_SIZE / 2.0);
    }
}