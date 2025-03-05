implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_center_norm_squared() {
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
        return (pow(sub_type.e23, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e31, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e12, 2) * pow(wedge.e415, 2)) + (pow(sub_type.scalar, 2) * pow(wedge.e415, 2));
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_center_norm_squared() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        AntiDipoleInversion sub_type_3 = AntiDipoleInversion(
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
            float4(float3(0.0), sub_type_3.e4 * other.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_3.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return (pow(sub_type.e321, 2) * pow(wedge.e45, 2)) + (pow(sub_type.e1, 2) * pow(wedge.e45, 2)) + (pow(sub_type.e2, 2) * pow(wedge.e45, 2)) + (pow(sub_type.e3, 2) * pow(wedge.e45, 2));
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension Circle {
    public float unitized_center_norm_squared() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        return pow((float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0))[0], 2) * pow(float4(float3(0.0), this.e321)[3], 2);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension CircleRotor {
    public float unitized_center_norm_squared() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        return pow((float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0))[0], 2) * pow(float4(float3(0.0), this.e321)[3], 2);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension Dipole {
    public float unitized_center_norm_squared() {
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
        return (pow(sub_type.e23, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e31, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e12, 2) * pow(wedge.e415, 2));
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension DipoleInversion {
    public float unitized_center_norm_squared() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        DipoleInversion sub_type_3 = DipoleInversion(
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
            float4(sub_type_3.group0.xyz, sub_type_3.e1234), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return (pow(sub_type.e23, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e31, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e12, 2) * pow(wedge.e415, 2));
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension MultiVector {
    public float unitized_center_norm_squared() {
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
        MultiVector sub_type_3 = MultiVector(
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
            float2(0.0, (other.e5 * sub_type_3.e1234) + (other.e12345 * sub_type_3.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.e5) * sub_type_3.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_3.group4.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_3.group7.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1234 */
            0.0
        );
        return 4.0*(sub_type.e15 * sub_type.e41 * wedge.e423 * wedge.e235) + 4.0 * (sub_type.e25 * sub_type.e42 * wedge.e423 * wedge.e235) + 4.0 * (sub_type.e35 * sub_type.e43 * wedge.e423 * wedge.e235) + 4.0 * (sub_type.e3215 * sub_type.e1234 * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.scalar, 2) * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.e1, 2) * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.e2, 2) * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.e3, 2) * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.e23, 2) * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.e31, 2) * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.e12, 2) * wedge.e423 * wedge.e235) + 2.0 * (pow(sub_type.e321, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e12345, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e45, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e415, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e425, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e435, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e4235, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e4315, 2) * wedge.e423 * wedge.e235) - 2.0 * (pow(sub_type.e4125, 2) * wedge.e423 * wedge.e235) - 4.0 * (sub_type.e4 * sub_type.e5 * wedge.e423 * wedge.e235) - 4.0 * (sub_type.e423 * sub_type.e235 * wedge.e423 * wedge.e235) - 4.0 * (sub_type.e431 * sub_type.e315 * wedge.e423 * wedge.e235) - 4.0 * (sub_type.e412 * sub_type.e125 * wedge.e423 * wedge.e235);
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension VersorEven {
    public float unitized_center_norm_squared() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        AntiDipoleInversion sub_type_3 = AntiDipoleInversion(
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
            float4(float3(0.0), sub_type_3.e4 * other.e5), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(sub_type_3.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return (pow(sub_type.e321, 2) * pow(wedge.e45, 2)) + (pow(sub_type.e1, 2) * pow(wedge.e45, 2)) + (pow(sub_type.e2, 2) * pow(wedge.e45, 2)) + (pow(sub_type.e3, 2) * pow(wedge.e45, 2));
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedCenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_center_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedCenterNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_center_norm_squared();
}
public extension VersorOdd {
    public float unitized_center_norm_squared() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        DipoleInversion sub_type_3 = DipoleInversion(
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
            float4(sub_type_3.group0.xyz, sub_type_3.e1234), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return (pow(sub_type.e23, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e31, 2) * pow(wedge.e415, 2)) + (pow(sub_type.e12, 2) * pow(wedge.e415, 2)) + (pow(sub_type.scalar, 2) * pow(wedge.e415, 2));
    }
}
