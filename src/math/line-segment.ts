import Line from "./line";
import { compare } from "./number";
import Rectangle from "./rectangle";
import Vector2 from "./vector2";

class LineSegment {
  public readonly a: Vector2;
  public readonly b: Vector2;

  private _line?: Line;
  private _boundingRectangle?: Rectangle;

  public get line(): Line {
    if (this._line === undefined) {
      this._line = this.computeLine();
    }

    return this._line;
  }

  public get boundingRectangle(): Rectangle {
    if (this._boundingRectangle === undefined) {
      this._boundingRectangle = this.computeBoundingRectangle();
    }

    return this._boundingRectangle;
  }

  public constructor(a: Vector2, b: Vector2) {
    this.a = a;
    this.b = b;
  }

  private computeLine(): Line {
    return Line.fromPoints(this.a, this.b);
  }

  private computeBoundingRectangle(): Rectangle {
    const [minX, maxX] = [this.a.x, this.b.x].sort(compare);
    const [minY, maxY] = [this.a.y, this.b.y].sort(compare);

    return new Rectangle(minX, minY, maxX - minX, maxY - minY);
  }
}

export default LineSegment;
