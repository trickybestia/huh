mod mouse_handler;
mod raycaster;
mod scene;
pub mod shader;
pub mod webgl_renderer;

pub use raycaster::Raycaster;
pub use scene::Scene;
pub use webgl_renderer::WebGlRenderer;

use crate::math::Vector2;

use self::mouse_handler::MouseHandler;

pub struct Engine {
    renderer: WebGlRenderer,
    scene: Option<Box<dyn Scene>>,
    mouse_handler: MouseHandler,
}

impl Engine {
    pub fn new(renderer: WebGlRenderer) -> Self {
        Self {
            renderer,
            scene: None,
            mouse_handler: MouseHandler::new(),
        }
    }

    pub fn set_scene(&mut self, scene: Box<dyn Scene>) {
        self.mouse_handler.reset();

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

    pub fn on_mouse_down(&mut self, position: &Vector2<i32>) {
        if let Some(scene) = &mut self.scene {
            self.mouse_handler
                .on_mouse_down(&self.renderer, scene.as_mut(), position);
        }
    }

    pub fn on_mouse_up(&mut self, position: &Vector2<i32>) {
        if let Some(scene) = &mut self.scene {
            self.mouse_handler
                .on_mouse_up(&self.renderer, scene.as_mut(), position);
        }
    }

    pub fn on_mouse_move(&mut self, position: &Vector2<i32>) {
        if let Some(scene) = &mut self.scene {
            self.mouse_handler
                .on_mouse_move(&self.renderer, scene.as_mut(), position);
        }
    }
}
