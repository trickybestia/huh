use crate::math::Vector2;

use super::{Scene, WebGlRenderer};

struct DragInfo {
    pub start_position: Vector2<i32>,
    pub previous_position: Vector2<i32>,
}

pub struct MouseHandler {
    mouse_moved: bool,
    drag_info: Option<DragInfo>,
}

impl MouseHandler {
    pub fn new() -> Self {
        Self {
            mouse_moved: false,
            drag_info: None,
        }
    }

    pub fn reset(&mut self) {
        self.mouse_moved = false;
        self.drag_info = None;
    }

    pub fn on_mouse_down(
        &mut self,
        _renderer: &WebGlRenderer,
        _scene: &mut dyn Scene,
        position: &Vector2<i32>,
    ) {
        self.mouse_moved = false;
        self.drag_info = Some(DragInfo {
            start_position: *position,
            previous_position: *position,
        });
    }

    pub fn on_mouse_up(
        &mut self,
        renderer: &WebGlRenderer,
        scene: &mut dyn Scene,
        position: &Vector2<i32>,
    ) {
        self.drag_info = None;

        if !self.mouse_moved {
            scene.on_click(renderer, position);
        }
    }

    pub fn on_mouse_move(
        &mut self,
        renderer: &WebGlRenderer,
        scene: &mut dyn Scene,
        position: &Vector2<i32>,
    ) {
        if let Some(drag_info) = &mut self.drag_info {
            self.mouse_moved = true;

            scene.on_drag(
                renderer,
                &drag_info.start_position,
                position,
                &drag_info.previous_position,
            );

            drag_info.previous_position = *position;
        }
    }
}
