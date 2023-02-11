import Line from "./line";
import LineSegment from "./line-segment";
import { compare, max, min } from "./number";
import Rectangle from "./rectangle";
import Vector2 from "./vector2";

class Polygon {
  public readonly points: readonly Vector2[];

  private _boundingRectangle?: Rectangle;
  private _sides?: readonly LineSegment[];

  public get boundingRectangle(): Rectangle {
    if (this._boundingRectangle === undefined) {
      this._boundingRectangle = this.computeBoundingRectangle();
    }

    return this._boundingRectangle;
  }

  public get sides(): readonly LineSegment[] {
    if (this._sides === undefined) {
      this._sides = this.computeSides();
    }

    return this._sides;
  }

  public constructor(points: readonly Vector2[]) {
    this.points = points;
  }

  public containsPoint(point: Vector2): boolean {
    if (!this.boundingRectangle.containsPoint(point)) return false;

    const line = new Line(0, 1, -point.y); // Line parallel to X-axis

    let intersectionsCount = 0;

    for (let i = 0; i < this.sides.length; i++) {
      const side = this.sides[i];

      if (side.b.equals(point)) {
        // We will get intersection with next side
        continue;
      }

      const intersectionPoint = line.getIntersection(side.line);

      if (
        intersectionPoint &&
        compare(intersectionPoint.x, point.x) >= 0 &&
        side.boundingRectangle.containsPoint(intersectionPoint)
      ) {
        intersectionsCount++;
      }
    }

    return intersectionsCount % 2 === 1;
  }

  private computeSides(): readonly LineSegment[] {
    const result = [];

    for (let i = 1; i < this.points.length; i++) {
      result.push(new LineSegment(this.points[i - 1], this.points[i]));
    }

    result.push(
      new LineSegment(this.points[this.points.length - 1], this.points[0])
    );

    return result;
  }

  private computeBoundingRectangle(): Rectangle {
    const startPoint = this.points[0];

    let minX = startPoint.x;
    let minY = startPoint.y;
    let maxX = startPoint.x;
    let maxY = startPoint.y;

    for (let i = 1; i < this.points.length; i++) {
      const point = this.points[i];

      minX = min(minX, point.x);
      maxX = max(maxX, point.x);
      minY = min(minY, point.y);
      maxY = max(maxY, point.y);
    }

    return new Rectangle(minX, minY, maxX - minX, maxY - minY);
  }
}

export default Polygon;
