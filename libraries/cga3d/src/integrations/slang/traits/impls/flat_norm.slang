implementing cga3d;
using traits;
// Fancy postfix trick
public MultiVector operator /(AntiCircleRotor lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.flat_norm();
}
public extension AntiCircleRotor {
    public MultiVector flat_norm() {
        float3 wedge_g0 = this.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], this.e45), 
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
public MultiVector operator /(AntiDipoleInversion lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.flat_norm();
}
public extension AntiDipoleInversion {
    public MultiVector flat_norm() {
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
public MultiVector operator /(Circle lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Circle rhs) {
    return rhs.flat_norm();
}
public extension Circle {
    public MultiVector flat_norm() {
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
public MultiVector operator /(CircleRotor lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.flat_norm();
}
public extension CircleRotor {
    public MultiVector flat_norm() {
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
public MultiVector operator /(Dipole lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.flat_norm();
}
public extension Dipole {
    public MultiVector flat_norm() {
        float3 wedge_g0 = this.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], this.e45), 
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
public MultiVector operator /(DipoleInversion lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.flat_norm();
}
public extension DipoleInversion {
    public MultiVector flat_norm() {
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
public MultiVector operator /(DualNum lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_norm();
}
public extension DualNum {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
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
public MultiVector operator /(FlatPoint lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.flat_norm();
}
public extension FlatPoint {
    public MultiVector flat_norm() {
        float3 wedge_g0 = this.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(-wedge_g0[0] * wedge_g0[0] - wedge_g0[1] * wedge_g0[1] - wedge_g0[2] * wedge_g0[2], this.e45), 
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
public MultiVector operator /(Flector lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_norm();
}
public extension Flector {
    public MultiVector flat_norm() {
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
public MultiVector operator /(Line lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_norm();
}
public extension Line {
    public MultiVector flat_norm() {
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
public MultiVector operator /(Motor lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_norm();
}
public extension Motor {
    public MultiVector flat_norm() {
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
public MultiVector operator /(MultiVector lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_norm();
}
public extension MultiVector {
    public MultiVector flat_norm() {
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
public MultiVector operator /(Plane lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_norm();
}
public extension Plane {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e3215, this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125) * float2(-1.0, 1.0), 
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
public MultiVector operator /(Sphere lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.flat_norm();
}
public extension Sphere {
    public MultiVector flat_norm() {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e3215, this.e4235 * this.e4235 + this.e4315 * this.e4315 + this.e4125 * this.e4125) * float2(-1.0, 1.0), 
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
public MultiVector operator /(VersorEven lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.flat_norm();
}
public extension VersorEven {
    public MultiVector flat_norm() {
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
public MultiVector operator /(VersorOdd lhs, FlatNormPrefixOrPostfix rhs)  {
    return lhs.flat_norm();
    }
// Fancy prefix trick
public MultiVector operator /(FlatNormPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.flat_norm();
}
public extension VersorOdd {
    public MultiVector flat_norm() {
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
