use js_sys::Float32Array;
use web_sys::WebGlRenderingContext;

use crate::{
    engine::{
        shader::SimpleShader,
        webgl_renderer::{Color, RenderingSettings},
        WebGlRenderer,
    },
    math::Polygon,
};

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
        let shader = SimpleShader::activate(gl);

        gl.uniform1f(Some(shader.scale()), settings.scale);
        gl.uniform1f(Some(shader.aspect_ratio()), settings.aspect_ratio);
        gl.uniform1f(Some(shader.z_coordinate()), z_coordinate);
        gl.uniform2f(
            Some(shader.camera_position()),
            settings.camera_position.x,
            settings.camera_position.y,
        );
        gl.uniform4f(Some(shader.color()), color.r, color.g, color.b, color.a);

        gl.enable_vertex_attrib_array(shader.position());

        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        let mut verticies = Vec::with_capacity(polygon.points().len() * 2);

        for point in polygon.points() {
            verticies.push(point.x);
            verticies.push(point.y);
        }

        unsafe {
            let verticies_float_array = Float32Array::view(verticies.as_slice());

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &verticies_float_array,
                WebGlRenderingContext::STREAM_DRAW,
            );
        }

        gl.vertex_attrib_pointer_with_i32(
            shader.position(),
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLE_FAN,
            0,
            polygon.points().len() as i32,
        );
    }
}
