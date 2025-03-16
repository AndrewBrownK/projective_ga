implementing cga3d;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g0 = this.group2.xyz;
        return Scalar(
            /* scalar */
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g1_xyz = this.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDualNum lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiDualNum {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e3215 * this.e3215 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlatPoint lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiFlatPoint {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g0_xyz = this.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiFlector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiFlector {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g1_xyz = this.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiLine lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiLine {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiMotor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiMotor {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35 - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiPlane lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension AntiPlane {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e5 * this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Circle {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e235 * this.e235 - this.e315 * this.e315 - this.e125 * this.e125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension CircleRotor {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g0_xyz = this.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Dipole {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension DipoleInversion {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35 - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DualNum lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension DualNum {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e5 * this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(FlatPoint lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension FlatPoint {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g0 = this.group0.xyz;
        return Scalar(
            /* scalar */
            -wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Flector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Flector {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35 - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Line lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Line {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e235 * this.e235 - this.e315 * this.e315 - this.e125 * this.e125
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Motor lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Motor {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g1_xyz = this.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension MultiVector {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35 - this.e235 * this.e235 - this.e315 * this.e315 - this.e125 * this.e125 - this.e3215 * this.e3215
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Plane {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e3215 * this.e3215 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(RoundPoint lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension RoundPoint {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e5 * this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension Sphere {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            this.e3215 * this.e3215 * -1.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension VersorEven {
    public Scalar flat_bulk_norm_squared() {
        float3 wedge_g1_xyz = this.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2]
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, FlatBulkNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_bulk_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(FlatBulkNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_bulk_norm_squared();
}
public extension VersorOdd {
    public Scalar flat_bulk_norm_squared() {
        return Scalar(
            /* scalar */
            -this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35 - this.e3215 * this.e3215
        );
    }
}
