precision highp float;

attribute vec2 position;
attribute vec2 texture_coordinates;

uniform float aspect_ratio;
uniform float scale;
uniform float z_coordinate;
uniform vec2 camera_position;

varying vec2 v_texture_coordinates;

void main() {
    v_texture_coordinates = texture_coordinates;

    vec2 vertex_position = (position - camera_position) * scale;

    gl_Position = vec4(vertex_position.x / aspect_ratio, vertex_position.y, z_coordinate, 1.0);
}
