use crate::declare_shader;

declare_shader!(
    pub SimpleShader,
    crate::resources::SIMPLE_VERTEX_SHADER_SOURCE,
    crate::resources::SIMPLE_FRAGMENT_SHADER_SOURCE,
    (position),
    (scale,
    aspect_ratio,
    camera_position,
    z_coordinate,
    color)
);
