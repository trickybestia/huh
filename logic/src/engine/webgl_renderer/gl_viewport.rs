use js_sys::Int32Array;
use web_sys::WebGlRenderingContext;

use crate::math::Rectangle;

pub fn get_viewport(gl: &WebGlRenderingContext) -> Rectangle<i32> {
    let viewport: Int32Array = gl
        .get_parameter(WebGlRenderingContext::VIEWPORT)
        .unwrap()
        .into();
    let mut result = [0i32; 4];

    viewport.copy_to(&mut result);

    Rectangle {
        x: result[0],
        y: result[1],
        width: result[2],
        height: result[3],
    }
}
