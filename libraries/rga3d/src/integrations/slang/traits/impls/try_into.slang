implementing rga3d;
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(DualNum dual_num) {

        float disallowed_scalar = dual_num.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e1234 */
            dual_num.e1234
        );
    }
}
public extension DualNum {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(DualNum dual_num) {

        float disallowed_e1234 = dual_num.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            dual_num.scalar
        );
    }
}
public extension DualNum {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension Horizon {
    public static Optional<Horizon> try_from(Flector flector) {

        float disallowed_e1 = flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = flector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e423 = flector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        return Horizon(
            /* e321 */
            flector.e321
        );
    }
}
public extension Flector {
    public Optional<Horizon> try_into_Horizon() {
        return Horizon.try_from(this);
    }
}
public extension Origin {
    public static Optional<Origin> try_from(Flector flector) {

        float disallowed_e1 = flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e423 = flector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Origin(
            /* e4 */
            flector.e4
        );
    }
}
public extension Flector {
    public Optional<Origin> try_into_Origin() {
        return Origin.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(Flector flector) {

        float disallowed_e1 = flector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = flector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = flector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = flector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        return Plane(
            /* e423, e431, e412, e321 */
            flector.group1
        );
    }
}
public extension Flector {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension Point {
    public static Optional<Point> try_from(Flector flector) {

        float disallowed_e423 = flector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = flector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = flector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = flector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Point(
            /* e1, e2, e3, e4 */
            flector.group0
        );
    }
}
public extension Flector {
    public Optional<Point> try_into_Point() {
        return Point.try_from(this);
    }
}
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(Motor motor) {

        float disallowed_e41 = motor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_scalar = motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e1234 */
            motor.e1234
        );
    }
}
public extension Motor {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(Motor motor) {

        float disallowed_e41 = motor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return DualNum(
            /* scalar, e1234 */
            float2(motor.scalar, motor.e1234)
        );
    }
}
public extension Motor {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(Motor motor) {

        float disallowed_e1234 = motor.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_scalar = motor.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        return Line(
            /* e41, e42, e43 */
            motor.group0.xyz, 
            /* e23, e31, e12 */
            motor.group1.xyz
        );
    }
}
public extension Motor {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(Motor motor) {

