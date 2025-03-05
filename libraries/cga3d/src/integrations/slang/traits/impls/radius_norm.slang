implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.radius_norm();
}
public extension AntiCircleRotor {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2) - pow(this.e45, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.radius_norm();
}
public extension AntiDipoleInversion {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e4 * this.e5)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.radius_norm();
}
public extension AntiDualNum {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.radius_norm();
}
public extension AntiFlatPoint {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.radius_norm();
}
public extension AntiFlector {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.radius_norm();
}
public extension AntiLine {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.radius_norm();
}
public extension AntiMotor {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.radius_norm();
}
public extension AntiPlane {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiScalar lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.radius_norm();
}
public extension AntiScalar {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.radius_norm();
}
public extension Circle {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e321, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.radius_norm();
}
public extension CircleRotor {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e321, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - pow(this.e12345, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.radius_norm();
}
public extension Dipole {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.radius_norm();
}
public extension DipoleInversion {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DualNum lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.radius_norm();
}
public extension DualNum {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(FlatPoint lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.radius_norm();
}
public extension FlatPoint {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            this.e45 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.radius_norm();
}
public extension Flector {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            -pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.radius_norm();
}
public extension Line {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            -pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.radius_norm();
}
public extension Motor {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            -pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - pow(this.e12345, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.radius_norm();
}
public extension MultiVector {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            2.0*(this.e15 * this.e41) + 2.0 * (this.e25 * this.e42) + 2.0 * (this.e35 * this.e43) + 2.0 * (this.e3215 * this.e1234) + pow(this.scalar, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.e321, 2) - pow(this.e12345, 2) - pow(this.e45, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2) - 2.0 * (this.e4 * this.e5) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.radius_norm();
}
public extension Plane {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            -pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.radius_norm();
}
public extension RoundPoint {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - 2.0 * (this.e4 * this.e5)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.radius_norm();
}
public extension Scalar {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.radius_norm();
}
public extension Sphere {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            2.0*(this.e3215 * this.e1234) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.radius_norm();
}
public extension VersorEven {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            pow(this.e321, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) - pow(this.e12345, 2) - pow(this.e415, 2) - pow(this.e425, 2) - pow(this.e435, 2) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e5 * this.e4)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, RadiusNormPrefixOrPostfix rhs)  {
    return lhs.radius_norm();
    }
// Fancy prefix trick
public Scalar operator /(RadiusNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.radius_norm();
}
public extension VersorOdd {
    public Scalar radius_norm() {
        return Scalar(
            /* scalar */
            2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + pow(this.scalar, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) - pow(this.e45, 2) - pow(this.e4235, 2) - pow(this.e4315, 2) - pow(this.e4125, 2)
        );
    }
}
