use num_traits::clamp;

use crate::{engine::WebGlRenderer, math::Vector2};

const CAMERA_MOVEMENT_MULTIPLIER: f32 = 0.002;
const SCALE_CHANGE_MULTIPLIER: f32 = 0.002;
const MIN_SCALE: f32 = 0.5;
const MAX_SCALE: f32 = 5.0;

pub struct MouseHandler;

impl MouseHandler {
    pub fn on_drag(renderer: &mut WebGlRenderer, position_delta: &Vector2<i32>) {
        let camera_position_delta = Vector2::new(position_delta.x as f32, position_delta.y as f32)
            .multiply(CAMERA_MOVEMENT_MULTIPLIER)
            .multiply_components(-1.0, 1.0)
            .divide(renderer.rendering_settings().scale);

        renderer.set_camera_position(
            renderer
                .rendering_settings()
                .camera_position
                .add_vector(&camera_position_delta),
        );
    }

    pub fn on_wheel(renderer: &mut WebGlRenderer, value: i32) {
        let scale_delta = -value as f32 * SCALE_CHANGE_MULTIPLIER;

        renderer.set_scale(clamp(
            renderer.rendering_settings().scale + scale_delta,
            MIN_SCALE,
            MAX_SCALE,
        ));
    }
}
