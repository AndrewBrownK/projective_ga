implementing cga3d;
// Fancy postfix trick
public CircleRotor operator /(AntiCircleRotor lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public CircleRotor operator /(RightAntiDualPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.right_anti_dual();
}
public extension AntiCircleRotor {
    public CircleRotor right_anti_dual() {
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
public DipoleInversion operator /(AntiDipoleInversion lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public DipoleInversion operator /(RightAntiDualPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.right_anti_dual();
}
public extension AntiDipoleInversion {
    public DipoleInversion right_anti_dual() {
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
public DualNum operator /(AntiDualNum lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public DualNum operator /(RightAntiDualPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.right_anti_dual();
}
public extension AntiDualNum {
    public DualNum right_anti_dual() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiFlatPoint lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public FlatPoint operator /(RightAntiDualPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.right_anti_dual();
}
public extension AntiFlatPoint {
    public FlatPoint right_anti_dual() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, this.e321 * -1.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiFlector lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Flector operator /(RightAntiDualPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.right_anti_dual();
}
public extension AntiFlector {
    public Flector right_anti_dual() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, this.e321 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(AntiLine lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Line operator /(RightAntiDualPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.right_anti_dual();
}
public extension AntiLine {
    public Line right_anti_dual() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(AntiMotor lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Motor operator /(RightAntiDualPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.right_anti_dual();
}
public extension AntiMotor {
    public Motor right_anti_dual() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(AntiPlane lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Plane operator /(RightAntiDualPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.right_anti_dual();
}
public extension AntiPlane {
    public Plane right_anti_dual() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiScalar lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Scalar operator /(RightAntiDualPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.right_anti_dual();
}
public extension AntiScalar {
    public Scalar right_anti_dual() {
        return Scalar(
            /* scalar */
            this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Circle lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Dipole operator /(RightAntiDualPrefixOrPostfix lhs, Circle rhs) {
    return rhs.right_anti_dual();
}
public extension Circle {
    public Dipole right_anti_dual() {
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
public AntiCircleRotor operator /(CircleRotor lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(RightAntiDualPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.right_anti_dual();
}
public extension CircleRotor {
    public AntiCircleRotor right_anti_dual() {
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
public Circle operator /(Dipole lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Circle operator /(RightAntiDualPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.right_anti_dual();
}
public extension Dipole {
    public Circle right_anti_dual() {
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
public AntiDipoleInversion operator /(DipoleInversion lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(RightAntiDualPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.right_anti_dual();
}
public extension DipoleInversion {
    public AntiDipoleInversion right_anti_dual() {
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
public AntiDualNum operator /(DualNum lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiDualNum operator /(RightAntiDualPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.right_anti_dual();
}
public extension DualNum {
    public AntiDualNum right_anti_dual() {
        return AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(FlatPoint lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(RightAntiDualPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.right_anti_dual();
}
public extension FlatPoint {
    public AntiFlatPoint right_anti_dual() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(Flector lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiFlector operator /(RightAntiDualPrefixOrPostfix lhs, Flector rhs) {
    return rhs.right_anti_dual();
}
public extension Flector {
    public AntiFlector right_anti_dual() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(Line lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiLine operator /(RightAntiDualPrefixOrPostfix lhs, Line rhs) {
    return rhs.right_anti_dual();
}
public extension Line {
    public AntiLine right_anti_dual() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(Motor lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiMotor operator /(RightAntiDualPrefixOrPostfix lhs, Motor rhs) {
    return rhs.right_anti_dual();
}
public extension Motor {
    public AntiMotor right_anti_dual() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, this.e12345 * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public MultiVector operator /(RightAntiDualPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.right_anti_dual();
}
public extension MultiVector {
    public MultiVector right_anti_dual() {
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
public AntiPlane operator /(Plane lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiPlane operator /(RightAntiDualPrefixOrPostfix lhs, Plane rhs) {
    return rhs.right_anti_dual();
}
public extension Plane {
    public AntiPlane right_anti_dual() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(RoundPoint lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Sphere operator /(RightAntiDualPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.right_anti_dual();
}
public extension RoundPoint {
    public Sphere right_anti_dual() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5 * -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Scalar lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiScalar operator /(RightAntiDualPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.right_anti_dual();
}
public extension Scalar {
    public AntiScalar right_anti_dual() {
        return AntiScalar(
            /* e12345 */
            this.scalar
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(Sphere lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public RoundPoint operator /(RightAntiDualPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.right_anti_dual();
}
public extension Sphere {
    public RoundPoint right_anti_dual() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz * float3(-1.0)), this.e1234), 
            /* e5 */
            this.e3215
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorEven lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public VersorOdd operator /(RightAntiDualPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.right_anti_dual();
}
public extension VersorEven {
    public VersorOdd right_anti_dual() {
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
public VersorEven operator /(VersorOdd lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public VersorEven operator /(RightAntiDualPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.right_anti_dual();
}
public extension VersorOdd {
    public VersorEven right_anti_dual() {
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
