use std::ops::{Div, Sub};

use chrono::{DateTime, Utc};
use dirty_fsm::{Action, ActionFlag};
use pkg_version::pkg_version_major;
use raylib::prelude::*;

use crate::{GameConfig, context::GameContext, utilities::{
        datastore::{load_texture_from_internal_data, ResourceLoadError},
        game_version::get_version_string,
        math::interpolate_exp,
        non_ref_raylib::HackedRaylibHandle,
        render_layer::ScreenSpaceRender,
    }};

use super::{Scenes, ScreenError};
use tracing::{debug, info, trace};

#[derive(Debug)]
pub struct PauseScreen {}

impl PauseScreen {
    /// Construct a new `PauseScreen`
    pub fn new() -> Self {
        Self {}
    }
}

impl Action<Scenes, ScreenError, GameContext> for PauseScreen {
    fn on_register(&mut self) -> Result<(), ScreenError> {
        debug!("Registered");
        Ok(())
    }

    fn on_first_run(&mut self, _context: &GameContext) -> Result<(), ScreenError> {
        debug!("Running PauseScreen for the first time");

        Ok(())
    }

    fn execute(
        &mut self,
        _delta: &chrono::Duration,
        context: &GameContext,
    ) -> Result<dirty_fsm::ActionFlag<Scenes>, ScreenError> {
        trace!("execute() called on PauseScreen");
        self.render_screen_space(&mut context.renderer.borrow_mut(), &context.config);

        //Mouse Position
        let mouse_position: Vector2 = context.renderer.borrow_mut().get_mouse_position();
        //Mouse Input
        let is_left_click = context.renderer.borrow_mut().is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);


        //"Hitboxes" for the resume and Main menu buttons
        if is_left_click && Rectangle::new(322.0,321.0,435.0,80.0).check_collision_point_rec(mouse_position) {
            return Ok(ActionFlag::SwitchState(Scenes::InGameScene));
        }

        if is_left_click && Rectangle::new(390.0,464.0,200.0,50.0).check_collision_point_rec(mouse_position) {
            return Ok(ActionFlag::SwitchState(Scenes::MainMenuScreen));
        }

        if context.renderer.borrow_mut().is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            Ok(ActionFlag::SwitchState(Scenes::InGameScene))
        } else {
            Ok(ActionFlag::Continue)
        }
    }


    fn on_finish(&mut self, _interrupted: bool) -> Result<(), ScreenError> {
        debug!("Finished PauseScreen");
        Ok(())
    }
}

impl ScreenSpaceRender for PauseScreen {

    fn render_screen_space(
        &self,
        raylib: &mut crate::utilities::non_ref_raylib::HackedRaylibHandle,
        config: &GameConfig
    ) {
        let screen_size = raylib.get_screen_size();

        // Render the background
        raylib.clear_background(Color::BLACK.fade(50.0));

        //Mouse Position
        let mouse_position: Vector2 = raylib.get_mouse_position();
        //Mouse Input
        let is_left_click = raylib.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

        //Pause Menu Texts With Glitchy Effect
        raylib.draw_text(
            "Paused",
            (screen_size.x as i32 / 2) - 223,
            (screen_size.y as i32 / 2) - 40,
            120,
            Color::RED
        );
        raylib.draw_text(
            "Paused",
            (screen_size.x as i32 / 2) - 217,
            (screen_size.y as i32 / 2) - 40,
            120,
            Color::BLUE
        );
        raylib.draw_text(
            "Paused",
            (screen_size.x as i32 / 2) - 220,
            (screen_size.y as i32 / 2) - 40,
            120,
            Color::WHITE
        );
        raylib.draw_text(
            "Click To Resume",
            (screen_size.x as i32 / 2) - 80,
            (screen_size.y as i32 / 2) + 60,
            20,
            Color::RED
        );
        raylib.draw_text(
            "Click To Resume",
            (screen_size.x as i32 / 2) - 80,
            (screen_size.y as i32 / 2) + 60,
            20,
            Color::BLUE
        );
        raylib.draw_text(
            "Click To Resume",
            (screen_size.x as i32 / 2) - 80,
            (screen_size.y as i32 / 2) + 60,
            20,
            Color::WHITE
        );
        raylib.draw_text(
            "Main Menu",
            (screen_size.x as i32 / 2) - 123,
            (screen_size.y as i32 / 2) + 100,
            50,
            Color::RED
        );
        raylib.draw_text(
            "Main Menu",
            (screen_size.x as i32 / 2) - 117,
            (screen_size.y as i32 / 2) + 100,
            50,
            Color::BLUE
        );
        raylib.draw_text(
            "Main Menu",
            (screen_size.x as i32 / 2) - 120,
            (screen_size.y as i32 / 2) + 100,
            50,
            Color::WHITE
        );

        if Rectangle::new(390.0,464.0,200.0,50.0).check_collision_point_rec(mouse_position) {
            raylib.draw_text(
                "Main Menu",
                (screen_size.x as i32 / 2) - 120,
                (screen_size.y as i32 / 2) + 100,
                50,
                Color::YELLOW
            );
        }

        if Rectangle::new(322.0,321.0,435.0,80.0).check_collision_point_rec(mouse_position) {
            raylib.draw_text(
                "Paused",
                (screen_size.x as i32 / 2) - 220,
                (screen_size.y as i32 / 2) - 40,
                120,
                Color::DARKBLUE
            );
        }


    }
}
