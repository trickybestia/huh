use crate::{
    engine::webgl_renderer::{extensions::DrawRectangleExt, Color},
    math::Rectangle,
};
use crate::{
    engine::{Raycaster, Scene, WebGlRenderer},
    math::{Polygon, Vector2},
};
use log::debug;
use web_sys::WebGlRenderingContext;

#[derive(Clone)]
enum PolygonID {
    Button,
}

enum ButtonState {
    One,
    Two,
}

impl ButtonState {
    pub fn switch(&self) -> Self {
        match self {
            ButtonState::One => ButtonState::Two,
            ButtonState::Two => ButtonState::One,
        }
    }
}

pub struct MenuScene {
    raycaster: Raycaster<PolygonID>,
    button_state: ButtonState,
}

impl MenuScene {
    pub fn new() -> Self {
        let mut raycaster = Raycaster::new();

        raycaster.add_polygon(
            Polygon::new(vec![
                Vector2::new(-0.3, -0.1),
                Vector2::new(-0.3, 0.1),
                Vector2::new(0.3, 0.1),
                Vector2::new(0.3, -0.1),
            ]),
            PolygonID::Button,
            0.0,
        );

        raycaster.sort();

        Self {
            raycaster,
            button_state: ButtonState::One,
        }
    }
}

impl Scene for MenuScene {
    fn render(&mut self, renderer: &mut WebGlRenderer) -> Option<Box<dyn Scene>> {
        let gl = renderer.gl();

        gl.clear_color(0.2, 0.2, 0.2, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        let button_color = match self.button_state {
            ButtonState::One => Color::new(1.0, 1.0, 1.0, 1.0),
            ButtonState::Two => Color::new(1.0, 0.0, 0.0, 1.0),
        };

        renderer.draw_rectangle(
            &Rectangle::new(-0.3, -0.1, 0.6, 0.2),
            0.0,
            &button_color,
            renderer.rendering_settings(),
        );

        None
    }

    fn on_click(&mut self, renderer: &WebGlRenderer, position: &Vector2<i32>) {
        let world_position = renderer.screen_to_world_position(position);

        debug!(
            "screen position: {:?}; world position: {:?}",
            position, world_position
        );

        if let Some(polygon_id) = self.raycaster.raycast(&world_position) {
            debug!("hit!");

            self.button_state = self.button_state.switch();
        }
    }
}
