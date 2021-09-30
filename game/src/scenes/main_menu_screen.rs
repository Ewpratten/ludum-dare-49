use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use pkg_version::pkg_version_major;
use raylib::prelude::*;

use crate::{context::GameContext, utilities::{datastore::{load_texture_from_internal_data, ResourceLoadError}, game_version::get_version_string, math::interpolate_exp, non_ref_raylib::HackedRaylibHandle, render_layer::ScreenSpaceRender}};

use super::{Scenes, ScreenError};
use tracing::{debug, info, trace};

#[derive(Debug)]
pub struct MainMenuScreen {}

impl MainMenuScreen {
    /// Construct a new `MainMenuScreen`
    pub fn new() -> Self {
        Self {}
    }
}

impl Action<Scenes, ScreenError, GameContext> for MainMenuScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, _context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running MainMenuScreen for the first time");

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on MainMenuScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut());

        Ok(ActionFlag::Continue)
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished MainMenuScreen");
        Ok(())
    }
}

impl ScreenSpaceRender for MainMenuScreen {
    fn render_screen_space(
        &self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
    ) {
        // Render the background
        raylib.clear_background(Color::WHITE);

        // Calculate the logo position
        let screen_size = raylib.get_screen_size();

        // Only in debug mode, render a debug message
        #[cfg(debug_assertions)]
        {
            raylib.draw_text(
                "Game in DEBUG MODE. Do not redistribute!",
                10,
                screen_size.y as i32 - 35,
                15,
                Color::BLACK,
            );
        }
        // Render the game version info
        raylib.draw_text(
            &format!("Version: {} Commit: {}", get_version_string(), env!("VERGEN_GIT_SHA_SHORT")),
            10,
            screen_size.y as i32 - 20,
            15,
            Color::BLACK,
        );
    }
}