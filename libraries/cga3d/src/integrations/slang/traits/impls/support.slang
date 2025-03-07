implementing cga3d;
// Fancy postfix trick
public Sphere operator /(AntiCircleRotor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Sphere operator /(SupportPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.support();
}
public extension AntiCircleRotor {
    public Sphere support() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, 0.0), 
            /* e1234 */
            this.e45
        );
    }
}
// Fancy postfix trick
public CircleRotor operator /(AntiDipoleInversion lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public CircleRotor operator /(SupportPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.support();
}
public extension AntiDipoleInversion {
    public CircleRotor support() {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz, 
            /* e415, e425, e435, e321 */
            float4((this.group2 * float4(1.0, 1.0, 1.0, -1.0)).xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), this.e5 * -1.0)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiDualNum lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public FlatPoint operator /(SupportPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.support();
}
public extension AntiDualNum {
    public FlatPoint support() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e3215)
        );
    }
}
// Fancy postfix trick
public Line operator /(AntiFlatPoint lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Line operator /(SupportPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.support();
}
public extension AntiFlatPoint {
    public Line support() {
        return Line(
            /* e415, e425, e435 */
            (this.group0 * float4(1.0, 1.0, 1.0, -1.0)).xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(AntiFlector lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Motor operator /(SupportPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.support();
}
public extension AntiFlector {
    public Motor support() {
        float4 right_anti_dual_g1 = this.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(float3(0.0), 1.0).wwwx * float4((this.group0 * float4(1.0, 1.0, 1.0, -1.0)).xyz, right_anti_dual_g1[0])) + float4(float3(0.0), right_anti_dual_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(AntiLine lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Plane operator /(SupportPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.support();
}
public extension AntiLine {
    public Plane support() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiMotor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Flector operator /(SupportPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.support();
}
public extension AntiMotor {
    public Flector support() {
        float4 right_anti_dual_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), right_anti_dual_g1[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiPlane lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public AntiScalar operator /(SupportPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.support();
}
public extension AntiPlane {
    public AntiScalar support() {
        return AntiScalar(
            /* e12345 */
            this.e5 * -1.0
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(AntiScalar lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public RoundPoint operator /(SupportPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.support();
}
public extension AntiScalar {
    public RoundPoint support() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e12345 * -1.0), 
            /* e5 */
            0.0
        );
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Circle operator /(SupportPrefixOrPostfix lhs, Circle rhs) {
    return rhs.support();
}
public extension Circle {
    public Circle support() {
        return Circle(
            /* e423, e431, e412 */
            (this.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(CircleRotor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(SupportPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.support();
}
public extension CircleRotor {
    public AntiDipoleInversion support() {
        float4 right_anti_dual_g2 = this.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g2.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), right_anti_dual_g2[3]), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Dipole lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Sphere operator /(SupportPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.support();
}
public extension Dipole {
    public Sphere support() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2.xyz * float3(-1.0)), 0.0), 
            /* e1234 */
            this.e45
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public DipoleInversion operator /(SupportPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.support();
}
public extension DipoleInversion {
    public DipoleInversion support() {
        float4 right_anti_dual_g3 = this.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            right_anti_dual_g3.xyz, 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual_g3[3]), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group2 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(DualNum lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public VersorEven operator /(SupportPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.support();
}
public extension DualNum {
    public VersorEven support() {
        float2 right_anti_dual_g0 = this.group0.xy * float2(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0[1])
        );
    }
}
// Fancy postfix trick
public Sphere operator /(FlatPoint lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Sphere operator /(SupportPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.support();
}
public extension FlatPoint {
    public Sphere support() {
        float4 right_anti_dual_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0.xyz, 0.0), 
            /* e1234 */
            right_anti_dual_g0[3]
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(Flector lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public DipoleInversion operator /(SupportPrefixOrPostfix lhs, Flector rhs) {
    return rhs.support();
}
public extension Flector {
    public DipoleInversion support() {
        float4 right_anti_dual_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = this.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            right_anti_dual_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Circle operator /(Line lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Circle operator /(SupportPrefixOrPostfix lhs, Line rhs) {
    return rhs.support();
}
public extension Line {
    public Circle support() {
        return Circle(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, 0.0), 
            /* e235, e315, e125 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(Motor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public VersorEven operator /(SupportPrefixOrPostfix lhs, Motor rhs) {
    return rhs.support();
}
public extension Motor {
    public VersorEven support() {
        float4 right_anti_dual_g0 = this.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = this.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3]), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g1.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0[3])
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public MultiVector operator /(SupportPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.support();
}
public extension MultiVector {
    public MultiVector support() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.group0.yx[0] * -1.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e3215), 
            /* e41, e42, e43 */
            (float4(this.group9.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)).xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group8.xyz, 0.0), 
            /* e423, e431, e412 */
            this.group6.xyz, 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group3.xyz * float3(-1.0)), 0.0), 
            /* e1234 */
            this.e45
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Plane lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Dipole operator /(SupportPrefixOrPostfix lhs, Plane rhs) {
    return rhs.support();
}
public extension Plane {
    public Dipole support() {
        float4 right_anti_dual_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Dipole(
            /* e41, e42, e43 */
            right_anti_dual_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual_g0[3]), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(RoundPoint lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public AntiScalar operator /(SupportPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.support();
}
public extension RoundPoint {
    public AntiScalar support() {
        return AntiScalar(
            /* e12345 */
            this.e5 * -1.0
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Sphere lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Dipole operator /(SupportPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.support();
}
public extension Sphere {
    public Dipole support() {
        return Dipole(
            /* e41, e42, e43 */
            (float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)).xyz, 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e3215), 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public VersorEven operator /(SupportPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.support();
}
public extension VersorEven {
    public VersorEven support() {
        float4 right_anti_dual_g3 = float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(float3(0.0), 1.0).wwwy * float4((this.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz, right_anti_dual_g3[1])) + float4(float3(0.0), right_anti_dual_g3[0]) + float4(float3(0.0), right_anti_dual_g3[3]), 
            /* e415, e425, e435, e321 */
            float4((float4(this.group2.xyz, this.e4) * float4(1.0, 1.0, 1.0, -1.0)).xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e12345 * -1.0)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(VersorOdd lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public DipoleInversion operator /(SupportPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.support();
}
public extension VersorOdd {
    public DipoleInversion support() {
        float4 right_anti_dual_g2 = float4(this.group2.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float4(this.group3.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)).xyz, 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g2.xyz, 0.0)
        );
    }
}
