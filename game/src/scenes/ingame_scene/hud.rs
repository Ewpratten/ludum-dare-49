use crate::{GameConfig, utilities::render_layer::ScreenSpaceRender};
use raylib::prelude::*;
use super::InGameScreen;

impl ScreenSpaceRender for InGameScreen {
    fn render_screen_space(
        &mut self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig
    ) {
        puffin::profile_function!();
        // Calculate the logo position
        let screen_size = raylib.get_screen_size();

        // Draw a thin glow box around the screen
        raylib.draw_rectangle_lines(0, 0, screen_size.x as i32, screen_size.y as i32, config.colors.red);
    }
}
