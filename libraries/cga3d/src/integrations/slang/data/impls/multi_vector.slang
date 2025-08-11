implementing cga3d;
public extension MultiVector {
    public MultiVector operator +(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz + this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz + other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(other.group3.xyz, other.e4), 
            /* e5 */
            other.e5 + this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412 */
            other.group0.xyz + this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz + other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar + this.scalar, this.e12345), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, other.e3215 + this.e3215), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, other.e321 + this.e321), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz + other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz + this.group1.xyz), this.e4), 
            /* e5 */
            other.e5 + this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, other.e321 + this.e321), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz + other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            other.group0.xyz + this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz + this.group3.xyz), this.e45), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz + other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, other.e3215 + this.e3215), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz + this.group1.xyz), this.e4), 
            /* e5 */
            other.e5 + this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412 */
            other.group0.xyz + this.group7.xyz, 
            /* e235, e315, e125 */
            other.group2.xyz + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            other.group1 + this.group6, 
            /* e423, e431, e412 */
            other.group0.xyz + this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz + other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            float4((other.group2.xyz + this.group3.xyz), other.e45 + this.e45), 
            /* e41, e42, e43 */
            other.group0.xyz + this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz + other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            other.group0.xyz + this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz + other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group3 + this.group9, 
            /* e1234 */
            other.e1234 + this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar, other.e12345 + this.e12345), 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            other.e5 + this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0 + this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            other.group0 + this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group1 + this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 + float4(other.group0.xyz, 0.0), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            other.group1.xyz + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            other.e5 + this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz + this.group6.xyz), this.e321), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz + other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e5 */
            other.e5 + this.e5, 
            /* e15, e25, e35, e45 */
            other.group3 + this.group3, 
            /* e41, e42, e43 */
            other.group4.xyz + this.group4.xyz, 
            /* e23, e31, e12 */
            other.group5.xyz + this.group5.xyz, 
            /* e415, e425, e435, e321 */
            other.group6 + this.group6, 
            /* e423, e431, e412 */
            other.group7.xyz + this.group7.xyz, 
            /* e235, e315, e125 */
            other.group8.xyz + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            other.group9 + this.group9, 
            /* e1234 */
            other.e1234 + this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e5 */
            this.e5 + other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group0, 
            /* e1234 */
            this.e1234 + other.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + other.group3, 
            /* e5 */
            other.e5 + this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 + other.group1, 
            /* e423, e431, e412 */
            this.group7.xyz + other.group0.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz + other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            this.group4.xyz + other.group0.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz + other.group1.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 + other.group3, 
            /* e1234 */
            other.e1234 + this.e1234
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<MultiVector> operator /(MultiVector lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiCircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiDipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiDualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiFlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiFlector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiLine other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiMotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(AntiPlane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public AntiScalar bit_xor(AntiScalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Circle other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(CircleRotor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Dipole other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(DipoleInversion other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(FlatPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(RoundPoint other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Sphere other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(VersorEven other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(VersorOdd other) {
        return this.wedge(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiCircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiDipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiDualNum other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiFlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiFlector other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiLine other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiMotor other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiPlane other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Circle other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(CircleRotor other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Dipole other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(DipoleInversion other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(FlatPoint other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(RoundPoint other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Sphere other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(VersorEven other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(VersorOdd other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector neg() {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e5 */
            this.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            this.group3 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group5.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group6 * float4(-1.0), 
            /* e423, e431, e412 */
            this.group7.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group8.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group9 * float4(-1.0), 
            /* e1234 */
            this.e1234 * -1.0
        );
    }
}
public extension MultiVector {
    public MultiVector not() {
        return this.right_dual();
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(-other.group2.xyz, other.e45 * -1.0), 
            /* e41, e42, e43 */
            this.group4.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(-other.group3.xyz, other.e4 * -1.0), 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412 */
            this.group7.xyz - other.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0) + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, this.e3215 - other.e3215), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, this.e321 - other.e321), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0) + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(-other.group1.xyz, 0.0), 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, this.e321 - other.e321), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0) + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(-other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz - other.group0.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(-other.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, this.e3215 - other.e3215), 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(-other.group0.xyz, 0.0), 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412 */
            this.group7.xyz - other.group0.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz - other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412 */
            this.group7.xyz - other.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0) + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(-other.group2.xyz, other.e45 * -1.0), 
            /* e41, e42, e43 */
            this.group4.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(-other.group2.xyz, other.e45 * -1.0), 
            /* e41, e42, e43 */
            this.group4.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group3, 
            /* e1234 */
            this.e1234 - other.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 - other.group0, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 - other.group0, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group1, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 + float4(-other.group0.xyz, 0.0), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz - other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 + float4(-other.group0.xyz, 0.0), 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0) + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3 - other.group3, 
            /* e41, e42, e43 */
            this.group4.xyz - other.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz - other.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 - other.group6, 
            /* e423, e431, e412 */
            this.group7.xyz - other.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz - other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group9, 
            /* e1234 */
            this.e1234 - other.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group0, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group0, 
            /* e1234 */
            this.e1234 - other.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e12345 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group3, 
            /* e5 */
            this.e5 - other.e5, 
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e41, e42, e43 */
            this.group4.xyz, 
            /* e23, e31, e12 */
            this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6 - other.group1, 
            /* e423, e431, e412 */
            float3(other.e423 * -1.0, other.e431 * -1.0, other.e412 * -1.0) + this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e235 * -1.0, other.e315 * -1.0, other.e125 * -1.0) + this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9, 
            /* e1234 */
            this.e1234
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e5 */
            this.e5, 
            /* e15, e25, e35, e45 */
            this.group3 + float4(-other.group2.xyz, other.e45 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0) + this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group5.xyz, 
            /* e415, e425, e435, e321 */
            this.group6, 
            /* e423, e431, e412 */
            this.group7.xyz, 
            /* e235, e315, e125 */
            this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            this.group9 - other.group3, 
            /* e1234 */
            this.e1234 - other.e1234
        );
    }
}
