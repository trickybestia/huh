use crate::{
    engine::webgl_renderer::{extensions::DrawRectangleExt, BROWN, WHITE},
    math::Rectangle,
    mouse_handler::MouseHandler,
};
use crate::{
    engine::{Raycaster, Scene, WebGlRenderer},
    math::Vector2,
};
use web_sys::WebGlRenderingContext;

use super::{EditorScene, GameScene};

#[derive(Clone, PartialEq, Eq)]
enum PolygonID {
    StartButton,
    EditorButton,
}

enum NextScene {
    Game,
    Editor,
}

pub struct MenuScene {
    raycaster: Raycaster<PolygonID>,
    start_button: Rectangle<f32>,
    editor_button: Rectangle<f32>,
    next_scene: Option<NextScene>,
}

impl MenuScene {
    pub fn new() -> Self {
        let mut raycaster = Raycaster::new();
        let start_button = Rectangle::from_center(&Vector2::new(0.0, 0.0), 0.6, 0.2);
        let editor_button = Rectangle::from_center(&Vector2::new(0.0, -0.8), 0.3, 0.1);

        raycaster.add_polygon(&start_button, PolygonID::StartButton, 0.0);
        raycaster.add_polygon(&editor_button, PolygonID::EditorButton, 0.0);

        raycaster.sort();

        Self {
            raycaster,
            start_button,
            editor_button,
            next_scene: None,
        }
    }
}

impl Scene for MenuScene {
    fn render(&mut self, renderer: &mut WebGlRenderer) -> Option<Box<dyn Scene>> {
        if let Some(next_scene) = &self.next_scene {
            match next_scene {
                NextScene::Game => return Some(Box::new(GameScene::new())),
                NextScene::Editor => return Some(Box::new(EditorScene::new())),
            }
        }

        let gl = renderer.gl();

        gl.clear_color(0.2, 0.2, 0.2, 1.0);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        renderer.draw_rectangle(
            &self.start_button,
            0.0,
            &WHITE,
            renderer.rendering_settings(),
        );
        renderer.draw_rectangle(
            &self.editor_button,
            0.0,
            &BROWN,
            renderer.rendering_settings(),
        );

        None
    }

    fn on_click(&mut self, renderer: &mut WebGlRenderer, position: &Vector2<i32>) {
        let world_position = renderer.screen_to_world_position(position);

        if let Some(polygon_id) = self.raycaster.raycast(&world_position) {
            match polygon_id {
                PolygonID::StartButton => self.next_scene = Some(NextScene::Game),
                PolygonID::EditorButton => self.next_scene = Some(NextScene::Editor),
            }
        }
    }
}
