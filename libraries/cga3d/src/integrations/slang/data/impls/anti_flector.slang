implementing cga3d;
public extension AntiFlector {
    public MultiVector operator +(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator +(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group1.xyz, other.e321 + this.e321), 
            /* e235, e315, e125, e4 */
            other.group2 + float4(this.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            other.group3 + this.group1
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiFlector operator +(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 + this.group0, 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
public extension AntiFlector {
    public AntiFlector operator +(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            other.group0 + this.group0, 
            /* e1, e2, e3, e5 */
            other.group1 + this.group1
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiFlector operator +(AntiPlane other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0, 
            /* e1, e2, e3, e5 */
            this.group1 + other.group0
        );
    }
}
public extension AntiFlector {
    public VersorEven operator +(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator +(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group1.xyz, this.e321 + other.e321), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz + this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
public extension AntiFlector {
    public VersorEven operator +(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.group1.xyz, this.e321 + other.e321), 
            /* e235, e315, e125, e5 */
            float4((this.group0.xyz + other.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension AntiFlector {
    public VersorEven operator +(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group1, 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator +(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, this.e321), 
            /* e235, e315, e125, e4 */
            float4((other.group1.xyz + this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
public extension AntiFlector {
    public VersorEven operator +(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, this.e321), 
            /* e235, e315, e125, e5 */
            other.group1 + float4(this.group0.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4((this.group1.xyz + other.group1.xyz), other.e4), 
            /* e5 */
            this.e5 + other.e5, 
            /* e15, e25, e35, e45 */
            other.group3, 
            /* e41, e42, e43 */
            other.group4.xyz, 
            /* e23, e31, e12 */
            other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.group6.xyz, this.e321 + other.e321), 
            /* e423, e431, e412 */
            other.group7.xyz, 
            /* e235, e315, e125 */
            other.group8.xyz + this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group9, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0, 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator +(RoundPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e235, e315, e125, e4 */
            float4(this.group0.xyz, other.e4), 
            /* e1, e2, e3, e5 */
            this.group1 + float4(other.group0.xyz, other.e5)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0, 
            /* e1234 */
            other.e1234
        );
    }
}
public extension AntiFlector {
    public VersorEven operator +(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0, 
            /* e415, e425, e435, e321 */
            float4(other.group1.xyz, this.e321 + other.e321), 
            /* e235, e315, e125, e5 */
            other.group2 + float4(this.group0.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4((this.group1.xyz + other.group3.xyz), other.e4)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator +(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3, 
            /* e1234 */
            other.e1234
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<AntiFlector> operator /(AntiFlector lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public VersorEven bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public DipoleInversion bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BitXorInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiFlector bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BitXorInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiDualNum bit_xor(AntiFlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiMotor bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BitXorInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BitXorInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiFlector bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiMotor bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public Sphere bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public Sphere bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public CircleRotor bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public CircleRotor bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public Flector bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public Motor bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public Motor bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public Plane bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public Flector bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiScalar bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public DipoleInversion bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BitXorInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiFlector bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public AntiScalar bit_xor(Sphere other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public DipoleInversion bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiFlector {
    public VersorEven bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension AntiFlector {
    public VersorEven operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorOdd operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiFlector operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiMotor operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiMotor operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiFlector operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiFlector operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiMotor operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public Flector operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorOdd operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorOdd operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorEven operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorEven operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public Flector operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public Motor operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public Motor operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public Flector operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public Flector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public Motor operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorOdd operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiFlector operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorEven operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorOdd operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public VersorEven operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension AntiFlector {
    public AntiFlector neg() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension AntiFlector {
    public Flector not() {
        return this.right_dual();
    }
}
public extension AntiFlector {
    public MultiVector operator -(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator -(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float3(-1.0)), this.e321 - other.e321), 
            /* e235, e315, e125, e4 */
            float4((this.group0.xy - other.group2.xy), this.e125 - other.e125, other.e4 * -1.0), 
            /* e1, e2, e3, e5 */
            this.group1 - other.group3
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiFlector operator -(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 - other.group0, 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
public extension AntiFlector {
    public AntiFlector operator -(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 - other.group0, 
            /* e1, e2, e3, e5 */
            this.group1 - other.group1
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiFlector operator -(AntiPlane other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0, 
            /* e1, e2, e3, e5 */
            this.group1 - other.group0
        );
    }
}
public extension AntiFlector {
    public VersorEven operator -(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator -(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float3(-1.0)), this.e321 - other.e321), 
            /* e235, e315, e125, e4 */
            float4((this.group0.xyz - other.group2.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
public extension AntiFlector {
    public VersorEven operator -(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float3(-1.0)), this.e321 - other.e321), 
            /* e235, e315, e125, e5 */
            float4((this.group0.xyz - other.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension AntiFlector {
    public VersorEven operator -(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group1 * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator -(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), this.e321), 
            /* e235, e315, e125, e4 */
            float4((this.group0.xyz - other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            this.group1
        );
    }
}
public extension AntiFlector {
    public VersorEven operator -(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), this.e321), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, this.e5) - other.group1, 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4((this.group1.xy - other.group1.xy), this.e3 - other.e3, other.e4 * -1.0), 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            other.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4((other.group6.xyz * float3(-1.0)), this.e321 - other.e321), 
            /* e423, e431, e412 */
            other.group7.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group0.xyz - other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group9 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public AntiDipoleInversion operator -(RoundPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e235, e315, e125, e4 */
            float4(this.group0.xyz, other.e4 * -1.0), 
            /* e1, e2, e3, e5 */
            this.group1 + float4(-other.group0.xyz, other.e5 * -1.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group0 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension AntiFlector {
    public VersorEven operator -(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0), 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float3(-1.0)), this.e321 - other.e321), 
            /* e235, e315, e125, e5 */
            float4(this.group0.xyz, this.e5) - other.group2, 
            /* e1, e2, e3, e4 */
            float4((this.group1.xy - other.group3.xy), this.e3 - other.e3, other.e4 * -1.0)
        );
    }
}
public extension AntiFlector {
    public MultiVector operator -(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz * float3(-1.0)), other.e45 * -1.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 * float4(-1.0), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
