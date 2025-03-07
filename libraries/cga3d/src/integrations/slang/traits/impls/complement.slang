implementing cga3d;
// Fancy postfix trick
public CircleRotor operator /(AntiCircleRotor lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public CircleRotor operator /(ComplementPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.complement();
}
public extension AntiCircleRotor {
    public CircleRotor complement() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group2.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(AntiDipoleInversion lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public DipoleInversion operator /(ComplementPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.complement();
}
public extension AntiDipoleInversion {
    public DipoleInversion complement() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, this.e5) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(AntiDualNum lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public VersorEven operator /(ComplementPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.complement();
}
public extension AntiDualNum {
    public VersorEven complement() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.scalar), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e3215)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(AntiFlatPoint lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Dipole operator /(ComplementPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.complement();
}
public extension AntiFlatPoint {
    public Dipole complement() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e321) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(AntiFlector lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public DipoleInversion operator /(ComplementPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.complement();
}
public extension AntiFlector {
    public DipoleInversion complement() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e321) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Circle operator /(AntiLine lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Circle operator /(ComplementPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.complement();
}
public extension AntiLine {
    public Circle complement() {
        return Circle(
            /* e423, e431, e412 */
            this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(AntiMotor lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public VersorEven operator /(ComplementPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.complement();
}
public extension AntiMotor {
    public VersorEven complement() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group1.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e3215)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(AntiPlane lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Sphere operator /(ComplementPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.complement();
}
public extension AntiPlane {
    public Sphere complement() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0), 
            /* e1234 */
            this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiScalar lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Scalar operator /(ComplementPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.complement();
}
public extension AntiScalar {
    public Scalar complement() {
        return Scalar(
            /* scalar */
            this.e12345
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Circle lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Dipole operator /(ComplementPrefixOrPostfix lhs, Circle rhs) {
    return rhs.complement();
}
public extension Circle {
    public Dipole complement() {
        return Dipole(
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiCircleRotor operator /(CircleRotor lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(ComplementPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.complement();
}
public extension CircleRotor {
    public AntiCircleRotor complement() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            float4(this.group0.xyz, this.e12345) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Circle operator /(Dipole lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Circle operator /(ComplementPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.complement();
}
public extension Dipole {
    public Circle complement() {
        return Circle(
            /* e423, e431, e412 */
            this.group2.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(DipoleInversion lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(ComplementPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.complement();
}
public extension DipoleInversion {
    public AntiDipoleInversion complement() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group2.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            float4(this.group0.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e4235, this.e4315, this.e4125, this.e1234)
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(DualNum lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public VersorOdd operator /(ComplementPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.complement();
}
public extension DualNum {
    public VersorOdd complement() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.e12345), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public Circle operator /(FlatPoint lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Circle operator /(ComplementPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.complement();
}
public extension FlatPoint {
    public Circle complement() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e45) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(Flector lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(ComplementPrefixOrPostfix lhs, Flector rhs) {
    return rhs.complement();
}
public extension Flector {
    public AntiDipoleInversion complement() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e45) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e3215), 
            /* e1, e2, e3, e5 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Line lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Dipole operator /(ComplementPrefixOrPostfix lhs, Line rhs) {
    return rhs.complement();
}
public extension Line {
    public Dipole complement() {
        return Dipole(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(Motor lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public VersorOdd operator /(ComplementPrefixOrPostfix lhs, Motor rhs) {
    return rhs.complement();
}
public extension Motor {
    public VersorOdd complement() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group1.xyz, this.e12345) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public MultiVector operator /(ComplementPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.complement();
}
public extension MultiVector {
    public MultiVector complement() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.yx, 
            /* e1, e2, e3, e4 */
            this.group9, 
            /* e5 */
            this.e1234, 
            /* e15, e25, e35, e45 */
            float4(this.group7.xyz, this.e321) * float4(-1.0), 
            /* e41, e42, e43 */
            this.group8.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group6.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(this.group5.xyz, this.e45) * float4(-1.0), 
            /* e423, e431, e412 */
            this.group3.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group4.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1, 
            /* e1234 */
            this.e5
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(Plane lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public RoundPoint operator /(ComplementPrefixOrPostfix lhs, Plane rhs) {
    return rhs.complement();
}
public extension Plane {
    public RoundPoint complement() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            0.0
        );
    }
}
// Fancy postfix trick
public Sphere operator /(RoundPoint lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public Sphere operator /(ComplementPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.complement();
}
public extension RoundPoint {
    public Sphere complement() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e5
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Scalar lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public AntiScalar operator /(ComplementPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.complement();
}
public extension Scalar {
    public AntiScalar complement() {
        return AntiScalar(
            /* e12345 */
            this.scalar
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(Sphere lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public RoundPoint operator /(ComplementPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.complement();
}
public extension Sphere {
    public RoundPoint complement() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorEven lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public VersorOdd operator /(ComplementPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.complement();
}
public extension VersorEven {
    public VersorOdd complement() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group2.xyz, this.e12345) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group0.xyz, this.e5) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorOdd lhs, ComplementPrefixOrPostfix rhs)  {
    return lhs.complement();
    }
// Fancy prefix trick
public VersorEven operator /(ComplementPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.complement();
}
public extension VersorOdd {
    public VersorEven complement() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group2.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
