implementing cga3d;
// Fancy postfix trick
public FlatPoint operator /(AntiCircleRotor lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public FlatPoint operator /(FlatBulkPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_bulk();
}
public extension AntiCircleRotor {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiDipoleInversion lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlector operator /(FlatBulkPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_bulk();
}
public extension AntiDipoleInversion {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiDualNum lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiDualNum operator /(FlatBulkPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.flat_bulk();
}
public extension AntiDualNum {
    public AntiDualNum flat_bulk() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.flat_bulk();
}
public extension AntiFlatPoint {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlector operator /(FlatBulkPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.flat_bulk();
}
public extension AntiFlector {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiLine lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public FlatPoint operator /(FlatBulkPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.flat_bulk();
}
public extension AntiLine {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiMotor lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Flector operator /(FlatBulkPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.flat_bulk();
}
public extension AntiMotor {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiPlane lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public DualNum operator /(FlatBulkPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.flat_bulk();
}
public extension AntiPlane {
    public DualNum flat_bulk() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(Circle lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_bulk();
}
public extension Circle {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(CircleRotor lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_bulk();
}
public extension CircleRotor {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(Dipole lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public FlatPoint operator /(FlatBulkPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_bulk();
}
public extension Dipole {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(DipoleInversion lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Flector operator /(FlatBulkPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_bulk();
}
public extension DipoleInversion {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public DualNum operator /(FlatBulkPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_bulk();
}
public extension DualNum {
    public DualNum flat_bulk() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5, 0.0)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public FlatPoint operator /(FlatBulkPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_bulk();
}
public extension FlatPoint {
    public FlatPoint flat_bulk() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Flector operator /(FlatBulkPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk();
}
public extension Flector {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(Line lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(FlatBulkPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk();
}
public extension Line {
    public AntiFlatPoint flat_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(Motor lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlector operator /(FlatBulkPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk();
}
public extension Motor {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public MultiVector operator /(FlatBulkPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk();
}
public extension MultiVector {
    public MultiVector flat_bulk() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group3.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(Plane lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiDualNum operator /(FlatBulkPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_bulk();
}
public extension Plane {
    public AntiDualNum flat_bulk() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215, 0.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(RoundPoint lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public DualNum operator /(FlatBulkPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.flat_bulk();
}
public extension RoundPoint {
    public DualNum flat_bulk() {
        return DualNum(
            /* e5, e12345 */
            float2(this.e5, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(Sphere lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiDualNum operator /(FlatBulkPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_bulk();
}
public extension Sphere {
    public AntiDualNum flat_bulk() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(VersorEven lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public AntiFlector operator /(FlatBulkPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_bulk();
}
public extension VersorEven {
    public AntiFlector flat_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
// Fancy postfix trick
public Flector operator /(VersorOdd lhs, FlatBulkPrefixOrPostfix rhs)  {
    return lhs.flat_bulk();
    }
// Fancy prefix trick
public Flector operator /(FlatBulkPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_bulk();
}
public extension VersorOdd {
    public Flector flat_bulk() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
