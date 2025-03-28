implementing rga3d;
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<DualNum> operator /(DualNum lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Flector> operator /(Flector lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (this.e4 * other.e4) + (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Line> operator /(Line lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Motor> operator /(Motor lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e1234 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (other.e1234 * this.e1234) + (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<MultiVector> operator /(MultiVector lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e1234 */
            (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e1234 */
            (this.e1234 * other.e1234) + (this.e41 * other.e41) + (this.e42 * other.e42) + (this.e43 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (other.e1234 * this.e1234) + (other.e41 * this.e41) + (other.e42 * this.e42) + (other.e43 * this.e43) + (other.e4 * this.e4) + (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e1234 */
            (this.e423 * other.e423) + (this.e431 * other.e431) + (this.e412 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Origin> operator /(Origin lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Origin {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Origin {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Origin> lhs, Origin rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Origin {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Origin {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Plane> operator /(Plane lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Plane {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Plane {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Plane {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e1234 */
            (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Point> operator /(Point lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Point {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Point {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Point {
    public AntiScalar anti_dot_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Point {
    public AntiScalar anti_dot_product(Point other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4
        );
    }
}
