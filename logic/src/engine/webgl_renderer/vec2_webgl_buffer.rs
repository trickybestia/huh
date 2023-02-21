use js_sys::Float32Array;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

use crate::math::Vector2;

pub struct Vec2WebGlBuffer {
    buffer: WebGlBuffer,
    points_count: usize,
}

impl Vec2WebGlBuffer {
    pub fn from_points(gl: &WebGlRenderingContext, points: &[Vector2<f32>]) -> Self {
        let mut verticies = Vec::with_capacity(points.len() * 2);

        for point in points {
            verticies.push(point.x);
            verticies.push(point.y);
        }

        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let verticies_array = Float32Array::view(&verticies);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &verticies_array,
                WebGlRenderingContext::STREAM_DRAW,
            );
        }

        Self {
            buffer,
            points_count: points.len(),
        }
    }

    pub fn bind_to_attribute(&self, gl: &WebGlRenderingContext, attribute: u32) {
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer));

        gl.vertex_attrib_pointer_with_i32(attribute, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    }

    pub fn buffer(&self) -> &WebGlBuffer {
        &self.buffer
    }

    pub fn points_count(&self) -> usize {
        self.points_count
    }
}
