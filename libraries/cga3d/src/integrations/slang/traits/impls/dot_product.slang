implementing cga3d;
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiCircleRotor {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            this.e4 * other.e5 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDipoleInversion {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiDualNum {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.scalar * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlatPoint {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlatPoint {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlatPoint {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlatPoint {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlatPoint {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlatPoint {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlatPoint {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiFlector> operator /(AntiFlector lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiFlector {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiLine> operator /(AntiLine lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiLine {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiLine {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiLine {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiLine {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiLine {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiLine {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiLine {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiMotor> operator /(AntiMotor lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiMotor {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiPlane> operator /(AntiPlane lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiPlane {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiPlane {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiPlane {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiPlane {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiPlane {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiPlane {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<AntiScalar> operator /(AntiScalar lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiScalar {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiScalar {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiScalar {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiScalar {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiScalar {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension AntiScalar {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Circle> operator /(Circle lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Circle {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<CircleRotor> operator /(CircleRotor lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension CircleRotor {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Dipole> operator /(Dipole lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Dipole {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e1234 * other.e3215) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DipoleInversion {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<DualNum> operator /(DualNum lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            other.e4 * this.e5 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
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
            this.e12345 * other.e12345 * -1.0
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
            -(this.e5 * other.e4) - (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension DualNum {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            -(this.e5 * other.e4) - (this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<FlatPoint> operator /(FlatPoint lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension FlatPoint {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension FlatPoint {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension FlatPoint {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension FlatPoint {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension FlatPoint {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            this.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension FlatPoint {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension FlatPoint {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Flector> operator /(Flector lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            -(other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
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
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
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
            -(this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Flector {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e3215 * other.e1234) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Line> operator /(Line lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
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
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
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
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Line {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Motor> operator /(Motor lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
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
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
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
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345)
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
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Motor {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e12345 * other.e12345) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<MultiVector> operator /(MultiVector lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            -(other.e5 * this.e4) - (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
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
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
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
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
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
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
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
            (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e321 * this.e321) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e12345 * this.e12345) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e45 * this.e45) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
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
            (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
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
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) + (this.e321 * other.e321) - (this.e12345 * other.e12345) - (this.e4 * other.e5) - (this.e5 * other.e4) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension MultiVector {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Plane> operator /(Plane lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
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
            (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
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
            -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Plane {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<RoundPoint> operator /(RoundPoint lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension RoundPoint {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Scalar> operator /(Scalar lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(AntiMotor other) {
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
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Scalar {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<Sphere> operator /(Sphere lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension Sphere {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e3215 * other.e1234) + (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<VersorEven> operator /(VersorEven lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(AntiFlector other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(Circle other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(CircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(DualNum other) {
        return Scalar(
            /* scalar */
            -(other.e5 * this.e4) - (other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(Motor other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e12345 * this.e12345) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e321 * this.e321) - (other.e12345 * this.e12345) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorEven {
    public Scalar dot_product(VersorEven other) {
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (first half)
public DotProductInfixPartial<VersorOdd> operator /(VersorOdd lhs, DotProductInfix rhs) {
    return DotProductInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(AntiDualNum other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(AntiMotor other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(FlatPoint other) {
        return Scalar(
            /* scalar */
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(Flector other) {
        return Scalar(
            /* scalar */
            (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(MultiVector other) {
        return Scalar(
            /* scalar */
            (other.scalar * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(Plane other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(Sphere other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(DotProductInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.dot_product(rhs);
}
public extension VersorOdd {
    public Scalar dot_product(VersorOdd other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.scalar * this.scalar) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
