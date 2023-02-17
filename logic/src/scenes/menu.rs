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
    Button(usize),
}

enum ButtonState {
    On,
    Off,
}

impl ButtonState {
    pub fn toggle(&self) -> Self {
        match self {
            ButtonState::On => ButtonState::Off,
            ButtonState::Off => ButtonState::On,
        }
    }
}

pub struct MenuScene {
    raycaster: Raycaster<PolygonID>,
    buttons: Vec<(Rectangle<f32>, ButtonState)>,
}

impl MenuScene {
    pub fn new() -> Self {
        let mut raycaster = Raycaster::new();

        let buttons = vec![
            (Rectangle::new(-0.3, -0.4, 0.6, 0.2), ButtonState::Off),
            (Rectangle::new(-0.3, -0.1, 0.6, 0.2), ButtonState::Off),
            (Rectangle::new(-0.3, 0.2, 0.6, 0.2), ButtonState::Off),
        ];

        for i in 0..buttons.len() {
            let button = &buttons[i].0;

            raycaster.add_polygon(
                Polygon::new(vec![
                    Vector2::new(button.x, button.y),
                    Vector2::new(button.x, button.y + button.height),
                    Vector2::new(button.x + button.width, button.y + button.height),
                    Vector2::new(button.x + button.width, button.y),
                ]),
                PolygonID::Button(i),
                0.0,
            );
        }

        raycaster.sort();

        Self { raycaster, buttons }
    }
}

impl Scene for MenuScene {
    fn render(&mut self, renderer: &mut WebGlRenderer) -> Option<Box<dyn Scene>> {
        let gl = renderer.gl();

        gl.clear_color(0.2, 0.2, 0.2, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        for button in &self.buttons {
            let color = match button.1 {
                ButtonState::On => Color::new(1.0, 1.0, 1.0, 1.0),
                ButtonState::Off => Color::new(1.0, 0.0, 0.0, 1.0),
            };

            renderer.draw_rectangle(&button.0, 0.0, &color, renderer.rendering_settings());
        }

        None
    }

    fn on_click(&mut self, renderer: &WebGlRenderer, position: &Vector2<i32>) {
        let world_position = renderer.screen_to_world_position(position);

        debug!(
            "screen position: {:?}; world position: {:?}",
            position, world_position
        );

        if let Some(PolygonID::Button(button_id)) = self.raycaster.raycast(&world_position) {
            self.buttons[button_id].1 = self.buttons[button_id].1.toggle();
        }
    }
}
