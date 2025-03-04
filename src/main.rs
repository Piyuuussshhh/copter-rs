use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::GamePlugin;

mod components;
mod constants;
mod enums;
mod game;
mod resources;
mod systems;

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
        .add_plugins(GamePlugin)
        .run();
}
