precision highp float;

varying vec2 v_texture_coordinates;

uniform sampler2D sampler;

void main() {
    gl_FragColor = texture2D(sampler, v_texture_coordinates);
}
