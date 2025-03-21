implementing cga3d;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiCircleRotor {
    public Scalar flat_bulk_norm() {
        float3 wedge_g0 = this.group2.xyz;
        return Scalar(
            /* scalar */
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiDipoleInversion {
    public Scalar flat_bulk_norm() {
        float3 wedge_g1_xyz = this.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiDualNum {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e3215 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiFlatPoint {
    public Scalar flat_bulk_norm() {
        float3 wedge_g0_xyz = this.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiFlector {
    public Scalar flat_bulk_norm() {
        float3 wedge_g1_xyz = this.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiLine {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiMotor {
    public Scalar flat_bulk_norm() {
        float3 sub_type_g0_xyz = this.group1.xyz;
        return Scalar(
            /* scalar */
            -sub_type_g0_xyz[0] * sub_type_g0_xyz[0] - sub_type_g0_xyz[1] * sub_type_g0_xyz[1] - sub_type_g0_xyz[2] * sub_type_g0_xyz[2] - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.flat_bulk_norm();
}
public extension AntiPlane {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_bulk_norm();
}
public extension Circle {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e235 * this.e235 + this.e315 * this.e315 + this.e125 * this.e125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_bulk_norm();
}
public extension CircleRotor {
    public Scalar flat_bulk_norm() {
        float3 wedge_g0_xyz = this.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_bulk_norm();
}
public extension Dipole {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_bulk_norm();
}
public extension DipoleInversion {
    public Scalar flat_bulk_norm() {
        float3 sub_type_g0_xyz = this.group2.xyz;
        return Scalar(
            /* scalar */
            -sub_type_g0_xyz[0] * sub_type_g0_xyz[0] - sub_type_g0_xyz[1] * sub_type_g0_xyz[1] - sub_type_g0_xyz[2] * sub_type_g0_xyz[2] - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DualNum lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_bulk_norm();
}
public extension DualNum {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(FlatPoint lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_bulk_norm();
}
public extension FlatPoint {
    public Scalar flat_bulk_norm() {
        float3 wedge_g0 = this.group0.xyz;
        return Scalar(
            /* scalar */
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk_norm();
}
public extension Flector {
    public Scalar flat_bulk_norm() {
        float3 sub_type_g0_xyz = this.group0.xyz;
        return Scalar(
            /* scalar */
            -sub_type_g0_xyz[0] * sub_type_g0_xyz[0] - sub_type_g0_xyz[1] * sub_type_g0_xyz[1] - sub_type_g0_xyz[2] * sub_type_g0_xyz[2] - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk_norm();
}
public extension Line {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e235 * this.e235 + this.e315 * this.e315 + this.e125 * this.e125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk_norm();
}
public extension Motor {
    public Scalar flat_bulk_norm() {
        float3 wedge_g1_xyz = this.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk_norm();
}
public extension MultiVector {
    public Scalar flat_bulk_norm() {
        float3 sub_type_g3_xyz = this.group3.xyz;
        return Scalar(
            /* scalar */
            this.e5 * this.e5 + this.e235 * this.e235 + this.e315 * this.e315 + this.e125 * this.e125 - sub_type_g3_xyz[0] * sub_type_g3_xyz[0] - sub_type_g3_xyz[1] * sub_type_g3_xyz[1] - sub_type_g3_xyz[2] * sub_type_g3_xyz[2] - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_bulk_norm();
}
public extension Plane {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e3215 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.flat_bulk_norm();
}
public extension RoundPoint {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_bulk_norm();
}
public extension Sphere {
    public Scalar flat_bulk_norm() {
        return Scalar(
            /* scalar */
            this.e3215 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_bulk_norm();
}
public extension VersorEven {
    public Scalar flat_bulk_norm() {
        float3 wedge_g1_xyz = this.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, FlatBulkNormPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_bulk_norm();
}
public extension VersorOdd {
    public Scalar flat_bulk_norm() {
        float3 sub_type_g0_xyz = this.group2.xyz;
        return Scalar(
            /* scalar */
            -sub_type_g0_xyz[0] * sub_type_g0_xyz[0] - sub_type_g0_xyz[1] * sub_type_g0_xyz[1] - sub_type_g0_xyz[2] * sub_type_g0_xyz[2] - this.e3215 * this.e3215
        );
    }
}