        float disallowed_e41 = motor.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = motor.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = motor.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e1234 = motor.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e23 = motor.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = motor.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = motor.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            motor.scalar
        );
    }
}
public extension Motor {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension AntiScalar {
    public static Optional<AntiScalar> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return AntiScalar(
            /* e1234 */
            multi_vector.e1234
        );
    }
}
public extension MultiVector {
    public Optional<AntiScalar> try_into_AntiScalar() {
        return AntiScalar.try_from(this);
    }
}
public extension DualNum {
    public static Optional<DualNum> try_from(MultiVector multi_vector) {

        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return DualNum(
            /* scalar, e1234 */
            multi_vector.group0.xy
        );
    }
}
public extension MultiVector {
    public Optional<DualNum> try_into_DualNum() {
        return DualNum.try_from(this);
    }
}
public extension Flector {
    public static Optional<Flector> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return Flector(
            /* e1, e2, e3, e4 */
            multi_vector.group1, 
            /* e423, e431, e412, e321 */
            multi_vector.group4
        );
    }
}
public extension MultiVector {
    public Optional<Flector> try_into_Flector() {
        return Flector.try_from(this);
    }
}
public extension Horizon {
    public static Optional<Horizon> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        return Horizon(
            /* e321 */
            multi_vector.e321
        );
    }
}
public extension MultiVector {
    public Optional<Horizon> try_into_Horizon() {
        return Horizon.try_from(this);
    }
}
public extension Line {
    public static Optional<Line> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Line(
            /* e41, e42, e43 */
            multi_vector.group2.xyz, 
            /* e23, e31, e12 */
            multi_vector.group3.xyz
        );
    }
}
public extension MultiVector {
    public Optional<Line> try_into_Line() {
        return Line.try_from(this);
    }
}
public extension Motor {
    public static Optional<Motor> try_from(MultiVector multi_vector) {

        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(multi_vector.e41, multi_vector.e42, multi_vector.e43, multi_vector.e1234), 
            /* e23, e31, e12, scalar */
            float4(multi_vector.e23, multi_vector.e31, multi_vector.e12, multi_vector.scalar)
        );
    }
}
public extension MultiVector {
    public Optional<Motor> try_into_Motor() {
        return Motor.try_from(this);
    }
}
public extension Origin {
    public static Optional<Origin> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Origin(
            /* e4 */
            multi_vector.e4
        );
    }
}
public extension MultiVector {
    public Optional<Origin> try_into_Origin() {
        return Origin.try_from(this);
    }
}
public extension Plane {
    public static Optional<Plane> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        return Plane(
            /* e423, e431, e412, e321 */
            multi_vector.group4
        );
    }
}
public extension MultiVector {
    public Optional<Plane> try_into_Plane() {
        return Plane.try_from(this);
    }
}
public extension Point {
    public static Optional<Point> try_from(MultiVector multi_vector) {

        float disallowed_scalar = multi_vector.scalar;
        if (disallowed_scalar != 0.0) {
            return none;
        }
        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Point(
            /* e1, e2, e3, e4 */
            multi_vector.group1
        );
    }
}
public extension MultiVector {
    public Optional<Point> try_into_Point() {
        return Point.try_from(this);
    }
}
public extension Scalar {
    public static Optional<Scalar> try_from(MultiVector multi_vector) {

        float disallowed_e1234 = multi_vector.e1234;
        if (disallowed_e1234 != 0.0) {
            return none;
        }
        float disallowed_e1 = multi_vector.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = multi_vector.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = multi_vector.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        float disallowed_e4 = multi_vector.e4;
        if (disallowed_e4 != 0.0) {
            return none;
        }
        float disallowed_e41 = multi_vector.e41;
        if (disallowed_e41 != 0.0) {
            return none;
        }
        float disallowed_e42 = multi_vector.e42;
        if (disallowed_e42 != 0.0) {
            return none;
        }
        float disallowed_e43 = multi_vector.e43;
        if (disallowed_e43 != 0.0) {
            return none;
        }
        float disallowed_e23 = multi_vector.e23;
        if (disallowed_e23 != 0.0) {
            return none;
        }
        float disallowed_e31 = multi_vector.e31;
        if (disallowed_e31 != 0.0) {
            return none;
        }
        float disallowed_e12 = multi_vector.e12;
        if (disallowed_e12 != 0.0) {
            return none;
        }
        float disallowed_e423 = multi_vector.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = multi_vector.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = multi_vector.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        float disallowed_e321 = multi_vector.e321;
        if (disallowed_e321 != 0.0) {
            return none;
        }
        return Scalar(
            /* scalar */
            multi_vector.scalar
        );
    }
}
public extension MultiVector {
    public Optional<Scalar> try_into_Scalar() {
        return Scalar.try_from(this);
    }
}
public extension Horizon {
    public static Optional<Horizon> try_from(Plane plane) {

        float disallowed_e423 = plane.e423;
        if (disallowed_e423 != 0.0) {
            return none;
        }
        float disallowed_e431 = plane.e431;
        if (disallowed_e431 != 0.0) {
            return none;
        }
        float disallowed_e412 = plane.e412;
        if (disallowed_e412 != 0.0) {
            return none;
        }
        return Horizon(
            /* e321 */
            plane.e321
        );
    }
}
public extension Plane {
    public Optional<Horizon> try_into_Horizon() {
        return Horizon.try_from(this);
    }
}
public extension Origin {
    public static Optional<Origin> try_from(Point point) {

        float disallowed_e1 = point.e1;
        if (disallowed_e1 != 0.0) {
            return none;
        }
        float disallowed_e2 = point.e2;
        if (disallowed_e2 != 0.0) {
            return none;
        }
        float disallowed_e3 = point.e3;
        if (disallowed_e3 != 0.0) {
            return none;
        }
        return Origin(
            /* e4 */
            point.e4
        );
    }
}
public extension Point {
    public Optional<Origin> try_into_Origin() {
        return Origin.try_from(this);
    }
}
