implementing cga3d;
// Fancy postfix trick
public AntiCircleRotor operator /(AntiCircleRotor lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_auto_morphism() {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiDualNum operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiDualNum {
    public AntiDualNum anti_auto_morphism() {
        return AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiFlector operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiFlector {
    public AntiFlector anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiLine operator /(AntiLine lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiLine operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiLine {
    public AntiLine anti_auto_morphism() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(AntiMotor lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiMotor operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiMotor {
    public AntiMotor anti_auto_morphism() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiPlane operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiPlane {
    public AntiPlane anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiScalar {
    public AntiScalar anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Circle operator /(AntiAutoMorphismPrefixOrPostfix lhs, Circle rhs) {
    return rhs.anti_auto_morphism();
}
public extension Circle {
    public Circle anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public CircleRotor operator /(CircleRotor lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public CircleRotor operator /(AntiAutoMorphismPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.anti_auto_morphism();
}
public extension CircleRotor {
    public CircleRotor anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Dipole operator /(AntiAutoMorphismPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.anti_auto_morphism();
}
public extension Dipole {
    public Dipole anti_auto_morphism() {
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
public DipoleInversion operator /(DipoleInversion lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public DipoleInversion operator /(AntiAutoMorphismPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.anti_auto_morphism();
}
public extension DipoleInversion {
    public DipoleInversion anti_auto_morphism() {
        return DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public DualNum operator /(AntiAutoMorphismPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_auto_morphism();
}
public extension DualNum {
    public DualNum anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public FlatPoint operator /(AntiAutoMorphismPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.anti_auto_morphism();
}
public extension FlatPoint {
    public FlatPoint anti_auto_morphism() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Flector operator /(AntiAutoMorphismPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_auto_morphism();
}
public extension Flector {
    public Flector anti_auto_morphism() {
        return Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Line operator /(AntiAutoMorphismPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_auto_morphism();
}
public extension Line {
    public Line anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Motor operator /(AntiAutoMorphismPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_auto_morphism();
}
public extension Motor {
    public Motor anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public MultiVector operator /(AntiAutoMorphismPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_auto_morphism();
}
public extension MultiVector {
    public MultiVector anti_auto_morphism() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy * float2(-1.0, 1.0), 
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
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 * float4(-1.0), 
            /* e1234 */
            this.e1234 * -1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Plane operator /(AntiAutoMorphismPrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_auto_morphism();
}
public extension Plane {
    public Plane anti_auto_morphism() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiAutoMorphismPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.anti_auto_morphism();
}
public extension RoundPoint {
    public RoundPoint anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Scalar operator /(AntiAutoMorphismPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_auto_morphism();
}
public extension Scalar {
    public Scalar anti_auto_morphism() {
        return Scalar(
            /* scalar */
            this.scalar * -1.0
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Sphere operator /(AntiAutoMorphismPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.anti_auto_morphism();
}
public extension Sphere {
    public Sphere anti_auto_morphism() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0), 
            /* e1234 */
            this.e1234 * -1.0
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public VersorEven operator /(AntiAutoMorphismPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.anti_auto_morphism();
}
public extension VersorEven {
    public VersorEven anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public VersorOdd operator /(AntiAutoMorphismPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.anti_auto_morphism();
}
public extension VersorOdd {
    public VersorOdd anti_auto_morphism() {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(-1.0)
        );
    }
}
