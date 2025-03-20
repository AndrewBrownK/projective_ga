implementing rga3d;
public extension DualNum {
    public DualNum operator +(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy
        );
    }
}
public extension DualNum {
    public DualNum operator +(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension DualNum {
    public Motor operator +(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar)
        );
    }
}
public extension DualNum {
    public Motor operator +(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 + other.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar + other.scalar)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public DualNum operator +(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<DualNum> operator /(DualNum lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public AntiScalar bit_xor(AntiScalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BitXorInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public DualNum bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Flector bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Horizon operator /(BitXorInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Horizon bit_xor(Horizon other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Line bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Motor bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Origin operator /(BitXorInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Origin bit_xor(Origin other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Plane bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Point operator /(BitXorInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public Point bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BitXorInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension DualNum {
    public DualNum bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension DualNum {
    public AntiScalar operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Line operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Origin operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum neg() {
        return DualNum(
            /* scalar, e1234 */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension DualNum {
    public AntiScalar not() {
        return this.right_dual();
    }
}
public extension DualNum {
    public DualNum operator -(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public DualNum operator -(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension DualNum {
    public Motor operator -(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz * float3(-1.0)), this.e1234), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * float3(-1.0)), this.scalar)
        );
    }
}
public extension DualNum {
    public Motor operator -(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz * float3(-1.0)), this.e1234 - other.e1234), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * float3(-1.0)), this.scalar - other.scalar)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public DualNum operator -(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy
        );
    }
}
