implementing cga3d;
using traits;
// Fancy postfix trick
public MultiVector operator /(AntiCircleRotor lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_norm();
}
public extension AntiCircleRotor {
    public MultiVector round_norm() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.scalar, 2), pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(AntiDipoleInversion lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_norm();
}
public extension AntiDipoleInversion {
    public MultiVector round_norm() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        AntiDipoleInversion sub_type_2 = AntiDipoleInversion(
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
            float4(float3(0.0), sub_type_2.e4 * other.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_2.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(sub_type.e321, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2), pow(wedge.e45, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(Circle lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_norm();
}
public extension Circle {
    public MultiVector round_norm() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e321, pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(CircleRotor lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_norm();
}
public extension CircleRotor {
    public MultiVector round_norm() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e321, pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(Dipole lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_norm();
}
public extension Dipole {
    public MultiVector round_norm() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2), pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(DipoleInversion lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_norm();
}
public extension DipoleInversion {
    public MultiVector round_norm() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        DipoleInversion sub_type_2 = DipoleInversion(
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
            float4(sub_type_2.group0.xyz, sub_type_2.e1234), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2), pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e12345, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(MultiVector lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_norm();
}
public extension MultiVector {
    public MultiVector round_norm() {
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
        MultiVector sub_type_2 = MultiVector(
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
            float2(0.0, (other.e5 * sub_type_2.e1234) + (other.e12345 * sub_type_2.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.e5) * sub_type_2.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_2.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_2.group7.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1234 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(sub_type.e15 * sub_type.e41) + 2.0 * (sub_type.e25 * sub_type.e42) + 2.0 * (sub_type.e35 * sub_type.e43) + 2.0 * (sub_type.e3215 * sub_type.e1234) + pow(sub_type.scalar, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2) + pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.e321, 2) - pow(sub_type.e12345, 2) - pow(sub_type.e45, 2) - pow(sub_type.e415, 2) - pow(sub_type.e425, 2) - pow(sub_type.e435, 2) - pow(sub_type.e4235, 2) - pow(sub_type.e4315, 2) - pow(sub_type.e4125, 2) - 2.0 * (sub_type.e4 * sub_type.e5) - 2.0 * (sub_type.e423 * sub_type.e235) - 2.0 * (sub_type.e431 * sub_type.e315) - 2.0 * (sub_type.e412 * sub_type.e125), 2.0*(wedge.e4 * wedge.e5) + 2.0 * (wedge.e423 * wedge.e235) + 2.0 * (wedge.e431 * wedge.e315) + 2.0 * (wedge.e412 * wedge.e125) + pow(wedge.e12345, 2) + pow(wedge.e45, 2) + pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2) - pow(wedge.scalar, 2) - pow(wedge.e1, 2) - pow(wedge.e2, 2) - pow(wedge.e3, 2) - pow(wedge.e23, 2) - pow(wedge.e31, 2) - pow(wedge.e12, 2) - pow(wedge.e321, 2) - 2.0 * (wedge.e15 * wedge.e41) - 2.0 * (wedge.e25 * wedge.e42) - 2.0 * (wedge.e35 * wedge.e43) - 2.0 * (wedge.e3215 * wedge.e1234)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(RoundPoint lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_norm();
}
public extension RoundPoint {
    public MultiVector round_norm() {
        AntiPlane sub_type = AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2), this.e4), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(VersorEven lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_norm();
}
public extension VersorEven {
    public MultiVector round_norm() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        AntiDipoleInversion sub_type_2 = AntiDipoleInversion(
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
            float4(float3(0.0), sub_type_2.e4 * other.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_2.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(sub_type.e321, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2), pow(wedge.e45, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
public MultiVector operator /(VersorOdd lhs, RoundNormPrefixOrPostfix rhs)  {
    return lhs.round_norm();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_norm();
}
public extension VersorOdd {
    public MultiVector round_norm() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        DipoleInversion sub_type_2 = DipoleInversion(
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
            float4(sub_type_2.group0.xyz, sub_type_2.e1234), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.scalar, 2), pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e12345, 2)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
