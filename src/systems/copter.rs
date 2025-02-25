use bevy::prelude::*;

use crate::{
    components::Copter,
    constants::{GRAVITY, LIFT, NEGATE_DELAY},
    resources::GameState,
};

pub fn copter_movement(
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut copter_query: Query<(&mut Copter, &mut Transform)>,
    game_state: Res<GameState>,
) {
    if game_state.game_over {
        return;
    }

    if let Ok((mut copter, mut transform)) = copter_query.get_single_mut() {
        copter.velocity -= GRAVITY * time.delta_secs() + NEGATE_DELAY;

        if mouse_input.pressed(MouseButton::Left) {
            copter.velocity += LIFT * time.delta_secs() + NEGATE_DELAY;
        }

        transform.translation.y += copter.velocity * time.delta_secs();
    }
}
