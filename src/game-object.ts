import { Raycaster, PolygonID } from "./rendering-pipeline/raycaster";
import WebGLRenderer from "./rendering-pipeline/webgl-renderer";
import scene from "./scene";

abstract class GameObject {
  private _willBeDestroyed = false;

  public get willBeDestroyed(): boolean {
    return this._willBeDestroyed;
  }

  public constructor() {
    scene.gameObjects.push(this);
  }

  public markToBeDestroyed() {
    this._willBeDestroyed = true;
  }

  public draw?(renderer: WebGLRenderer, raycaster: Raycaster): void;
  public onClick?(polygonID: PolygonID): void;
  public onDestroy?(): void;
}

export default GameObject;
