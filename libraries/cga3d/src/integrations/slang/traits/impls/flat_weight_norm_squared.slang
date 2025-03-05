implementing cga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiCircleRotor lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension AntiCircleRotor {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            pow(float4(float3(0.0), this.e45)[3], 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiDipoleInversion lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension AntiDipoleInversion {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(AntiScalar lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension AntiScalar {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            pow(this.e12345, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Circle lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Circle {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(CircleRotor lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension CircleRotor {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(Dipole lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Dipole {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            pow(float4(float3(0.0), this.e45)[3], 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DipoleInversion lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension DipoleInversion {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(DualNum lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension DualNum {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            pow(this.e12345, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(FlatPoint lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension FlatPoint {
    public AntiScalar flat_weight_norm_squared() {
        return AntiScalar(
            /* e12345 */
            pow(float4(float3(0.0), this.e45)[3], 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Flector {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(Line lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Line {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(Motor lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Motor {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(MultiVector lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension MultiVector {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(Plane lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Plane {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(Sphere lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Sphere {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(VersorEven lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension VersorEven {
    public AntiScalar flat_weight_norm_squared() {
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
public AntiScalar operator /(VersorOdd lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension VersorOdd {
    public AntiScalar flat_weight_norm_squared() {
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
