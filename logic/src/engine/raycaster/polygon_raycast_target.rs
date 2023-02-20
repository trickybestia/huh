use crate::math::{Polygon, Vector2};

use super::RaycastTarget;

impl RaycastTarget for Polygon {
    fn contains_point(&self, point: &Vector2<f32>) -> bool {
        self.contains_point(point)
    }
}
