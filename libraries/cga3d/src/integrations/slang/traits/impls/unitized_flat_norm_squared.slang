implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_flat_norm_squared() {
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
        FlatPoint sub_type_2 = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
        return -(pow(wedge.e415, 2) * pow(sub_type_2.e45, -2)) - (pow(wedge.e425, 2) * pow(sub_type_2.e45, -2)) - (pow(wedge.e435, 2) * pow(sub_type_2.e45, -2));
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_flat_norm_squared() {
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
        return pow(this.group1.xyz[0], 2) * wedge.e41 * wedge.e15 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Circle {
    public float unitized_flat_norm_squared() {
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
        return pow(this.group1.xyz[0], 2) * wedge.e3215 * wedge.e1234 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension CircleRotor {
    public float unitized_flat_norm_squared() {
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
        return pow(float4((float3(other.e4) * sub_type.group0.xyz * float3(-1.0)), (sub_type.e235 * other.e1) + (sub_type.e315 * other.e2) + (sub_type.e125 * other.e3) + (sub_type.e321 * other.e5))[0], 2) * pow(this.e415, 2) * -1.0;
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Dipole {
    public float unitized_flat_norm_squared() {
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
        FlatPoint sub_type_2 = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
        return -(pow(wedge.e415, 2) * pow(sub_type_2.e45, -2)) - (pow(wedge.e425, 2) * pow(sub_type_2.e45, -2)) - (pow(wedge.e435, 2) * pow(sub_type_2.e45, -2));
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension DipoleInversion {
    public float unitized_flat_norm_squared() {
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
        return pow((float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)))[0], 2) * pow(float4(float3(0.0), this.e45)[3], 2) * -1.0;
    }
}
// Fancy postfix trick
public float operator /(DualNum lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension DualNum {
    public float unitized_flat_norm_squared() {
        return pow(this.e5, 2) * pow(this.e12345, -2);
    }
}
// Fancy postfix trick
public float operator /(FlatPoint lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension FlatPoint {
    public float unitized_flat_norm_squared() {
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
        FlatPoint sub_type_2 = FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
        return -(pow(wedge.e415, 2) * pow(sub_type_2.e45, -2)) - (pow(wedge.e425, 2) * pow(sub_type_2.e45, -2)) - (pow(wedge.e435, 2) * pow(sub_type_2.e45, -2));
    }
}
// Fancy postfix trick
public float operator /(Flector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Flector {
    public float unitized_flat_norm_squared() {
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
        return pow((float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)))[0], 2) * pow(float4(float3(0.0), this.e45)[3], 2) * -1.0;
    }
}
// Fancy postfix trick
public float operator /(Line lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Line {
    public float unitized_flat_norm_squared() {
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
        return pow(this.e415, 2) * wedge.e3215 * wedge.e1234 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(Motor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Motor {
    public float unitized_flat_norm_squared() {
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
        return pow(this.e415, 2) * wedge.e41 * wedge.e15 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension MultiVector {
    public float unitized_flat_norm_squared() {
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
        return sub_type_2.e423 * sub_type_2.e235 * wedge.e15 * wedge.e41 * 4.0;
    }
}
// Fancy postfix trick
public float operator /(Plane lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Plane {
    public float unitized_flat_norm_squared() {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            float4(float3(0.0), 1.0)[3] * this.e3215
        );
        Plane sub_type_2 = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
        return -(pow(wedge.e12345, 2) * pow(sub_type_2.e4235, 2)) - (pow(wedge.e12345, 2) * pow(sub_type_2.e4315, 2)) - (pow(wedge.e12345, 2) * pow(sub_type_2.e4125, 2));
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Sphere {
    public float unitized_flat_norm_squared() {
        AntiScalar wedge = AntiScalar(
            /* e12345 */
            float4(float3(0.0), 1.0)[3] * this.e3215
        );
        Plane sub_type_2 = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
        return -(pow(wedge.e12345, 2) * pow(sub_type_2.e4235, 2)) - (pow(wedge.e12345, 2) * pow(sub_type_2.e4315, 2)) - (pow(wedge.e12345, 2) * pow(sub_type_2.e4125, 2));
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension VersorEven {
    public float unitized_flat_norm_squared() {
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
        return pow(this.e415, 2) * wedge.e41 * wedge.e15 * 2.0;
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension VersorOdd {
    public float unitized_flat_norm_squared() {
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
        return pow((float4(sub_type.e45 * other.e1 * -1.0, sub_type.e45 * other.e2 * -1.0, sub_type.e45 * other.e3 * -1.0, (sub_type.e4315 * other.e2) + (sub_type.e4125 * other.e3) + (sub_type.e3215 * other.e4)) + (other.group0.wwwx * float4(sub_type.group0.xyz, sub_type.e4235)))[0], 2) * pow(float4(float3(0.0), this.e45)[3], 2) * -1.0;
    }
}
