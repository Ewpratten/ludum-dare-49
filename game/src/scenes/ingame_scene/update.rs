use std::ops::Div;

use super::InGameScreen;
use crate::{
    character::CharacterState,
    utilities::{non_ref_raylib::HackedRaylibHandle, render_layer::FrameUpdate},
    GameConfig,
};
use chrono::Duration;
use raylib::prelude::*;

impl FrameUpdate for InGameScreen {
    fn update(
        &mut self,
        raylib: &HackedRaylibHandle,
        delta_seconds: &Duration,
        config: &GameConfig,
    ) {
        puffin::profile_function!();

        // Get the current level
        let cur_level = self.levels.get(self.current_level_idx).unwrap();

        // Set the camera's offset based on screen size
        self.camera.offset = raylib.get_screen_size().div(Vector2::new(2.0, 1.05));
        self.camera.target = Vector2::new(self.player.position.x, self.camera.target.y);

        // Check the only possible keyboard inputs
        let is_jump = raylib.is_key_pressed(KeyboardKey::KEY_SPACE)
            && !(self.player.current_state == CharacterState::Jumping);
        let is_dash = raylib.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT)
            && !(self.player.current_state == CharacterState::Dashing);

        let collision_result = if is_jump {
            self.player.update_player(
                Some(CharacterState::Jumping),
                &cur_level.colliders,
                -cur_level.platform_tex.height as f32,
            )
        } else if is_dash {
            self.player.update_player(
                Some(CharacterState::Dashing),
                &cur_level.colliders,
                -cur_level.platform_tex.height as f32,
            )
        } else {
            if self.player.current_state != CharacterState::Jumping
                && self.player.current_state != CharacterState::Dashing
            {
                self.player.update_player(
                    Some(CharacterState::Running),
                    &cur_level.colliders,
                    -cur_level.platform_tex.height as f32,
                )
            } else {
                self.player.update_player(
                    None,
                    &cur_level.colliders,
                    -cur_level.platform_tex.height as f32,
                )
            }
        };

        // Handle running into a wall
        if let Err(_) = collision_result {
            self.player_dead = true;
        }
    }
}
