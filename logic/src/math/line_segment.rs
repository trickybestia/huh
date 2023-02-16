use once_cell::unsync::OnceCell;

use super::{order, Line, Rectangle, Vector2};

#[derive(Debug)]
pub struct LineSegment {
    a: Vector2<f32>,
    b: Vector2<f32>,
    line: OnceCell<Line>,
    bounding_rectangle: OnceCell<Rectangle<f32>>,
}

impl LineSegment {
    pub fn new(a: Vector2<f32>, b: Vector2<f32>) -> Self {
        Self {
            a,
            b,
            line: OnceCell::new(),
            bounding_rectangle: OnceCell::new(),
        }
    }

    pub fn a(&self) -> &Vector2<f32> {
        &self.a
    }

    pub fn b(&self) -> &Vector2<f32> {
        &self.b
    }

    pub fn line(&self) -> &Line {
        if self.line.get().is_none() {
            _ = self.line.set(Line::from_points(self.a, self.b));
        }

        self.line.get().unwrap()
    }

    pub fn bounding_rectangle(&self) -> &Rectangle<f32> {
        if self.bounding_rectangle.get().is_none() {
            let (min_x, max_x) = order((self.a.x, self.b.x));
            let (min_y, max_y) = order((self.a.y, self.b.y));

            _ = self.bounding_rectangle.set(Rectangle::new(
                min_x,
                min_y,
                max_x - min_x,
                max_y - min_y,
            ));
        }

        self.bounding_rectangle.get().unwrap()
    }
}
