implementing cga3d;
// Fancy postfix trick
public MultiVector operator /(AntiCircleRotor lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_norm_squared();
}
public extension AntiCircleRotor {
    public MultiVector flat_norm_squared() {
        float3 wedge_g0 = this.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], this.e45 * this.e45), 
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
public MultiVector operator /(AntiDipoleInversion lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_norm_squared();
}
public extension AntiDipoleInversion {
    public MultiVector flat_norm_squared() {
        float3 wedge_g1_xyz = this.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2], this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435), 
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
public MultiVector operator /(Circle lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_norm_squared();
}
public extension Circle {
    public MultiVector flat_norm_squared() {
        float3 wedge_g0_xyz = this.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2], this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435), 
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
public MultiVector operator /(CircleRotor lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_norm_squared();
}
public extension CircleRotor {
    public MultiVector flat_norm_squared() {
        float3 wedge_g0_xyz = this.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2], this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345), 
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
public MultiVector operator /(Dipole lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_norm_squared();
}
public extension Dipole {
    public MultiVector flat_norm_squared() {
        float3 wedge_g0 = this.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], this.e45 * this.e45), 
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
public MultiVector operator /(DipoleInversion lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_norm_squared();
}
public extension DipoleInversion {
    public MultiVector flat_norm_squared() {
        float4 wedge_g0 = (float4(float3(0.0), 1.0).wwwx * float4(this.group2.xyz, 0.0)) + float4(float3(0.0), this.e3215);
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3], this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125), 
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
public MultiVector operator /(DualNum lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_norm_squared();
}
public extension DualNum {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e5 * this.e5, this.e12345 * this.e12345), 
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
public MultiVector operator /(FlatPoint lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_norm_squared();
}
public extension FlatPoint {
    public MultiVector flat_norm_squared() {
        float3 wedge_g0 = this.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], this.e45 * this.e45), 
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
public MultiVector operator /(Flector lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_norm_squared();
}
public extension Flector {
    public MultiVector flat_norm_squared() {
        float4 wedge_g0 = (float4(float3(0.0), 1.0).wwwx * float4(this.group0.xyz, 0.0)) + float4(float3(0.0), this.e3215);
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3], this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125), 
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
public MultiVector operator /(Line lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_norm_squared();
}
public extension Line {
    public MultiVector flat_norm_squared() {
        float3 wedge_g0_xyz = this.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0_xyz[0] * wedge_g0_xyz[0] - wedge_g0_xyz[1] * wedge_g0_xyz[1] - wedge_g0_xyz[2] * wedge_g0_xyz[2], this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435), 
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
public MultiVector operator /(Motor lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_norm_squared();
}
public extension Motor {
    public MultiVector flat_norm_squared() {
        float3 wedge_g1_xyz = this.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2], this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e12345 * this.e12345), 
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
public MultiVector operator /(MultiVector lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_norm_squared();
}
public extension MultiVector {
    public MultiVector flat_norm_squared() {
        float4 wedge_g9 = float4(this.group8.xyz, 0.0) * float4(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e5 * this.e5 - wedge_g9[0] * wedge_g9[0] - wedge_g9[1] * wedge_g9[1] - wedge_g9[2] * wedge_g9[2] - this.e15 * this.e15 - this.e25 * this.e25 - this.e35 * this.e35 - this.e3215 * this.e3215, this.e12345 * this.e12345 + this.e45 * this.e45 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125), 
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
public MultiVector operator /(Plane lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_norm_squared();
}
public extension Plane {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e3215 * this.e3215, this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125) * float2(-1.0, 1.0), 
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
public MultiVector operator /(Sphere lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_norm_squared();
}
public extension Sphere {
    public MultiVector flat_norm_squared() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e3215 * this.e3215, this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125) * float2(-1.0, 1.0), 
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
public MultiVector operator /(VersorEven lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_norm_squared();
}
public extension VersorEven {
    public MultiVector flat_norm_squared() {
        float3 wedge_g1_xyz = this.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e5 * this.e5 - wedge_g1_xyz[0] * wedge_g1_xyz[0] - wedge_g1_xyz[1] * wedge_g1_xyz[1] - wedge_g1_xyz[2] * wedge_g1_xyz[2], this.e12345 * this.e12345 + this.e415 * this.e415 + this.e425 * this.e425 + this.e435 * this.e435), 
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
public MultiVector operator /(VersorOdd lhs, FlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_norm_squared();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_norm_squared();
}
public extension VersorOdd {
    public MultiVector flat_norm_squared() {
        float4 wedge_g0 = (float4(float3(0.0), 1.0).wwwx * float4(this.group2.xyz, 0.0)) + float4(float3(0.0), this.e3215);
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2] - wedge_g0[3] * wedge_g0[3], this.e45 * this.e45 + this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125), 
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
