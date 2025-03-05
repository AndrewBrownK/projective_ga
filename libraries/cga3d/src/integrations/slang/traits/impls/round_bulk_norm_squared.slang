implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar round_bulk_norm_squared() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.scalar, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar round_bulk_norm_squared() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e321, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiDualNum {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            pow(this.scalar, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiFlatPoint {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            pow(float4(float3(0.0), this.e321)[3], 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiFlector {
    public Scalar round_bulk_norm_squared() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group1.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e321, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiLine {
    public Scalar round_bulk_norm_squared() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiMotor {
    public Scalar round_bulk_norm_squared() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0, 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.scalar, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension AntiPlane {
    public Scalar round_bulk_norm_squared() {
        AntiPlane sub_type = AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension Circle {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            pow(float4(float3(0.0), this.e321)[3], 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension CircleRotor {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            pow(float4(float3(0.0), this.e321)[3], 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension Dipole {
    public Scalar round_bulk_norm_squared() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension DipoleInversion {
    public Scalar round_bulk_norm_squared() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension MultiVector {
    public Scalar round_bulk_norm_squared() {
        MultiVector sub_type = MultiVector(
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
        return Scalar(
            /* scalar */
            2.0*(sub_type.e15 * sub_type.e41) + 2.0 * (sub_type.e25 * sub_type.e42) + 2.0 * (sub_type.e35 * sub_type.e43) + 2.0 * (sub_type.e3215 * sub_type.e1234) + pow(sub_type.scalar, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2) + pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.e321, 2) - pow(sub_type.e12345, 2) - pow(sub_type.e45, 2) - pow(sub_type.e415, 2) - pow(sub_type.e425, 2) - pow(sub_type.e435, 2) - pow(sub_type.e4235, 2) - pow(sub_type.e4315, 2) - pow(sub_type.e4125, 2) - 2.0 * (sub_type.e4 * sub_type.e5) - 2.0 * (sub_type.e423 * sub_type.e235) - 2.0 * (sub_type.e431 * sub_type.e315) - 2.0 * (sub_type.e412 * sub_type.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension RoundPoint {
    public Scalar round_bulk_norm_squared() {
        AntiPlane sub_type = AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension Scalar {
    public Scalar round_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            pow(this.scalar, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension VersorEven {
    public Scalar round_bulk_norm_squared() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e321, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, RoundBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_bulk_norm_squared();
}
public extension VersorOdd {
    public Scalar round_bulk_norm_squared() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.scalar, 2)
        );
    }
}
