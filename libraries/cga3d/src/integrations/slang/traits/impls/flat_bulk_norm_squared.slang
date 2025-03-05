implementing cga3d;
using traits;
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
        FlatPoint sub_type = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(other.e4) * sub_type.group0.xyz) - (float3(sub_type.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2)
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
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((sub_type.group1.yzx * other.group0.zxy) - (sub_type.group1.zxy * other.group0.yzx)), sub_type.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * sub_type.group1.xyz) - (float3(sub_type.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        return Scalar(
            /* scalar */
            2.0*(wedge.e41 * wedge.e15) + 2.0 * (wedge.e42 * wedge.e25) + 2.0 * (wedge.e43 * wedge.e35) + 2.0 * (wedge.e1234 * wedge.e3215) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) - pow(wedge.e45, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
            pow(this.e3215, 2) * -1.0
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
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((sub_type.group1.yzx * other.group0.zxy) - (sub_type.group1.zxy * other.group0.yzx)), sub_type.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * sub_type.group1.xyz) - (float3(sub_type.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        return Scalar(
            /* scalar */
            2.0*(wedge.e41 * wedge.e15) + 2.0 * (wedge.e42 * wedge.e25) + 2.0 * (wedge.e43 * wedge.e35) + 2.0 * (wedge.e1234 * wedge.e3215) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) - pow(wedge.e45, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
        FlatPoint sub_type = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(other.e4) * sub_type.group0.xyz) - (float3(sub_type.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2)
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
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)), 0.0)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e12345, 2)
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
            pow(this.e5, 2)
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
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5)), 
            /* e1234 */
            sub_type.e321 * other.e4 * -1.0
        );
        return Scalar(
            /* scalar */
            2.0*(wedge.e3215 * wedge.e1234) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
        FlatPoint sub_type = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(other.e4) * sub_type.group0.xyz) - (float3(sub_type.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2)
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
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)), 0.0)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e12345, 2)
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
            pow(this.e5, 2)
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
        FlatPoint sub_type = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(other.e4) * sub_type.group0.xyz) - (float3(sub_type.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2)
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
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)), 0.0)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e12345, 2)
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
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5)), 
            /* e1234 */
            sub_type.e321 * other.e4 * -1.0
        );
        return Scalar(
            /* scalar */
            2.0*(wedge.e3215 * wedge.e1234) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((sub_type.group1.yzx * other.group0.zxy) - (sub_type.group1.zxy * other.group0.yzx)), sub_type.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * sub_type.group1.xyz) - (float3(sub_type.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        return Scalar(
            /* scalar */
            2.0*(wedge.e41 * wedge.e15) + 2.0 * (wedge.e42 * wedge.e25) + 2.0 * (wedge.e43 * wedge.e35) + 2.0 * (wedge.e1234 * wedge.e3215) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) - pow(wedge.e45, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
        MultiVector sub_type = MultiVector(
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
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (sub_type.e4235 * other.e1) + (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4) + (sub_type.e1234 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * sub_type.group1) - (float4(sub_type.e5) * other.group0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * sub_type.group3.xyz), 0.0) + float4((float3(other.e5) * sub_type.group4.xyz), 0.0) - float4((float3(sub_type.e45) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(other.e5) * sub_type.group5.xyz) + (sub_type.group3.zxy * other.group0.yzx) - (sub_type.group3.yzx * other.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(sub_type.e425 * other.e3) - (sub_type.e235 * other.e4), -(sub_type.e435 * other.e1) - (sub_type.e315 * other.e4), -(sub_type.e415 * other.e2) - (sub_type.e125 * other.e4), (sub_type.e321 * other.e5) + (sub_type.e125 * other.e3)) + (other.group0.yzxy * float4(sub_type.group6.zxy, sub_type.e315)) + float4((float3(other.e5) * sub_type.group7.xyz), sub_type.e235 * other.e1), 
            /* e1234 */
            0.0
        );
        return Scalar(
            /* scalar */
            2.0*(wedge.e15 * wedge.e41) + 2.0 * (wedge.e25 * wedge.e42) + 2.0 * (wedge.e35 * wedge.e43) + 2.0 * (wedge.e3215 * wedge.e1234) + pow(wedge.scalar, 2) + pow(wedge.e1, 2) + pow(wedge.e2, 2) + pow(wedge.e3, 2) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) + pow(wedge.e321, 2) - pow(wedge.e12345, 2) - pow(wedge.e45, 2) - pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2) - 2.0 * (wedge.e4 * wedge.e5) - 2.0 * (wedge.e423 * wedge.e235) - 2.0 * (wedge.e431 * wedge.e315) - 2.0 * (wedge.e412 * wedge.e125)
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
            pow(this.e3215, 2) * -1.0
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
            pow(this.e5, 2)
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
            pow(this.e3215, 2) * -1.0
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
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((sub_type.group1.yzx * other.group0.zxy) - (sub_type.group1.zxy * other.group0.yzx)), sub_type.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * sub_type.group1.xyz) - (float3(sub_type.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        return Scalar(
            /* scalar */
            2.0*(wedge.e41 * wedge.e15) + 2.0 * (wedge.e42 * wedge.e25) + 2.0 * (wedge.e43 * wedge.e35) + 2.0 * (wedge.e1234 * wedge.e3215) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) - pow(wedge.e45, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2)
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
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)), 0.0)
        );
        return Scalar(
            /* scalar */
            -pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e12345, 2)
        );
    }
}
