implementing cga3d;
using traits;
// Fancy postfix trick
public AntiCircleRotor operator /(AntiCircleRotor lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(UnitizePrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.unitize();
}
public extension AntiCircleRotor {
    public AntiCircleRotor unitize() {
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(geometric_anti_product.e12345) * this.group2
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(UnitizePrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.unitize();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion unitize() {
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
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e45, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Circle operator /(UnitizePrefixOrPostfix lhs, Circle rhs) {
    return rhs.unitize();
}
public extension Circle {
    public Circle unitize() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
        return Circle(
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(geometric_anti_product.e12345) * this.group2.xyz
        );
    }
}
// Fancy postfix trick
public CircleRotor operator /(CircleRotor lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public CircleRotor operator /(UnitizePrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.unitize();
}
public extension CircleRotor {
    public CircleRotor unitize() {
        DualNum other = DualNum(
            /* e5, e12345 */
            float2(1.0, 0.0)
        );
        Plane wedge = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xy.xx, other.e5, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
        return CircleRotor(
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(geometric_anti_product.e12345) * this.group2
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Dipole operator /(UnitizePrefixOrPostfix lhs, Dipole rhs) {
    return rhs.unitize();
}
public extension Dipole {
    public Dipole unitize() {
        Line wedge = Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            float3(0.0)
        );
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2)
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(geometric_anti_product.e12345) * this.group2.xyz
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public DipoleInversion operator /(UnitizePrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.unitize();
}
public extension DipoleInversion {
    public DipoleInversion unitize() {
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
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e12345, 2)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public MultiVector operator /(UnitizePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitize();
}
public extension MultiVector {
    public MultiVector unitize() {
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
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            2.0*(wedge.e4 * wedge.e5) + 2.0 * (wedge.e423 * wedge.e235) + 2.0 * (wedge.e431 * wedge.e315) + 2.0 * (wedge.e412 * wedge.e125) + pow(wedge.e12345, 2) + pow(wedge.e45, 2) + pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2) - pow(wedge.scalar, 2) - pow(wedge.e1, 2) - pow(wedge.e2, 2) - pow(wedge.e3, 2) - pow(wedge.e23, 2) - pow(wedge.e31, 2) - pow(wedge.e12, 2) - pow(wedge.e321, 2) - 2.0 * (wedge.e15 * wedge.e41) - 2.0 * (wedge.e25 * wedge.e42) - 2.0 * (wedge.e35 * wedge.e43) - 2.0 * (wedge.e3215 * wedge.e1234)
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(geometric_anti_product.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e5 */
            geometric_anti_product.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(geometric_anti_product.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(geometric_anti_product.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(geometric_anti_product.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group9, 
            /* e1234 */
            geometric_anti_product.e12345 * this.e1234
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public RoundPoint operator /(UnitizePrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.unitize();
}
public extension RoundPoint {
    public RoundPoint unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.e4
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e5 */
            geometric_anti_product.e12345 * this.e5
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Sphere operator /(UnitizePrefixOrPostfix lhs, Sphere rhs) {
    return rhs.unitize();
}
public extension Sphere {
    public Sphere unitize() {
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            1.0/this.e1234
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e1234 */
            geometric_anti_product.e12345 * this.e1234
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public VersorEven operator /(UnitizePrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.unitize();
}
public extension VersorEven {
    public VersorEven unitize() {
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
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e45, 2) + pow(wedge.e4235, 2) + pow(wedge.e4315, 2) + pow(wedge.e4125, 2)
        );
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public VersorOdd operator /(UnitizePrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.unitize();
}
public extension VersorOdd {
    public VersorOdd unitize() {
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
        AntiScalar geometric_anti_product = AntiScalar(
            /* e12345 */
            pow(wedge.e415, 2) + pow(wedge.e425, 2) + pow(wedge.e435, 2) + pow(wedge.e12345, 2)
        );
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(geometric_anti_product.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(geometric_anti_product.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product.e12345) * this.group3
        );
    }
}
