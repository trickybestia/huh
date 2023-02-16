use crate::math::Vector2;

#[derive(Default, Debug)]
pub struct RenderingSettings {
    /// Width to height ratio.
    pub aspect_ratio: f32,
    pub scale: f32,
    pub camera_position: Vector2<f32>,
}
