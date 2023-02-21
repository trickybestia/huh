use crate::{
    engine::{
        game_objects::{ClickablePolygon, ClickableTexture},
        raycaster::Raycaster,
        webgl_renderer::{BROWN, WHITE},
        GameObject, Scene, WebGlRenderer,
    },
    global,
    math::{Rectangle, Vector2},
    mouse_handler::MouseHandler,
    textures,
};
use web_sys::WebGlRenderingContext;

use super::{EditorScene, GameScene};

#[derive(PartialEq, Eq, Hash, Clone)]
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
    test_button_bounding_rectangle: Rectangle<f32>,
    start_button: ClickablePolygon<TargetID>,
    editor_button: ClickablePolygon<TargetID>,
    test_button: Option<Box<dyn GameObject>>,
    next_scene: Option<NextScene>,
    test_button_state: TestButtonState,
}

impl MenuScene {
    pub fn new() -> Self {
        let raycaster = Raycaster::new();

        let start_button = ClickablePolygon::new(
            &Rectangle::from_center(0.0, 0.0, 0.6, 0.2),
            TargetID::StartButton,
            raycaster.clone(),
            0.0,
            WHITE,
        );
        let editor_button = ClickablePolygon::new(
            &Rectangle::from_center(0.0, -0.8, 0.3, 0.1),
            TargetID::EditorButton,
            raycaster.clone(),
            0.0,
            BROWN,
        );
        let test_button_bounding_rectangle = Rectangle::from_center(0.5, 0.5, 0.5, 0.5);

        let mut result = Self {
            raycaster,
            start_button,
            editor_button,
            test_button: None,
            test_button_bounding_rectangle,
            next_scene: None,
            test_button_state: TestButtonState::Transparent,
        };

        result.update_test_button();

        result
    }

    fn update_test_button(&mut self) {
        self.test_button = None;
        self.test_button = Some(match self.test_button_state {
            TestButtonState::Transparent => Box::new(ClickableTexture::new(
                self.test_button_bounding_rectangle.clone(),
                TargetID::TestButton,
                self.raycaster.clone(),
                global!(&textures::TRICKYBESTIA),
                0.0,
            )),
            TestButtonState::Opaque => Box::new(ClickablePolygon::new(
                &self.test_button_bounding_rectangle,
                TargetID::TestButton,
                self.raycaster.clone(),
                0.0,
                WHITE,
            )),
        });
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

        self.start_button.render(renderer);
        self.editor_button.render(renderer);
        self.test_button.as_mut().unwrap().render(renderer);

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

                    self.update_test_button();
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
