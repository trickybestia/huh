import fragmentShaderSource from "../../../shaders/simple-fragment.glsl";
import vertexShaderSource from "../../../shaders/simple-vertex.glsl";
import Vector2 from "../../math/vector2";
import Color from "../../ui/color";
import { RenderingPipelineSettings } from "../settings";

import compileShader from "./compile-shader";

class WebGLRenderer {
  public readonly glContext: WebGLRenderingContext;

  private readonly pipelineSettings: RenderingPipelineSettings;

  private positionAttribute?: GLint;
  private colorUniform?: WebGLUniformLocation;
  private aspectRatioUniform?: WebGLUniformLocation;
  private scaleUniform?: WebGLUniformLocation;
  private cameraPositionUniform?: WebGLUniformLocation;

  public constructor(
    glContext: WebGLRenderingContext,
    pipelineSettings: RenderingPipelineSettings
  ) {
    this.glContext = glContext;
    this.pipelineSettings = pipelineSettings;

    this.initShaders();

    this.updateScale();
    this.updateAspectRatio();
    this.updateCameraPosition();

    pipelineSettings.addPropertyChangedEventHandler(
      this.onRenderingPipelineSettingsPropertyChanged.bind(this)
    );
  }

  public drawSquare(x: number, y: number, sideSize: number, color: Color) {
    const leftX = x - sideSize / 2;
    const bottomY = y - sideSize / 2;
    const rightX = leftX + sideSize;
    const topY = bottomY + sideSize;

    /* eslint-disable prettier/prettier */
    const verticies = [
      leftX, bottomY,
      leftX, topY,
      rightX, bottomY,
      rightX, topY,
    ];
    /* eslint-enable prettier/prettier */

    const gl = this.glContext;

    gl.uniform3f(this.colorUniform!, color.r, color.g, color.b);

    const vertexBuffer = gl.createBuffer();

    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(verticies), gl.STREAM_DRAW);
    gl.vertexAttribPointer(this.positionAttribute!, 2, gl.FLOAT, false, 0, 0);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
  }

  public drawLine(a: Vector2, b: Vector2, color: Color) {
    /* eslint-disable prettier/prettier */
    const verticies = [
      a.x, a.y,
      b.x, b.y
    ];
    /* eslint-enable prettier/prettier */

    const gl = this.glContext;

    gl.uniform3f(this.colorUniform!, color.r, color.g, color.b);

    const vertexBuffer = gl.createBuffer();

    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(verticies), gl.STREAM_DRAW);
    gl.vertexAttribPointer(this.positionAttribute!, 2, gl.FLOAT, false, 0, 0);
    gl.drawArrays(gl.LINES, 0, 2);
  }

  private onRenderingPipelineSettingsPropertyChanged(
    propertyName: keyof RenderingPipelineSettings
  ) {
    switch (propertyName) {
      case "scale":
        this.updateScale();

        break;
      case "aspectRatio":
        this.updateAspectRatio();

        break;
      case "cameraPosition":
        this.updateCameraPosition();

        break;
    }
  }

  private updateScale() {
    this.glContext.uniform1f(this.scaleUniform!, this.pipelineSettings.scale);
  }

  private updateAspectRatio() {
    this.glContext.uniform1f(
      this.aspectRatioUniform!,
      this.pipelineSettings.aspectRatio
    );
  }

  private updateCameraPosition() {
    const cameraPosition = this.pipelineSettings.cameraPosition;

    this.glContext.uniform2f(
      this.cameraPositionUniform!,
      cameraPosition.x,
      cameraPosition.y
    );
  }

  private initShaders() {
    const gl = this.glContext;

    const fragmentShader = compileShader(
      gl,
      fragmentShaderSource,
      gl.FRAGMENT_SHADER
    );
    const vertexShader = compileShader(
      gl,
      vertexShaderSource,
      gl.VERTEX_SHADER
    );

    const shaderProgram = gl.createProgram()!;

    gl.attachShader(shaderProgram, fragmentShader);
    gl.attachShader(shaderProgram, vertexShader);

    gl.linkProgram(shaderProgram);

    gl.useProgram(shaderProgram);

    this.positionAttribute = gl.getAttribLocation(shaderProgram, "position");

    gl.enableVertexAttribArray(this.positionAttribute);

    this.colorUniform = gl.getUniformLocation(shaderProgram, "color")!;
    this.aspectRatioUniform = gl.getUniformLocation(
      shaderProgram,
      "aspectRatio"
    )!;
    this.scaleUniform = gl.getUniformLocation(shaderProgram, "scale")!;
    this.cameraPositionUniform = gl.getUniformLocation(
      shaderProgram,
      "cameraPosition"
    )!;
  }
}

export default WebGLRenderer;
