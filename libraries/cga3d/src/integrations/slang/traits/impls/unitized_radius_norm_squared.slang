implementing cga3d;
using traits;
// Fancy postfix trick
public float operator /(AntiCircleRotor lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension AntiCircleRotor {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            pow(this.e45, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - pow(this.scalar, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return -(pow(wedge.e415, 2) * anti_dot_product.e12345) - (pow(wedge.e425, 2) * anti_dot_product.e12345) - (pow(wedge.e435, 2) * anti_dot_product.e12345);
    }
}
// Fancy postfix trick
public float operator /(AntiDipoleInversion lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension AntiDipoleInversion {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e4 * this.e5) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) - pow(this.e321, 2) - pow(this.e1, 2) - pow(this.e2, 2) - pow(this.e3, 2)
        );
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
        return -(pow(wedge.e45, 2) * anti_dot_product.e12345) - (pow(wedge.e4235, 2) * anti_dot_product.e12345) - (pow(wedge.e4315, 2) * anti_dot_product.e12345) - (pow(wedge.e4125, 2) * anti_dot_product.e12345);
    }
}
// Fancy postfix trick
public float operator /(Circle lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Circle {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) - pow(this.e321, 2)
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return -(pow(wedge.e4235, 2) * anti_dot_product.e12345) - (pow(wedge.e4315, 2) * anti_dot_product.e12345) - (pow(wedge.e4125, 2) * anti_dot_product.e12345);
    }
}
// Fancy postfix trick
public float operator /(CircleRotor lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension CircleRotor {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) + pow(this.e12345, 2) - pow(this.e321, 2)
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return -(pow(wedge.e4235, 2) * anti_dot_product.e12345) - (pow(wedge.e4315, 2) * anti_dot_product.e12345) - (pow(wedge.e4125, 2) * anti_dot_product.e12345);
    }
}
// Fancy postfix trick
public float operator /(Dipole lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Dipole {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            pow(this.e45, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35)
        );
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        return -(pow(wedge.e415, 2) * anti_dot_product.e12345) - (pow(wedge.e425, 2) * anti_dot_product.e12345) - (pow(wedge.e435, 2) * anti_dot_product.e12345);
    }
}
// Fancy postfix trick
public float operator /(DipoleInversion lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension DipoleInversion {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            pow(this.e45, 2) + pow(this.e4235, 2) + pow(this.e4315, 2) + pow(this.e4125, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215)
        );
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
        return -(pow(wedge.e415, 2) * anti_dot_product.e12345) - (pow(wedge.e425, 2) * anti_dot_product.e12345) - (pow(wedge.e435, 2) * anti_dot_product.e12345) - (pow(wedge.e12345, 2) * anti_dot_product.e12345);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension MultiVector {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            2.0*(this.e4 * this.e5) + 2.0 * (this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + pow(this.e12345, 2) + pow(this.e45, 2) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) + pow(this.e4235, 2) + pow(this.e4315, 2) + pow(this.e4125, 2) - pow(this.scalar, 2) - pow(this.e1, 2) - pow(this.e2, 2) - pow(this.e3, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - pow(this.e321, 2) - 2.0 * (this.e15 * this.e41) - 2.0 * (this.e25 * this.e42) - 2.0 * (this.e35 * this.e43) - 2.0 * (this.e3215 * this.e1234)
        );
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
        return 2.0*(anti_dot_product.e12345 * wedge.e15 * wedge.e41) + 2.0 * (anti_dot_product.e12345 * wedge.e25 * wedge.e42) + 2.0 * (anti_dot_product.e12345 * wedge.e35 * wedge.e43) + 2.0 * (anti_dot_product.e12345 * wedge.e3215 * wedge.e1234) + (pow(wedge.scalar, 2) * anti_dot_product.e12345) + (pow(wedge.e1, 2) * anti_dot_product.e12345) + (pow(wedge.e2, 2) * anti_dot_product.e12345) + (pow(wedge.e3, 2) * anti_dot_product.e12345) + (pow(wedge.e23, 2) * anti_dot_product.e12345) + (pow(wedge.e31, 2) * anti_dot_product.e12345) + (pow(wedge.e12, 2) * anti_dot_product.e12345) + (pow(wedge.e321, 2) * anti_dot_product.e12345) - (pow(wedge.e12345, 2) * anti_dot_product.e12345) - (pow(wedge.e45, 2) * anti_dot_product.e12345) - (pow(wedge.e415, 2) * anti_dot_product.e12345) - (pow(wedge.e425, 2) * anti_dot_product.e12345) - (pow(wedge.e435, 2) * anti_dot_product.e12345) - (pow(wedge.e4235, 2) * anti_dot_product.e12345) - (pow(wedge.e4315, 2) * anti_dot_product.e12345) - (pow(wedge.e4125, 2) * anti_dot_product.e12345) - 2.0 * (anti_dot_product.e12345 * wedge.e4 * wedge.e5) - 2.0 * (anti_dot_product.e12345 * wedge.e423 * wedge.e235) - 2.0 * (anti_dot_product.e12345 * wedge.e431 * wedge.e315) - 2.0 * (anti_dot_product.e12345 * wedge.e412 * wedge.e125);
    }
}
// Fancy postfix trick
public float operator /(RoundPoint lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension RoundPoint {
    public float unitized_radius_norm_squared() {
        RoundPoint sub_type = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e5 */
            0.0
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        return (pow(this.e1, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2)) + (pow(this.e2, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2)) + (pow(this.e3, 2) * pow(other.e5, -2) * pow(sub_type.e4, -2)) - 2.0 * (this.e4 * this.e5 * pow(other.e5, -2) * pow(sub_type.e4, -2));
    }
}
// Fancy postfix trick
public float operator /(Sphere lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension Sphere {
    public float unitized_radius_norm_squared() {
        Sphere sub_type = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            this.e1234
        );
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        return 2.0*(this.e3215 * this.e1234 * pow(other.e5, -2) * pow(sub_type.e1234, -2)) - (pow(this.e4235, 2) * pow(other.e5, -2) * pow(sub_type.e1234, -2)) - (pow(this.e4315, 2) * pow(other.e5, -2) * pow(sub_type.e1234, -2)) - (pow(this.e4125, 2) * pow(other.e5, -2) * pow(sub_type.e1234, -2));
    }
}
// Fancy postfix trick
public float operator /(VersorEven lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension VersorEven {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            2.0*(this.e423 * this.e235) + 2.0 * (this.e431 * this.e315) + 2.0 * (this.e412 * this.e125) + 2.0 * (this.e5 * this.e4) + pow(this.e12345, 2) + pow(this.e415, 2) + pow(this.e425, 2) + pow(this.e435, 2) - pow(this.e321, 2) - pow(this.e1, 2) - pow(this.e2, 2) - pow(this.e3, 2)
        );
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
        return -(pow(wedge.e45, 2) * anti_dot_product.e12345) - (pow(wedge.e4235, 2) * anti_dot_product.e12345) - (pow(wedge.e4315, 2) * anti_dot_product.e12345) - (pow(wedge.e4125, 2) * anti_dot_product.e12345);
    }
}
// Fancy postfix trick
public float operator /(VersorOdd lhs, UnitizedRadiusNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_radius_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedRadiusNormSquaredPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitized_radius_norm_squared();
}
public extension VersorOdd {
    public float unitized_radius_norm_squared() {
        AntiScalar anti_dot_product = AntiScalar(
            /* e12345 */
            pow(this.e45, 2) + pow(this.e4235, 2) + pow(this.e4315, 2) + pow(this.e4125, 2) - pow(this.scalar, 2) - pow(this.e23, 2) - pow(this.e31, 2) - pow(this.e12, 2) - 2.0 * (this.e41 * this.e15) - 2.0 * (this.e42 * this.e25) - 2.0 * (this.e43 * this.e35) - 2.0 * (this.e1234 * this.e3215)
        );
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
        return -(pow(wedge.e415, 2) * anti_dot_product.e12345) - (pow(wedge.e425, 2) * anti_dot_product.e12345) - (pow(wedge.e435, 2) * anti_dot_product.e12345) - (pow(wedge.e12345, 2) * anti_dot_product.e12345);
    }
}
