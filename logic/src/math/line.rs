use super::{compare, Vector2};

/// Line equation: ax + by + c = 0.
#[derive(Debug, Clone)]
pub struct Line {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Line {
    pub fn from_points(a: Vector2<f32>, b: Vector2<f32>) -> Line {
        if a.equals(&b) {
            panic!("Different points expected");
        }

        Self {
            a: b.y - a.y,
            b: a.x - b.x,
            c: b.x * a.y - a.x * b.y,
        }
    }

    pub fn intersection(&self, other: &Line) -> Option<Vector2<f32>> {
        let coefficient_product_difference = self.a * other.b - other.a * self.b;

        if compare(coefficient_product_difference, 0.0).is_eq() {
            None
        } else {
            Some(Vector2::new(
                (self.b * other.c - other.b * self.c) / coefficient_product_difference,
                (other.a * self.c - self.a * other.c) / coefficient_product_difference,
            ))
        }
    }
}
