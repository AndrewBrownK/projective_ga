implementing cga3d;
public extension Sphere {
    public VersorOdd operator +(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public MultiVector operator +(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, other.e4), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e235, e315, e125 */
            other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public VersorOdd operator +(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, other.e3215 + this.e3215)
        );
    }
}
public extension Sphere {
    public MultiVector operator +(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
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
            float4(float3(0.0), other.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public DipoleInversion operator +(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group1.xyz, this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public VersorOdd operator +(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group1.xyz, this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, other.e3215 + this.e3215)
        );
    }
}
public extension Sphere {
    public MultiVector operator +(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0), 
            /* e5 */
            other.e5, 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
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
            other.group1, 
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e235, e315, e125 */
            other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
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
            other.group1, 
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e235, e315, e125 */
            other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public DipoleInversion operator +(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public DipoleInversion operator +(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, other.e1234 + this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group0
        );
    }
}
public extension Sphere {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5, 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public DipoleInversion operator +(FlatPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public DipoleInversion operator +(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group0
        );
    }
}
public extension Sphere {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
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
            float4(other.group0.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            other.group3, 
            /* e41, e42, e43 */
            other.group4.xyz, 
            /* e23, e31, e12 */
            other.group5.xyz, 
            /* e415, e425, e435, e321 */
            other.group6, 
            /* e423, e431, e412 */
            other.group7.xyz, 
            /* e235, e315, e125 */
            other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group9 + this.group0, 
            /* e1234 */
            other.e1234 + this.e1234
        );
    }
}
public extension Sphere {
    public Sphere operator +(Plane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            other.group0 + this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e5 */
            other.e5, 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public VersorOdd operator +(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public Sphere operator +(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            other.group0 + this.group0, 
            /* e1234 */
            other.e1234 + this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator +(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            other.group3, 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e235, e315, e125 */
            other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public VersorOdd operator +(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, this.e1234 + other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 + other.group3
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Sphere> operator /(Sphere lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public Sphere bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public AntiScalar bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public Sphere bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public AntiScalar bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public Sphere bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public AntiScalar bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public AntiScalar bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public AntiScalar bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public AntiScalar bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public Sphere bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public AntiScalar bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Sphere {
    public Sphere bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension Sphere {
    public DipoleInversion operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorEven operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorOdd operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public AntiDipoleInversion operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorEven operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public DipoleInversion operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorOdd operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public CircleRotor operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public RoundPoint operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public AntiDipoleInversion operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public AntiDipoleInversion operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public DipoleInversion operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorOdd operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorEven operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public DipoleInversion operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorOdd operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public AntiDipoleInversion operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorEven operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public AntiCircleRotor operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public CircleRotor operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public Sphere operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public AntiCircleRotor operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorEven operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public VersorOdd operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension Sphere {
    public Sphere neg() {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0), 
            /* e1234 */
            this.e1234 * -1.0
        );
    }
}
public extension Sphere {
    public RoundPoint not() {
        return this.right_dual();
    }
}
public extension Sphere {
    public VersorOdd operator -(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public MultiVector operator -(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group3.xyz * float3(-1.0)), other.e4 * -1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public VersorOdd operator -(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e3215 - other.e3215)
        );
    }
}
public extension Sphere {
    public MultiVector operator -(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
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
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group0.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group0.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public DipoleInversion operator -(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public VersorOdd operator -(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e3215 - other.e3215)
        );
    }
}
public extension Sphere {
    public MultiVector operator -(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e5 */
            other.e5 * -1.0, 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
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
            other.group1 * float4(-1.0), 
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
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
            other.group1 * float4(-1.0), 
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public DipoleInversion operator -(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public DipoleInversion operator -(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), this.e1234 - other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group3
        );
    }
}
public extension Sphere {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * -1.0, 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public DipoleInversion operator -(FlatPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public DipoleInversion operator -(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz * float3(-1.0)), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group1
        );
    }
}
public extension Sphere {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
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
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            other.group1.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            other.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group6 * float4(-1.0), 
            /* e423, e431, e412 */
            other.group7.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group9, 
            /* e1234 */
            this.e1234 - other.e1234
        );
    }
}
public extension Sphere {
    public Sphere operator -(Plane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e5 */
            other.e5 * -1.0, 
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
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public VersorOdd operator -(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Sphere {
    public Sphere operator -(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group0, 
            /* e1234 */
            this.e1234 - other.e1234
        );
    }
}
public extension Sphere {
    public MultiVector operator -(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            other.group2.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension Sphere {
    public VersorOdd operator -(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), this.e1234 - other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group3
        );
    }
}
