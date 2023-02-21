use once_cell::unsync::OnceCell;
use web_sys::WebGlRenderingContext;

use crate::{
    engine::{
        webgl_renderer::{
            shaders::TextureShader, vec2_webgl_buffer::Vec2WebGlBuffer, RenderingSettings, Texture,
        },
        WebGlRenderer,
    },
    math::Vector2,
};

static mut TEXTURE_COORDINATES_BUFFER: OnceCell<Vec2WebGlBuffer> = OnceCell::new();

fn texture_coordinates_buffer(gl: &WebGlRenderingContext) -> &Vec2WebGlBuffer {
    unsafe {
        if TEXTURE_COORDINATES_BUFFER.get().is_none() {
            let points = [
                Vector2::new(0.0, 0.0),
                Vector2::new(0.0, 1.0),
                Vector2::new(1.0, 1.0),
                Vector2::new(1.0, 0.0),
            ];

            _ = TEXTURE_COORDINATES_BUFFER.set(Vec2WebGlBuffer::from_points(gl, &points));
        }

        TEXTURE_COORDINATES_BUFFER.get().unwrap()
    }
}

pub trait DrawTriangleFanTextureExt {
    fn draw_triangle_fan_texture(
        &self,
        bounding_rectangle_vertex_buffer: &Vec2WebGlBuffer,
        z_coordinate: f32,
        texture: &Texture,
        settings: &RenderingSettings,
    );
}

impl DrawTriangleFanTextureExt for WebGlRenderer {
    fn draw_triangle_fan_texture(
        &self,
        bounding_rectangle_vertex_buffer: &Vec2WebGlBuffer,
        z_coordinate: f32,
        texture: &Texture,
        settings: &RenderingSettings,
    ) {
        if bounding_rectangle_vertex_buffer.points_count() != 4 {
            panic!("Expected Vec2WebGlBuffer with 4 points")
        }

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

        bounding_rectangle_vertex_buffer.bind_to_attribute(gl, shader.position());
        texture_coordinates_buffer(gl).bind_to_attribute(gl, shader.texture_coordinates());

        gl.active_texture(WebGlRenderingContext::TEXTURE0);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(texture.texture(gl)));

        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);
    }
}
