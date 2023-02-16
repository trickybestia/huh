use num_traits::Num;

use super::{compare, Vector2};

#[derive(Debug, Default)]
pub struct Rectangle<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: Num + Copy> Rectangle<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Rectangle<f32> {
    pub fn contains_point(&self, point: &Vector2<f32>) -> bool {
        compare(point.x, self.x).is_ge()
            && compare(point.y, self.y).is_ge()
            && compare(point.x, self.x + self.width).is_le()
            && compare(point.y, self.y + self.height).is_le()
    }
}
