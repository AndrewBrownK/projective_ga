implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_round_norm();
}
public extension AntiCircleRotor {
    public float unitized_round_norm() {
        return pow(this.e41, 2) * pow(this.e23, 2);
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_round_norm();
}
public extension AntiDipoleInversion {
    public float unitized_round_norm() {
        return pow(this.e321, 2) * pow(this.e4, 2);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_round_norm();
}
public extension Circle {
    public float unitized_round_norm() {
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return (pow(wedge.e4235, 2) * sub_type.e321) + (pow(wedge.e4315, 2) * sub_type.e321) + (pow(wedge.e4125, 2) * sub_type.e321);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_round_norm();
}
public extension CircleRotor {
    public float unitized_round_norm() {
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return (pow(wedge.e4235, 2) * sub_type.e321) + (pow(wedge.e4315, 2) * sub_type.e321) + (pow(wedge.e4125, 2) * sub_type.e321);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_round_norm();
}
public extension Dipole {
    public float unitized_round_norm() {
        return pow(this.e41, 2) * pow(this.e23, 2);
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_round_norm();
}
public extension DipoleInversion {
    public float unitized_round_norm() {
        return pow(this.e41, 2) * pow(this.e23, 2);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_round_norm();
}
public extension MultiVector {
    public float unitized_round_norm() {
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
        return (sub_type.e15 * sub_type.e41 * wedge.e4 * wedge.e5) * 4.0;
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_round_norm();
}
public extension RoundPoint {
    public float unitized_round_norm() {
        AntiPlane sub_type = AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
        FlatPoint wedge = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e4)
        );
        return (pow(sub_type.e1, 2) / (wedge.e45)) + (pow(sub_type.e2, 2) / (wedge.e45)) + (pow(sub_type.e3, 2) / (wedge.e45));
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_round_norm();
}
public extension VersorEven {
    public float unitized_round_norm() {
        return pow(this.e321, 2) * pow(this.e4, 2);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRoundNormPrefixOrPostfix rhs)  {
    return lhs.unitized_round_norm();
    }
// Fancy prefix trick
public float operator /(UnitizedRoundNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_round_norm();
}
public extension VersorOdd {
    public float unitized_round_norm() {
        return pow(this.e41, 2) * pow(this.e23, 2);
    }
}
