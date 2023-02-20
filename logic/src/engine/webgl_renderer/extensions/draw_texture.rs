use web_sys::WebGlRenderingContext;

use crate::{
    engine::{
        webgl_renderer::{shaders::TextureShader, RenderingSettings, Texture},
        WebGlRenderer,
    },
    math::Rectangle,
};

use super::utils::bind_float32_vec2_buffer;

pub trait DrawTextureExt {
    fn draw_texture(
        &self,
        rectangle: &Rectangle<f32>,
        z_coordinate: f32,
        texture: &Texture,
        settings: &RenderingSettings,
    );
}

impl DrawTextureExt for WebGlRenderer {
    fn draw_texture(
        &self,
        rectangle: &Rectangle<f32>,
        z_coordinate: f32,
        texture: &Texture,
        settings: &RenderingSettings,
    ) {
        let gl = self.gl();
        let shader = TextureShader::activate(gl);

        gl.uniform1f(Some(shader.scale()), settings.scale);
        gl.uniform1f(Some(shader.aspect_ratio()), settings.aspect_ratio);
        gl.uniform1f(Some(shader.z_coordinate()), z_coordinate);
        gl.uniform1i(Some(shader.sampler()), 0);
        gl.uniform2f(
            Some(shader.camera_position()),
            settings.camera_position.x,
            settings.camera_position.y,
        );

        let verticies: [f32; 8] = [
            rectangle.x,
            rectangle.y,
            rectangle.x,
            rectangle.y + rectangle.height,
            rectangle.x + rectangle.width,
            rectangle.y,
            rectangle.x + rectangle.width,
            rectangle.y + rectangle.height,
        ];

        let verticies_buffer =
            bind_float32_vec2_buffer(gl, verticies.as_slice(), shader.position());

        let texture_coordinates: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];

        bind_float32_vec2_buffer(
            gl,
            texture_coordinates.as_slice(),
            shader.texture_coordinates(),
        );

        gl.active_texture(WebGlRenderingContext::TEXTURE0);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(texture.texture(gl)));

        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
        drop(verticies_buffer);
    }
}
