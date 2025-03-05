implementing cga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiCircleRotor lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_weight_norm();
}
public extension AntiCircleRotor {
    public AntiScalar round_weight_norm() {
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiDipoleInversion lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_weight_norm();
}
public extension AntiDipoleInversion {
    public AntiScalar round_weight_norm() {
        AntiDipoleInversion sub_type = AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e4), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), sub_type.e4 * other.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e45, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Circle lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_weight_norm();
}
public extension Circle {
    public AntiScalar round_weight_norm() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(CircleRotor lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_weight_norm();
}
public extension CircleRotor {
    public AntiScalar round_weight_norm() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Dipole lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_weight_norm();
}
public extension Dipole {
    public AntiScalar round_weight_norm() {
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(DipoleInversion lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_weight_norm();
}
public extension DipoleInversion {
    public AntiScalar round_weight_norm() {
        DipoleInversion sub_type = DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.group0.xyz, sub_type.e1234), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e12345, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_weight_norm();
}
public extension MultiVector {
    public AntiScalar round_weight_norm() {
        MultiVector sub_type = MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            this.e1234
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e5 * sub_type.e1234) + (other.e12345 * sub_type.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.e5) * sub_type.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type.group7.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1234 */
            0.0
        );
        return AntiScalar(
            /* e12345 */
            2.0*(wedge.e4 * wedge.e5) + 2.0 * (wedge.e423 * wedge.e235) + 2.0 * (wedge.e431 * wedge.e315) + 2.0 * (wedge.e412 * wedge.e125) + pow(wedge.e12345, 2) + pow(wedge.e45, 2) + pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2) - pow(wedge.scalar, 2) - pow(wedge.e1, 2) - pow(wedge.e2, 2) - pow(wedge.e3, 2) - pow(wedge.e23, 2) - pow(wedge.e31, 2) - pow(wedge.e12, 2) - pow(wedge.e321, 2) - 2.0 * (wedge.e15 * wedge.e41) - 2.0 * (wedge.e25 * wedge.e42) - 2.0 * (wedge.e35 * wedge.e43) - 2.0 * (wedge.e3215 * wedge.e1234)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(RoundPoint lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_weight_norm();
}
public extension RoundPoint {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e4
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Sphere lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.round_weight_norm();
}
public extension Sphere {
    public AntiScalar round_weight_norm() {
        return AntiScalar(
            /* e12345 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorEven lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_weight_norm();
}
public extension VersorEven {
    public AntiScalar round_weight_norm() {
        AntiDipoleInversion sub_type = AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e4), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Flector wedge = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), sub_type.e4 * other.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e45, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(VersorOdd lhs, RoundWeightNormPrefixOrPostfix rhs)  {
    return lhs.round_weight_norm();
    }
// Fancy prefix trick
public AntiScalar operator /(RoundWeightNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_weight_norm();
}
public extension VersorOdd {
    public AntiScalar round_weight_norm() {
        DipoleInversion sub_type = DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.group0.xyz, sub_type.e1234), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e12345, 2)
        );
    }
}
