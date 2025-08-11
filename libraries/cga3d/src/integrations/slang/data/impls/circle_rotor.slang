implementing cga3d;
public extension CircleRotor {
    public MultiVector operator +(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator +(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz + this.group0.xyz), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz + this.group2.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e4)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator +(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e12345 */
            float4((other.group0.xyz + this.group2.xyz), this.e12345)
        );
    }
}
public extension CircleRotor {
    public VersorEven operator +(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz + this.group2.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator +(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator +(AntiScalar other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.group2.xyz, this.e12345 + other.e12345)
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator +(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz + this.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(other.group2.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator +(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz + this.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e12345 */
            other.group2 + this.group2
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension CircleRotor {
    public VersorEven operator +(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 + this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator +(Line other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 + float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(other.group1.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public VersorEven operator +(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345 + other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz + other.group1.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345) + other.group0.xy, 
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
            this.group1 + other.group6, 
            /* e423, e431, e412 */
            this.group0.xyz + other.group7.xyz, 
            /* e235, e315, e125 */
            other.group8.xyz + this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group9, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator +(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            other.group0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension CircleRotor {
    public VersorEven operator +(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz + other.group0.xyz), this.e12345 + other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz + other.group2.xyz), other.e5), 
            /* e1, e2, e3, e4 */
            other.group3
        );
    }
}
public extension CircleRotor {
    public MultiVector operator +(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234 */
            other.e1234
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<CircleRotor> operator /(CircleRotor lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public CircleRotor bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public Sphere bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public CircleRotor bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public Sphere bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public AntiScalar bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public CircleRotor bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public Sphere bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public AntiScalar bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public AntiScalar bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public Plane bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public AntiScalar bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public AntiScalar bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public Plane bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public Sphere bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public CircleRotor bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public Sphere bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension CircleRotor {
    public CircleRotor bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public DipoleInversion operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public AntiCircleRotor operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public AntiDipoleInversion operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public DipoleInversion operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public CircleRotor operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public AntiDipoleInversion operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorOdd operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public VersorEven operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension CircleRotor {
    public CircleRotor neg() {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0)
        );
    }
}
public extension CircleRotor {
    public AntiCircleRotor not() {
        return this.right_dual();
    }
}
public extension CircleRotor {
    public MultiVector operator -(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator -(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz - other.group0.xyz), this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xy - other.group2.xy), this.e125 - other.e125, other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group3.xyz * float3(-1.0)), other.e4 * -1.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator -(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, this.e321 - other.e321), 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(-other.group0.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public VersorEven operator -(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, this.e321 - other.e321), 
            /* e235, e315, e125, e5 */
            float4((this.group2.xy - other.group0.xy), this.e125 - other.e125, other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator -(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator -(AntiScalar other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.group2.xyz, this.e12345 - other.e12345)
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator -(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz - other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(-other.group2.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator -(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz - other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e12345 */
            this.group2 - other.group2
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator -(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345 - other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public CircleRotor operator -(Line other) {
        return CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 + float4(-other.group0.xyz, 0.0), 
            /* e235, e315, e125, e12345 */
            this.group2 + float4(-other.group1.xyz, 0.0)
        );
    }
}
public extension CircleRotor {
    public VersorEven operator -(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345 - other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(-other.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4((this.group2.xy - other.group1.xy), this.e125 - other.e125, other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, this.e12345 - other.e12345), 
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
            this.group1 - other.group6, 
            /* e423, e431, e412 */
            this.group0.xyz - other.group7.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz - other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group9 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator -(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
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
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension CircleRotor {
    public VersorEven operator -(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, 0.0) - other.group0, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xy - other.group2.xy), this.e125 - other.e125, other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0)
        );
    }
}
public extension CircleRotor {
    public MultiVector operator -(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
