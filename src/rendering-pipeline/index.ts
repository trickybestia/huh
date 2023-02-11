import Vector2 from "../math/vector2";
import scene from "../scene";
import { Raycaster } from "./raycaster";
import { RenderingPipelineSettings } from "./settings";
import WebGLRenderer from "./webgl-renderer";

class RenderingPipeline {
  private readonly raycaster: Raycaster;
  private readonly renderer: WebGLRenderer;
  private readonly glContext: WebGLRenderingContext;
  private readonly pipelineSettings: RenderingPipelineSettings;

  public constructor(
    glContext: WebGLRenderingContext,
    pipelineSettings: RenderingPipelineSettings
  ) {
    this.raycaster = new Raycaster();
    this.renderer = new WebGLRenderer(glContext, pipelineSettings);
    this.glContext = glContext;
    this.pipelineSettings = pipelineSettings;
  }

  public render() {
    const gl = this.glContext;

    this.raycaster.clear();

    gl.clearColor(0.2, 0.2, 0.2, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);

    scene.gameObjects.forEach((gameObject) => {
      gameObject.draw && gameObject.draw(this.renderer, this.raycaster);
    });

    this.raycaster.sort();
  }

  public onClick(worldPosition: Vector2) {
    const intersection = this.raycaster.getContainingPolygonId(worldPosition);

    if (intersection) {
      const [gameObject, polygonID] = intersection;

      gameObject.onClick && gameObject.onClick(polygonID);
    }
  }

  public screenToWorldPosition(screenPosition: Vector2): Vector2 {
    const [, , canvasWidth, canvasHeight] = this.glContext.getParameter(
      this.glContext.VIEWPORT
    ) as number[];

    return screenPosition
      .multiplyComponents(2 / canvasWidth, -2 / canvasHeight)
      .add(new Vector2(-1, 1))
      .multiplyComponents(this.pipelineSettings.aspectRatio, 1)
      .divide(this.pipelineSettings.scale)
      .add(this.pipelineSettings.cameraPosition);
  }
}

export default RenderingPipeline;
