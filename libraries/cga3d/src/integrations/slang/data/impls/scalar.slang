implementing cga3d;
public extension Scalar {
    public AntiCircleRotor operator +(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, scalar */
            float4(other.group2.xyz, other.scalar + this.scalar)
        );
    }
}
public extension Scalar {
    public MultiVector operator +(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiDualNum operator +(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(0.0, this.scalar) + other.group0.xy
        );
    }
}
public extension Scalar {
    public MultiVector operator +(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator +(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiMotor operator +(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(other.group1.xyz, 0.0)
        );
    }
}
public extension Scalar {
    public AntiMotor operator +(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.group0.xyz, other.scalar + this.scalar), 
            /* e15, e25, e35, e3215 */
            other.group1
        );
    }
}
public extension Scalar {
    public MultiVector operator +(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
public extension Scalar {
    public MultiVector operator +(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator +(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiCircleRotor operator +(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, scalar */
            float4(other.group2.xyz, this.scalar)
        );
    }
}
public extension Scalar {
    public VersorOdd operator +(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            other.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
public extension Scalar {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiCircleRotor operator +(FlatPoint other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45), 
            /* e15, e25, e35, scalar */
            float4(other.group0.xyz, this.scalar)
        );
    }
}
public extension Scalar {
    public VersorOdd operator +(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45), 
            /* e15, e25, e35, e1234 */
            float4(other.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1
        );
    }
}
public extension Scalar {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0) + other.group0.xy, 
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
            other.group9, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension Scalar {
    public VersorOdd operator +(Plane other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
public extension Scalar {
    public MultiVector operator +(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public Scalar operator +(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar + this.scalar
        );
    }
}
public extension Scalar {
    public VersorOdd operator +(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
public extension Scalar {
    public MultiVector operator +(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public VersorOdd operator +(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar + other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1, 
            /* e15, e25, e35, e1234 */
            other.group2, 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Scalar> operator /(Scalar lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BitXorInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiCircleRotor bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BitXorInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiDipoleInversion bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BitXorInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiDualNum bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BitXorInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiFlatPoint bit_xor(AntiFlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BitXorInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiFlector bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BitXorInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiLine bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiMotor bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BitXorInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiPlane bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiScalar bit_xor(AntiScalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Circle operator /(BitXorInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Circle bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public CircleRotor bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BitXorInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Dipole bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public DipoleInversion bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BitXorInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public DualNum bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BitXorInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public FlatPoint bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Flector bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Line bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Motor bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Plane bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BitXorInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public RoundPoint bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BitXorInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Scalar bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Sphere bit_xor(Sphere other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public VersorEven bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BitXorInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public VersorOdd bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension Scalar {
    public AntiCircleRotor operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiDipoleInversion operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiDualNum operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiFlatPoint operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiFlector operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiLine operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiMotor operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiPlane operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public AntiScalar operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Circle operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public CircleRotor operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Dipole operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public DipoleInversion operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public DualNum operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public FlatPoint operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Line operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Plane operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public RoundPoint operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Scalar operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Sphere operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public VersorEven operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public VersorOdd operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Scalar neg() {
        return Scalar(
            /* scalar */
            this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiScalar not() {
        return this.right_dual();
    }
}
public extension Scalar {
    public AntiCircleRotor operator -(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz * float3(-1.0)), this.scalar - other.scalar)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiDualNum operator -(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215, this.scalar - other.scalar) * float2(-1.0, 1.0)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator -(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiMotor operator -(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension Scalar {
    public AntiMotor operator -(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar - other.scalar), 
            /* e15, e25, e35, e3215 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
public extension Scalar {
    public MultiVector operator -(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator -(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiCircleRotor operator -(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, scalar */
            float4((other.group2.xyz * float3(-1.0)), this.scalar)
        );
    }
}
public extension Scalar {
    public VersorOdd operator -(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public AntiCircleRotor operator -(FlatPoint other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar)
        );
    }
}
public extension Scalar {
    public VersorOdd operator -(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar - other.scalar, other.e12345) * float2(1.0, -1.0), 
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
            other.group9 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension Scalar {
    public VersorOdd operator -(Plane other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public Scalar operator -(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar - other.scalar
        );
    }
}
public extension Scalar {
    public VersorOdd operator -(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension Scalar {
    public VersorOdd operator -(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar - other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 * float4(-1.0), 
            /* e15, e25, e35, e1234 */
            other.group2 * float4(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
