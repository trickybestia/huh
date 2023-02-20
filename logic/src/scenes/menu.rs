use crate::{
    engine::{
        raycaster::{Raycaster, TextureRaycastTarget},
        webgl_renderer::{
            extensions::{DrawConvexPolygonExt, DrawTextureExt},
            BROWN, WHITE,
        },
    },
    math::{Polygon, Rectangle},
    mouse_handler::MouseHandler,
    textures,
};
use crate::{
    engine::{Scene, WebGlRenderer},
    math::Vector2,
};
use web_sys::WebGlRenderingContext;

use super::{EditorScene, GameScene};

#[derive(Clone, PartialEq, Eq)]
enum TargetID {
    StartButton,
    EditorButton,
    TestButton,
}

enum NextScene {
    Game,
    Editor,
}

enum TestButtonState {
    Transparent,
    Opaque,
}

pub struct MenuScene {
    raycaster: Raycaster<TargetID>,
    start_button: Polygon,
    editor_button: Polygon,
    test_button: Rectangle<f32>,
    next_scene: Option<NextScene>,
    test_button_state: TestButtonState,
}

impl MenuScene {
    pub fn new() -> Self {
        let raycaster = Raycaster::new();
        let start_button: Polygon =
            (&Rectangle::from_center(&Vector2::new(0.0, 0.0), 0.6, 0.2)).into();
        let editor_button: Polygon =
            (&Rectangle::from_center(&Vector2::new(0.0, -0.8), 0.3, 0.1)).into();
        let test_button = Rectangle::from_center(&Vector2::new(0.5, 0.5), 0.5, 0.5);

        let mut result = Self {
            raycaster,
            start_button,
            editor_button,
            test_button,
            next_scene: None,
            test_button_state: TestButtonState::Transparent,
        };

        result.add_raycast_targets();

        result.raycaster.sort();

        result
    }

    fn add_raycast_targets(&mut self) {
        self.raycaster
            .add_target(self.start_button.clone(), TargetID::StartButton, 0.0);
        self.raycaster
            .add_target(self.editor_button.clone(), TargetID::EditorButton, 0.0);

        match self.test_button_state {
            TestButtonState::Transparent => unsafe {
                self.raycaster.add_target(
                    TextureRaycastTarget::new(self.test_button.clone(), &textures::TRICKYBESTIA),
                    TargetID::TestButton,
                    0.0,
                );
            },
            TestButtonState::Opaque => self.raycaster.add_target(
                Polygon::from(&self.test_button),
                TargetID::TestButton,
                0.0,
            ),
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

        renderer.draw_convex_polygon(
            &self.start_button,
            0.0,
            &WHITE,
            renderer.rendering_settings(),
        );
        renderer.draw_convex_polygon(
            &self.editor_button,
            0.0,
            &BROWN,
            renderer.rendering_settings(),
        );
        match self.test_button_state {
            TestButtonState::Transparent => unsafe {
                renderer.draw_texture(
                    &self.test_button,
                    0.0,
                    &textures::TRICKYBESTIA,
                    renderer.rendering_settings(),
                );
            },
            TestButtonState::Opaque => renderer.draw_convex_polygon(
                &(&self.test_button).into(),
                0.0,
                &WHITE,
                renderer.rendering_settings(),
            ),
        }

        None
    }

    fn on_click(&mut self, renderer: &mut WebGlRenderer, position: &Vector2<i32>) {
        let world_position = renderer.screen_to_world_position(position);

        if let Some(target_id) = self.raycaster.raycast(&world_position) {
            match target_id {
                TargetID::StartButton => self.next_scene = Some(NextScene::Game),
                TargetID::EditorButton => self.next_scene = Some(NextScene::Editor),
                TargetID::TestButton => {
                    self.test_button_state = match self.test_button_state {
                        TestButtonState::Transparent => TestButtonState::Opaque,
                        TestButtonState::Opaque => TestButtonState::Transparent,
                    };

                    self.raycaster.clear();

                    self.add_raycast_targets();

                    self.raycaster.sort();
                }
            }
        }
    }

    fn on_drag(
        &mut self,
        renderer: &mut WebGlRenderer,
        _position: &Vector2<i32>,
        position_delta: &Vector2<i32>,
    ) {
        MouseHandler::on_drag(renderer, position_delta);
    }

    fn on_wheel(&mut self, renderer: &mut WebGlRenderer, value: i32) {
        MouseHandler::on_wheel(renderer, value);
    }
}
