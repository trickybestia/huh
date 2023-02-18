use web_sys::WebGlRenderingContext;

use crate::{engine::Scene, mouse_handler::MouseHandler};

pub struct GameScene {}

impl GameScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for GameScene {
    fn render(&mut self, renderer: &mut crate::engine::WebGlRenderer) -> Option<Box<dyn Scene>> {
        let gl = renderer.gl();

        gl.clear_color(0.2, 0.2, 0.2, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        None
    }

    fn on_drag(
        &mut self,
        renderer: &mut crate::engine::WebGlRenderer,
        _position: &crate::math::Vector2<i32>,
        position_delta: &crate::math::Vector2<i32>,
    ) {
        MouseHandler::on_drag(renderer, position_delta);
    }

    fn on_wheel(&mut self, renderer: &mut crate::engine::WebGlRenderer, value: i32) {
        MouseHandler::on_wheel(renderer, value);
    }
}
