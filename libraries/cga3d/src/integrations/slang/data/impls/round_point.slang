implementing cga3d;
public extension RoundPoint {
    public MultiVector operator +(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
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
public extension RoundPoint {
    public AntiDipoleInversion operator +(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e4 */
            float4(other.group2.xyz, other.e4 + this.e4), 
            /* e1, e2, e3, e5 */
            other.group3 + float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public MultiVector operator +(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public AntiDipoleInversion operator +(AntiFlatPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e4 */
            float4(other.group0.xyz, this.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator +(AntiFlector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e4 */
            float4(other.group0.xyz, this.e4), 
            /* e1, e2, e3, e5 */
            other.group1 + float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public MultiVector operator +(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public MultiVector operator +(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public RoundPoint operator +(AntiPlane other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz + this.group0.xyz), this.e4), 
            /* e5 */
            other.e5 + this.e5
        );
    }
}
public extension RoundPoint {
    public VersorEven operator +(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator +(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e4 */
            float4(other.group2.xyz, this.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public VersorEven operator +(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public MultiVector operator +(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
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
public extension RoundPoint {
    public MultiVector operator +(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
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
public extension RoundPoint {
    public VersorEven operator +(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public MultiVector operator +(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public AntiDipoleInversion operator +(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e4 */
            float4(other.group1.xyz, this.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public VersorEven operator +(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(other.group1.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group0, 
            /* e5 */
            other.e5 + this.e5, 
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
public extension RoundPoint {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public RoundPoint operator +(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e5 */
            other.e5 + this.e5
        );
    }
}
public extension RoundPoint {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public MultiVector operator +(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public VersorEven operator +(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0, 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e5 + this.e5), 
            /* e1, e2, e3, e4 */
            this.group0 + other.group3
        );
    }
}
public extension RoundPoint {
    public MultiVector operator +(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.group2.xyz, other.e45), 
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
public BitXorInfixPartial<RoundPoint> operator /(RoundPoint lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BitXorInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public DipoleInversion bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public VersorEven bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Sphere bit_xor(AntiFlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public DipoleInversion bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Circle operator /(BitXorInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Circle bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public VersorEven bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BitXorInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Dipole bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Sphere bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BitXorInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Sphere bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Circle operator /(BitXorInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Circle bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BitXorInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public CircleRotor bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BitXorInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public FlatPoint bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Line bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Motor bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Plane bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Flector bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public AntiScalar bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BitXorInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public Dipole bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BitXorInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public RoundPoint bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public AntiScalar bit_xor(Sphere other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BitXorInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public DipoleInversion bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BitXorInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension RoundPoint {
    public VersorEven bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorOdd operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorEven operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public DipoleInversion operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorOdd operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorEven operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public AntiCircleRotor operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public Sphere operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public DipoleInversion operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public DipoleInversion operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorEven operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorOdd operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorEven operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public DipoleInversion operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorOdd operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public CircleRotor operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public AntiCircleRotor operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public RoundPoint operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public CircleRotor operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorOdd operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public VersorEven operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension RoundPoint {
    public RoundPoint neg() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e5 */
            this.e5 * -1.0
        );
    }
}
public extension RoundPoint {
    public Sphere not() {
        return this.right_dual();
    }
}
public extension RoundPoint {
    public MultiVector operator -(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public AntiDipoleInversion operator -(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * float3(-1.0)), this.e4 - other.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5) - other.group3
        );
    }
}
public extension RoundPoint {
    public MultiVector operator -(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public AntiDipoleInversion operator -(AntiFlatPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e4 */
            float4((other.group0.xyz * float3(-1.0)), this.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator -(AntiFlector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e4 */
            float4((other.group0.xyz * float3(-1.0)), this.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5) - other.group1
        );
    }
}
public extension RoundPoint {
    public MultiVector operator -(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public MultiVector operator -(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public RoundPoint operator -(AntiPlane other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0 + float4(-other.group0.xyz, 0.0), 
            /* e5 */
            this.e5 - other.e5
        );
    }
}
public extension RoundPoint {
    public VersorEven operator -(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public AntiDipoleInversion operator -(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e4 */
            float4((other.group2.xyz * float3(-1.0)), this.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public VersorEven operator -(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz * float3(-1.0)), this.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public MultiVector operator -(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public MultiVector operator -(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public VersorEven operator -(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public MultiVector operator -(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public AntiDipoleInversion operator -(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e235, e315, e125, e4 */
            float4((other.group1.xyz * float3(-1.0)), this.e4), 
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, this.e5)
        );
    }
}
public extension RoundPoint {
    public VersorEven operator -(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((other.group1.xyz * float3(-1.0)), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension RoundPoint {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0 - other.group1, 
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
public extension RoundPoint {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public RoundPoint operator -(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e5 */
            this.e5 - other.e5
        );
    }
}
public extension RoundPoint {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public MultiVector operator -(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
public extension RoundPoint {
    public VersorEven operator -(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0 * float4(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz * float3(-1.0)), this.e5 - other.e5), 
            /* e1, e2, e3, e4 */
            this.group0 - other.group3
        );
    }
}
public extension RoundPoint {
    public MultiVector operator -(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
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
