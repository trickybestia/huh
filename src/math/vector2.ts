import { compare } from "./number";

class Vector2 {
  public readonly x: number;
  public readonly y: number;

  public constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  public multiply(value: number): Vector2 {
    return new Vector2(this.x * value, this.y * value);
  }

  public multiplyComponents(xMultiplier: number, yMultiplier: number): Vector2 {
    return new Vector2(this.x * xMultiplier, this.y * yMultiplier);
  }

  public divide(value: number): Vector2 {
    return new Vector2(this.x / value, this.y / value);
  }

  public add(other: Vector2): Vector2 {
    return new Vector2(this.x + other.x, this.y + other.y);
  }

  public subtract(other: Vector2): Vector2 {
    return new Vector2(this.x - other.x, this.y - other.y);
  }

  public equals(other: Vector2): boolean {
    return compare(this.x, other.x) === 0 && compare(this.y, other.y) === 0;
  }
}

export default Vector2;
