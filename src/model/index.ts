/*import Vector2 from "../math/vector2";

type OwnerID = number;

enum UnitKind {
  Level1,
  Level2,
  Level3,
  Level4
}

class Unit {
  public readonly ownerID: OwnerID;
  public position: Vector2;

  public constructor(ownerID: OwnerID, position: Vector2) {
    this.ownerID = ownerID;
    this.position = position;
  }
}

enum TileContent {
  Empty,
  Tree,
  Townhall,
  House
}

class Tile {
  public ownerID?: OwnerID;

  public constructor(ownerID?: OwnerID) {
    this.ownerID = ownerID;
  }

  public hasOwner(): boolean {
    return this.ownerID;
  }
}

class Model {
  public readonly tiles: Array<Array<Tile>>;
  public readonly units: Array<Unit>;
  public readonly width: number;
  public readonly height: number;

  public constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.tiles = [];
    this.units = [];

    for (let i = 0; i < width; i++) {
      this.tiles[i] = [];

      for (let j = 0; j < height; j++) {
        this.tiles[i].push(new Tile());
      }
    }
  }
}

export { Model, Tile, OwnerID, Unit };
*/
