use once_cell::unsync::OnceCell;

use super::{compare, Line, LineSegment, Rectangle, Vector2};

pub struct Polygon {
    points: Vec<Vector2<f32>>,
    bounding_rectangle: OnceCell<Rectangle<f32>>,
    sides: OnceCell<Vec<LineSegment>>,
}

impl Polygon {
    pub fn new(points: Vec<Vector2<f32>>) -> Self {
        if points.len() < 3 {
            panic!("At least 3 points expected.");
        }

        Self {
            points,
            bounding_rectangle: OnceCell::new(),
            sides: OnceCell::new(),
        }
    }

    pub fn points(&self) -> &[Vector2<f32>] {
        &self.points
    }

    pub fn bounding_rectangle(&self) -> &Rectangle<f32> {
        if self.bounding_rectangle.get().is_none() {
            let start_point = self.points[0];

            let mut min_x = start_point.x;
            let mut min_y = start_point.y;
            let mut max_x = start_point.x;
            let mut max_y = start_point.y;

            for point in &self.points[1..] {
                min_x = f32::min(min_x, point.x);
                max_x = f32::max(max_x, point.x);
                min_y = f32::min(min_y, point.y);
                max_y = f32::max(max_y, point.y);
            }

            _ = self.bounding_rectangle.set(Rectangle::new(
                min_x,
                min_y,
                max_x - min_x,
                max_y - min_y,
            ));
        }

        self.bounding_rectangle.get().unwrap()
    }

    pub fn sides(&self) -> &[LineSegment] {
        if self.sides.get().is_none() {
            let mut sides = Vec::with_capacity(self.points.len());

            for i in 1..self.points.len() {
                sides.push(LineSegment::new(self.points[i - 1], self.points[i]));
            }

            sides.push(LineSegment::new(
                *self.points.last().unwrap(),
                *self.points.first().unwrap(),
            ));

            _ = self.sides.set(sides);
        }

        self.sides.get().unwrap()
    }

    pub fn contains_point(&self, point: &Vector2<f32>) -> bool {
        if !self.bounding_rectangle().contains_point(point) {
            return false;
        }

        let line = Line {
            a: 0.0,
            b: 1.0,
            c: -point.y,
        }; // Line parallel to X-axis

        let mut intersections_count = 0;

        for side in self.sides() {
            let intersection_point = line.intersection(side.line());

            if let Some(intersection_point) = intersection_point {
                if compare(intersection_point.x, point.x).is_ge()
                    && side
                        .bounding_rectangle()
                        .contains_point(&intersection_point)
                {
                    if intersection_point.equals(point) {
                        return true;
                    }

                    intersections_count += 1;
                }
            }
        }

        intersections_count % 2 == 1
    }
}
