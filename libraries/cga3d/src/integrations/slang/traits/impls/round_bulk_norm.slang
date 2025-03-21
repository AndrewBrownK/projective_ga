implementing cga3d;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiCircleRotor {
    public Scalar round_bulk_norm() {
        float3 sub_type_g0_xyz = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiDipoleInversion {
    public Scalar round_bulk_norm() {
        float3 sub_type_g1_xyz = this.group3.xyz;
        return Scalar(
            /* scalar */
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiDualNum {
    public Scalar round_bulk_norm() {
        return Scalar(
            /* scalar */
            this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiFlatPoint {
    public Scalar round_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiFlector {
    public Scalar round_bulk_norm() {
        float3 sub_type_g1_xyz = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiLine {
    public Scalar round_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiMotor {
    public Scalar round_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.round_bulk_norm();
}
public extension AntiPlane {
    public Scalar round_bulk_norm() {
        float3 sub_type_g0_xyz = this.group0.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_bulk_norm();
}
public extension Circle {
    public Scalar round_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_bulk_norm();
}
public extension CircleRotor {
    public Scalar round_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_bulk_norm();
}
public extension Dipole {
    public Scalar round_bulk_norm() {
        float3 sub_type_g0 = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_bulk_norm();
}
public extension DipoleInversion {
    public Scalar round_bulk_norm() {
        float3 sub_type_g0 = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_bulk_norm();
}
public extension MultiVector {
    public Scalar round_bulk_norm() {
        float3 sub_type_g1_xyz = this.group1.xyz;
        return Scalar(
            /* scalar */
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.scalar * this.scalar + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_bulk_norm();
}
public extension RoundPoint {
    public Scalar round_bulk_norm() {
        float3 sub_type_g0_xyz = this.group0.xyz;
        return Scalar(
            /* scalar */
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.round_bulk_norm();
}
public extension Scalar {
    public Scalar round_bulk_norm() {
        return this;
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_bulk_norm();
}
public extension VersorEven {
    public Scalar round_bulk_norm() {
        float3 sub_type_g1_xyz = this.group3.xyz;
        return Scalar(
            /* scalar */
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0] + sub_type_g1_xyz[1] * sub_type_g1_xyz[1] + sub_type_g1_xyz[2] * sub_type_g1_xyz[2] + this.e321 * this.e321
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, RoundBulkNormPrefixOrPostfix rhs)  {
    return lhs.round_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(RoundBulkNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_bulk_norm();
}
public extension VersorOdd {
    public Scalar round_bulk_norm() {
        return Scalar(
            /* scalar */
            this.scalar * this.scalar + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12
        );
    }
}
