implementing rga3d;
public extension Plane {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public Flector operator +(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group0
        );
    }
}
public extension Plane {
    public Plane operator +(Horizon other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, other.e321 + this.e321)
        );
    }
}
public extension Plane {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 + this.group0
        );
    }
}
public extension Plane {
    public Flector operator +(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public Plane operator +(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            other.group0 + this.group0
        );
    }
}
public extension Plane {
    public Flector operator +(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Plane> operator /(Plane lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(Origin other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public AntiScalar bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Plane {
    public Plane bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension Plane {
    public Origin operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Flector operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Motor operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Motor operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Flector operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Flector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public AntiScalar operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Motor operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Motor operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Plane operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Plane {
    public Plane neg() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Plane {
    public Origin not() {
        return this.right_dual();
    }
}
public extension Plane {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public Flector operator -(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0 - other.group1
        );
    }
}
public extension Plane {
    public Plane operator -(Horizon other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, this.e321 - other.e321)
        );
    }
}
public extension Plane {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, other.e1234 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0 - other.group4
        );
    }
}
public extension Plane {
    public Flector operator -(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public Plane operator -(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 - other.group0
        );
    }
}
public extension Plane {
    public Flector operator -(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Plane {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
