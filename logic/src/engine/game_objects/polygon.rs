use std::rc::Rc;

use once_cell::unsync::OnceCell;

use crate::{
    engine::{
        game_object::GameObject,
        webgl_renderer::{
            extensions::DrawTriangleFanPolygonExt, vec2_webgl_buffer::Vec2WebGlBuffer, Color,
        },
        WebGlRenderer,
    },
    math,
};

pub struct Polygon {
    polygon: Rc<math::Polygon>,
    color: Color,
    vertex_buffer: OnceCell<Vec2WebGlBuffer>,
    z_coordinate: f32,
}

impl Polygon {
    pub fn new(polygon: impl Into<math::Polygon>, z_index: f32, color: Color) -> Self {
        let polygon: Rc<math::Polygon> = Rc::new(polygon.into());

        Self {
            polygon,
            color,
            vertex_buffer: OnceCell::new(),
            z_coordinate: z_index,
        }
    }

    pub fn polygon(&self) -> &Rc<math::Polygon> {
        &self.polygon
    }
}

impl GameObject for Polygon {
    fn render(&mut self, renderer: &mut WebGlRenderer) {
        let gl = renderer.gl();

        if self.vertex_buffer.get().is_none() {
            _ = self
                .vertex_buffer
                .set(Vec2WebGlBuffer::from_points(gl, self.polygon.points()));
        }

        let vertex_buffer = self.vertex_buffer.get().unwrap();

        renderer.draw_triangle_fan_polygon(
            vertex_buffer,
            self.z_coordinate,
            &self.color,
            renderer.rendering_settings(),
        );
    }
}
