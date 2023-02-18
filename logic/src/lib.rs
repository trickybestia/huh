mod engine;
mod math;
mod mouse_handler;
mod resources;
mod scenes;
mod utils;

use engine::{Engine, WebGlRenderer};
use log::Level;
use math::Vector2;
use once_cell::unsync::OnceCell;
use scenes::MenuScene;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

static mut ENGINE: OnceCell<Engine> = OnceCell::new();

#[wasm_bindgen]
pub fn create_engine(gl_context: WebGlRenderingContext) {
    let renderer = WebGlRenderer::new(gl_context);

    unsafe {
        if ENGINE.get().is_none() {
            let mut engine = Engine::new(renderer);

            engine.set_scene(Box::new(MenuScene::new()));

            _ = ENGINE.set(engine);
        }
    }
}

#[wasm_bindgen]
pub fn render() {
    unsafe {
        if let Some(engine) = ENGINE.get_mut() {
            engine.render();
        }
    }
}

#[wasm_bindgen]
pub fn on_click(x: i32, y: i32) {
    unsafe {
        if let Some(engine) = ENGINE.get_mut() {
            engine.on_click(&Vector2::new(x, y));
        }
    }
}

#[wasm_bindgen]
pub fn on_drag(x: i32, y: i32, movement_x: i32, movement_y: i32) {
    unsafe {
        if let Some(engine) = ENGINE.get_mut() {
            engine.on_drag(&Vector2::new(x, y), &Vector2::new(movement_x, movement_y));
        }
    }
}

#[wasm_bindgen]
pub fn on_wheel(value: i32) {
    unsafe {
        if let Some(engine) = ENGINE.get_mut() {
            engine.on_wheel(value);
        }
    }
}

#[wasm_bindgen(start)]
fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    utils::set_panic_hook();
}
