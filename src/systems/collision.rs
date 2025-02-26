use bevy::prelude::*;

use crate::{
    components::{Border, Copter, Obstacle},
    constants::{COPTER_SIZE, WINDOW_HEIGHT},
    resources::GameState,
};

/*
    X-axis check:
        If the absolute distance between the center of the obstacle and
        the center of the copter is less than the sum of their widths * 0.5,
        then collision has occured.
        Absolute difference is necessary so that obstacles that we have
        already passed through don't erroneously report collision w the copter.

    Y-axis check:
        If the position of the copter on the y-axis is in the range of
        pixels that an obstacle occupies, then a collision is possible.
        Take the middle point (obstacle_pos.y) of the obstacle and add
        half of the height (obstacle_size.y) to get the topmost point of the
        obstacle, and subtract it to get the lowest point.
        Now, if the y-coordinate of the copter's position is in between these
        points, then a collision is possible.
*/

fn collide(
    copter_pos: &Vec3,
    copter_size: &Vec2,
    obstacle_pos: &Vec3,
    obstacle_size: &Vec2,
) -> bool {
    if (obstacle_pos.x - copter_pos.x).abs() < (copter_size.x + obstacle_size.x) * 0.5
        && (obstacle_pos.y - obstacle_size.y * 0.5 <= copter_pos.y
            && copter_pos.y <= obstacle_pos.y + obstacle_size.y * 0.5)
    {
        return true;
    }
    false
}

pub fn collision_detection(
    mut game_state: ResMut<GameState>,
    copter_query: Query<&mut Transform, (With<Copter>, Without<Obstacle>)>,
    obstacle_query: Query<(&Transform, &Sprite), With<Obstacle>>,
    border_query: Query<(&Transform, &Border), (Without<Obstacle>, Without<Copter>)>
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

        for (_, border) in border_query.iter() {
            if copter_pos.y > WINDOW_HEIGHT * 0.5 - border.height as f32 - COPTER_SIZE.y * 0.5
                || copter_pos.y < -WINDOW_HEIGHT * 0.5 + border.height as f32 + COPTER_SIZE.y * 0.5
            {
                game_state.game_over = true;
                return;
            }
        }
    }
}
