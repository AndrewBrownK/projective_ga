implementing rga3d;
public extension Flector {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public Flector operator +(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group1
        );
    }
}
public extension Flector {
    public Flector operator +(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, this.e321 + other.e321)
        );
    }
}
public extension Flector {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group0 + other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group4
        );
    }
}
public extension Flector {
    public Flector operator +(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e4 + other.e4), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public Flector operator +(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group0
        );
    }
}
public extension Flector {
    public Flector operator +(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 + other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Flector> operator /(Flector lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public Flector bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public Motor bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public AntiScalar bit_xor(Horizon other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public Plane bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public Flector bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public Motor bit_xor(Origin other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public AntiScalar bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public Motor bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Flector {
    public Flector bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension Flector {
    public Flector operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Flector operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Motor operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Motor operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Flector operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Flector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Motor operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Motor operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Motor operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Flector operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Flector {
    public Flector neg() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Flector {
    public Flector not() {
        return this.right_dual();
    }
}
public extension Flector {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public Flector operator -(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group1
        );
    }
}
public extension Flector {
    public Flector operator -(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, this.e321 - other.e321)
        );
    }
}
public extension Flector {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, other.e1234 * -1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0 - other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 - other.group4
        );
    }
}
public extension Flector {
    public Flector operator -(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e4 - other.e4), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public Flector operator -(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group0
        );
    }
}
public extension Flector {
    public Flector operator -(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Flector {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
