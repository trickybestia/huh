import Vector2 from "../math/vector2";
import nameof from "../utils/nameof";

type PropertyChangedEventHandler = (
  propertyName: keyof RenderingPipelineSettings
) => void;

class RenderingPipelineSettings {
  private readonly propertyChangedEventHandlers: PropertyChangedEventHandler[] =
    [];

  private _scale = 1;
  private _aspectRatio = 1;
  private _cameraPosition: Vector2 = new Vector2(0, 0);

  public get scale(): number {
    return this._scale;
  }

  public set scale(value: number) {
    this._scale = value;

    this.onPropertyChanged(nameof<RenderingPipelineSettings>("scale"));
  }

  public get aspectRatio(): number {
    return this._aspectRatio;
  }

  public set aspectRatio(value: number) {
    this._aspectRatio = value;

    this.onPropertyChanged(nameof<RenderingPipelineSettings>("aspectRatio"));
  }

  public get cameraPosition(): Vector2 {
    return this._cameraPosition;
  }

  public set cameraPosition(value: Vector2) {
    this._cameraPosition = value;

    this.onPropertyChanged(nameof<RenderingPipelineSettings>("cameraPosition"));
  }

  public constructor() {
    this.propertyChangedEventHandlers = [];
  }

  public addPropertyChangedEventHandler(handler: PropertyChangedEventHandler) {
    this.propertyChangedEventHandlers.push(handler);
  }

  public removePropertyChangedEventHandler(
    handler: PropertyChangedEventHandler
  ) {
    const handlerIndex = this.propertyChangedEventHandlers.indexOf(handler);

    if (handlerIndex !== -1) {
      this.propertyChangedEventHandlers.splice(handlerIndex, 1);
    }
  }

  private onPropertyChanged(propertyName: keyof RenderingPipelineSettings) {
    this.propertyChangedEventHandlers.forEach((handler) =>
      handler(propertyName)
    );
  }
}

export { RenderingPipelineSettings, PropertyChangedEventHandler };
