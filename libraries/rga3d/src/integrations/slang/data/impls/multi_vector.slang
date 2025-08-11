implementing rga3d;
public extension MultiVector {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 + this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.group4.xyz, this.e321 + other.e321)
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            other.group0.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz + other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz + other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e41, e42, e43 */
            other.group2.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 + this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, this.e4 + other.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + other.group0
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<MultiVector> operator /(MultiVector lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<MultiVector>(lhs);
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
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(DualNum other) {
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
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Horizon other) {
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
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Origin other) {
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
public MultiVector operator /(BitXorInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension MultiVector {
    public MultiVector bit_xor(Point other) {
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
public extension MultiVector {
    public MultiVector operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Horizon other) {
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
    public MultiVector operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public MultiVector neg() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
public extension MultiVector {
    public MultiVector not() {
        return this.right_dual();
    }
}
public extension MultiVector {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group1
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.group4.xyz, this.e321 - other.e321)
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            float3(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0) + this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, this.e4 - other.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group0
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
