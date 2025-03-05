implementing cga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiCircleRotor lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_weight_norm();
}
public extension AntiCircleRotor {
    public AntiScalar flat_weight_norm() {
        return AntiScalar(
            /* e12345 */
            float4(float3(0.0), this.e45)[3]
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiDipoleInversion lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_weight_norm();
}
public extension AntiDipoleInversion {
    public AntiScalar flat_weight_norm() {
        Line sub_type = Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e415, 2) + pow(sub_type.e425, 2) + pow(sub_type.e435, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.flat_weight_norm();
}
public extension AntiScalar {
    public AntiScalar flat_weight_norm() {
        return this;
    }
}
// Fancy postfix trick
public AntiScalar operator /(Circle lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_weight_norm();
}
public extension Circle {
    public AntiScalar flat_weight_norm() {
        Line sub_type = Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e415, 2) + pow(sub_type.e425, 2) + pow(sub_type.e435, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(CircleRotor lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_weight_norm();
}
public extension CircleRotor {
    public AntiScalar flat_weight_norm() {
        Motor sub_type = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e415, 2) + pow(sub_type.e425, 2) + pow(sub_type.e435, 2) + pow(sub_type.e12345, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Dipole lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_weight_norm();
}
public extension Dipole {
    public AntiScalar flat_weight_norm() {
        return AntiScalar(
            /* e12345 */
            float4(float3(0.0), this.e45)[3]
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DipoleInversion lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_weight_norm();
}
public extension DipoleInversion {
    public AntiScalar flat_weight_norm() {
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e45, 2) + pow(sub_type.e4235, 2) + pow(sub_type.e4315, 2) + pow(sub_type.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_weight_norm();
}
public extension DualNum {
    public AntiScalar flat_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e12345
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(FlatPoint lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_weight_norm();
}
public extension FlatPoint {
    public AntiScalar flat_weight_norm() {
        return AntiScalar(
            /* e12345 */
            float4(float3(0.0), this.e45)[3]
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_weight_norm();
}
public extension Flector {
    public AntiScalar flat_weight_norm() {
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e45, 2) + pow(sub_type.e4235, 2) + pow(sub_type.e4315, 2) + pow(sub_type.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_weight_norm();
}
public extension Line {
    public AntiScalar flat_weight_norm() {
        Line sub_type = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e415, 2) + pow(sub_type.e425, 2) + pow(sub_type.e435, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_weight_norm();
}
public extension Motor {
    public AntiScalar flat_weight_norm() {
        Motor sub_type = Motor(
            /* e415, e425, e435, e12345 */
            this.group0, 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e415, 2) + pow(sub_type.e425, 2) + pow(sub_type.e435, 2) + pow(sub_type.e12345, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_weight_norm();
}
public extension MultiVector {
    public AntiScalar flat_weight_norm() {
        MultiVector sub_type = MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, 0.0), 
            /* e1234 */
            0.0
        );
        return AntiScalar(
            /* e12345 */
            2.0*(sub_type.e4 * sub_type.e5) + 2.0 * (sub_type.e423 * sub_type.e235) + 2.0 * (sub_type.e431 * sub_type.e315) + 2.0 * (sub_type.e412 * sub_type.e125) + pow(sub_type.e12345, 2) + pow(sub_type.e45, 2) + pow(sub_type.e415, 2) + pow(sub_type.e425, 2) + pow(sub_type.e435, 2) + pow(sub_type.e4235, 2) + pow(sub_type.e4315, 2) + pow(sub_type.e4125, 2) - pow(sub_type.scalar, 2) - pow(sub_type.e1, 2) - pow(sub_type.e2, 2) - pow(sub_type.e3, 2) - pow(sub_type.e23, 2) - pow(sub_type.e31, 2) - pow(sub_type.e12, 2) - pow(sub_type.e321, 2) - 2.0 * (sub_type.e15 * sub_type.e41) - 2.0 * (sub_type.e25 * sub_type.e42) - 2.0 * (sub_type.e35 * sub_type.e43) - 2.0 * (sub_type.e3215 * sub_type.e1234)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_weight_norm();
}
public extension Plane {
    public AntiScalar flat_weight_norm() {
        Plane sub_type = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e4235, 2) + pow(sub_type.e4315, 2) + pow(sub_type.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Sphere lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_weight_norm();
}
public extension Sphere {
    public AntiScalar flat_weight_norm() {
        Plane sub_type = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e4235, 2) + pow(sub_type.e4315, 2) + pow(sub_type.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorEven lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_weight_norm();
}
public extension VersorEven {
    public AntiScalar flat_weight_norm() {
        Motor sub_type = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e415, 2) + pow(sub_type.e425, 2) + pow(sub_type.e435, 2) + pow(sub_type.e12345, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorOdd lhs, FlatWeightNormPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_weight_norm();
}
public extension VersorOdd {
    public AntiScalar flat_weight_norm() {
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(sub_type.e45, 2) + pow(sub_type.e4235, 2) + pow(sub_type.e4315, 2) + pow(sub_type.e4125, 2)
        );
    }
}
