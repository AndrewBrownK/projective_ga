implementing cga3d;
using traits;
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
        FlatPoint sub_type = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(other.e4) * sub_type.group0.xyz) - (float3(sub_type.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2), pow(this.e45, 2)), 
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
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((sub_type.group1.yzx * other.group0.zxy) - (sub_type.group1.zxy * other.group0.yzx)), sub_type.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * sub_type.group1.xyz) - (float3(sub_type.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        Line sub_type_2 = Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(wedge.e41 * wedge.e15) + 2.0 * (wedge.e42 * wedge.e25) + 2.0 * (wedge.e43 * wedge.e35) + 2.0 * (wedge.e1234 * wedge.e3215) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) - pow(wedge.e45, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2), pow(sub_type_2.e415, 2) + pow(sub_type_2.e425, 2) + pow(sub_type_2.e435, 2)), 
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
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5)), 
            /* e1234 */
            sub_type.e321 * other.e4 * -1.0
        );
        Line sub_type_2 = Line(
            /* e415, e425, e435 */
            this.group1.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(wedge.e3215 * wedge.e1234) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2), pow(sub_type_2.e415, 2) + pow(sub_type_2.e425, 2) + pow(sub_type_2.e435, 2)), 
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
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        Motor sub_type_2 = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2), pow(sub_type_2.e415, 2) + pow(sub_type_2.e425, 2) + pow(sub_type_2.e435, 2) + pow(sub_type_2.e12345, 2)), 
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
        FlatPoint sub_type = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(other.e4) * sub_type.group0.xyz) - (float3(sub_type.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2), pow(this.e45, 2)), 
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
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)), 0.0)
        );
        Flector sub_type_2 = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e12345, 2), pow(sub_type_2.e45, 2) + pow(sub_type_2.e4235, 2) + pow(sub_type_2.e4315, 2) + pow(sub_type_2.e4125, 2)), 
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
            float2(pow(this.e5, 2), pow(this.e12345, 2)), 
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
        FlatPoint sub_type = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            (float3(other.e4) * sub_type.group0.xyz) - (float3(sub_type.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2), pow(this.e45, 2)), 
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
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)), 0.0)
        );
        Flector sub_type_2 = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e12345, 2), pow(sub_type_2.e45, 2) + pow(sub_type_2.e4235, 2) + pow(sub_type_2.e4315, 2) + pow(sub_type_2.e4125, 2)), 
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
        AntiFlatPoint sub_type = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Sphere wedge = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5)), 
            /* e1234 */
            sub_type.e321 * other.e4 * -1.0
        );
        Line sub_type_2 = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(wedge.e3215 * wedge.e1234) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2), pow(sub_type_2.e415, 2) + pow(sub_type_2.e425, 2) + pow(sub_type_2.e435, 2)), 
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
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((sub_type.group1.yzx * other.group0.zxy) - (sub_type.group1.zxy * other.group0.yzx)), sub_type.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * sub_type.group1.xyz) - (float3(sub_type.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        Motor sub_type_2 = Motor(
            /* e415, e425, e435, e12345 */
            this.group0, 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(wedge.e41 * wedge.e15) + 2.0 * (wedge.e42 * wedge.e25) + 2.0 * (wedge.e43 * wedge.e35) + 2.0 * (wedge.e1234 * wedge.e3215) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) - pow(wedge.e45, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2), pow(sub_type_2.e415, 2) + pow(sub_type_2.e425, 2) + pow(sub_type_2.e435, 2) + pow(sub_type_2.e12345, 2)), 
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
        MultiVector sub_type = MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group3.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        MultiVector wedge = MultiVector(
            /* scalar, e12345 */
            float2(0.0, (sub_type.e4235 * other.e1) + (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4) + (sub_type.e1234 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * sub_type.group1) - (float4(sub_type.e5) * other.group0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * sub_type.group3.xyz), 0.0) + float4((float3(other.e5) * sub_type.group4.xyz), 0.0) - float4((float3(sub_type.e45) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(other.e5) * sub_type.group5.xyz) + (sub_type.group3.zxy * other.group0.yzx) - (sub_type.group3.yzx * other.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(sub_type.e425 * other.e3) - (sub_type.e235 * other.e4), -(sub_type.e435 * other.e1) - (sub_type.e315 * other.e4), -(sub_type.e415 * other.e2) - (sub_type.e125 * other.e4), (sub_type.e321 * other.e5) + (sub_type.e125 * other.e3)) + (other.group0.yzxy * float4(sub_type.group6.zxy, sub_type.e315)) + float4((float3(other.e5) * sub_type.group7.xyz), sub_type.e235 * other.e1), 
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
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(wedge.e15 * wedge.e41) + 2.0 * (wedge.e25 * wedge.e42) + 2.0 * (wedge.e35 * wedge.e43) + 2.0 * (wedge.e3215 * wedge.e1234) + pow(wedge.scalar, 2) + pow(wedge.e1, 2) + pow(wedge.e2, 2) + pow(wedge.e3, 2) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) + pow(wedge.e321, 2) - pow(wedge.e12345, 2) - pow(wedge.e45, 2) - pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2) - 2.0 * (wedge.e4 * wedge.e5) - 2.0 * (wedge.e423 * wedge.e235) - 2.0 * (wedge.e431 * wedge.e315) - 2.0 * (wedge.e412 * wedge.e125), 2.0*(sub_type_2.e4 * sub_type_2.e5) + 2.0 * (sub_type_2.e423 * sub_type_2.e235) + 2.0 * (sub_type_2.e431 * sub_type_2.e315) + 2.0 * (sub_type_2.e412 * sub_type_2.e125) + pow(sub_type_2.e12345, 2) + pow(sub_type_2.e45, 2) + pow(sub_type_2.e415, 2) + pow(sub_type_2.e425, 2) + pow(sub_type_2.e435, 2) + pow(sub_type_2.e4235, 2) + pow(sub_type_2.e4315, 2) + pow(sub_type_2.e4125, 2) - pow(sub_type_2.scalar, 2) - pow(sub_type_2.e1, 2) - pow(sub_type_2.e2, 2) - pow(sub_type_2.e3, 2) - pow(sub_type_2.e23, 2) - pow(sub_type_2.e31, 2) - pow(sub_type_2.e12, 2) - pow(sub_type_2.e321, 2) - 2.0 * (sub_type_2.e15 * sub_type_2.e41) - 2.0 * (sub_type_2.e25 * sub_type_2.e42) - 2.0 * (sub_type_2.e35 * sub_type_2.e43) - 2.0 * (sub_type_2.e3215 * sub_type_2.e1234)), 
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
        Plane sub_type_2 = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(float4(float3(0.0), 1.0)[3], 2) * pow(this.e3215, 2), pow(sub_type_2.e4235, 2) + pow(sub_type_2.e4315, 2) + pow(sub_type_2.e4125, 2)) * float2(-1.0, 1.0), 
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
        Plane sub_type_2 = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(pow(float4(float3(0.0), 1.0)[3], 2) * pow(this.e3215, 2), pow(sub_type_2.e4235, 2) + pow(sub_type_2.e4315, 2) + pow(sub_type_2.e4125, 2)) * float2(-1.0, 1.0), 
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
        AntiFlector sub_type = AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        DipoleInversion wedge = DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * sub_type.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((sub_type.group1.yzx * other.group0.zxy) - (sub_type.group1.zxy * other.group0.yzx)), sub_type.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * sub_type.group1.xyz) - (float3(sub_type.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))
        );
        Motor sub_type_2 = Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(2.0*(wedge.e41 * wedge.e15) + 2.0 * (wedge.e42 * wedge.e25) + 2.0 * (wedge.e43 * wedge.e35) + 2.0 * (wedge.e1234 * wedge.e3215) + pow(wedge.e23, 2) + pow(wedge.e31, 2) + pow(wedge.e12, 2) - pow(wedge.e45, 2) - pow(wedge.e4235, 2) - pow(wedge.e4315, 2) - pow(wedge.e4125, 2), pow(sub_type_2.e415, 2) + pow(sub_type_2.e425, 2) + pow(sub_type_2.e435, 2) + pow(sub_type_2.e12345, 2)), 
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
        Flector sub_type = Flector(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
        RoundPoint other = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        Motor wedge = Motor(
            /* e415, e425, e435, e12345 */
            float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((sub_type.group0.zxy * other.group0.yzx) - (sub_type.group0.yzx * other.group0.zxy)), 0.0)
        );
        Flector sub_type_2 = Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(-pow(wedge.e415, 2) - pow(wedge.e425, 2) - pow(wedge.e435, 2) - pow(wedge.e12345, 2), pow(sub_type_2.e45, 2) + pow(sub_type_2.e4235, 2) + pow(sub_type_2.e4315, 2) + pow(sub_type_2.e4125, 2)), 
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
