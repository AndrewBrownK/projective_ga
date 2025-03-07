implementing cga3d;
using traits;
// Fancy postfix trick
public AntiCircleRotor operator /(AntiCircleRotor lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(AntiInversePrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.anti_inverse();
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_inverse() {
        float other_g0 = this.e45 * this.e45 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - this.scalar * this.scalar - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(AntiInversePrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.anti_inverse();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_inverse() {
        float other_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e4 * this.e5) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321 - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(other_g0) * this.group3
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiDualNum operator /(AntiInversePrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.anti_inverse();
}
public extension AntiDualNum {
    public AntiDualNum anti_inverse() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(pow(this.scalar, -2) * -1.0) * this.group0.xy
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(AntiInversePrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.anti_inverse();
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_inverse() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(pow(this.e321, -2) * -1.0) * float4(this.e235 * -1.0, this.e315 * -1.0, this.e125 * -1.0, this.e321 * -1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiFlector operator /(AntiInversePrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.anti_inverse();
}
public extension AntiFlector {
    public AntiFlector anti_inverse() {
        float other_g0 = -this.e321 * this.e321 - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other_g0) * this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(other_g0) * this.group1
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(AntiLine lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiLine operator /(AntiInversePrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.anti_inverse();
}
public extension AntiLine {
    public AntiLine anti_inverse() {
        float other_g0 = -this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12;
        return AntiLine(
            /* e23, e31, e12 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            float3(other_g0) * this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(AntiMotor lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiMotor operator /(AntiInversePrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.anti_inverse();
}
public extension AntiMotor {
    public AntiMotor anti_inverse() {
        float other_g0 = -this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - this.scalar * this.scalar;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(other_g0) * this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiPlane operator /(AntiInversePrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.anti_inverse();
}
public extension AntiPlane {
    public AntiPlane anti_inverse() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3) * this.group0
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiInversePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_inverse();
}
public extension AntiScalar {
    public AntiScalar anti_inverse() {
        return AntiScalar(
            /* e12345 */
            1.0/this.e12345
        );
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Circle operator /(AntiInversePrefixOrPostfix lhs, Circle rhs) {
    return rhs.anti_inverse();
}
public extension Circle {
    public Circle anti_inverse() {
        float other_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321;
        return Circle(
            /* e423, e431, e412 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e235, e315, e125 */
            float3(other_g0) * this.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public CircleRotor operator /(CircleRotor lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public CircleRotor operator /(AntiInversePrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.anti_inverse();
}
public extension CircleRotor {
    public CircleRotor anti_inverse() {
        float other_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345 - this.e321 * this.e321;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Dipole operator /(AntiInversePrefixOrPostfix lhs, Dipole rhs) {
    return rhs.anti_inverse();
}
public extension Dipole {
    public Dipole anti_inverse() {
        float other_g0 = this.e45 * this.e45 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35);
        return Dipole(
            /* e41, e42, e43 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e15, e25, e35 */
            float3(other_g0) * this.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public DipoleInversion operator /(AntiInversePrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.anti_inverse();
}
public extension DipoleInversion {
    public DipoleInversion anti_inverse() {
        float other_g0 = this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group3
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public DualNum operator /(AntiInversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_inverse();
}
public extension DualNum {
    public DualNum anti_inverse() {
        return DualNum(
            /* e5, e12345 */
            float2(pow(this.e12345, -2)) * this.group0.xy
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public FlatPoint operator /(AntiInversePrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.anti_inverse();
}
public extension FlatPoint {
    public FlatPoint anti_inverse() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(pow(this.e45, -2)) * float4(this.e15 * -1.0, this.e25 * -1.0, this.e35 * -1.0, this.e45 * -1.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Flector operator /(AntiInversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_inverse();
}
public extension Flector {
    public Flector anti_inverse() {
        float other_g0 = this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other_g0) * this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group1
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Line operator /(AntiInversePrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_inverse();
}
public extension Line {
    public Line anti_inverse() {
        float other_g0 = this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435;
        return Line(
            /* e415, e425, e435 */
            float3(other_g0) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(other_g0) * this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Motor operator /(AntiInversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_inverse();
}
public extension Motor {
    public Motor anti_inverse() {
        float other_g0 = this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other_g0) * this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public MultiVector operator /(AntiInversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_inverse();
}
public extension MultiVector {
    public MultiVector anti_inverse() {
        float other_g0 = 2.0*(this.e4 * this.e5) + 2.0 * (this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + this.e12345 * this.e12345 + this.e45 * this.e45 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.scalar * this.scalar - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3 - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - this.e321 * this.e321 - 2.0 * (this.e15 * this.e41) - 2.0 * (this.e25 * this.e42) - 2.0 * (this.e35 * this.e43) - 2.0 * (this.e3215 * this.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(other_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group1, 
            /* e5 */
            other_g0 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(other_g0) * this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(other_g0) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(other_g0) * this.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(other_g0) * this.group6 * float4(-1.0), 
            /* e423, e431, e412 */
            float3(other_g0) * this.group7.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(other_g0) * this.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group9, 
            /* e1234 */
            other_g0 * this.e1234
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Plane operator /(AntiInversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_inverse();
}
public extension Plane {
    public Plane anti_inverse() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125) * this.group0
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiInversePrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.anti_inverse();
}
public extension RoundPoint {
    public RoundPoint anti_inverse() {
        float other_g0 = 2.0*(this.e4 * this.e5) - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group0, 
            /* e5 */
            other_g0 * this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Scalar operator /(AntiInversePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_inverse();
}
public extension Scalar {
    public Scalar anti_inverse() {
        return Scalar(
            /* scalar */
            1.0/this.scalar * -1.0
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Sphere operator /(AntiInversePrefixOrPostfix lhs, Sphere rhs) {
    return rhs.anti_inverse();
}
public extension Sphere {
    public Sphere anti_inverse() {
        float other_g0 = this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - 2.0 * (this.e3215 * this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group0, 
            /* e1234 */
            other_g0 * this.e1234
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public VersorEven operator /(AntiInversePrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.anti_inverse();
}
public extension VersorEven {
    public VersorEven anti_inverse() {
        float other_g0 = 2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e5 * this.e4) + this.e12345 * this.e12345 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 - this.e321 * this.e321 - this.e1 * this.e1 - this.e2 * this.e2 - this.e3 * this.e3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group3
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public VersorOdd operator /(AntiInversePrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.anti_inverse();
}
public extension VersorOdd {
    public VersorOdd anti_inverse() {
        float other_g0 = this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125 - this.scalar * this.scalar - this.e23 * this.e23 - this.e31 * this.e31 - this.e12 * this.e12 - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4(other_g0) * this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group3
        );
    }
}
