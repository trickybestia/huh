use crate::math::Vector2;

#[derive(Debug)]
pub struct RenderingSettings {
    /// Width to height ratio.
    pub aspect_ratio: f32,
    pub scale: f32,
    pub camera_position: Vector2<f32>,
}

impl Default for RenderingSettings {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            scale: 1.0,
            camera_position: Vector2::new(0.0, 0.0),
        }
    }
}
