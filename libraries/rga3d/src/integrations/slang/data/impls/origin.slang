implementing rga3d;
public extension Origin {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public Flector operator +(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, other.e4 + this.e4), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Origin {
    public Flector operator +(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Origin {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, other.e4 + this.e4), 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Origin {
    public Origin operator +(Origin other) {
        return Origin(
            /* e4 */
            other.e4 + this.e4
        );
    }
}
public extension Origin {
    public Flector operator +(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Origin {
    public Point operator +(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, this.e4 + other.e4)
        );
    }
}
public extension Origin {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Origin> operator /(Origin lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(BitXorInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public Origin bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public Motor bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public AntiScalar bit_xor(Horizon other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public Plane bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public Flector bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public AntiScalar bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public Line bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Origin operator /(BitXorInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Origin {
    public Origin bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension Origin {
    public Origin operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Motor operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public AntiScalar operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Plane operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Flector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public AntiScalar operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Line operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Origin operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Origin neg() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Origin {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public Flector operator -(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), this.e4 - other.e4), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Origin {
    public Flector operator -(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * float3(-1.0)), this.e4 - other.e4), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Origin {
    public Origin operator -(Origin other) {
        return Origin(
            /* e4 */
            this.e4 - other.e4
        );
    }
}
public extension Origin {
    public Flector operator -(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Origin {
    public Point operator -(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(-1.0)), this.e4 - other.e4)
        );
    }
}
public extension Origin {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
