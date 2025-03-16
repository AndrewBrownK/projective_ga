implementing cga3d;
public extension Plane {
    public VersorOdd operator +(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e41, other.e42, other.e43, other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public VersorOdd operator +(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, other.e3215 + this.e3215)
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public DipoleInversion operator +(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public VersorOdd operator +(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(other.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, other.e3215 + this.e3215)
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public DipoleInversion operator +(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public DipoleInversion operator +(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            other.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public Flector operator +(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public Flector operator +(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
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
            other.e1234
        );
    }
}
public extension Plane {
    public Plane operator +(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            other.group0 + this.group0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public VersorOdd operator +(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public Sphere operator +(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 + other.group0, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public VersorOdd operator +(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            other.group2, 
            /* e4235, e4315, e4125, e3215 */
            this.group0 + other.group3
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Plane> operator /(Plane lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension Plane {
    public DipoleInversion operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public VersorEven operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Flector operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Motor operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Motor operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Flector operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Flector operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Motor operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiPlane operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiDipoleInversion operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiDipoleInversion operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public DipoleInversion operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public VersorOdd operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiFlector operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiMotor operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiMotor operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiFlector operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiFlector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiMotor operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public CircleRotor operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Plane operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiCircleRotor operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public VersorEven operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public VersorOdd operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Plane neg() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Plane {
    public AntiPlane not() {
        return this.right_dual();
    }
}
public extension Plane {
    public VersorOdd operator -(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public VersorOdd operator -(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e3215 - other.e3215)
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public DipoleInversion operator -(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public VersorOdd operator -(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e3215 - other.e3215)
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) * float2(0.0, -1.0), 
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
            0.0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public MultiVector operator -(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) * float2(0.0, -1.0), 
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
            0.0
        );
    }
}
public extension Plane {
    public DipoleInversion operator -(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public DipoleInversion operator -(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group3
        );
    }
}
public extension Plane {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) * float2(0.0, -1.0), 
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
            0.0
        );
    }
}
public extension Plane {
    public Flector operator -(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public Flector operator -(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group1
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) * float2(0.0, -1.0), 
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
            0.0
        );
    }
}
public extension Plane {
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
            other.e1234 * -1.0
        );
    }
}
public extension Plane {
    public Plane operator -(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group0
        );
    }
}
public extension Plane {
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
            0.0
        );
    }
}
public extension Plane {
    public VersorOdd operator -(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.scalar * -1.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0
        );
    }
}
public extension Plane {
    public Sphere operator -(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group0, 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension Plane {
    public MultiVector operator -(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) * float2(0.0, -1.0), 
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
            0.0
        );
    }
}
public extension Plane {
    public VersorOdd operator -(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group0 - other.group3
        );
    }
}
