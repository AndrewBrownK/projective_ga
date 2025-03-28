implementing rga3d;
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
    public Flector operator +(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Point {
    public Flector operator +(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group0, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Point {
    public Point operator +(Origin other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e4 + other.e4)
        );
    }
}
public extension Point {
    public Flector operator +(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Point {
    public Point operator +(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Point> operator /(Point lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(BitXorInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public Point bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public Motor bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public AntiScalar bit_xor(Horizon other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public Plane bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public Flector bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public Line bit_xor(Origin other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public AntiScalar bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Line operator /(BitXorInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public Line bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Point operator /(BitXorInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Point {
    public Point bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension Point {
    public Plane operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Flector operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Motor operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Motor operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Flector operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Flector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Line operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Motor operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Motor operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Point operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Point {
    public Point neg() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    public Plane not() {
        return this.right_dual();
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
    public Flector operator -(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Point {
    public Flector operator -(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
public extension Point {
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
            other.group4 * float4(-1.0)
        );
    }
}
public extension Point {
    public Point operator -(Origin other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e4 - other.e4)
        );
    }
}
public extension Point {
    public Flector operator -(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    public Point operator -(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0
        );
    }
}
public extension Point {
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
            float4(0.0)
        );
    }
}
