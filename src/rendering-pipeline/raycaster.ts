import GameObject from "../game-object";
import { compare } from "../math/number";
import Polygon from "../math/polygon";
import Vector2 from "../math/vector2";

type PolygonID = string;

class Raycaster {
  private readonly polygons: {
    zIndex: number;
    polygon: Polygon;
    id: PolygonID;
    gameObject: GameObject;
  }[];

  public constructor() {
    this.polygons = [];
  }

  public insert(
    polygon: Polygon,
    id: PolygonID,
    zIndex: number,
    gameObject: GameObject
  ) {
    this.polygons.push({ zIndex, polygon, id, gameObject });
  }

  public sort() {
    this.polygons.sort((a, b) => -compare(a.zIndex, b.zIndex));
  }

  public getContainingPolygonId(
    point: Vector2
  ): [GameObject, PolygonID] | undefined {
    for (let i = 0; i != this.polygons.length; i++) {
      if (this.polygons[i].polygon.containsPoint(point)) {
        return [this.polygons[i].gameObject, this.polygons[i].id];
      }
    }

    return undefined;
  }

  public clear() {
    this.polygons.length = 0;
  }
}

export { Raycaster, PolygonID };
