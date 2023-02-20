use web_sys::WebGlRenderingContext;

use crate::{
    engine::{
        webgl_renderer::{shaders::PolygonShader, Color, RenderingSettings},
        WebGlRenderer,
    },
    math::Polygon,
};

use super::utils::bind_float32_vec2_buffer;

pub trait DrawConvexPolygonExt {
    fn draw_convex_polygon(
        &self,
        polygon: &Polygon,
        z_coordinate: f32,
        color: &Color,
        settings: &RenderingSettings,
    );
}

impl DrawConvexPolygonExt for WebGlRenderer {
    fn draw_convex_polygon(
        &self,
        polygon: &Polygon,
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

        let mut verticies = Vec::with_capacity(polygon.points().len() * 2);

        for point in polygon.points() {
            verticies.push(point.x);
            verticies.push(point.y);
        }

        let verticies_buffer =
            bind_float32_vec2_buffer(gl, verticies.as_slice(), shader.position());

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLE_FAN,
            0,
            polygon.points().len() as i32,
        );

        drop(verticies_buffer);
    }
}
