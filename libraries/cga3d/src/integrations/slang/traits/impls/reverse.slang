implementing cga3d;
// Fancy postfix trick
public AntiCircleRotor operator /(AntiCircleRotor lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(ReversePrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.reverse();
}
public extension AntiCircleRotor {
    public AntiCircleRotor reverse() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(ReversePrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.reverse();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion reverse() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiDualNum operator /(ReversePrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.reverse();
}
public extension AntiDualNum {
    public AntiDualNum reverse() {
        return this;
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(ReversePrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.reverse();
}
public extension AntiFlatPoint {
    public AntiFlatPoint reverse() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiFlector operator /(ReversePrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.reverse();
}
public extension AntiFlector {
    public AntiFlector reverse() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(AntiLine lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiLine operator /(ReversePrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.reverse();
}
public extension AntiLine {
    public AntiLine reverse() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(AntiMotor lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiMotor operator /(ReversePrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.reverse();
}
public extension AntiMotor {
    public AntiMotor reverse() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiPlane operator /(ReversePrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.reverse();
}
public extension AntiPlane {
    public AntiPlane reverse() {
        return this;
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiScalar operator /(ReversePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.reverse();
}
public extension AntiScalar {
    public AntiScalar reverse() {
        return this;
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Circle operator /(ReversePrefixOrPostfix lhs, Circle rhs) {
    return rhs.reverse();
}
public extension Circle {
    public Circle reverse() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public CircleRotor operator /(CircleRotor lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public CircleRotor operator /(ReversePrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.reverse();
}
public extension CircleRotor {
    public CircleRotor reverse() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Dipole operator /(ReversePrefixOrPostfix lhs, Dipole rhs) {
    return rhs.reverse();
}
public extension Dipole {
    public Dipole reverse() {
        return Dipole(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35 */
            this.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public DipoleInversion operator /(ReversePrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.reverse();
}
public extension DipoleInversion {
    public DipoleInversion reverse() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public DualNum operator /(ReversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.reverse();
}
public extension DualNum {
    public DualNum reverse() {
        return this;
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public FlatPoint operator /(ReversePrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.reverse();
}
public extension FlatPoint {
    public FlatPoint reverse() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Flector operator /(ReversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.reverse();
}
public extension Flector {
    public Flector reverse() {
        return Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Line operator /(ReversePrefixOrPostfix lhs, Line rhs) {
    return rhs.reverse();
}
public extension Line {
    public Line reverse() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Motor operator /(ReversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.reverse();
}
public extension Motor {
    public Motor reverse() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public MultiVector operator /(ReversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.reverse();
}
public extension MultiVector {
    public MultiVector reverse() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0), 
            /* e423, e431, e412 */
            this.group7.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Plane operator /(ReversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.reverse();
}
public extension Plane {
    public Plane reverse() {
        return this;
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public RoundPoint operator /(ReversePrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.reverse();
}
public extension RoundPoint {
    public RoundPoint reverse() {
        return this;
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Scalar operator /(ReversePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.reverse();
}
public extension Scalar {
    public Scalar reverse() {
        return this;
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Sphere operator /(ReversePrefixOrPostfix lhs, Sphere rhs) {
    return rhs.reverse();
}
public extension Sphere {
    public Sphere reverse() {
        return this;
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public VersorEven operator /(ReversePrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.reverse();
}
public extension VersorEven {
    public VersorEven reverse() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            this.group3
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public VersorOdd operator /(ReversePrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.reverse();
}
public extension VersorOdd {
    public VersorOdd reverse() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3
        );
    }
}
