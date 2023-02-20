mod color;
pub mod extensions;
mod gl_viewport;
mod rendering_settings;
mod shader;
mod shaders;
mod texture;

use web_sys::WebGlRenderingContext;

use crate::math::{Rectangle, Vector2};

pub use color::*;
pub use rendering_settings::RenderingSettings;
pub use texture::Texture;

use self::gl_viewport::get_viewport;

pub struct WebGlRenderer {
    gl: WebGlRenderingContext,
    viewport: Rectangle<i32>,
    rendering_settings: RenderingSettings,
}

impl WebGlRenderer {
    pub fn new(gl: WebGlRenderingContext) -> Self {
        gl.enable(WebGlRenderingContext::BLEND);
        gl.blend_func(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::DST_ALPHA,
        );

        Self {
            gl,
            viewport: Default::default(),
            rendering_settings: Default::default(),
        }
    }

    pub fn reset_settings(&mut self) {
        self.rendering_settings = RenderingSettings {
            aspect_ratio: self.rendering_settings.aspect_ratio,
            ..Default::default()
        };
    }

    pub fn calculate_rendering_settings(&mut self) {
        self.viewport = get_viewport(&self.gl);

        self.rendering_settings = RenderingSettings {
            aspect_ratio: self.viewport.width as f32 / self.viewport.height as f32,
            ..self.rendering_settings
        };
    }

    pub fn screen_to_world_position(&self, position: &Vector2<i32>) -> Vector2<f32> {
        let rendering_settings = self.rendering_settings();

        Vector2::new(position.x as f32, position.y as f32)
            .multiply_components(
                2.0 / self.viewport.width as f32,
                -2.0 / self.viewport.height as f32,
            )
            .add_vector(&Vector2::new(-1.0, 1.0))
            .multiply_components(rendering_settings.aspect_ratio, 1.0)
            .divide(rendering_settings.scale)
            .add_vector(&rendering_settings.camera_position)
    }

    pub fn rendering_settings(&self) -> &RenderingSettings {
        &self.rendering_settings
    }

    pub fn gl(&self) -> &WebGlRenderingContext {
        &self.gl
    }

    pub fn viewport(&self) -> &Rectangle<i32> {
        &self.viewport
    }

    pub fn set_camera_position(&mut self, camera_position: Vector2<f32>) {
        self.rendering_settings = RenderingSettings {
            camera_position,
            ..self.rendering_settings
        };
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.rendering_settings = RenderingSettings {
            scale,
            ..self.rendering_settings
        };
    }
}
