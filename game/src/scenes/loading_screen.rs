use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use raylib::prelude::*;

use crate::{
    context::GameContext,
    utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        math::interpolate_exp,
        non_ref_raylib::HackedRaylibHandle,
        render_layer::ScreenSpaceRender,
    },
};

use super::{Scenes, ScreenError};
use tracing::{debug, info, trace};

/// Defines how long the loading screen should be displayed.
const LOADING_SCREEN_DURATION_SECONDS: u8 = 3;

#[derive(Debug)]
pub struct LoadingScreen {
    start_timestamp: Option<DateTime<Utc>>,
    game_logo_texture: Texture2D,
    game_logo_size: Vector2,
}

impl LoadingScreen {
    /// Construct a new `LoadingScreen`
    pub fn new(
        raylib_handle: &mut HackedRaylibHandle,
        thread: &RaylibThread,
    ) -> Result<Self, ResourceLoadError> {
        // Load the game logo asset
        let game_logo =
            load_texture_from_internal_data(raylib_handle, thread, "logos/game-logo.png")?;

        Ok(Self {
            start_timestamp: None,
            game_logo_size: Vector2::new(game_logo.width as f32, game_logo.height as f32),
            game_logo_texture: game_logo,
        })
    }
}

impl Action<Scenes, ScreenError, GameContext> for LoadingScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, _context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running LoadingScreen for the first time");

        // Keep track of when this screen is opened
        self.start_timestamp = Some(Utc::now());

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on LoadingScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut());

        // Keep rendering until we pass the loading screen duration
        if let Some(start_timestamp) = self.start_timestamp {
            let duration = Utc::now().signed_duration_since(start_timestamp);
            if duration.num_seconds() >= LOADING_SCREEN_DURATION_SECONDS as i64 {
                info!("LoadingScreen duration reached, moving to next screen");
                Ok(ActionFlag::SwitchState(Scenes::FsmErrorScreen))
            } else {
                Ok(ActionFlag::Continue)
            }
        } else {
            Ok(ActionFlag::Continue)
        }
    }

    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished LoadingScreen");

        // Reset the start timestamp
        self.start_timestamp = None;

        Ok(())
    }
}

impl ScreenSpaceRender for LoadingScreen {
    fn render_screen_space(
        &self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
    ) {
        // Calculate the loading screen fade in/out value
        // This makes the loading screen fade in/out over the duration of the loading screen
        let cur_time = Utc::now();
        let time_since_start =
            cur_time.signed_duration_since(self.start_timestamp.unwrap_or(cur_time));
        let fade_percentage = interpolate_exp(
            time_since_start.num_milliseconds() as f32,
            0.0..(LOADING_SCREEN_DURATION_SECONDS as f32 * 1000.0),
            0.0..1.0,
            8.0,
        );
        trace!("Loading screen fade at {:.2}%", fade_percentage);

        // Render the background
        raylib.clear_background(Color::WHITE);

        // Calculate the logo position
        let screen_size = raylib.get_screen_size();

        // Render the game logo
        raylib.draw_texture_ex(
            &self.game_logo_texture,
            screen_size.div(2.0).sub(self.game_logo_size.div(2.0)),
            0.0,
            1.0,
            Color::WHITE.fade(fade_percentage),
        );
    }
}
