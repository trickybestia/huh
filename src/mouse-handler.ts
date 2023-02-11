import Vector2 from "./math/vector2";
import RenderingPipeline from "./rendering-pipeline";
import { RenderingPipelineSettings } from "./rendering-pipeline/settings";

const CAMERA_MOVEMENT_MULTIPLIER = 0.002;
const SCALE_CHANGE_MULTIPLIER = 0.002;
const MIN_SCALE = 0.5;
const MAX_SCALE = 5;

type MouseEventHandler = (event: MouseEvent) => void;

class MouseHandler {
  private readonly renderingPipeline: RenderingPipeline;
  private readonly renderingPipelineSettings: RenderingPipelineSettings;

  private readonly boundMouseDownHandler: MouseEventHandler;
  private readonly boundMouseUpHandler: MouseEventHandler;
  private readonly boundMouseMoveHandler: MouseEventHandler;
  private readonly boundMouseWheelHandler: (event: WheelEvent) => void;

  private mouseMoved = false;

  public constructor(
    renderer: RenderingPipeline,
    renderingPipelineSettings: RenderingPipelineSettings
  ) {
    this.renderingPipeline = renderer;
    this.renderingPipelineSettings = renderingPipelineSettings;

    this.boundMouseDownHandler = this.onMouseDown.bind(this);
    this.boundMouseUpHandler = this.onMouseUp.bind(this);
    this.boundMouseMoveHandler = this.onMouseMove.bind(this);
    this.boundMouseWheelHandler = this.onMouseWheel.bind(this);
  }

  public start() {
    document.addEventListener("mousedown", this.boundMouseDownHandler);
    document.addEventListener("mouseup", this.boundMouseUpHandler);
    document.addEventListener("wheel", this.boundMouseWheelHandler);
  }

  public stop() {
    document.removeEventListener("mousedown", this.boundMouseDownHandler);
    document.removeEventListener("mouseup", this.boundMouseUpHandler);
    document.removeEventListener("mousemove", this.boundMouseMoveHandler);
    document.removeEventListener("wheel", this.boundMouseWheelHandler);
  }

  private onMouseDown() {
    document.addEventListener("mousemove", this.boundMouseMoveHandler);
  }

  private onMouseUp(event: MouseEvent) {
    document.removeEventListener("mousemove", this.boundMouseMoveHandler);

    if (!this.mouseMoved) this.onMouseClick(event.clientX, event.clientY);

    this.mouseMoved = false;
  }

  private onMouseMove(event: MouseEvent) {
    this.mouseMoved = true;

    const movementVector = new Vector2(event.movementX, event.movementY)
      .multiply(CAMERA_MOVEMENT_MULTIPLIER)
      .multiplyComponents(-1, 1)
      .divide(this.renderingPipelineSettings.scale);

    this.renderingPipelineSettings.cameraPosition =
      this.renderingPipelineSettings.cameraPosition.add(movementVector);
  }

  private onMouseWheel(event: WheelEvent) {
    const previousScale = this.renderingPipelineSettings.scale;
    const scaleDelta = -event.deltaY * SCALE_CHANGE_MULTIPLIER;

    this.renderingPipelineSettings.scale = Math.max(
      MIN_SCALE,
      Math.min(previousScale + scaleDelta, MAX_SCALE)
    );
  }

  private onMouseClick(mouseX: number, mouseY: number) {
    this.renderingPipeline.onClick(
      this.renderingPipeline.screenToWorldPosition(new Vector2(mouseX, mouseY))
    );
  }
}

export default MouseHandler;
