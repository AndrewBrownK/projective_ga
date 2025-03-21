implementing cga3d;
public extension AntiDipoleInversion {
    public MultiVector operator +(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator +(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz + this.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group1, 
            /* e235, e315, e125, e4 */
            other.group2 + this.group2, 
            /* e1, e2, e3, e5 */
            other.group3 + this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator +(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator +(AntiFlatPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, this.e321 + other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator +(AntiFlector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, this.e321 + other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3 + other.group1
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator +(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator +(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator +(AntiPlane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 + other.group0
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator +(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator +(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz + other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator +(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz + other.group0.xyz), other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz + other.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator +(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
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
public extension AntiDipoleInversion {
    public MultiVector operator +(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
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
public extension AntiDipoleInversion {
    public VersorEven operator +(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5 + other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator +(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator +(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 + float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            this.group2 + float4(other.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator +(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            this.group1 + float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            other.group1 + float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + float4(this.group3.xyz, this.e4), 
            /* e5 */
            this.e5 + other.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator +(RoundPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            float4(this.group2.xyz, this.e4 + other.e4), 
            /* e1, e2, e3, e5 */
            this.group3 + float4(float3(0.0), other.e5) + float4(other.group0.xyz, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator +(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public VersorEven operator +(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 + float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 + other.group1, 
            /* e235, e315, e125, e5 */
            other.group2 + float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            other.group3 + float4(this.group3.xyz, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator +(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e15, other.e25, other.e35, other.e45), 
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
public BitXorInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bit_xor(AntiFlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Flector bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Motor bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Motor bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Plane bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public Flector bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar bit_xor(Sphere other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorOdd operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion neg() {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0)
        );
    }
}
public extension AntiDipoleInversion {
    public DipoleInversion not() {
        return this.right_dual();
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator -(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz - other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e4 */
            this.group2 - other.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group3
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator -(AntiFlatPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, this.e321 - other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 - float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator -(AntiFlector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group1.xyz, this.e321 - other.e321), 
            /* e235, e315, e125, e4 */
            this.group2 - float4(other.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3 - other.group1
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator -(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator -(AntiPlane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            this.group2, 
            /* e1, e2, e3, e5 */
            this.group3 - other.group0
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator -(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator -(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz - other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e4 */
            this.group2 - float4(other.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator -(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz - other.group0.xyz), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz - other.group2.xyz), this.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator -(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public VersorEven operator -(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator -(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1 - float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            this.group2 - float4(other.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            this.group3
        );
    }
}
public extension AntiDipoleInversion {
    public VersorEven operator -(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.xyz, other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 - float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5) - other.group1, 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e4) - other.group1, 
            /* e5 */
            this.e5 - other.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public AntiDipoleInversion operator -(RoundPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group1, 
            /* e235, e315, e125, e4 */
            float4(this.group2.xyz, this.e4 - other.e4), 
            /* e1, e2, e3, e5 */
            this.group3 + float4(float3(0.0), other.e5 * -1.0) - float4(other.group0.xyz, 0.0)
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public MultiVector operator -(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
public extension AntiDipoleInversion {
    public VersorEven operator -(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz - other.group0.xyz), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            this.group1 - other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e5) - other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e4) - other.group3
        );
    }
}
public extension AntiDipoleInversion {
    public MultiVector operator -(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, this.e4), 
            /* e5 */
            this.e5, 
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
