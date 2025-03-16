implementing rga3d;
public extension Scalar {
    public DualNum operator +(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar, other.e1234)
        );
    }
}
public extension Scalar {
    public DualNum operator +(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar, 0.0) + other.group0.xy
        );
    }
}
public extension Scalar {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public MultiVector operator +(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public Motor operator +(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar)
        );
    }
}
public extension Scalar {
    public Motor operator +(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            other.group0, 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, other.scalar + this.scalar)
        );
    }
}
public extension Scalar {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0) + other.group0.xy, 
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
public extension Scalar {
    public MultiVector operator +(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public MultiVector operator +(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public Scalar operator +(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar + this.scalar
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Scalar> operator /(Scalar lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public AntiScalar bit_xor(AntiScalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BitXorInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public DualNum bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Flector bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Horizon operator /(BitXorInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Horizon bit_xor(Horizon other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Line bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Motor bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Origin operator /(BitXorInfixPartial<Scalar> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Origin bit_xor(Origin other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Plane bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Point operator /(BitXorInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Point bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BitXorInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Scalar {
    public Scalar bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension Scalar {
    public AntiScalar operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public DualNum operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Horizon operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Line operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Origin operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Plane operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Point operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Scalar operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Scalar {
    public Scalar neg() {
        return Scalar(
            /* scalar */
            this.scalar * -1.0
        );
    }
}
public extension Scalar {
    public AntiScalar not() {
        return this.right_dual();
    }
}
public extension Scalar {
    public DualNum operator -(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar, other.e1234) * float2(1.0, -1.0)
        );
    }
}
public extension Scalar {
    public DualNum operator -(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar - other.scalar, other.e1234) * float2(1.0, -1.0)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public MultiVector operator -(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public Motor operator -(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * float3(-1.0)), this.scalar)
        );
    }
}
public extension Scalar {
    public Motor operator -(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * float3(-1.0)), this.scalar - other.scalar)
        );
    }
}
public extension Scalar {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar - other.scalar, other.e1234) * float2(1.0, -1.0), 
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
public extension Scalar {
    public MultiVector operator -(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public MultiVector operator -(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
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
public extension Scalar {
    public Scalar operator -(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar - other.scalar
        );
    }
}
