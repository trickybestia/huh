use num_traits::Num;

use super::compare;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn divide(&self, value: T) -> Self {
        Self::new(self.x / value, self.y / value)
    }

    pub fn add_vector(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn multiply_components(&self, x_multiplier: T, y_multiplier: T) -> Self {
        Self::new(self.x * x_multiplier, self.y * y_multiplier)
    }
}

impl Vector2<f32> {
    pub fn equals(&self, other: &Self) -> bool {
        compare(self.x, other.x).is_eq() && compare(self.y, other.y).is_eq()
    }
}
