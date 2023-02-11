import { compare, max, min } from "./number";
import Vector2 from "./vector2";

class Rectangle {
  public readonly x: number;
  public readonly y: number;
  public readonly width: number;
  public readonly height: number;

  public constructor(x: number, y: number, width: number, height: number) {
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;
  }

  public getIntersection(other: Rectangle): Rectangle | undefined {
    const x = max(this.x, other.x);
    const y = max(this.y, other.y);
    const x1 = min(this.x + this.width, other.x + other.width);
    const y1 = min(this.y + this.height, other.y + other.height);

    const width = x1 - x;
    const height = y1 - y;

    if (compare(width, 0) < 0 || compare(height, 0) < 0) return undefined;

    return new Rectangle(x, y, width, height);
  }

  public containsPoint(point: Vector2): boolean {
    return (
      compare(point.x, this.x) >= 0 &&
      compare(point.y, this.y) >= 0 &&
      compare(point.x, this.x + this.width) <= 0 &&
      compare(point.y, this.y + this.height) <= 0
    );
  }
}

export default Rectangle;
