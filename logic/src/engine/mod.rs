pub mod raycaster;
mod scene;
pub mod webgl_renderer;

pub use scene::Scene;
pub use webgl_renderer::WebGlRenderer;

use crate::math::Vector2;

pub struct Engine {
    renderer: WebGlRenderer,
    scene: Option<Box<dyn Scene>>,
}

impl Engine {
    pub fn new(renderer: WebGlRenderer) -> Self {
        Self {
            renderer,
            scene: None,
        }
    }

    pub fn set_scene(&mut self, scene: Box<dyn Scene>) {
        self.renderer.reset_settings();

        self.scene = Some(scene);
    }

    pub fn render(&mut self) {
        if let Some(scene) = &mut self.scene {
            self.renderer.calculate_rendering_settings();

            if let Some(new_scene) = scene.render(&mut self.renderer) {
                self.set_scene(new_scene);
            }
        }
    }

    pub fn on_click(&mut self, position: &Vector2<i32>) {
        if let Some(scene) = &mut self.scene {
            scene.on_click(&mut self.renderer, position);
        }
    }

    pub fn on_drag(&mut self, position: &Vector2<i32>, position_delta: &Vector2<i32>) {
        if let Some(scene) = &mut self.scene {
            scene.on_drag(&mut self.renderer, position, position_delta);
        }
    }

    pub fn on_wheel(&mut self, value: i32) {
        if let Some(scene) = &mut self.scene {
            scene.on_wheel(&mut self.renderer, value);
        }
    }
}
