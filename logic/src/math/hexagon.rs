use super::{Polygon, Vector2};

pub struct Hexagon {
    pub center: Vector2<f32>,
    pub circle_radius: f32,
}

impl Hexagon {
    pub const fn new(center: Vector2<f32>, circle_radius: f32) -> Self {
        Self {
            center,
            circle_radius,
        }
    }
}

const SIN_60_DEG: f32 = 0.8660254;
const COS_60_DEG: f32 = 0.5;

impl From<&Hexagon> for Polygon {
    fn from(value: &Hexagon) -> Self {
        let center = &value.center;
        let circle_radius = value.circle_radius;

        let vertical_coordinate_delta = circle_radius * SIN_60_DEG;
        let horizontal_coordinate_delta = circle_radius * COS_60_DEG;

        Polygon::new([
            Vector2::new(center.x - circle_radius, center.y),
            Vector2::new(
                center.x - horizontal_coordinate_delta,
                center.y + vertical_coordinate_delta,
            ),
            Vector2::new(
                center.x + horizontal_coordinate_delta,
                center.y + vertical_coordinate_delta,
            ),
            Vector2::new(center.x + circle_radius, center.y),
            Vector2::new(
                center.x + horizontal_coordinate_delta,
                center.y - vertical_coordinate_delta,
            ),
            Vector2::new(
                center.x - horizontal_coordinate_delta,
                center.y - vertical_coordinate_delta,
            ),
        ])
    }
}
