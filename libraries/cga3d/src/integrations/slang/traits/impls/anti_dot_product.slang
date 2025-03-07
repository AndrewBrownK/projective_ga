implementing cga3d;
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e4 * other.e5) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e4 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            this.e3215 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e3215 * other.e1234) - (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e3215 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiDualNum {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            -(this.e3215 * other.e1234) - (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiFlector> operator /(AntiFlector lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e4 * this.e5) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            -(other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            -(this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiFlector {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4) - (this.e321 * other.e321) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiLine> operator /(AntiLine lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiLine {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiLine {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiLine {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiLine {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiLine {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiLine {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiLine {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiMotor> operator /(AntiMotor lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e3215 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiMotor {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<AntiPlane> operator /(AntiPlane lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            -(other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            -(other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiPlane {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
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
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
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
            /* e12345 */
            this.e12345 * other.e12345
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
            /* e12345 */
            this.e12345 * other.e12345
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
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Circle> operator /(Circle lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Circle {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<CircleRotor> operator /(CircleRotor lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e12345 * other.e12345) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e12345 * other.e12345) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Dipole> operator /(Dipole lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Dipole {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DipoleInversion {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e1234 * other.e3215) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<DualNum> operator /(DualNum lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            other.e4 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
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
            /* e12345 */
            other.e12345 * this.e12345
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
            /* e12345 */
            this.e12345 * other.e12345
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
            /* e12345 */
            (this.e5 * other.e4) + (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension DualNum {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e5 * other.e4) + (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<FlatPoint> operator /(FlatPoint lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            this.e45 * other.e45
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension FlatPoint {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Flector> operator /(Flector lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
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
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
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
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Flector {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Line> operator /(Line lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
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
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435)
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
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Line {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Motor> operator /(Motor lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345)
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
            /* e12345 */
            other.e12345 * this.e12345
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
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435)
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
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345)
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
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Motor {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e12345 * other.e12345) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) + (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<MultiVector> operator /(MultiVector lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            -(other.e3215 * this.e1234) - (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
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
            /* e12345 */
            (other.e5 * this.e4) + (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
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
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
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
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412)
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
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4)
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
            /* e12345 */
            (other.e12345 * this.e12345) + (other.e4 * this.e5) + (other.e5 * this.e4) + (other.e45 * this.e45) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.scalar * this.scalar) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e321 * this.e321) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
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
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e12345 * other.e12345) + (this.e4 * other.e5) + (this.e5 * other.e4) + (this.e415 * other.e415) + (this.e425 * other.e425) + (this.e435 * other.e435) + (this.e423 * other.e235) + (this.e431 * other.e315) + (this.e412 * other.e125) + (this.e235 * other.e423) + (this.e315 * other.e431) + (this.e125 * other.e412) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3) - (this.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension MultiVector {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e45 * other.e45) + (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.scalar * other.scalar) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Plane> operator /(Plane lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Plane {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Plane {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
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
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215)
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
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Plane {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Plane {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<RoundPoint> operator /(RoundPoint lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension RoundPoint {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e5) + (this.e5 * other.e4) - (this.e1 * other.e1) - (this.e2 * other.e2) - (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Scalar> operator /(Scalar lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Scalar {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Scalar {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Scalar {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Scalar {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Scalar {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Scalar {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<Sphere> operator /(Sphere lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension Sphere {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e3215 * other.e1234) - (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<VersorEven> operator /(VersorEven lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e12345 * this.e12345) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(DualNum other) {
        return AntiScalar(
            /* e12345 */
            (other.e5 * this.e4) + (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(Line other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(Motor other) {
        return AntiScalar(
            /* e12345 */
            (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e12345 * this.e12345) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e12345 * this.e12345) + (other.e4 * this.e5) + (other.e5 * this.e4) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3) - (other.e321 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e5) + (other.e5 * this.e4) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorEven {
    public AntiScalar anti_dot_product(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (other.e423 * this.e235) + (other.e431 * this.e315) + (other.e412 * this.e125) + (other.e12345 * this.e12345) + (other.e415 * this.e415) + (other.e425 * this.e425) + (other.e435 * this.e435) + (other.e235 * this.e423) + (other.e315 * this.e431) + (other.e125 * this.e412) + (other.e5 * this.e4) + (other.e4 * this.e5) - (other.e321 * this.e321) - (other.e1 * this.e1) - (other.e2 * this.e2) - (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (first half)
public AntiDotProductInfixPartial<VersorOdd> operator /(VersorOdd lhs, AntiDotProductInfix rhs) {
    return AntiDotProductInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            -(other.e3215 * this.e1234) - (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.scalar * this.scalar) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e3215 * this.e1234) - (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiDotProductInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.anti_dot_product(rhs);
}
public extension VersorOdd {
    public AntiScalar anti_dot_product(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            (other.e45 * this.e45) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.scalar * this.scalar) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e1234 * this.e3215) - (other.e3215 * this.e1234)
        );
    }
}
