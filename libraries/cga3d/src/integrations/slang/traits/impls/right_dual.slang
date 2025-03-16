implementing cga3d;
// Fancy postfix trick
public CircleRotor operator /(AntiCircleRotor lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public CircleRotor operator /(RightDualPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.right_dual();
}
public extension AntiCircleRotor {
    public CircleRotor right_dual() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(AntiDipoleInversion lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public DipoleInversion operator /(RightDualPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.right_dual();
}
public extension AntiDipoleInversion {
    public DipoleInversion right_dual() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, this.e321 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiDualNum lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public DualNum operator /(RightDualPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.right_dual();
}
public extension AntiDualNum {
    public DualNum right_dual() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiFlatPoint lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public FlatPoint operator /(RightDualPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.right_dual();
}
public extension AntiFlatPoint {
    public FlatPoint right_dual() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, this.e321 * -1.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiFlector lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Flector operator /(RightDualPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.right_dual();
}
public extension AntiFlector {
    public Flector right_dual() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, this.e321 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(AntiLine lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Line operator /(RightDualPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.right_dual();
}
public extension AntiLine {
    public Line right_dual() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(AntiMotor lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Motor operator /(RightDualPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.right_dual();
}
public extension AntiMotor {
    public Motor right_dual() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(AntiPlane lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Plane operator /(RightDualPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.right_dual();
}
public extension AntiPlane {
    public Plane right_dual() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiScalar lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Scalar operator /(RightDualPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.right_dual();
}
public extension AntiScalar {
    public Scalar right_dual() {
        return Scalar(
            /* scalar */
            this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Circle lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Dipole operator /(RightDualPrefixOrPostfix lhs, Circle rhs) {
    return rhs.right_dual();
}
public extension Circle {
    public Dipole right_dual() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, this.e321 * -1.0), 
            /* e15, e25, e35 */
            this.group2.xyz
        );
    }
}
// Fancy postfix trick
public AntiCircleRotor operator /(CircleRotor lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(RightDualPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.right_dual();
}
public extension CircleRotor {
    public AntiCircleRotor right_dual() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, this.e321 * -1.0), 
            /* e15, e25, e35, scalar */
            float4(this.group2.xyz, this.e12345 * -1.0)
        );
    }
}
// Fancy postfix trick
public Circle operator /(Dipole lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Circle operator /(RightDualPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.right_dual();
}
public extension Dipole {
    public Circle right_dual() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(DipoleInversion lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(RightDualPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.right_dual();
}
public extension DipoleInversion {
    public AntiDipoleInversion right_dual() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(DualNum lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiDualNum operator /(RightDualPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.right_dual();
}
public extension DualNum {
    public AntiDualNum right_dual() {
        return AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(FlatPoint lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(RightDualPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.right_dual();
}
public extension FlatPoint {
    public AntiFlatPoint right_dual() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(Flector lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiFlector operator /(RightDualPrefixOrPostfix lhs, Flector rhs) {
    return rhs.right_dual();
}
public extension Flector {
    public AntiFlector right_dual() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(Line lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiLine operator /(RightDualPrefixOrPostfix lhs, Line rhs) {
    return rhs.right_dual();
}
public extension Line {
    public AntiLine right_dual() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(Motor lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiMotor operator /(RightDualPrefixOrPostfix lhs, Motor rhs) {
    return rhs.right_dual();
}
public extension Motor {
    public AntiMotor right_dual() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, this.e12345 * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public MultiVector operator /(RightDualPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.right_dual();
}
public extension MultiVector {
    public MultiVector right_dual() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((this.group9.xyz * float3(-1.0)), this.e1234), 
            /* e5 */
            this.e3215, 
            /* e15, e25, e35, e45 */
            float4(this.group8.xyz, this.e321 * -1.0), 
            /* e41, e42, e43 */
            this.group7.xyz, 
            /* e23, e31, e12 */
            this.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.group5.xyz * float3(-1.0)), this.e45), 
            /* e423, e431, e412 */
            this.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, this.e5 * -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(Plane lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiPlane operator /(RightDualPrefixOrPostfix lhs, Plane rhs) {
    return rhs.right_dual();
}
public extension Plane {
    public AntiPlane right_dual() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(RoundPoint lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public Sphere operator /(RightDualPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.right_dual();
}
public extension RoundPoint {
    public Sphere right_dual() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5 * -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Scalar lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public AntiScalar operator /(RightDualPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.right_dual();
}
public extension Scalar {
    public AntiScalar right_dual() {
        return AntiScalar(
            /* e12345 */
            this.scalar
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(Sphere lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public RoundPoint operator /(RightDualPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.right_dual();
}
public extension Sphere {
    public RoundPoint right_dual() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz * float3(-1.0)), this.e1234), 
            /* e5 */
            this.e3215
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorEven lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public VersorOdd operator /(RightDualPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.right_dual();
}
public extension VersorEven {
    public VersorOdd right_dual() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group0.xyz, this.e12345 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(this.group1.xyz, this.e321 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorOdd lhs, RightDualPrefixOrPostfix rhs)  {
    return lhs.right_dual();
    }
// Fancy prefix trick
public VersorEven operator /(RightDualPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.right_dual();
}
public extension VersorOdd {
    public VersorEven right_dual() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz * float3(-1.0)), this.e3215), 
            /* e1, e2, e3, e4 */
            float4((this.group3.xyz * float3(-1.0)), this.e1234)
        );
    }
}
