implementing cga3d;
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
        float geometric_anti_product_g0 = this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(geometric_anti_product_g0) * this.group2
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
        float geometric_anti_product_g0 = this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412 + this.e4 * this.e4;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(geometric_anti_product_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(geometric_anti_product_g0) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(geometric_anti_product_g0) * this.group3
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
        float geometric_anti_product_g0 = this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412;
        return Circle(
            /* e423, e431, e412 */
            float3(geometric_anti_product_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e235, e315, e125 */
            float3(geometric_anti_product_g0) * this.group2.xyz
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
        float geometric_anti_product_g0 = this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(geometric_anti_product_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(geometric_anti_product_g0) * this.group2
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
        float geometric_anti_product_g0 = this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43;
        return Dipole(
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e15, e25, e35 */
            float3(geometric_anti_product_g0) * this.group2.xyz
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
        float geometric_anti_product_g0 = this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(geometric_anti_product_g0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product_g0) * this.group3
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
        float geometric_anti_product_g0 = this.e4 * this.e4 + this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412 + this.e1234 * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2(geometric_anti_product_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e5 */
            geometric_anti_product_g0 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(geometric_anti_product_g0) * this.group3, 
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product_g0) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product_g0) * this.group6, 
            /* e423, e431, e412 */
            float3(geometric_anti_product_g0) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(geometric_anti_product_g0) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product_g0) * this.group9, 
            /* e1234 */
            geometric_anti_product_g0 * this.e1234
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
        float geometric_anti_product_g0 = 1.0 / this.e4;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_g0) * this.group0, 
            /* e5 */
            geometric_anti_product_g0 * this.e5
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
        float geometric_anti_product_g0 = 1.0 / this.e1234;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product_g0) * this.group0, 
            /* e1234 */
            geometric_anti_product_g0 * this.e1234
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
        float geometric_anti_product_g0 = this.group0[0] * this.group0[0] + this.group0[1] * this.group0[1] + this.group0[2] * this.group0[2] + this.e4 * this.e4;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(geometric_anti_product_g0) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(geometric_anti_product_g0) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_g0) * this.group3
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
        float3 sub_type_g0 = this.group0.xyz;
        float geometric_anti_product_g0 = sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] + this.e1234 * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(geometric_anti_product_g0) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(geometric_anti_product_g0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product_g0) * this.group3
        );
    }
}
