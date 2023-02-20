use crate::declare_shader;

declare_shader!(
    pub TextureShader,
    include_str!("../../../../resources/shaders/texture/vertex.glsl"),
    include_str!("../../../../resources/shaders/texture/fragment.glsl"),
    (position, texture_coordinates),
    (scale,
    aspect_ratio,
    camera_position,
    z_coordinate,
    sampler)
);
