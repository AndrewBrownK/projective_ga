implementing cga3d;
using traits;
// Fancy postfix trick
public MultiVector operator /(AntiCircleRotor lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.round_norm_squared();
}
public extension AntiCircleRotor {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar, this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43), 
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
public MultiVector operator /(AntiDipoleInversion lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.round_norm_squared();
}
public extension AntiDipoleInversion {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3, this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412 + this.e4 * this.e4), 
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
public MultiVector operator /(Circle lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.round_norm_squared();
}
public extension Circle {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e321 * this.e321, this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412), 
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
public MultiVector operator /(CircleRotor lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.round_norm_squared();
}
public extension CircleRotor {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e321 * this.e321, this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412), 
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
public MultiVector operator /(Dipole lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.round_norm_squared();
}
public extension Dipole {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12, this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43), 
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
public MultiVector operator /(DipoleInversion lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.round_norm_squared();
}
public extension DipoleInversion {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12, this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234), 
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
public MultiVector operator /(MultiVector lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.round_norm_squared();
}
public extension MultiVector {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * this.scalar + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321, this.e4 * this.e4 + this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412 + this.e1234 * this.e1234), 
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
public MultiVector operator /(RoundPoint lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.round_norm_squared();
}
public extension RoundPoint {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3, this.e4 * this.e4), 
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
public MultiVector operator /(VersorEven lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.round_norm_squared();
}
public extension VersorEven {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e321 * this.e321 + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3, this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412 + this.e4 * this.e4), 
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
public MultiVector operator /(VersorOdd lhs, RoundNormSquaredPrefixOrPostfix rhs)  {
    return lhs.round_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(RoundNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.round_norm_squared();
}
public extension VersorOdd {
    public MultiVector round_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * this.scalar + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12, this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234), 
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
