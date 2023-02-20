use image::GenericImageView;

use crate::{
    engine::webgl_renderer::Texture,
    math::{Rectangle, Vector2},
};

use super::RaycastTarget;

pub struct TextureRaycastTarget {
    bounding_rectangle: Rectangle<f32>,
    texture: &'static Texture,
}

impl TextureRaycastTarget {
    pub fn new(bounding_rectangle: Rectangle<f32>, texture: &'static Texture) -> Self {
        Self {
            bounding_rectangle,
            texture,
        }
    }
}

impl RaycastTarget for TextureRaycastTarget {
    fn contains_point(&self, point: &Vector2<f32>) -> bool {
        if !self.bounding_rectangle.contains_point(point) {
            return false;
        }

        let texel_position = Vector2::new(
            point.x - self.bounding_rectangle.x,
            self.bounding_rectangle.height - (point.y - self.bounding_rectangle.y),
        )
        .divide_components(
            self.bounding_rectangle.width,
            self.bounding_rectangle.height,
        )
        .multiply_components(
            self.texture.image().width() as f32,
            self.texture.image().height() as f32,
        );
        let texel_position = Vector2::new(
            texel_position.x.round() as u32,
            texel_position.y.round() as u32,
        );

        self.texture
            .image()
            .get_pixel(texel_position.x, texel_position.y)
            .0[3]
            != 0
    }
}
