use crate::declare_shader;

declare_shader!(
    pub PolygonShader,
    include_str!("../../../../resources/shaders/polygon/vertex.glsl"),
    include_str!("../../../../resources/shaders/polygon/fragment.glsl"),
    (position),
    (scale,
    aspect_ratio,
    camera_position,
    z_coordinate,
    color)
);
