use crate::math::Vector2;

use super::webgl_renderer::WebGlRenderer;

pub trait Scene {
    fn render(&mut self, renderer: &mut WebGlRenderer) -> Option<Box<dyn Scene>>;
    fn on_click(&mut self, _renderer: &mut WebGlRenderer, _position: &Vector2<i32>) {}
    fn on_drag(
        &mut self,
        _renderer: &mut WebGlRenderer,
        _position: &Vector2<i32>,
        _position_delta: &Vector2<i32>,
    ) {
    }
    fn on_wheel(&mut self, _renderer: &mut WebGlRenderer, _value: i32) {}
}
