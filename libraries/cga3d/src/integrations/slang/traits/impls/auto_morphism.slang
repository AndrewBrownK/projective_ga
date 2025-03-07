implementing cga3d;
// Fancy postfix trick
public AntiCircleRotor operator /(AntiCircleRotor lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(AutoMorphismPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.auto_morphism();
}
public extension AntiCircleRotor {
    public AntiCircleRotor auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(AutoMorphismPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.auto_morphism();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion auto_morphism() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiDualNum operator /(AutoMorphismPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.auto_morphism();
}
public extension AntiDualNum {
    public AntiDualNum auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(AutoMorphismPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.auto_morphism();
}
public extension AntiFlatPoint {
    public AntiFlatPoint auto_morphism() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiFlector operator /(AutoMorphismPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.auto_morphism();
}
public extension AntiFlector {
    public AntiFlector auto_morphism() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(AntiLine lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiLine operator /(AutoMorphismPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.auto_morphism();
}
public extension AntiLine {
    public AntiLine auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiMotor operator /(AntiMotor lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiMotor operator /(AutoMorphismPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.auto_morphism();
}
public extension AntiMotor {
    public AntiMotor auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiPlane operator /(AutoMorphismPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.auto_morphism();
}
public extension AntiPlane {
    public AntiPlane auto_morphism() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiScalar operator /(AutoMorphismPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.auto_morphism();
}
public extension AntiScalar {
    public AntiScalar auto_morphism() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * -1.0
        );
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Circle operator /(AutoMorphismPrefixOrPostfix lhs, Circle rhs) {
    return rhs.auto_morphism();
}
public extension Circle {
    public Circle auto_morphism() {
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
public CircleRotor operator /(CircleRotor lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public CircleRotor operator /(AutoMorphismPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.auto_morphism();
}
public extension CircleRotor {
    public CircleRotor auto_morphism() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Dipole operator /(AutoMorphismPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.auto_morphism();
}
public extension Dipole {
    public Dipole auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public DipoleInversion operator /(AutoMorphismPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.auto_morphism();
}
public extension DipoleInversion {
    public DipoleInversion auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public DualNum operator /(AutoMorphismPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.auto_morphism();
}
public extension DualNum {
    public DualNum auto_morphism() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy * float2(-1.0)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public FlatPoint operator /(AutoMorphismPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.auto_morphism();
}
public extension FlatPoint {
    public FlatPoint auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Flector operator /(AutoMorphismPrefixOrPostfix lhs, Flector rhs) {
    return rhs.auto_morphism();
}
public extension Flector {
    public Flector auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Line operator /(AutoMorphismPrefixOrPostfix lhs, Line rhs) {
    return rhs.auto_morphism();
}
public extension Line {
    public Line auto_morphism() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Motor operator /(AutoMorphismPrefixOrPostfix lhs, Motor rhs) {
    return rhs.auto_morphism();
}
public extension Motor {
    public Motor auto_morphism() {
        return Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public MultiVector operator /(AutoMorphismPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.auto_morphism();
}
public extension MultiVector {
    public MultiVector auto_morphism() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e5 */
            this.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
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
public Plane operator /(Plane lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Plane operator /(AutoMorphismPrefixOrPostfix lhs, Plane rhs) {
    return rhs.auto_morphism();
}
public extension Plane {
    public Plane auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public RoundPoint operator /(AutoMorphismPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.auto_morphism();
}
public extension RoundPoint {
    public RoundPoint auto_morphism() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e5 */
            this.e5 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Scalar operator /(AutoMorphismPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.auto_morphism();
}
public extension Scalar {
    public Scalar auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Sphere operator /(AutoMorphismPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.auto_morphism();
}
public extension Sphere {
    public Sphere auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public VersorEven operator /(AutoMorphismPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.auto_morphism();
}
public extension VersorEven {
    public VersorEven auto_morphism() {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e4 */
            this.group3 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public VersorOdd operator /(AutoMorphismPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.auto_morphism();
}
public extension VersorOdd {
    public VersorOdd auto_morphism() {
        return this;
    }
}
