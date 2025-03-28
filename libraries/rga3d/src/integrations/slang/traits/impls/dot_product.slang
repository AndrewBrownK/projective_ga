implementing rga3d;
// Fancy infix trick (first half)
public DotProductInfixPartial<DualNum> operator /(DualNum lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Flector> operator /(Flector lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(Point other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Horizon> operator /(Horizon lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Horizon {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Horizon {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Horizon {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Horizon {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Line> operator /(Line lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Motor> operator /(Motor lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.scalar * this.scalar) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<MultiVector> operator /(MultiVector lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            (this.scalar * other.scalar) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.scalar * this.scalar) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Point other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Plane> operator /(Plane lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(Horizon other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Point> operator /(Point lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Point {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Point {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Point {
    public Scalar dot_product(Point other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Scalar> operator /(Scalar lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
