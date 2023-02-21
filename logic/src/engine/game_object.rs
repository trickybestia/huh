use super::WebGlRenderer;

pub trait GameObject {
    fn render(&mut self, renderer: &mut WebGlRenderer);
}
