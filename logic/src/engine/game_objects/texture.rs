use std::rc::Rc;

use once_cell::unsync::OnceCell;

use crate::{
    engine::{
        game_object::GameObject,
        webgl_renderer::{
            self, extensions::DrawTriangleFanTextureExt, vec2_webgl_buffer::Vec2WebGlBuffer,
        },
    },
    math::{self, Rectangle},
};

pub struct Texture {
    texture: &'static webgl_renderer::Texture,
    bounding_rectangle: Rc<Rectangle<f32>>,
    z_coordinate: f32,
    vertex_buffer: OnceCell<Vec2WebGlBuffer>,
}

impl Texture {
    pub fn new(
        bounding_rectangle: Rectangle<f32>,
        texture: &'static webgl_renderer::Texture,
        z_coordinate: f32,
    ) -> Self {
        Self {
            texture,
            bounding_rectangle: Rc::new(bounding_rectangle),
            z_coordinate,
            vertex_buffer: OnceCell::new(),
        }
    }

    pub fn bounding_rectangle(&self) -> &Rc<Rectangle<f32>> {
        &self.bounding_rectangle
    }
}

impl GameObject for Texture {
    fn render(&mut self, renderer: &mut webgl_renderer::WebGlRenderer) {
        let gl = renderer.gl();

        if self.vertex_buffer.get().is_none() {
            _ = self.vertex_buffer.set(Vec2WebGlBuffer::from_points(
                gl,
                math::Polygon::from(self.bounding_rectangle.as_ref()).points(),
            ));
        }

        let vertex_buffer = self.vertex_buffer.get().unwrap();

        renderer.draw_triangle_fan_texture(
            vertex_buffer,
            self.z_coordinate,
            self.texture,
            renderer.rendering_settings(),
        )
    }
}
