implementing cga3d;
// Fancy postfix trick
public AntiCircleRotor operator /(AntiCircleRotor lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(InversePrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.inverse();
}
public extension AntiCircleRotor {
    public AntiCircleRotor inverse() {
        float other_g0 = 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar - this.e45 * this.e45;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(InversePrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.inverse();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion inverse() {
        float other_g0 = this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e4 * this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(other_g0) * this.group3
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiDualNum operator /(InversePrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.inverse();
}
public extension AntiDualNum {
    public AntiDualNum inverse() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215 /  (this.scalar * this.scalar), 1.0 / this.scalar)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(InversePrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.inverse();
}
public extension AntiFlatPoint {
    public AntiFlatPoint inverse() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e235 /  (this.e321 * this.e321), this.e315 /  (this.e321 * this.e321), this.e125 /  (this.e321 * this.e321), 1.0 / this.e321) * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiFlector operator /(InversePrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.inverse();
}
public extension AntiFlector {
    public AntiFlector inverse() {
        float other_g0 = this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other_g0 * -1.0) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other_g0) * this.group1
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(AntiLine lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiLine operator /(InversePrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.inverse();
}
public extension AntiLine {
    public AntiLine inverse() {
        float other_g0 = this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12;
        return AntiLine(
            /* e23, e31, e12 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other_g0 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(AntiMotor lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiMotor operator /(InversePrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.inverse();
}
public extension AntiMotor {
    public AntiMotor inverse() {
        float other_g0 = this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(other_g0) * this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiPlane operator /(InversePrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.inverse();
}
public extension AntiPlane {
    public AntiPlane inverse() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(pow(this.e1, 3), pow(this.e2, 3), pow(this.e3, 3), this.e3 * this.e3 * this.e5) + (float4_pow(this.group0.yxxx, 2) * this.group0) + (float4_pow(this.group0.zzyy, 2) * this.group0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public AntiScalar operator /(InversePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.inverse();
}
public extension AntiScalar {
    public AntiScalar inverse() {
        return AntiScalar(
            /* e12345 */
            -1.0 / this.e12345
        );
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Circle operator /(InversePrefixOrPostfix lhs, Circle rhs) {
    return rhs.inverse();
}
public extension Circle {
    public Circle inverse() {
        float other_g0 = this.e321 * this.e321 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125);
        return Circle(
            /* e423, e431, e412 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e235, e315, e125 */
            float3(other_g0 * -1.0) * this.group2.xyz
        );
    }
}
// Fancy postfix trick
public CircleRotor operator /(CircleRotor lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public CircleRotor operator /(InversePrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.inverse();
}
public extension CircleRotor {
    public CircleRotor inverse() {
        float other_g0 = this.e321 * this.e321 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e12345 * this.e12345 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Dipole operator /(InversePrefixOrPostfix lhs, Dipole rhs) {
    return rhs.inverse();
}
public extension Dipole {
    public Dipole inverse() {
        float other_g0 = 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45;
        return Dipole(
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e15, e25, e35 */
            float3(other_g0 * -1.0) * this.group2.xyz
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public DipoleInversion operator /(InversePrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.inverse();
}
public extension DipoleInversion {
    public DipoleInversion inverse() {
        float other_g0 = 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group3
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public DualNum operator /(InversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.inverse();
}
public extension DualNum {
    public DualNum inverse() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5 /  (this.e12345 * this.e12345), 1.0 / this.e12345) * float2(-1.0)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public FlatPoint operator /(InversePrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.inverse();
}
public extension FlatPoint {
    public FlatPoint inverse() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e15 /  (this.e45 * this.e45), this.e25 /  (this.e45 * this.e45), this.e35 /  (this.e45 * this.e45), 1.0 / this.e45)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Flector operator /(InversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.inverse();
}
public extension Flector {
    public Flector inverse() {
        float other_g0 = -this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other_g0 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group1
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Line operator /(InversePrefixOrPostfix lhs, Line rhs) {
    return rhs.inverse();
}
public extension Line {
    public Line inverse() {
        float other_g0 = -this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435;
        return Line(
            /* e415, e425, e435 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other_g0 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Motor operator /(InversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.inverse();
}
public extension Motor {
    public Motor inverse() {
        float other_g0 = -this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e12345 * this.e12345;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other_g0) * this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public MultiVector operator /(InversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.inverse();
}
public extension MultiVector {
    public MultiVector inverse() {
        float other_g0 = 2.0*(this.e15 * this.e41) + 2.0 * (this.e25 * this.e42) + 2.0 * (this.e35 * this.e43) + 2.0 * (this.e3215 * this.e1234) + this.scalar * this.scalar + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321 - this.e12345 * this.e12345 - this.e45 * this.e45 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125 - 2.0 * (this.e4 * this.e5) - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125);
        return MultiVector(
            /* scalar, e12345 */
            float2(other_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group1, 
            /* e5 */
            other_g0 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(other_g0 * -1.0) * this.group3, 
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * -1.0) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other_g0 * -1.0) * this.group6, 
            /* e423, e431, e412 */
            float3(other_g0 * -1.0) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other_g0 * -1.0) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group9, 
            /* e1234 */
            other_g0 * this.e1234
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Plane operator /(InversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.inverse();
}
public extension Plane {
    public Plane inverse() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(pow(this.e4235, 3) * -1.0, pow(this.e4315, 3) * -1.0, pow(this.e4125, 3) * -1.0, this.e4125 * this.e4125 * this.e3215 * -1.0) - (float4_pow(this.group0.yxxx, 2) * this.group0) - (float4_pow(this.group0.zzyy, 2) * this.group0)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public RoundPoint operator /(InversePrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.inverse();
}
public extension RoundPoint {
    public RoundPoint inverse() {
        float other_g0 = this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - 2.0 * (this.e4 * this.e5);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group0, 
            /* e5 */
            other_g0 * this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Scalar operator /(InversePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.inverse();
}
public extension Scalar {
    public Scalar inverse() {
        return Scalar(
            /* scalar */
            1.0 / this.scalar
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Sphere operator /(InversePrefixOrPostfix lhs, Sphere rhs) {
    return rhs.inverse();
}
public extension Sphere {
    public Sphere inverse() {
        float other_g0 = 2.0*(this.e3215 * this.e1234) - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group0, 
            /* e1234 */
            other_g0 * this.e1234
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public VersorEven operator /(InversePrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.inverse();
}
public extension VersorEven {
    public VersorEven inverse() {
        float other_g0 = this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - this.e12345 * this.e12345 - this.e415 * this.e415 - this.e425 * this.e425 - this.e435 * this.e435 - 2.0 * (this.e423 * this.e235) - 2.0 * (this.e431 * this.e315) - 2.0 * (this.e412 * this.e125) - 2.0 * (this.e5 * this.e4);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group3
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public VersorOdd operator /(InversePrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.inverse();
}
public extension VersorOdd {
    public VersorOdd inverse() {
        float other_g0 = 2.0*(this.e41 * this.e15) + 2.0 * (this.e42 * this.e25) + 2.0 * (this.e43 * this.e35) + 2.0 * (this.e1234 * this.e3215) + this.scalar * this.scalar + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 - this.e45 * this.e45 - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other_g0) * this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other_g0) * this.group3
        );
    }
}
