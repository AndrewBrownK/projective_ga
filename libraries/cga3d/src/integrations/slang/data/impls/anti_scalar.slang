implementing cga3d;
public extension AntiScalar {
    public MultiVector operator +(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
public extension AntiScalar {
    public VersorEven operator +(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, other.e4)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
    public CircleRotor operator +(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e12345 */
            float4(other.group0.xyz, this.e12345)
        );
    }
}
public extension AntiScalar {
    public VersorEven operator +(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321), 
            /* e235, e315, e125, e5 */
            float4(other.group0.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, 0.0)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public VersorEven operator +(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e5), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, 0.0)
        );
    }
}
public extension AntiScalar {
    public AntiScalar operator +(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 + this.e12345
        );
    }
}
public extension AntiScalar {
    public CircleRotor operator +(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.group2.xyz, this.e12345)
        );
    }
}
public extension AntiScalar {
    public CircleRotor operator +(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz, 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.group2.xyz, this.e12345 + other.e12345)
        );
    }
}
public extension AntiScalar {
    public MultiVector operator +(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
public extension AntiScalar {
    public MultiVector operator +(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
public extension AntiScalar {
    public DualNum operator +(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(0.0, this.e12345) + other.group0.xy
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public Motor operator +(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xyz, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.group1.xyz, 0.0)
        );
    }
}
public extension AntiScalar {
    public Motor operator +(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xyz, this.e12345 + other.e12345), 
            /* e235, e315, e125, e5 */
            other.group1
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public VersorEven operator +(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e5), 
            /* e1, e2, e3, e4 */
            other.group0
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public VersorEven operator +(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.xyz, this.e12345 + other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1, 
            /* e235, e315, e125, e5 */
            other.group2, 
            /* e1, e2, e3, e4 */
            other.group3
        );
    }
}
public extension AntiScalar {
    public MultiVector operator +(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
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
public BitXorInfixPartial<AntiScalar> operator /(AntiScalar lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiScalar {
    public AntiScalar bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiScalar {
    public AntiScalar bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiScalar {
    public AntiScalar bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiScalar {
    public AntiScalar bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiScalar {
    public AntiScalar bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension AntiScalar {
    public AntiScalar bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension AntiScalar {
    public CircleRotor operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public DipoleInversion operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public DualNum operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public FlatPoint operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Flector operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Line operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Motor operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Plane operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Scalar operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Dipole operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiCircleRotor operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Circle operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiDipoleInversion operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiDualNum operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiFlatPoint operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiFlector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiLine operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiMotor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiPlane operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Sphere operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiScalar operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public RoundPoint operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public VersorOdd operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public VersorEven operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiScalar neg() {
        return AntiScalar(
            /* e12345 */
            this.e12345 * -1.0
        );
    }
}
public extension AntiScalar {
    public Scalar not() {
        return this.right_dual();
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
    public VersorEven operator -(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group2.xyz * float3(-1.0)), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group3.xyz * float3(-1.0)), other.e4 * -1.0)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
    public CircleRotor operator -(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345)
        );
    }
}
public extension AntiScalar {
    public VersorEven operator -(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz * float3(-1.0)), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public VersorEven operator -(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar {
    public AntiScalar operator -(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 - other.e12345
        );
    }
}
public extension AntiScalar {
    public CircleRotor operator -(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz * float3(-1.0)), this.e12345)
        );
    }
}
public extension AntiScalar {
    public CircleRotor operator -(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz * float3(-1.0)), this.e12345 - other.e12345)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public DualNum operator -(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * -1.0, this.e12345 - other.e12345)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public Motor operator -(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345), 
            /* e235, e315, e125, e5 */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar {
    public Motor operator -(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345 - other.e12345), 
            /* e235, e315, e125, e5 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public VersorEven operator -(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension AntiScalar {
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
public extension AntiScalar {
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
public extension AntiScalar {
    public VersorEven operator -(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(-1.0)), this.e12345 - other.e12345), 
            /* e415, e425, e435, e321 */
            other.group1 * float4(-1.0), 
            /* e235, e315, e125, e5 */
            other.group2 * float4(-1.0), 
            /* e1, e2, e3, e4 */
            other.group3 * float4(-1.0)
        );
    }
}
public extension AntiScalar {
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
