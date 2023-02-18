use js_sys::Float32Array;
use web_sys::WebGlRenderingContext;

use crate::{
    engine::{
        shader::SimpleShader,
        webgl_renderer::{Color, RenderingSettings},
        WebGlRenderer,
    },
    math::Rectangle,
};

pub trait DrawRectangleExt {
    fn draw_rectangle(
        &self,
        rectangle: &Rectangle<f32>,
        z_coordinate: f32,
        color: &Color,
        settings: &RenderingSettings,
    );
}

impl DrawRectangleExt for WebGlRenderer {
    fn draw_rectangle(
        &self,
        rectangle: &Rectangle<f32>,
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

        let vericies: [f32; 8] = [
            rectangle.x,
            rectangle.y,
            rectangle.x,
            rectangle.y + rectangle.height,
            rectangle.x + rectangle.width,
            rectangle.y,
            rectangle.x + rectangle.width,
            rectangle.y + rectangle.height,
        ];

        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let verticies_float_array = Float32Array::view(&vericies);

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

        gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);
    }
}
