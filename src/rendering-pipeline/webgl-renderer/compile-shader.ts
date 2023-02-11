type ShaderType =
  | WebGLRenderingContext["FRAGMENT_SHADER"]
  | WebGLRenderingContext["VERTEX_SHADER"];

const compileShader = (
  gl: WebGLRenderingContext,
  source: string,
  type: ShaderType
): WebGLShader => {
  const shader = gl.createShader(type)!;

  gl.shaderSource(shader, source);
  gl.compileShader(shader);

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    alert(
      "Failed to compile shader: " +
        gl.getShaderInfoLog(shader)! +
        "\n" +
        source
    );

    gl.deleteShader(shader);
  }

  return shader;
};

export default compileShader;
