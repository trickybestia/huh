attribute vec2 position;

uniform highp float aspectRatio;
uniform highp float scale;
uniform highp vec2 cameraPosition;

void main() {
  vec2 vertexPosition = position - cameraPosition;
  vertexPosition *= scale;

  gl_Position = vec4(vertexPosition[0] / aspectRatio, vertexPosition[1], 0.0, 1.0);
}
