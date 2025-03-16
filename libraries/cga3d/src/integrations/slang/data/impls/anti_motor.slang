implementing cga3d;
public extension AntiMotor {
    public VersorOdd operator +(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, other.scalar + this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, other.e4), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator +(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, other.scalar + this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, other.e3215 + this.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator +(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 + float4(other.group0.xyz, 0.0), 
            /* e15, e25, e35, e3215 */
            this.group1 + float4(other.group1.xyz, 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator +(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0 + this.group0, 
            /* e15, e25, e35, e3215 */
            other.group1 + this.group1
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator +(Dipole other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz + this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator +(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            other.group2 + float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 + other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator +(FlatPoint other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator +(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz + other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group1.xyz, this.e3215 + other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0) + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e5 */
            other.e5, 
            /* e15, e25, e35, e45 */
            other.group3 + float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            other.group4.xyz, 
            /* e23, e31, e12 */
            other.group5.xyz + this.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group6, 
            /* e423, e431, e412 */
            other.group7.xyz, 
            /* e235, e315, e125 */
            other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group9.xyz, this.e3215 + other.e3215), 
            /* e1234 */
            other.e1234
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator +(Plane other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 + other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator +(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, this.scalar + other.scalar), 
            /* e15, e25, e35, e3215 */
            this.group1
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator +(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.xyz, this.e3215 + other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator +(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator +(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.xyz, this.scalar + other.scalar), 
            /* e23, e31, e12, e45 */
            other.group1 + float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            other.group2 + float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group3.xyz, this.e3215 + other.e3215)
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<AntiMotor> operator /(AntiMotor lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(BitXorInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public VersorOdd bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public VersorEven bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiMotor bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BitXorInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiFlatPoint bit_xor(AntiFlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BitXorInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiFlector bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiMotor bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiMotor bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BitXorInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiFlector bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiScalar bit_xor(AntiScalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public CircleRotor bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public CircleRotor bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public DipoleInversion bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public DipoleInversion bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public Motor bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public Flector bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public Flector bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public Motor bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public Motor bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public Plane bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public VersorEven bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BitXorInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public AntiMotor bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public Sphere bit_xor(Sphere other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public VersorEven bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BitXorInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiMotor {
    public VersorOdd bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension AntiMotor {
    public VersorOdd operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorEven operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiMotor operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiFlector operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiFlector operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiMotor operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiMotor operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiFlector operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public Motor operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorEven operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorEven operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorOdd operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorOdd operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public Motor operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public Flector operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public Motor operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public Flector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorEven operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiMotor operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorOdd operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorEven operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public VersorOdd operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension AntiMotor {
    public AntiMotor neg() {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension AntiMotor {
    public Motor not() {
        return this.right_dual();
    }
}
public extension AntiMotor {
    public VersorOdd operator -(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar - other.scalar), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator -(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, this.scalar - other.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group1.xyz, this.e3215 - other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator -(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 + float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e15, e25, e35, e3215 */
            this.group1 + float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator -(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 - other.group0, 
            /* e15, e25, e35, e3215 */
            this.group1 - other.group1
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator -(Dipole other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator -(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.group3.xyz * float3(-1.0)), this.e3215 - other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator -(FlatPoint other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator -(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.group1.xyz * float3(-1.0)), this.e3215 - other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar - other.scalar, other.e12345) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e5 */
            other.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4((this.group1.xyz - other.group3.xyz), other.e45 * -1.0), 
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
            float4((other.group9.xyz * float3(-1.0)), this.e3215 - other.e3215), 
            /* e1234 */
            other.e1234 * -1.0
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator -(Plane other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * float3(-1.0)), this.e3215 - other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, 0.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public AntiMotor operator -(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xyz, this.scalar - other.scalar), 
            /* e15, e25, e35, e3215 */
            this.group1
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator -(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group1.xyz, other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * float3(-1.0)), this.e3215 - other.e3215)
        );
    }
}
public extension AntiMotor {
    public MultiVector operator -(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345) * float2(1.0, -1.0), 
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
            float4(float3(0.0), this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
public extension AntiMotor {
    public VersorOdd operator -(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((other.group0.xyz * float3(-1.0)), this.scalar - other.scalar), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz - other.group1.xyz), other.e45 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4((this.group1.xyz - other.group2.xyz), other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.group3.xyz * float3(-1.0)), this.e3215 - other.e3215)
        );
    }
}
