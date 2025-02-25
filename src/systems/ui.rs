use bevy::prelude::*;

use crate::{components::ScoreText, resources::GameState};

pub fn setup_ui(mut commands: Commands) {
    // TODO Score text.
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..Default::default()
        },
        ScoreText,
    ));

    // TODO Instruction text.
    commands.spawn((
        Text::new("Hold left click to fly up. Press R to restart when game over."),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            ..Default::default()
        },
    ));
}

pub fn update_score(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut score_query: Query<(&mut Text, &Node), With<ScoreText>>,
) {
    if game_state.game_over {
        return;
    }
    // println!("Time elapsed since last update: {}", time.delta_secs());
    game_state.score += time.delta_secs();

    if let Ok((mut score_text, _)) = score_query.get_single_mut() {
        score_text.0 = format!("Score: {}", game_state.score as u32);
        // println!("{}", score_text.0);
    }
}
