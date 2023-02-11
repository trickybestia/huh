import { compare } from "./number";
import Vector2 from "./vector2";

/**
 * Line equation: ax + by + c = 0
 */
class Line {
  public readonly a: number;
  public readonly b: number;
  public readonly c: number;

  public static fromPoints(a: Vector2, b: Vector2): Line {
    return new Line(b.y - a.y, a.x - b.x, b.x * a.y - a.x * b.y);
  }

  public constructor(a: number, b: number, c: number) {
    this.a = a;
    this.b = b;
    this.c = c;
  }

  public getIntersection(other: Line): Vector2 | undefined {
    const coefficientProductDifference = this.a * other.b - other.a * this.b;

    if (compare(coefficientProductDifference, 0) === 0) return undefined;

    return new Vector2(
      (this.b * other.c - other.b * this.c) / coefficientProductDifference,
      (other.a * this.c - this.a * other.c) / coefficientProductDifference
    );
  }
}

export default Line;
