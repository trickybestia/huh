use web_sys::WebGlRenderingContext;

use crate::engine::{
    webgl_renderer::{
        shaders::PolygonShader, vec2_webgl_buffer::Vec2WebGlBuffer, Color, RenderingSettings,
    },
    WebGlRenderer,
};

pub trait DrawTriangleFanPolygonExt {
    fn draw_triangle_fan_polygon(
        &self,
        vertex_buffer: &Vec2WebGlBuffer,
        z_coordinate: f32,
        color: &Color,
        settings: &RenderingSettings,
    );
}

impl DrawTriangleFanPolygonExt for WebGlRenderer {
    fn draw_triangle_fan_polygon(
        &self,
        vertex_buffer: &Vec2WebGlBuffer,
        z_coordinate: f32,
        color: &Color,
        settings: &RenderingSettings,
    ) {
        let gl = self.gl();
        let shader = PolygonShader::activate(gl);

        gl.uniform1f(Some(shader.scale()), settings.scale);
        gl.uniform1f(Some(shader.aspect_ratio()), settings.aspect_ratio);
        gl.uniform1f(Some(shader.z_coordinate()), z_coordinate);
        gl.uniform2f(
            Some(shader.camera_position()),
            settings.camera_position.x,
            settings.camera_position.y,
        );
        gl.uniform4f(Some(shader.color()), color.r, color.g, color.b, color.a);

        vertex_buffer.bind_to_attribute(gl, shader.position());

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLE_FAN,
            0,
            vertex_buffer.points_count() as i32,
        );
    }
}
