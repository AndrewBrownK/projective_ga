implementing cga3d;
using traits;
// Fancy postfix trick
public Scalar operator /(AntiCircleRotor lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.center_norm_squared();
}
public extension AntiCircleRotor {
    public Scalar center_norm_squared() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.scalar, 2) - pow(float4(float3(0.0), this.e45)[3], 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(AntiDipoleInversion lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.center_norm_squared();
}
public extension AntiDipoleInversion {
    public Scalar center_norm_squared() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        Line sub_type_2 = Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e321, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2) - pow(sub_type_2.e415, 2) - pow(sub_type_2.e425, 2) - pow(sub_type_2.e435, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Circle lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.center_norm_squared();
}
public extension Circle {
    public Scalar center_norm_squared() {
        Line sub_type_2 = Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            pow(this.e321, 2) - pow(sub_type_2.e415, 2) - pow(sub_type_2.e425, 2) - pow(sub_type_2.e435, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(CircleRotor lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.center_norm_squared();
}
public extension CircleRotor {
    public Scalar center_norm_squared() {
        Motor sub_type_2 = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return Scalar(
            /* scalar */
            pow(this.e321, 2) - pow(sub_type_2.e415, 2) - pow(sub_type_2.e425, 2) - pow(sub_type_2.e435, 2) - pow(sub_type_2.e12345, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Dipole lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.center_norm_squared();
}
public extension Dipole {
    public Scalar center_norm_squared() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) - pow(float4(float3(0.0), this.e45)[3], 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(DipoleInversion lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.center_norm_squared();
}
public extension DipoleInversion {
    public Scalar center_norm_squared() {
        AntiLine sub_type = AntiLine(
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e15, e25, e35 */
            float3(0.0)
        );
        Flector sub_type_2 = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) - pow(sub_type_2.e45, 2) - pow(sub_type_2.e4235, 2) - pow(sub_type_2.e4315, 2) - pow(sub_type_2.e4125, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(MultiVector lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.center_norm_squared();
}
public extension MultiVector {
    public Scalar center_norm_squared() {
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
        return Scalar(
            /* scalar */
            2.0*(sub_type.e15 * sub_type.e41) + 2.0 * (sub_type.e25 * sub_type.e42) + 2.0 * (sub_type.e35 * sub_type.e43) + 2.0 * (sub_type.e3215 * sub_type.e1234) + 2.0 * (sub_type_2.e15 * sub_type_2.e41) + 2.0 * (sub_type_2.e25 * sub_type_2.e42) + 2.0 * (sub_type_2.e35 * sub_type_2.e43) + 2.0 * (sub_type_2.e3215 * sub_type_2.e1234) + pow(sub_type.scalar, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2) + pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.e321, 2) + pow(sub_type_2.scalar, 2) + pow(sub_type_2.e1, 2) + pow(sub_type_2.e2, 2) + pow(sub_type_2.e3, 2) + pow(sub_type_2.e23, 2) + pow(sub_type_2.e31, 2) + pow(sub_type_2.e12, 2) + pow(sub_type_2.e321, 2) - pow(sub_type.e12345, 2) - pow(sub_type.e45, 2) - pow(sub_type.e415, 2) - pow(sub_type.e425, 2) - pow(sub_type.e435, 2) - pow(sub_type.e4235, 2) - pow(sub_type.e4315, 2) - pow(sub_type.e4125, 2) - pow(sub_type_2.e12345, 2) - pow(sub_type_2.e45, 2) - pow(sub_type_2.e415, 2) - pow(sub_type_2.e425, 2) - pow(sub_type_2.e435, 2) - pow(sub_type_2.e4235, 2) - pow(sub_type_2.e4315, 2) - pow(sub_type_2.e4125, 2) - 2.0 * (sub_type.e4 * sub_type.e5) - 2.0 * (sub_type.e423 * sub_type.e235) - 2.0 * (sub_type.e431 * sub_type.e315) - 2.0 * (sub_type.e412 * sub_type.e125) - 2.0 * (sub_type_2.e4 * sub_type_2.e5) - 2.0 * (sub_type_2.e423 * sub_type_2.e235) - 2.0 * (sub_type_2.e431 * sub_type_2.e315) - 2.0 * (sub_type_2.e412 * sub_type_2.e125)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorEven lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.center_norm_squared();
}
public extension VersorEven {
    public Scalar center_norm_squared() {
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
        Motor sub_type_2 = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e321, 2) + pow(sub_type.e1, 2) + pow(sub_type.e2, 2) + pow(sub_type.e3, 2) - pow(sub_type_2.e415, 2) - pow(sub_type_2.e425, 2) - pow(sub_type_2.e435, 2) - pow(sub_type_2.e12345, 2)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(VersorOdd lhs, CenterNormSquaredPrefixOrPostfix rhs)  {
    return lhs.center_norm_squared();
    }
// Fancy prefix trick
public Scalar operator /(CenterNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.center_norm_squared();
}
public extension VersorOdd {
    public Scalar center_norm_squared() {
        AntiMotor sub_type = AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
        Flector sub_type_2 = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
        return Scalar(
            /* scalar */
            pow(sub_type.e23, 2) + pow(sub_type.e31, 2) + pow(sub_type.e12, 2) + pow(sub_type.scalar, 2) - pow(sub_type_2.e45, 2) - pow(sub_type_2.e4235, 2) - pow(sub_type_2.e4315, 2) - pow(sub_type_2.e4125, 2)
        );
    }
}
