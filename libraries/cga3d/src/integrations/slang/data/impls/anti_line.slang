implementing cga3d;
public extension AntiLine {
    public AntiCircleRotor operator +(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, scalar */
            other.group2 + float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator +(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group3.xyz, other.e4), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public AntiMotor operator +(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, other.e3215)
        );
    }
}
public extension AntiLine {
    public MultiVector operator +(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator +(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public AntiLine operator +(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            other.group0.xyz + this.group0.xyz, 
            /* e15, e25, e35 */
            other.group1.xyz + this.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiMotor operator +(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            other.group1 + float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator +(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator +(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator +(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public Dipole operator +(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35 */
            this.group1.xyz + other.group2.xyz
        );
    }
}
public extension AntiLine {
    public DipoleInversion operator +(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            other.group2 + float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
public extension AntiLine {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public Dipole operator +(FlatPoint other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35 */
            this.group1.xyz + other.group0.xyz
        );
    }
}
public extension AntiLine {
    public DipoleInversion operator +(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1
        );
    }
}
public extension AntiLine {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            other.group3 + float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            other.group4.xyz, 
            /* e23, e31, e12 */
            this.group0.xyz + other.group5.xyz, 
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
public extension AntiLine {
    public DipoleInversion operator +(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
public extension AntiLine {
    public MultiVector operator +(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public AntiMotor operator +(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    public DipoleInversion operator +(Sphere other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            other.group0
        );
    }
}
public extension AntiLine {
    public MultiVector operator +(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345), 
            /* e1, e2, e3, e4 */
            other.group3, 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public VersorOdd operator +(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0, 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            other.group2 + float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<AntiLine> operator /(AntiLine lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public DipoleInversion bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public CircleRotor bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BitXorInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiLine bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BitXorInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BitXorInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiDualNum bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiMotor bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BitXorInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiScalar bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiScalar bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public Sphere bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public Sphere bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BitXorInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<AntiLine> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public Plane bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public Plane bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiScalar bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public Motor bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Circle operator /(BitXorInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public Circle bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BitXorInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public AntiLine bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public CircleRotor bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiLine {
    public DipoleInversion bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension AntiLine {
    public VersorOdd operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public VersorEven operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiLine operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiFlector operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiFlector operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiMotor operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiMotor operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiFlector operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public Line operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public VersorEven operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public VersorEven operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public VersorOdd operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public VersorOdd operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public Line operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public Flector operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public Motor operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public Flector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiDipoleInversion operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiLine operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public DipoleInversion operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public VersorEven operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public VersorOdd operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension AntiLine {
    public AntiLine neg() {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension AntiLine {
    public Line not() {
        return this.right_dual();
    }
}
public extension AntiLine {
    public AntiCircleRotor operator -(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group0.xyz), other.e45 * -1.0), 
            /* e15, e25, e35, scalar */
            float4((float3(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0) + this.group1.xyz), other.scalar * -1.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator -(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group3.xyz * float3(-1.0)), other.e4 * -1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public AntiMotor operator -(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, other.e3215 * -1.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator -(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator -(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public AntiLine operator -(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz - other.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz - other.group1.xyz
        );
    }
}
public extension AntiLine {
    public AntiMotor operator -(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group0.xyz), other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0) + this.group1.xyz), other.e3215 * -1.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator -(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator -(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator -(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public Dipole operator -(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group0.xyz), other.e45 * -1.0), 
            /* e15, e25, e35 */
            this.group1.xyz - other.group2.xyz
        );
    }
}
public extension AntiLine {
    public DipoleInversion operator -(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group0.xyz), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0) + this.group1.xyz), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public Dipole operator -(FlatPoint other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45 * -1.0), 
            /* e15, e25, e35 */
            float3(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0) + this.group1.xyz
        );
    }
}
public extension AntiLine {
    public DipoleInversion operator -(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4((float3(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0) + this.group1.xyz), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group0.xyz - other.group5.xyz, 
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
public extension AntiLine {
    public DipoleInversion operator -(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator -(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public AntiMotor operator -(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiLine {
    public DipoleInversion operator -(Sphere other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension AntiLine {
    public MultiVector operator -(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz, 
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
public extension AntiLine {
    public VersorOdd operator -(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group0.xyz), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0) + this.group1.xyz), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0)
        );
    }
}
