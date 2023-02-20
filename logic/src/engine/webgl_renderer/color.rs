pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const BROWN: Color = Color::new(0.545, 0.27, 0.075, 1.0);
