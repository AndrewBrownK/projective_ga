implementing cga3d;
// Fancy postfix trick
public Plane operator /(AntiCircleRotor lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Plane operator /(CoCarrierPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.co_carrier();
}
public extension AntiCircleRotor {
    public Plane co_carrier() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group0.xyz * float3(-1.0)), this.e45)
        );
    }
}
// Fancy postfix trick
public Motor operator /(AntiDipoleInversion lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Motor operator /(CoCarrierPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.co_carrier();
}
public extension AntiDipoleInversion {
    public Motor co_carrier() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xyz, this.e4 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiScalar lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public DualNum operator /(CoCarrierPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.co_carrier();
}
public extension AntiScalar {
    public DualNum co_carrier() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345 * -1.0, 0.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(Circle lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Line operator /(CoCarrierPrefixOrPostfix lhs, Circle rhs) {
    return rhs.co_carrier();
}
public extension Circle {
    public Line co_carrier() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(CircleRotor lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Motor operator /(CoCarrierPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.co_carrier();
}
public extension CircleRotor {
    public Motor co_carrier() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.group1.xyz, this.e12345 * -1.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(Dipole lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Plane operator /(CoCarrierPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.co_carrier();
}
public extension Dipole {
    public Plane co_carrier() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group0.xyz * float3(-1.0)), this.e45)
        );
    }
}
// Fancy postfix trick
public Flector operator /(DipoleInversion lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Flector operator /(CoCarrierPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.co_carrier();
}
public extension DipoleInversion {
    public Flector co_carrier() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.group3.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group0.xyz * float3(-1.0)), this.e45)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public DualNum operator /(CoCarrierPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.co_carrier();
}
public extension DualNum {
    public DualNum co_carrier() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345 * -1.0, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(FlatPoint lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public AntiDualNum operator /(CoCarrierPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.co_carrier();
}
public extension FlatPoint {
    public AntiDualNum co_carrier() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e45, 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Flector operator /(CoCarrierPrefixOrPostfix lhs, Flector rhs) {
    return rhs.co_carrier();
}
public extension Flector {
    public Flector co_carrier() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.group1.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e45)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(Line lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(CoCarrierPrefixOrPostfix lhs, Line rhs) {
    return rhs.co_carrier();
}
public extension Line {
    public AntiFlatPoint co_carrier() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(Motor lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public AntiFlector operator /(CoCarrierPrefixOrPostfix lhs, Motor rhs) {
    return rhs.co_carrier();
}
public extension Motor {
    public AntiFlector co_carrier() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e12345 * -1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public MultiVector operator /(CoCarrierPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.co_carrier();
}
public extension MultiVector {
    public MultiVector co_carrier() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e4 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e12345 * -1.0, 
            /* e15, e25, e35, e45 */
            float4((this.group9.xyz * float3(-1.0)), this.e1234), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group7.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group6.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group4.xyz * float3(-1.0)), this.e45), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(Plane lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public FlatPoint operator /(CoCarrierPrefixOrPostfix lhs, Plane rhs) {
    return rhs.co_carrier();
}
public extension Plane {
    public FlatPoint co_carrier() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(RoundPoint lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public AntiScalar operator /(CoCarrierPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.co_carrier();
}
public extension RoundPoint {
    public AntiScalar co_carrier() {
        return AntiScalar(
            /* e12345 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(Sphere lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public FlatPoint operator /(CoCarrierPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.co_carrier();
}
public extension Sphere {
    public FlatPoint co_carrier() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.group0.xyz * float3(-1.0)), this.e1234)
        );
    }
}
// Fancy postfix trick
public Motor operator /(VersorEven lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Motor operator /(CoCarrierPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.co_carrier();
}
public extension VersorEven {
    public Motor co_carrier() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xyz, this.e4 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group1.xyz, this.e12345 * -1.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(VersorOdd lhs, CoCarrierPrefixOrPostfix rhs)  {
    return lhs.co_carrier();
    }
// Fancy prefix trick
public Flector operator /(CoCarrierPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.co_carrier();
}
public extension VersorOdd {
    public Flector co_carrier() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((this.group3.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group0.xyz * float3(-1.0)), this.e45)
        );
    }
}
