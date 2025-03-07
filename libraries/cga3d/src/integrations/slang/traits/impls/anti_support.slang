implementing cga3d;
using traits;
// Fancy postfix trick
public AntiMotor operator /(AntiCircleRotor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiMotor operator /(AntiSupportPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.anti_support();
}
public extension AntiCircleRotor {
    public AntiMotor anti_support() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((this.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, this.scalar)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiDipoleInversion lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiFlector operator /(AntiSupportPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.anti_support();
}
public extension AntiDipoleInversion {
    public AntiFlector anti_support() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.group3 * float4(1.0, 1.0, 1.0, -1.0)).xyz, this.e4 * -1.0) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiDualNum operator /(AntiSupportPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.anti_support();
}
public extension AntiDualNum {
    public AntiDualNum anti_support() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiFlatPoint lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public DualNum operator /(AntiSupportPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.anti_support();
}
public extension AntiFlatPoint {
    public DualNum anti_support() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e321, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiFlector operator /(AntiSupportPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.anti_support();
}
public extension AntiFlector {
    public AntiFlector anti_support() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((this.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e321 * -1.0) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiLine lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public FlatPoint operator /(AntiSupportPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.anti_support();
}
public extension AntiLine {
    public FlatPoint anti_support() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiMotor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Flector operator /(AntiSupportPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.anti_support();
}
public extension AntiMotor {
    public Flector anti_support() {
        float4 right_dual_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g0[3])
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiPlane lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(AntiSupportPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.anti_support();
}
public extension AntiPlane {
    public AntiFlatPoint anti_support() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(Circle lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiPlane operator /(AntiSupportPrefixOrPostfix lhs, Circle rhs) {
    return rhs.anti_support();
}
public extension Circle {
    public AntiPlane anti_support() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(CircleRotor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiPlane operator /(AntiSupportPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.anti_support();
}
public extension CircleRotor {
    public AntiPlane anti_support() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(Dipole lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiLine operator /(AntiSupportPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.anti_support();
}
public extension Dipole {
    public AntiLine anti_support() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (this.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(DipoleInversion lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiMotor operator /(AntiSupportPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.anti_support();
}
public extension DipoleInversion {
    public AntiMotor anti_support() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz * float3(-1.0)), this.e1234), 
            /* e15, e25, e35, e3215 */
            float4((this.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public MultiVector operator /(AntiSupportPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_support();
}
public extension MultiVector {
    public MultiVector anti_support() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e1234, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group7.xyz * float3(-1.0)), 0.0), 
            /* e5 */
            this.e321, 
            /* e15, e25, e35, e45 */
            float4((float4(this.group5.xyz, this.e45) * float4(-1.0, -1.0, -1.0, 1.0)).xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group4.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e4 * -1.0) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float4(this.group1.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.group0.yx[1]), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(RoundPoint lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(AntiSupportPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.anti_support();
}
public extension RoundPoint {
    public AntiFlatPoint anti_support() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float4(this.group0.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz, this.e4 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(Scalar lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiDualNum operator /(AntiSupportPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_support();
}
public extension Scalar {
    public AntiDualNum anti_support() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Scalar operator /(AntiSupportPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.anti_support();
}
public extension Sphere {
    public Scalar anti_support() {
        return Scalar(
            /* scalar */
            this.e1234
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(VersorEven lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiFlector operator /(AntiSupportPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.anti_support();
}
public extension VersorEven {
    public AntiFlector anti_support() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz, this.e4 * -1.0) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4((this.group0 * float4(1.0, 1.0, 1.0, -1.0)).xyz, this.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(VersorOdd lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiMotor operator /(AntiSupportPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.anti_support();
}
public extension VersorOdd {
    public AntiMotor anti_support() {
        float4 right_dual_g0 = this.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0.xyz, this.e1234), 
            /* e15, e25, e35, e3215 */
            float4((this.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, right_dual_g0[3])
        );
    }
}
