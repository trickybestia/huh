use js_sys::Float32Array;
use web_sys::WebGlRenderingContext;

pub(super) fn bind_float32_vec2_buffer(
    gl: &WebGlRenderingContext,
    content: &[f32],
    attribute: u32,
) {
    if content.len() % 2 != 0 {
        panic!("Expected slice with even length");
    }

    let buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let float_content = Float32Array::view(content);

        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &float_content,
            WebGlRenderingContext::STREAM_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(attribute, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
}
