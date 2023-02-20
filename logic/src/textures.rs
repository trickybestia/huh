use crate::engine::webgl_renderer::Texture;

pub static mut TRICKYBESTIA: Texture =
    Texture::new(include_bytes!("../resources/textures/trickybestia.png"));
