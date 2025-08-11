implementing cga3d;
// Fancy postfix trick
public AntiMotor operator /(AntiCircleRotor lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiMotor operator /(RoundBulkPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_bulk();
}
public extension AntiCircleRotor {
    public AntiMotor round_bulk() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiDipoleInversion lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiFlector operator /(RoundBulkPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_bulk();
}
public extension AntiDipoleInversion {
    public AntiFlector round_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.round_bulk();
}
public extension AntiDualNum {
    public Scalar round_bulk() {
        return Scalar(
            /* scalar */
            this.scalar
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(RoundBulkPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.round_bulk();
}
public extension AntiFlatPoint {
    public AntiFlatPoint round_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiFlector lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiFlector operator /(RoundBulkPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.round_bulk();
}
public extension AntiFlector {
    public AntiFlector round_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(AntiLine lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiLine operator /(RoundBulkPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.round_bulk();
}
public extension AntiLine {
    public AntiLine round_bulk() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(AntiMotor lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiMotor operator /(RoundBulkPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.round_bulk();
}
public extension AntiMotor {
    public AntiMotor round_bulk() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0, 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiPlane operator /(RoundBulkPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.round_bulk();
}
public extension AntiPlane {
    public AntiPlane round_bulk() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(Circle lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(RoundBulkPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_bulk();
}
public extension Circle {
    public AntiFlatPoint round_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(CircleRotor lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(RoundBulkPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_bulk();
}
public extension CircleRotor {
    public AntiFlatPoint round_bulk() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(Dipole lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiLine operator /(RoundBulkPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_bulk();
}
public extension Dipole {
    public AntiLine round_bulk() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(DipoleInversion lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiLine operator /(RoundBulkPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_bulk();
}
public extension DipoleInversion {
    public AntiLine round_bulk() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public MultiVector operator /(RoundBulkPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_bulk();
}
public extension MultiVector {
    public MultiVector round_bulk() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(RoundPoint lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiPlane operator /(RoundBulkPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_bulk();
}
public extension RoundPoint {
    public AntiPlane round_bulk() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.round_bulk();
}
public extension Scalar {
    public Scalar round_bulk() {
        return this;
    }
}
// Fancy postfix trick
public AntiFlector operator /(VersorEven lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiFlector operator /(RoundBulkPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_bulk();
}
public extension VersorEven {
    public AntiFlector round_bulk() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(VersorOdd lhs, RoundBulkPrefixOrPostfix rhs)  {
    return lhs.round_bulk();
    }
// Fancy prefix trick
public AntiMotor operator /(RoundBulkPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_bulk();
}
public extension VersorOdd {
    public AntiMotor round_bulk() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
