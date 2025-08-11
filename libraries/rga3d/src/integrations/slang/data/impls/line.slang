implementing rga3d;
public extension Line {
    public Motor operator +(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    public Motor operator +(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar)
        );
    }
}
public extension Line {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Line {
    public MultiVector operator +(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Line {
    public Line operator +(Line other) {
        return Line(
            /* e41, e42, e43 */
            other.group0.xyz + this.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group1.xyz
        );
    }
}
public extension Line {
    public Motor operator +(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            other.group0 + float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            other.group1 + float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            this.group0.xyz + other.group2.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz + other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Line {
    public MultiVector operator +(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Line {
    public MultiVector operator +(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line {
    public Motor operator +(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Line> operator /(Line lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public Line bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public Plane bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public AntiScalar bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public Motor bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public Plane bit_xor(Origin other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public Plane bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Line {
    public Line bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension Line {
    public Line operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Line operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Flector operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Motor operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Plane operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Flector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Flector operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Line operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Line {
    public Line neg() {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension Line {
    public Line not() {
        return this.right_dual();
    }
}
public extension Line {
    public Motor operator -(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    public Motor operator -(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar * -1.0)
        );
    }
}
public extension Line {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Line {
    public MultiVector operator -(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Line {
    public Line operator -(Line other) {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz - other.group1.xyz
        );
    }
}
public extension Line {
    public Motor operator -(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0) + this.group0.xyz), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) + this.group1.xyz), other.scalar * -1.0)
        );
    }
}
public extension Line {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Line {
    public MultiVector operator -(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Line {
    public MultiVector operator -(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line {
    public Motor operator -(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar * -1.0)
        );
    }
}
