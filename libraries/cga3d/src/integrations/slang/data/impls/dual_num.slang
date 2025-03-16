implementing cga3d;
public extension DualNum {
    public MultiVector operator +(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
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
public extension DualNum {
    public VersorEven operator +(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, other.e4)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
            float4(float3(0.0), other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public VersorEven operator +(AntiFlatPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e5 */
            float4(other.group0.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public VersorEven operator +(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e5 */
            float4(other.group0.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz, 
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
public extension DualNum {
    public MultiVector operator +(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public VersorEven operator +(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0)
        );
    }
}
public extension DualNum {
    public DualNum operator +(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(0.0, other.e12345) + this.group0.xy
        );
    }
}
public extension DualNum {
    public VersorEven operator +(Circle other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public VersorEven operator +(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, other.e12345 + this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
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
public extension DualNum {
    public MultiVector operator +(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension DualNum {
    public DualNum operator +(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            other.group0.xy + this.group0.xy
        );
    }
}
public extension DualNum {
    public MultiVector operator +(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0, 
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
public extension DualNum {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0, 
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
            other.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public Motor operator +(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xyz, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.group1.xyz, this.e5)
        );
    }
}
public extension DualNum {
    public Motor operator +(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xyz, this.e12345 + other.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.group1.xyz, this.e5 + other.e5)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345) + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e5 */
            this.e5 + other.e5, 
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
            other.group9, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
            other.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public VersorEven operator +(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.e5 + other.e5), 
            /* e1, e2, e3, e4 */
            other.group0
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
public extension DualNum {
    public MultiVector operator +(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
            other.group0, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension DualNum {
    public VersorEven operator +(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, this.e12345 + other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, this.e5 + other.e5), 
            /* e1, e2, e3, e4 */
            other.group3
        );
    }
}
public extension DualNum {
    public MultiVector operator +(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234 */
            other.e1234
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<DualNum> operator /(DualNum lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Motor bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Flector bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BitXorInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public DualNum bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BitXorInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public AntiDualNum bit_xor(AntiFlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Flector bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BitXorInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public AntiFlatPoint bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Motor bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BitXorInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public FlatPoint bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Plane bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Plane bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Line bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Motor bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BitXorInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public FlatPoint bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BitXorInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public DualNum bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public AntiScalar bit_xor(Sphere other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Flector bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Motor bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension DualNum {
    public VersorEven operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public VersorOdd operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Line operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Motor operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiDualNum operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DipoleInversion operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public VersorOdd operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiDipoleInversion operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public VersorEven operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiDualNum operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiFlector operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiFlector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiLine operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiMotor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public AntiFlector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public VersorOdd operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public VersorEven operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public VersorOdd operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public VersorEven operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum neg() {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension DualNum {
    public AntiDualNum not() {
        return this.right_dual();
    }
}
public extension DualNum {
    public MultiVector operator -(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
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
public extension DualNum {
    public VersorEven operator -(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz * float3(-1.0)), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group3.xyz * float3(-1.0)), other.e4 * -1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public VersorEven operator -(AntiFlatPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz * float3(-1.0)), this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public VersorEven operator -(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz * float3(-1.0)), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz * float3(-1.0), 
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
public extension DualNum {
    public MultiVector operator -(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public VersorEven operator -(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension DualNum {
    public DualNum operator -(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy + (float2(0.0, other.e12345) * float2(0.0, -1.0))
        );
    }
}
public extension DualNum {
    public VersorEven operator -(Circle other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz * float3(-1.0)), this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public VersorEven operator -(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345 - other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz * float3(-1.0)), this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
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
public extension DualNum {
    public MultiVector operator -(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension DualNum {
    public DualNum operator -(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            this.group0.xy - other.group0.xy
        );
    }
}
public extension DualNum {
    public MultiVector operator -(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
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
public extension DualNum {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
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
            other.group1 * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public Motor operator -(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345), 
            /* e235, e315, e125, e5 */
            float4((other.group1.xyz * float3(-1.0)), this.e5)
        );
    }
}
public extension DualNum {
    public Motor operator -(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345 - other.e12345), 
            /* e235, e315, e125, e5 */
            float4((other.group1.xyz * float3(-1.0)), this.e5 - other.e5)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345 - other.e12345) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e5 */
            this.e5 - other.e5, 
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
            other.group9 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
            other.group0 * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public VersorEven operator -(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
public extension DualNum {
    public MultiVector operator -(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
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
            other.group0 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension DualNum {
    public VersorEven operator -(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345 - other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz * float3(-1.0)), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
