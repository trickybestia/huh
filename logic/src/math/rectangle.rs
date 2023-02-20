use num_traits::Num;

use super::{compare, Polygon, Vector2};

#[derive(Debug, Default, Clone)]
pub struct Rectangle<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: Num + Copy> Rectangle<T> {
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_center(center: &Vector2<T>, width: T, height: T) -> Self {
        let two = T::one() + T::one();

        Self {
            x: center.x - width / two,
            y: center.y - height / two,
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

impl From<&Rectangle<f32>> for Polygon {
    fn from(value: &Rectangle<f32>) -> Self {
        Polygon::new([
            Vector2::new(value.x, value.y),
            Vector2::new(value.x, value.y + value.height),
            Vector2::new(value.x + value.width, value.y + value.height),
            Vector2::new(value.x + value.width, value.y),
        ])
    }
}
