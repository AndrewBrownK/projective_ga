implementing cga3d;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e4 * this.e5)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiDualNum {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiFlatPoint {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiFlector {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiLine {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiMotor {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiPlane {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiScalar lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.radius_norm_squared();
}
public extension AntiScalar {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.radius_norm_squared();
}
public extension Circle {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.radius_norm_squared();
}
public extension CircleRotor {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e12345 * this.e12345 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.radius_norm_squared();
}
public extension Dipole {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.radius_norm_squared();
}
public extension DipoleInversion {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DualNum lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.radius_norm_squared();
}
public extension DualNum {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(FlatPoint lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.radius_norm_squared();
}
public extension FlatPoint {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e45 * this.e45 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.radius_norm_squared();
}
public extension Flector {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.radius_norm_squared();
}
public extension Line {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.radius_norm_squared();
}
public extension Motor {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e12345 * this.e12345
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.radius_norm_squared();
}
public extension MultiVector {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            2.0*(this.e15 * this.e41) + 2.0 * (this.e25 * this.e42) + 2.0 * (this.e35 * this.e43) + 2.0 * (this.e3215 * this.e1234) + this.scalar * this.scalar + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321 - this.e12345 * this.e12345 - this.e45 * this.e45 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125 - 2.0 * (this.e4 * this.e5) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.radius_norm_squared();
}
public extension Plane {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.radius_norm_squared();
}
public extension RoundPoint {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - 2.0 * (this.e4 * this.e5)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.radius_norm_squared();
}
public extension Scalar {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.radius_norm_squared();
}
public extension Sphere {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            2.0*(this.e3215 * this.e1234) - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.radius_norm_squared();
}
public extension VersorEven {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - this.e12345 * this.e12345 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e5 * this.e4)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, RadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.radius_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.radius_norm_squared();
}
public extension VersorOdd {
    public Scalar radius_norm_squared() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + this.scalar * this.scalar + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125
        );
    }
}
