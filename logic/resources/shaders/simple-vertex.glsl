attribute vec2 position;

uniform highp float aspect_ratio;
uniform highp float scale;
uniform highp float z_coordinate;
uniform highp vec2 camera_position;

void main() {
    vec2 vertex_position = (position - camera_position) * scale;

    gl_Position = vec4(vertex_position.x / aspect_ratio, vertex_position.y, z_coordinate, 1.0);
}
