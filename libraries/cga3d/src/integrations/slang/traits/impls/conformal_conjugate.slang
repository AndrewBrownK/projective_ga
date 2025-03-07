implementing cga3d;
// Fancy postfix trick
public AntiCircleRotor operator /(AntiCircleRotor lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(ConformalConjugatePrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiCircleRotor {
    public AntiCircleRotor conformal_conjugate() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(ConformalConjugatePrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion conformal_conjugate() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiDualNum operator /(ConformalConjugatePrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiDualNum {
    public AntiDualNum conformal_conjugate() {
        return AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(ConformalConjugatePrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiFlatPoint {
    public AntiFlatPoint conformal_conjugate() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiFlector operator /(ConformalConjugatePrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiFlector {
    public AntiFlector conformal_conjugate() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(AntiLine lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiLine operator /(ConformalConjugatePrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiLine {
    public AntiLine conformal_conjugate() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(AntiMotor lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiMotor operator /(ConformalConjugatePrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiMotor {
    public AntiMotor conformal_conjugate() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0, 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiPlane operator /(ConformalConjugatePrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiPlane {
    public AntiPlane conformal_conjugate() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public AntiScalar operator /(ConformalConjugatePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.conformal_conjugate();
}
public extension AntiScalar {
    public AntiScalar conformal_conjugate() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Circle operator /(ConformalConjugatePrefixOrPostfix lhs, Circle rhs) {
    return rhs.conformal_conjugate();
}
public extension Circle {
    public Circle conformal_conjugate() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public CircleRotor operator /(CircleRotor lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public CircleRotor operator /(ConformalConjugatePrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.conformal_conjugate();
}
public extension CircleRotor {
    public CircleRotor conformal_conjugate() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Dipole operator /(ConformalConjugatePrefixOrPostfix lhs, Dipole rhs) {
    return rhs.conformal_conjugate();
}
public extension Dipole {
    public Dipole conformal_conjugate() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public DipoleInversion operator /(ConformalConjugatePrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.conformal_conjugate();
}
public extension DipoleInversion {
    public DipoleInversion conformal_conjugate() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public DualNum operator /(ConformalConjugatePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.conformal_conjugate();
}
public extension DualNum {
    public DualNum conformal_conjugate() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy * float2(-1.0)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public FlatPoint operator /(ConformalConjugatePrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.conformal_conjugate();
}
public extension FlatPoint {
    public FlatPoint conformal_conjugate() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Flector operator /(ConformalConjugatePrefixOrPostfix lhs, Flector rhs) {
    return rhs.conformal_conjugate();
}
public extension Flector {
    public Flector conformal_conjugate() {
        return Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Line operator /(ConformalConjugatePrefixOrPostfix lhs, Line rhs) {
    return rhs.conformal_conjugate();
}
public extension Line {
    public Line conformal_conjugate() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Motor operator /(ConformalConjugatePrefixOrPostfix lhs, Motor rhs) {
    return rhs.conformal_conjugate();
}
public extension Motor {
    public Motor conformal_conjugate() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public MultiVector operator /(ConformalConjugatePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.conformal_conjugate();
}
public extension MultiVector {
    public MultiVector conformal_conjugate() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 * float4(-1.0), 
            /* e1234 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Plane operator /(ConformalConjugatePrefixOrPostfix lhs, Plane rhs) {
    return rhs.conformal_conjugate();
}
public extension Plane {
    public Plane conformal_conjugate() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public RoundPoint operator /(ConformalConjugatePrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.conformal_conjugate();
}
public extension RoundPoint {
    public RoundPoint conformal_conjugate() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            this.e5 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Scalar operator /(ConformalConjugatePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.conformal_conjugate();
}
public extension Scalar {
    public Scalar conformal_conjugate() {
        return this;
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public Sphere operator /(ConformalConjugatePrefixOrPostfix lhs, Sphere rhs) {
    return rhs.conformal_conjugate();
}
public extension Sphere {
    public Sphere conformal_conjugate() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0), 
            /* e1234 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public VersorEven operator /(ConformalConjugatePrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.conformal_conjugate();
}
public extension VersorEven {
    public VersorEven conformal_conjugate() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, ConformalConjugatePrefixOrPostfix rhs)  {
    return lhs.conformal_conjugate();
    }
// Fancy prefix trick
public VersorOdd operator /(ConformalConjugatePrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.conformal_conjugate();
}
public extension VersorOdd {
    public VersorOdd conformal_conjugate() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
