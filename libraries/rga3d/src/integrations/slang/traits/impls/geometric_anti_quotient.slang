implementing rga3d;
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<AntiScalar> operator /(AntiScalar lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public AntiScalar geometric_anti_quotient(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 / other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public DualNum geometric_anti_quotient(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e1234 / other.e1234) * float2(other.scalar / other.e1234, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public Flector geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other_g0 * this.e1234 * -1.0) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(other_g0 * this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public Line geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        return Line(
            /* e41, e42, e43 */
            float3(other_g0 * this.e1234 * -1.0) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * this.e1234 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public Motor geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other_g0 * this.e1234) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other_g0 * this.e1234) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        return MultiVector(
            /* scalar, e1234 */
            float2(other_g0 * this.e1234) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_g0 * this.e1234 * -1.0) * other.group1, 
            /* e41, e42, e43 */
            float3(other_g0 * this.e1234 * -1.0) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * this.e1234 * -1.0) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_g0 * this.e1234) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public Origin geometric_anti_quotient(Origin other) {
        return Origin(
            /* e4 */
            this.e1234 * -1.0 / other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public Plane geometric_anti_quotient(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            (float4(this.e1234) * float4(pow(other.e423, 3), pow(other.e431, 3), pow(other.e412, 3), other.e423 * other.e423 * other.e321)) + (float4(other.e431 * other.e431 * this.e1234, other.e423 * other.e423 * this.e1234, other.e423 * other.e423 * this.e1234, other.e431 * other.e431 * this.e1234) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412 * this.e1234), other.e431 * other.e431 * this.e1234, other.e412 * other.e412 * this.e1234))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiQuotientInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension AntiScalar {
    public Point geometric_anti_quotient(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234 * -1.0 / other.e4) * float4((other.group0.xyz / float4(other.e4).xyz), 1.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<DualNum> operator /(DualNum lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public DualNum geometric_anti_quotient(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(1.0 / other.e1234) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public DualNum geometric_anti_quotient(DualNum other) {
        float geometric_anti_product_g0_y = 1.0 / other.e1234;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0_y * this.scalar) + (other.scalar * this.e1234 /  (other.e1234 * other.e1234)), geometric_anti_product_g0_y * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product_g1.xyz) + (float3(this.e1234) * geometric_anti_product_g0.xyz)), this.e1234 * geometric_anti_product_g0[3]), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g1.xyz * float3(float2(this.e1234), this.e1234)), (this.scalar * geometric_anti_product_g0[3]) + (this.e1234 * geometric_anti_product_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public Line geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        float3 geometric_anti_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        return Line(
            /* e41, e42, e43 */
            geometric_anti_product_g0 * float3(this.e1234), 
            /* e23, e31, e12 */
            (geometric_anti_product_g0 * float3(this.scalar)) - (float3(other_g0 * this.e1234) * other.group1.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public Motor geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            geometric_anti_product_g0 * float4(this.e1234), 
            /* e23, e31, e12, scalar */
            (geometric_anti_product_g0 * float4(this.scalar)) + float4(-(float3(other_g0 * this.e1234) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float3 geometric_anti_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[0] * this.e1234) + (geometric_anti_product_g0[1] * this.scalar), geometric_anti_product_g0[1] * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product_g4.xyz) + (float3(this.e1234) * geometric_anti_product_g1.xyz)), this.e1234 * geometric_anti_product_g1[3]), 
            /* e41, e42, e43 */
            geometric_anti_product_g2 * float3(this.e1234), 
            /* e23, e31, e12 */
            (geometric_anti_product_g2 * float3(this.scalar)) - (float3(other_g0 * this.e1234) * other.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g4.xyz * float3(float2(this.e1234), this.e1234)), (this.scalar * geometric_anti_product_g1[3]) + (this.e1234 * geometric_anti_product_g4[3]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_anti_quotient(Origin other) {
        float geometric_anti_product_g0 = -1.0 / other.e4;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_anti_product_g0 * this.e1234), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product_g0 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(1.0, 1.0, this.scalar, 0.0) * float4((geometric_anti_product_g0.xyz * float3(float2(this.scalar), 1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            geometric_anti_product_g0 * float4(this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_anti_quotient(Point other) {
        float4 geometric_anti_product_g0 = float4((other.group0.xyz * float3(-1.0) /  (float4(other.e4).xyz * float4(other.e4).xyz)), -1.0 / other.e4);
        return Flector(
            /* e1, e2, e3, e4 */
            geometric_anti_product_g0 * float4(this.e1234), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * geometric_anti_product_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Flector> operator /(Flector lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Flector geometric_anti_quotient(AntiScalar other) {
        float geometric_anti_product_g0 = 1.0 / other.e1234;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_g0) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Flector geometric_anti_quotient(DualNum other) {
        float geometric_anti_product_g0_x = other.scalar /  (other.e1234 * other.e1234);
        float geometric_anti_product_g0_y = 1.0 / other.e1234;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0_y) * this.group0.xyz) - (float3(geometric_anti_product_g0_x) * this.group1.xyz)), geometric_anti_product_g0_y * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product_g0_y) * this.group1.xyz), (geometric_anti_product_g0_y * this.e321) - (geometric_anti_product_g0_x * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Motor geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0, 0.0, -(geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[2] * this.e4), 0.0) + (geometric_anti_product_g1.yzxx * this.group1.zxyx) + float4((-(float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412)) - (float2(this.e431, this.e4) * geometric_anti_product_g1.zy)), 0.0, 0.0) - (float4(geometric_anti_product_g0[3]) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(geometric_anti_product_g1[3]) * float4(this.group1.xyz, this.e4)) + (geometric_anti_product_g1.yzxz * this.group0.zxyz) + float4(((float2(geometric_anti_product_g0[0]) * float2(this.e4, this.e412)) + (float2(this.e431, this.e4) * geometric_anti_product_g0.zy) - (float2(geometric_anti_product_g1[0]) * float2(this.e321, this.e3)) - (float2(this.e2, this.e321) * geometric_anti_product_g1.zy)), (geometric_anti_product_g0[1] * this.e423) + (geometric_anti_product_g0[2] * this.e4) - (geometric_anti_product_g1[1] * this.e1) - (geometric_anti_product_g1[2] * this.e321), (geometric_anti_product_g1[0] * this.e1) + (geometric_anti_product_g1[1] * this.e2) - (geometric_anti_product_g0[2] * this.e412) - (geometric_anti_product_g0[3] * this.e321)) - (geometric_anti_product_g0.yzxx * this.group1.zxyx) - (geometric_anti_product_g0.wwwy * float4(this.group0.xyz, this.e431))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Flector geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        float3 geometric_anti_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_anti_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e2, this.e321, this.e321) * geometric_anti_product_g0.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g0.xxy) + (geometric_anti_product_g1.yzx * this.group1.zxy) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1.zyz) - (geometric_anti_product_g0.yzx * this.group0.zxy)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.e4, this.e412, this.e423) * geometric_anti_product_g0.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g0.zyz) - (geometric_anti_product_g0.yzx * this.group1.zxy)), (geometric_anti_product_g1[0] * this.e423) + (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g0[0] * this.e1) - (geometric_anti_product_g0[1] * this.e2) - (geometric_anti_product_g0[2] * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Flector geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e2) + (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g0[1] * this.e3) - (geometric_anti_product_g1[2] * this.e431), (geometric_anti_product_g0[1] * this.e321) + (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g0[2] * this.e1) - (geometric_anti_product_g1[1] * this.e4), (geometric_anti_product_g0[1] * this.e1) + (geometric_anti_product_g0[2] * this.e321) + (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g0[0] * this.e2) - (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[2] * this.e4)) + (float3(geometric_anti_product_g0[3]) * this.group0.xyz) + float3(((float2(geometric_anti_product_g0[0]) * float2(this.e321, this.e3)) - (float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412))), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group1.xyz)), geometric_anti_product_g0[3] * this.e4), 
            /* e423, e431, e412, e321 */
            (this.group1.xyzz * float4(float3(geometric_anti_product_g0[3]), geometric_anti_product_g1[2])) + float4(((float2(geometric_anti_product_g0[0]) * float2(this.e4, this.e412)) + (float2(this.e431, this.e4) * geometric_anti_product_g0.zy)), (geometric_anti_product_g0[1] * this.e423) + (geometric_anti_product_g0[2] * this.e4), (geometric_anti_product_g1[0] * this.e423) + (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g0[1] * this.e2) - (geometric_anti_product_g0[2] * this.e3) - (geometric_anti_product_g1[3] * this.e4)) - (geometric_anti_product_g0.yzxx * float4(this.group1.zxy, this.e1))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float3 geometric_anti_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_anti_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g4[3] * this.e4) - (geometric_anti_product_g1[0] * this.e423) - (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g1[2] * this.e412) - (geometric_anti_product_g1[3] * this.e321), geometric_anti_product_g1[3] * this.e4 * -1.0) + (float2(geometric_anti_product_g4[0]) * float2(this.e1, this.e423)) + (float2(geometric_anti_product_g4[1]) * float2(this.e2, this.e431)) + (float2(geometric_anti_product_g4[2]) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0[1]) * this.group0.xyz) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (geometric_anti_product_g3.yzx * this.group1.zxy) - (float3(geometric_anti_product_g0[0]) * this.group1.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (geometric_anti_product_g2.yzx * this.group0.zxy)), geometric_anti_product_g0[1] * this.e4), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[1] * this.e412) - (geometric_anti_product_g4[2] * this.e431), (geometric_anti_product_g4[2] * this.e423) - (geometric_anti_product_g4[1] * this.e4), (geometric_anti_product_g4[0] * this.e431) - (geometric_anti_product_g4[1] * this.e423) - (geometric_anti_product_g4[2] * this.e4)) + float3(-(float2(geometric_anti_product_g4[0]) * float2(this.e4, this.e412)), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g1[2] * this.e431) + (geometric_anti_product_g4[1] * this.e3) - (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g4[2] * this.e2), (geometric_anti_product_g1[1] * this.e4) + (geometric_anti_product_g4[2] * this.e1) - (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g4[1] * this.e321), (geometric_anti_product_g1[1] * this.e423) + (geometric_anti_product_g1[2] * this.e4) + (geometric_anti_product_g4[0] * this.e2) - (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g4[1] * this.e1) - (geometric_anti_product_g4[2] * this.e321)) + (float3(geometric_anti_product_g4[3]) * this.group1.xyz) + float3(((float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412)) - (float2(geometric_anti_product_g4[0]) * float2(this.e321, this.e3))), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product_g0[1]) * this.group1) + float4(((float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz) - (geometric_anti_product_g2.yzx * this.group1.zxy)), (geometric_anti_product_g3[0] * this.e423) + (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g0[0] * this.e4) - (geometric_anti_product_g2[0] * this.e1) - (geometric_anti_product_g2[1] * this.e2) - (geometric_anti_product_g2[2] * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Motor geometric_anti_quotient(Origin other) {
        float geometric_anti_product_g0 = -1.0 / other.e4;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product_g0 * -1.0) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product_g0 * -1.0) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Motor geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3((geometric_anti_product_g0[1] * this.e412) - (geometric_anti_product_g0[2] * this.e431), (geometric_anti_product_g0[2] * this.e423) - (geometric_anti_product_g0[1] * this.e4), (geometric_anti_product_g0[0] * this.e431) - (geometric_anti_product_g0[1] * this.e423) - (geometric_anti_product_g0[2] * this.e4)) + float3(-(float2(geometric_anti_product_g0[0]) * float2(this.e4, this.e412)), 0.0)), geometric_anti_product_g0[0] * this.e423), 
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, -(geometric_anti_product_g0[1] * this.e1) - (geometric_anti_product_g0[2] * this.e321), 0.0) + (geometric_anti_product_g0.yzxx * this.group0.zxyx) + (geometric_anti_product_g0.wwwy * float4(this.group1.xyz, this.e2)) + float4((-(float2(geometric_anti_product_g0[0]) * float2(this.e321, this.e3)) - (float2(this.e2, this.e321) * geometric_anti_product_g0.zy)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Flector {
    public Motor geometric_anti_quotient(Point other) {
        float3 geometric_anti_product_g0_xyz = float3(-1.0 /  (other.e4 * other.e4)) * other.group0.xyz;
        float geometric_anti_product_g0_w = -1.0 / other.e4;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product_g0_w * -1.0) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4, this.e412, this.e423) * geometric_anti_product_g0_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g0_xyz.zyz) - (float3(geometric_anti_product_g0_w) * this.group0.xyz) - (geometric_anti_product_g0_xyz.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Horizon> operator /(Horizon lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Horizon geometric_anti_quotient(AntiScalar other) {
        return Horizon(
            /* e321 */
            this.e321 / other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Horizon geometric_anti_quotient(DualNum other) {
        return Horizon(
            /* e321 */
            this.e321 / other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Motor geometric_anti_quotient(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4((-(float3(this.e321) * float3(pow(other.e423, 3), pow(other.e431, 3), pow(other.e412, 3))) - (float3(this.e321) * float3(other.e431 * other.e431 * other.e423, other.e423 * other.e423 * other.e431, other.e423 * other.e423 * other.e412)) - (float3(this.e321) * float3(other.e412 * other.e412 * other.e423, other.e412 * other.e412 * other.e431, other.e431 * other.e431 * other.e412)) - (float3(other.e4 * other.e4 * this.e321) * other.group1.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Point geometric_anti_quotient(Line other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((-(float3_pow(other.group0.xyz, 3) * float3(this.e321)) - (float3_pow(other.group0.yxx, 2) * float3(this.e321) * other.group0.xyz) - (float3_pow(other.group0.zzy, 2) * float3(this.e321) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Flector geometric_anti_quotient(Motor other) {
        float4 geometric_anti_product_g0 = float4(0.0, 0.0, -pow(other.e43, 3) - (other.e41 * other.e41 * other.e43) - (other.e42 * other.e42 * other.e43), 0.0) + float4((float3((-float2_pow(other.group0.xy, 3) - (float2(other.e43 * other.e43) * other.group0.xy) - (float2_pow(other.group0.yx, 2) * other.group0.xy)), 0.0) - (float3(other.e1234 * other.e1234) * other.group0.xyz)), 0.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_anti_product_g0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product_g0[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        return MultiVector(
            /* scalar, e1234 */
            float2(other_g0 * other.e4 * this.e321, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other_g0 * this.e321 * -1.0) * other.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other_g0 * this.e321 * -1.0) * other.group4.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other_g0 * other.e1234 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Scalar geometric_anti_quotient(Origin other) {
        return Scalar(
            /* scalar */
            this.e321 / other.e4
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Line geometric_anti_quotient(Plane other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            -(float3(this.e321) * float3(pow(other.e423, 3), pow(other.e431, 3), pow(other.e412, 3))) - (float3(this.e321) * float3(other.e431 * other.e431 * other.e423, other.e423 * other.e423 * other.e431, other.e423 * other.e423 * other.e412)) - (float3(this.e321) * float3(other.e412 * other.e412 * other.e423, other.e412 * other.e412 * other.e431, other.e431 * other.e431 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiQuotientInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Horizon {
    public Scalar geometric_anti_quotient(Point other) {
        return Scalar(
            /* scalar */
            this.e321 / other.e4
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Line> operator /(Line lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiQuotientInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Line geometric_anti_quotient(AntiScalar other) {
        float geometric_anti_product_g0 = 1.0 / other.e1234;
        return Line(
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product_g0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiQuotientInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Line geometric_anti_quotient(DualNum other) {
        float geometric_anti_product_g0_y = 1.0 / other.e1234;
        return Line(
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0_y) * this.group0.xyz, 
            /* e23, e31, e12 */
            (float3(geometric_anti_product_g0_y) * this.group1.xyz) + (float3(other.scalar /  (other.e1234 * other.e1234)) * this.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Flector geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((this.group0.xyx * float3(float2(geometric_anti_product_g1[3]), geometric_anti_product_g0[1])) + (this.group0.yzz * float3(geometric_anti_product_g0.zx, geometric_anti_product_g1[3])) + (this.group1.xyx * float3(float2(geometric_anti_product_g0[3]), geometric_anti_product_g1[1])) + (this.group1.yzz * float3(geometric_anti_product_g1.zx, geometric_anti_product_g0[3])) - (this.group0.zxy * geometric_anti_product_g0.yzx) - (this.group1.zxy * geometric_anti_product_g1.yzx)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((this.group0.xyx * float3(float2(geometric_anti_product_g0[3]), geometric_anti_product_g1[1])) + (this.group0.yzz * float3(geometric_anti_product_g1.zx, geometric_anti_product_g0[3])) - (this.group0.zxy * geometric_anti_product_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Motor geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        float3 geometric_anti_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_anti_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((geometric_anti_product_g0.zxy * this.group0.yzx) - (geometric_anti_product_g0.yzx * this.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((geometric_anti_product_g0.zxy * this.group1.yzx) + (geometric_anti_product_g1.zxy * this.group0.yzx) - (geometric_anti_product_g0.yzx * this.group1.zxy) - (geometric_anti_product_g1.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Motor geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((this.group0.xyx * geometric_anti_product_g0.wwy) + (this.group0.yzz * geometric_anti_product_g0.zxw) - (this.group0.zxy * geometric_anti_product_g0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.xyx * geometric_anti_product_g1.wwy) + (this.group0.yzz * geometric_anti_product_g1.zxw) + (this.group1.xyx * geometric_anti_product_g0.wwy) + (this.group1.yzz * geometric_anti_product_g0.zxw) - (this.group0.zxy * geometric_anti_product_g1.yzx) - (this.group1.zxy * geometric_anti_product_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float3 geometric_anti_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_anti_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(-(geometric_anti_product_g3[0] * this.e41) - (geometric_anti_product_g3[1] * this.e42) - (geometric_anti_product_g3[2] * this.e43), 0.0) - (float2(geometric_anti_product_g2[0]) * float2(this.e23, this.e41)) - (float2(geometric_anti_product_g2[1]) * float2(this.e31, this.e42)) - (float2(geometric_anti_product_g2[2]) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4(((this.group0.xyx * float3(float2(geometric_anti_product_g4[3]), geometric_anti_product_g1[1])) + (this.group0.yzz * float3(geometric_anti_product_g1.zx, geometric_anti_product_g4[3])) + (this.group1.xyx * float3(float2(geometric_anti_product_g1[3]), geometric_anti_product_g4[1])) + (this.group1.yzz * float3(geometric_anti_product_g4.zx, geometric_anti_product_g1[3])) - (this.group0.zxy * geometric_anti_product_g1.yzx) - (this.group1.zxy * geometric_anti_product_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product_g0[1]) * this.group0.xyz) + (geometric_anti_product_g2.zxy * this.group0.yzx) - (geometric_anti_product_g2.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product_g0[0]) * this.group0.xyz) + (float3(geometric_anti_product_g0[1]) * this.group1.xyz) + (geometric_anti_product_g2.zxy * this.group1.yzx) + (geometric_anti_product_g3.zxy * this.group0.yzx) - (geometric_anti_product_g2.yzx * this.group1.zxy) - (geometric_anti_product_g3.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(((this.group0.xyx * float3(float2(geometric_anti_product_g1[3]), geometric_anti_product_g4[1])) + (this.group0.yzz * float3(geometric_anti_product_g4.zx, geometric_anti_product_g1[3])) - (this.group0.zxy * geometric_anti_product_g4.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Flector geometric_anti_quotient(Origin other) {
        float geometric_anti_product_g0 = -1.0 / other.e4;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_anti_product_g0) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product_g0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Flector geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0[3]) * this.group0.xyz) + (this.group1.yzx * geometric_anti_product_g0.zxy) - (this.group1.zxy * geometric_anti_product_g0.yzx)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((this.group0.yzx * geometric_anti_product_g0.zxy) - (this.group0.zxy * geometric_anti_product_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Line {
    public Flector geometric_anti_quotient(Point other) {
        float3 geometric_anti_product_g0_xyz = float3(-1.0 /  (other.e4 * other.e4)) * other.group0.xyz;
        float geometric_anti_product_g0_w = -1.0 / other.e4;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0_w) * this.group1.xyz) + (geometric_anti_product_g0_xyz.zxy * this.group0.yzx) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product_g0_w) * this.group0.xyz), -(geometric_anti_product_g0_xyz[0] * this.e41) - (geometric_anti_product_g0_xyz[1] * this.e42) - (geometric_anti_product_g0_xyz[2] * this.e43))
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Motor> operator /(Motor lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Motor geometric_anti_quotient(AntiScalar other) {
        float geometric_anti_product_g0 = 1.0 / other.e1234;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product_g0) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Motor geometric_anti_quotient(DualNum other) {
        float geometric_anti_product_g0_y = 1.0 / other.e1234;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product_g0_y) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(geometric_anti_product_g0_y) * this.group1) + (float4(other.scalar /  (other.e1234 * other.e1234)) * this.group0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Flector geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e42) + (geometric_anti_product_g1[2] * this.e31) - (geometric_anti_product_g0[1] * this.e43) - (geometric_anti_product_g1[1] * this.e12), (geometric_anti_product_g0[1] * this.e1234) + (geometric_anti_product_g1[1] * this.scalar) - (geometric_anti_product_g0[2] * this.e41) - (geometric_anti_product_g1[2] * this.e23), (geometric_anti_product_g0[1] * this.e41) + (geometric_anti_product_g0[2] * this.e1234) + (geometric_anti_product_g1[1] * this.e23) + (geometric_anti_product_g1[2] * this.scalar) - (geometric_anti_product_g0[0] * this.e42) - (geometric_anti_product_g1[0] * this.e31)) + (float3(geometric_anti_product_g0[3]) * this.group1.xyz) + (float3(geometric_anti_product_g1[3]) * this.group0.xyz) + float3(((float2(geometric_anti_product_g0[0]) * this.group0.wz) + (float2(geometric_anti_product_g1[0]) * this.group1.wz)), 0.0)), geometric_anti_product_g0[3] * this.e1234), 
            /* e423, e431, e412, e321 */
            float4(0.0, 0.0, (geometric_anti_product_g1[2] * this.e1234) - (geometric_anti_product_g1[0] * this.e42), 0.0) + (float4(geometric_anti_product_g0[3]) * float4(this.group0.xyz, this.scalar)) + (geometric_anti_product_g1.xxyw * this.group0.wzxw) + float4(((geometric_anti_product_g1.zy * this.group0.yw) - (geometric_anti_product_g1.yz * this.group0.zx)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Motor geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        float3 geometric_anti_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_anti_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((geometric_anti_product_g0.xxy * this.group0.wzx) + (geometric_anti_product_g0.zyz * this.group0.yww) - (geometric_anti_product_g0.yzx * this.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((geometric_anti_product_g0.xxy * this.group1.wzx) + (geometric_anti_product_g0.zyz * this.group1.yww) + (geometric_anti_product_g1.xxy * this.group0.wzx) + (geometric_anti_product_g1.zyz * this.group0.yww) - (geometric_anti_product_g0.yzx * this.group1.zxy) - (geometric_anti_product_g1.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Motor geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3((geometric_anti_product_g0[2] * this.e42) - (geometric_anti_product_g0[1] * this.e43), (geometric_anti_product_g0[1] * this.e1234) - (geometric_anti_product_g0[2] * this.e41), (geometric_anti_product_g0[1] * this.e41) + (geometric_anti_product_g0[2] * this.e1234) - (geometric_anti_product_g0[0] * this.e42)) + (float3(geometric_anti_product_g0[3]) * this.group0.xyz) + float3((float2(geometric_anti_product_g0[0]) * this.group0.wz), 0.0)), geometric_anti_product_g0[3] * this.e1234), 
            /* e23, e31, e12, scalar */
            (geometric_anti_product_g0.xxyw * this.group1.wzxw) + float4(((float3(geometric_anti_product_g0[3]) * this.group1.xyz) + (float3(geometric_anti_product_g1[3]) * this.group0.xyz) + float3(((float2(geometric_anti_product_g1[0]) * this.group0.wz) + (geometric_anti_product_g0.zy * this.group1.yw) + (geometric_anti_product_g1.zy * this.group0.yw) - (geometric_anti_product_g0.yz * this.group1.zx) - (geometric_anti_product_g1.yz * this.group0.zx)), (geometric_anti_product_g0[2] * this.scalar) + (geometric_anti_product_g1[1] * this.e41) + (geometric_anti_product_g1[2] * this.e1234) - (geometric_anti_product_g0[0] * this.e31) - (geometric_anti_product_g1[0] * this.e42))), geometric_anti_product_g1[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float3 geometric_anti_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_anti_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[1] * this.scalar) - (geometric_anti_product_g2[2] * this.e12) - (geometric_anti_product_g3[0] * this.e41) - (geometric_anti_product_g3[1] * this.e42) - (geometric_anti_product_g3[2] * this.e43), geometric_anti_product_g2[0] * this.e41 * -1.0) + (geometric_anti_product_g0 * float2(this.e1234)) - (float2(this.e23, this.e42) * geometric_anti_product_g2.xy) - (float2(this.e31, this.e43) * geometric_anti_product_g2.yz), 
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g1[2] * this.e42) + (geometric_anti_product_g4[2] * this.e31) - (geometric_anti_product_g1[1] * this.e43) - (geometric_anti_product_g4[1] * this.e12), (geometric_anti_product_g1[1] * this.e1234) + (geometric_anti_product_g4[1] * this.scalar) - (geometric_anti_product_g1[2] * this.e41) - (geometric_anti_product_g4[2] * this.e23), (geometric_anti_product_g1[1] * this.e41) + (geometric_anti_product_g1[2] * this.e1234) + (geometric_anti_product_g4[1] * this.e23) + (geometric_anti_product_g4[2] * this.scalar) - (geometric_anti_product_g1[0] * this.e42) - (geometric_anti_product_g4[0] * this.e31)) + (float3(geometric_anti_product_g1[3]) * this.group1.xyz) + (float3(geometric_anti_product_g4[3]) * this.group0.xyz) + float3(((float2(geometric_anti_product_g1[0]) * this.group0.wz) + (float2(geometric_anti_product_g4[0]) * this.group1.wz)), 0.0)), geometric_anti_product_g1[3] * this.e1234), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product_g0[1]) * this.group0.xyz) + (geometric_anti_product_g2.xxy * this.group0.wzx) + (geometric_anti_product_g2.zyz * this.group0.yww) - (geometric_anti_product_g2.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product_g0[0]) * this.group0.xyz) + (float3(geometric_anti_product_g0[1]) * this.group1.xyz) + (geometric_anti_product_g2.xxy * this.group1.wzx) + (geometric_anti_product_g2.zyz * this.group1.yww) + (geometric_anti_product_g3.xxy * this.group0.wzx) + (geometric_anti_product_g3.zyz * this.group0.yww) - (geometric_anti_product_g2.yzx * this.group1.zxy) - (geometric_anti_product_g3.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(0.0, 0.0, (geometric_anti_product_g4[2] * this.e1234) - (geometric_anti_product_g4[0] * this.e42), 0.0) + (float4(geometric_anti_product_g1[3]) * float4(this.group0.xyz, this.scalar)) + (geometric_anti_product_g4.xxyw * this.group0.wzxw) + float4(((geometric_anti_product_g4.zy * this.group0.yw) - (geometric_anti_product_g4.yz * this.group0.zx)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Flector geometric_anti_quotient(Origin other) {
        float geometric_anti_product_g0 = -1.0 / other.e4;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_g0) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product_g0) * float4(this.group0.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Flector geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, (geometric_anti_product_g0[1] * this.e23) + (geometric_anti_product_g0[2] * this.scalar) - (geometric_anti_product_g0[0] * this.e31), 0.0) + float4(((float3(geometric_anti_product_g0[3]) * this.group0.xyz) + float3(((float2(geometric_anti_product_g0[0]) * this.group1.wz) + (geometric_anti_product_g0.zy * this.group1.yw) - (geometric_anti_product_g0.yz * this.group1.zx)), 0.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3((geometric_anti_product_g0[2] * this.e42) - (geometric_anti_product_g0[1] * this.e43), (geometric_anti_product_g0[1] * this.e1234) - (geometric_anti_product_g0[2] * this.e41), (geometric_anti_product_g0[1] * this.e41) + (geometric_anti_product_g0[2] * this.e1234) - (geometric_anti_product_g0[0] * this.e42)) + float3((float2(geometric_anti_product_g0[0]) * this.group0.wz), 0.0)), geometric_anti_product_g0[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Motor {
    public Flector geometric_anti_quotient(Point other) {
        float3 geometric_anti_product_g0_xyz = float3(-1.0 /  (other.e4 * other.e4)) * other.group0.xyz;
        float geometric_anti_product_g0_w = -1.0 / other.e4;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0_w) * this.group1.xyz) + (geometric_anti_product_g0_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g0_xyz.zyz * this.group0.yww) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy)), geometric_anti_product_g0_w * this.e1234), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product_g0_w) * this.group0.xyz), (geometric_anti_product_g0_w * this.scalar) - (geometric_anti_product_g0_xyz[0] * this.e41) - (geometric_anti_product_g0_xyz[1] * this.e42) - (geometric_anti_product_g0_xyz[2] * this.e43))
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<MultiVector> operator /(MultiVector lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(AntiScalar other) {
        float geometric_anti_product_g0 = 1.0 / other.e1234;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_anti_product_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_g0) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product_g0) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product_g0) * this.group4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(DualNum other) {
        float geometric_anti_product_g0_x = other.scalar /  (other.e1234 * other.e1234);
        float geometric_anti_product_g0_y = 1.0 / other.e1234;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0_x * this.e1234) + (geometric_anti_product_g0_y * this.scalar), geometric_anti_product_g0_y * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0_y) * this.group1.xyz) - (float3(geometric_anti_product_g0_x) * this.group4.xyz)), geometric_anti_product_g0_y * this.e4), 
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0_y) * this.group2.xyz, 
            /* e23, e31, e12 */
            (float3(geometric_anti_product_g0_x) * this.group2.xyz) + (float3(geometric_anti_product_g0_y) * this.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product_g0_y) * this.group4.xyz), (geometric_anti_product_g0_y * this.e321) - (geometric_anti_product_g0_x * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1[3] * this.e4) - (geometric_anti_product_g0[0] * this.e423) - (geometric_anti_product_g0[1] * this.e431) - (geometric_anti_product_g0[2] * this.e412) - (geometric_anti_product_g0[3] * this.e321), geometric_anti_product_g0[3] * this.e4 * -1.0) + (float2(geometric_anti_product_g1[0]) * float2(this.e1, this.e423)) + (float2(geometric_anti_product_g1[1]) * float2(this.e2, this.e431)) + (float2(geometric_anti_product_g1[2]) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product_g1.xyz) + (float3(this.e1234) * geometric_anti_product_g0.xyz) + (this.group2.xyx * float3(float2(geometric_anti_product_g1[3]), geometric_anti_product_g0[1])) + (this.group2.yzz * float3(geometric_anti_product_g0.zx, geometric_anti_product_g1[3])) + (this.group3.xyx * float3(float2(geometric_anti_product_g0[3]), geometric_anti_product_g1[1])) + (this.group3.yzz * float3(geometric_anti_product_g1.zx, geometric_anti_product_g0[3])) - (this.group2.zxy * geometric_anti_product_g0.yzx) - (this.group3.zxy * geometric_anti_product_g1.yzx)), this.e1234 * geometric_anti_product_g0[3]), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g1[2] * this.e431), (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g1[1] * this.e4), (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[2] * this.e4)) + float3(-(float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412)), 0.0) - (float3(geometric_anti_product_g0[3]) * this.group4.xyz), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g0[2] * this.e431) + (geometric_anti_product_g1[1] * this.e3) - (geometric_anti_product_g0[1] * this.e412) - (geometric_anti_product_g1[2] * this.e2), (geometric_anti_product_g0[1] * this.e4) + (geometric_anti_product_g1[2] * this.e1) - (geometric_anti_product_g0[2] * this.e423) - (geometric_anti_product_g1[1] * this.e321), (geometric_anti_product_g0[1] * this.e423) + (geometric_anti_product_g0[2] * this.e4) + (geometric_anti_product_g1[0] * this.e2) - (geometric_anti_product_g0[0] * this.e431) - (geometric_anti_product_g1[1] * this.e1) - (geometric_anti_product_g1[2] * this.e321)) + (float3(geometric_anti_product_g1[3]) * this.group4.xyz) + float3(((float2(geometric_anti_product_g0[0]) * float2(this.e4, this.e412)) - (float2(geometric_anti_product_g1[0]) * float2(this.e321, this.e3))), 0.0) - (float3(geometric_anti_product_g0[3]) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.e1234) * geometric_anti_product_g1.xyz) + (this.group2.xyx * float3(float2(geometric_anti_product_g0[3]), geometric_anti_product_g1[1])) + (this.group2.yzz * float3(geometric_anti_product_g1.zx, geometric_anti_product_g0[3])) - (this.group2.zxy * geometric_anti_product_g1.yzx)), (this.scalar * geometric_anti_product_g0[3]) + (this.e1234 * geometric_anti_product_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        float3 geometric_anti_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_anti_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(-(geometric_anti_product_g1[0] * this.e41) - (geometric_anti_product_g1[1] * this.e42) - (geometric_anti_product_g1[2] * this.e43), 0.0) - (float2(geometric_anti_product_g0[0]) * float2(this.e23, this.e41)) - (float2(geometric_anti_product_g0[1]) * float2(this.e31, this.e42)) - (float2(geometric_anti_product_g0[2]) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e2, this.e321, this.e321) * geometric_anti_product_g0.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g0.xxy) + (geometric_anti_product_g1.yzx * this.group4.zxy) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1.zyz) - (geometric_anti_product_g0.yzx * this.group1.zxy)), 0.0), 
            /* e41, e42, e43 */
            (geometric_anti_product_g0 * float3(this.e1234)) + (geometric_anti_product_g0.zxy * this.group2.yzx) - (geometric_anti_product_g0.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (geometric_anti_product_g0 * float3(this.scalar)) + (geometric_anti_product_g1 * float3(this.e1234)) + (geometric_anti_product_g0.zxy * this.group3.yzx) + (geometric_anti_product_g1.zxy * this.group2.yzx) - (geometric_anti_product_g0.yzx * this.group3.zxy) - (geometric_anti_product_g1.yzx * this.group2.zxy), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.e4, this.e412, this.e423) * geometric_anti_product_g0.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g0.zyz) - (geometric_anti_product_g0.yzx * this.group4.zxy)), (geometric_anti_product_g1[0] * this.e423) + (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g0[0] * this.e1) - (geometric_anti_product_g0[1] * this.e2) - (geometric_anti_product_g0[2] * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((this.e1234 * geometric_anti_product_g1[3]) - (this.e43 * geometric_anti_product_g1[2]) - (this.e23 * geometric_anti_product_g0[0]) - (this.e31 * geometric_anti_product_g0[1]) - (this.e12 * geometric_anti_product_g0[2]), this.e41 * geometric_anti_product_g0[0] * -1.0) + (float2(geometric_anti_product_g0[3]) * this.group0.xy) - (float2(geometric_anti_product_g1[0], geometric_anti_product_g0[1]) * this.group2.xyz.xy) - (float2(geometric_anti_product_g1[1], geometric_anti_product_g0[2]) * this.group2.xyz.yz), 
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e2) + (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g0[1] * this.e3) - (geometric_anti_product_g1[2] * this.e431), (geometric_anti_product_g0[1] * this.e321) + (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g0[2] * this.e1) - (geometric_anti_product_g1[1] * this.e4), (geometric_anti_product_g0[1] * this.e1) + (geometric_anti_product_g0[2] * this.e321) + (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g0[0] * this.e2) - (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[2] * this.e4)) + (float3(geometric_anti_product_g0[3]) * this.group1.xyz) + float3(((float2(geometric_anti_product_g0[0]) * float2(this.e321, this.e3)) - (float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412))), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group4.xyz)), geometric_anti_product_g0[3] * this.e4), 
            /* e41, e42, e43 */
            (float3(this.e1234) * geometric_anti_product_g0.xyz) + (this.group2.xyx * geometric_anti_product_g0.wwy) + (this.group2.yzz * geometric_anti_product_g0.zxw) - (this.group2.zxy * geometric_anti_product_g0.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_anti_product_g0.xyz) + (float3(this.e1234) * geometric_anti_product_g1.xyz) + (this.group2.xyx * geometric_anti_product_g1.wwy) + (this.group2.yzz * geometric_anti_product_g1.zxw) + (this.group3.xyx * geometric_anti_product_g0.wwy) + (this.group3.yzz * geometric_anti_product_g0.zxw) - (this.group2.zxy * geometric_anti_product_g1.yzx) - (this.group3.zxy * geometric_anti_product_g0.yzx), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * geometric_anti_product_g0.xxyw) + (this.group4.xyzy * float4(float3(geometric_anti_product_g0[3]), geometric_anti_product_g1[1])) + float4((float2(this.e431, this.e4) * geometric_anti_product_g0.zy), geometric_anti_product_g0[2] * this.e4, (geometric_anti_product_g1[0] * this.e423) - (geometric_anti_product_g0[1] * this.e2) - (geometric_anti_product_g0[2] * this.e3) - (geometric_anti_product_g1[3] * this.e4)) - (geometric_anti_product_g0.yzxx * float4(this.group4.zxy, this.e1))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float3 geometric_anti_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_anti_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[1] * this.scalar) + (geometric_anti_product_g4[2] * this.e3) + (geometric_anti_product_g4[3] * this.e4) - (geometric_anti_product_g2[1] * this.e31) - (geometric_anti_product_g2[2] * this.e12) - (geometric_anti_product_g3[0] * this.e41) - (geometric_anti_product_g3[1] * this.e42) - (geometric_anti_product_g3[2] * this.e43) - (geometric_anti_product_g1[0] * this.e423) - (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g1[2] * this.e412) - (geometric_anti_product_g1[3] * this.e321), (geometric_anti_product_g4[0] * this.e423) - (geometric_anti_product_g2[0] * this.e41) - (geometric_anti_product_g2[1] * this.e42) - (geometric_anti_product_g1[3] * this.e4)) + (geometric_anti_product_g0 * float2(this.e1234)) + (float2(this.e1, this.e431) * geometric_anti_product_g4.xy) + (float2(this.e2, this.e412) * geometric_anti_product_g4.yz) - (float2(this.e23, this.e43) * geometric_anti_product_g2.xz), 
            /* e1, e2, e3, e4 */
            (float4(geometric_anti_product_g0[1]) * this.group1) + float4(((float3(this.scalar) * geometric_anti_product_g4.xyz) + (float3(this.e1234) * geometric_anti_product_g1.xyz) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (geometric_anti_product_g3.yzx * this.group4.zxy) + (this.group2.xyx * float3(float2(geometric_anti_product_g4[3]), geometric_anti_product_g1[1])) + (this.group2.yzz * float3(geometric_anti_product_g1.zx, geometric_anti_product_g4[3])) + (this.group3.xyx * float3(float2(geometric_anti_product_g1[3]), geometric_anti_product_g4[1])) + (this.group3.yzz * float3(geometric_anti_product_g4.zx, geometric_anti_product_g1[3])) - (float3(geometric_anti_product_g0[0]) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (geometric_anti_product_g2.yzx * this.group1.zxy) - (this.group2.zxy * geometric_anti_product_g1.yzx) - (this.group3.zxy * geometric_anti_product_g4.yzx)), this.e1234 * geometric_anti_product_g1[3]), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[1] * this.e412) - (geometric_anti_product_g4[2] * this.e431), (geometric_anti_product_g4[2] * this.e423) - (geometric_anti_product_g4[1] * this.e4), (geometric_anti_product_g4[0] * this.e431) - (geometric_anti_product_g4[1] * this.e423) - (geometric_anti_product_g4[2] * this.e4)) + (geometric_anti_product_g2 * float3(this.e1234)) + (float3(geometric_anti_product_g0[1]) * this.group2.xyz) + (geometric_anti_product_g2.zxy * this.group2.yzx) + float3(-(float2(geometric_anti_product_g4[0]) * float2(this.e4, this.e412)), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group4.xyz) - (geometric_anti_product_g2.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g1[2] * this.e431) + (geometric_anti_product_g4[1] * this.e3) - (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g4[2] * this.e2), (geometric_anti_product_g1[1] * this.e4) + (geometric_anti_product_g4[2] * this.e1) - (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g4[1] * this.e321), (geometric_anti_product_g1[1] * this.e423) + (geometric_anti_product_g1[2] * this.e4) + (geometric_anti_product_g4[0] * this.e2) - (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g4[1] * this.e1) - (geometric_anti_product_g4[2] * this.e321)) + (geometric_anti_product_g2 * float3(this.scalar)) + (geometric_anti_product_g3 * float3(this.e1234)) + (float3(geometric_anti_product_g0[0]) * this.group2.xyz) + (float3(geometric_anti_product_g0[1]) * this.group3.xyz) + (float3(geometric_anti_product_g4[3]) * this.group4.xyz) + (geometric_anti_product_g2.zxy * this.group3.yzx) + (geometric_anti_product_g3.zxy * this.group2.yzx) + float3(((float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412)) - (float2(geometric_anti_product_g4[0]) * float2(this.e321, this.e3))), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group1.xyz) - (geometric_anti_product_g2.yzx * this.group3.zxy) - (geometric_anti_product_g3.yzx * this.group2.zxy), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product_g0[1]) * this.group4) + float4(((float3(this.e1234) * geometric_anti_product_g4.xyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz) + (this.group2.xyx * float3(float2(geometric_anti_product_g1[3]), geometric_anti_product_g4[1])) + (this.group2.yzz * float3(geometric_anti_product_g4.zx, geometric_anti_product_g1[3])) - (geometric_anti_product_g2.yzx * this.group4.zxy) - (this.group2.zxy * geometric_anti_product_g4.yzx)), (this.scalar * geometric_anti_product_g1[3]) + (this.e1234 * geometric_anti_product_g4[3]) + (geometric_anti_product_g3[0] * this.e423) + (geometric_anti_product_g3[1] * this.e431) + (geometric_anti_product_g3[2] * this.e412) - (geometric_anti_product_g0[0] * this.e4) - (geometric_anti_product_g2[0] * this.e1) - (geometric_anti_product_g2[1] * this.e2) - (geometric_anti_product_g2[2] * this.e3) - (this.e41 * geometric_anti_product_g1[0]) - (this.e42 * geometric_anti_product_g1[1]) - (this.e43 * geometric_anti_product_g1[2]) - (this.e23 * geometric_anti_product_g4[0]) - (this.e31 * geometric_anti_product_g4[1]) - (this.e12 * geometric_anti_product_g4[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Origin other) {
        float geometric_anti_product_g0 = -1.0 / other.e4;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_anti_product_g0 * -1.0) * float2(this.e321, this.e4), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_g0) * float4(this.group3.xyz, this.e1234), 
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0 * -1.0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product_g0 * -1.0) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product_g0) * float4(this.group2.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_anti_product_g0[3] * this.e4, 0.0) + (float2(geometric_anti_product_g0[0]) * float2(this.e1, this.e423)) + (float2(geometric_anti_product_g0[1]) * float2(this.e2, this.e431)) + (float2(geometric_anti_product_g0[2]) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * geometric_anti_product_g0.xyz) + (float3(geometric_anti_product_g0[3]) * this.group2.xyz) + (this.group3.yzx * geometric_anti_product_g0.zxy) - (this.group3.zxy * geometric_anti_product_g0.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g0[1] * this.e412) - (geometric_anti_product_g0[2] * this.e431), (geometric_anti_product_g0[2] * this.e423) - (geometric_anti_product_g0[1] * this.e4), (geometric_anti_product_g0[0] * this.e431) - (geometric_anti_product_g0[1] * this.e423) - (geometric_anti_product_g0[2] * this.e4)) + float3(-(float2(geometric_anti_product_g0[0]) * float2(this.e4, this.e412)), 0.0), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g0[1] * this.e3) - (geometric_anti_product_g0[2] * this.e2), (geometric_anti_product_g0[2] * this.e1) - (geometric_anti_product_g0[1] * this.e321), (geometric_anti_product_g0[0] * this.e2) - (geometric_anti_product_g0[1] * this.e1) - (geometric_anti_product_g0[2] * this.e321)) + (float3(geometric_anti_product_g0[3]) * this.group4.xyz) + float3(-(float2(geometric_anti_product_g0[0]) * float2(this.e321, this.e3)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.e1234) * geometric_anti_product_g0.xyz) + (this.group2.yzx * geometric_anti_product_g0.zxy) - (this.group2.zxy * geometric_anti_product_g0.yzx)), this.e1234 * geometric_anti_product_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_quotient(Point other) {
        float3 geometric_anti_product_g0_xyz = float3(-1.0 /  (other.e4 * other.e4)) * other.group0.xyz;
        float geometric_anti_product_g0_w = -1.0 / other.e4;
        return MultiVector(
            /* scalar, e1234 */
            float2(-(geometric_anti_product_g0_w * this.e321) - (geometric_anti_product_g0_xyz[0] * this.e423) - (geometric_anti_product_g0_xyz[1] * this.e431) - (geometric_anti_product_g0_xyz[2] * this.e412), geometric_anti_product_g0_w * this.e4 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(((geometric_anti_product_g0_xyz * float3(this.e1234)) + (float3(geometric_anti_product_g0_w) * this.group3.xyz) + (geometric_anti_product_g0_xyz.zxy * this.group2.yzx) - (geometric_anti_product_g0_xyz.yzx * this.group2.zxy)), geometric_anti_product_g0_w * this.e1234), 
            /* e41, e42, e43 */
            float3(geometric_anti_product_g0_w * -1.0) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g0_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g0_xyz.zyz) - (float3(geometric_anti_product_g0_w) * this.group1.xyz) - (geometric_anti_product_g0_xyz.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_anti_product_g0_w) * this.group2.xyz), (geometric_anti_product_g0_w * this.scalar) - (geometric_anti_product_g0_xyz[0] * this.e41) - (geometric_anti_product_g0_xyz[1] * this.e42) - (geometric_anti_product_g0_xyz[2] * this.e43))
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Origin> operator /(Origin lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public Origin geometric_anti_quotient(AntiScalar other) {
        return Origin(
            /* e4 */
            this.e4 / other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public Flector geometric_anti_quotient(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4 / other.e1234), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0 /  (other.e1234 * other.e1234))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public Motor geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * -1.0) * float4(geometric_anti_product_g1.xyz, geometric_anti_product_g0[3]), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(geometric_anti_product_g0.xyz, geometric_anti_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public Flector geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other_g0 * this.e4) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other_g0 * this.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public Flector geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4((geometric_anti_product_g1.xyz * float3(-1.0)), geometric_anti_product_g0[3]), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(geometric_anti_product_g0.xyz, geometric_anti_product_g1[3]) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e4) * float2(geometric_anti_product_g4[3], geometric_anti_product_g1[3]) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4((float3(other_g0) * other.group3.xyz), geometric_anti_product_g0[1]), 
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * geometric_anti_product_g4.xyz, 
            /* e23, e31, e12 */
            float3(this.e4) * geometric_anti_product_g1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e4 * -1.0) * float4((float3(other_g0) * other.group2.xyz), geometric_anti_product_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public AntiScalar geometric_anti_quotient(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e4 / other.e4
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public Motor geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4 * -1.0) * geometric_anti_product_g0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_anti_product_g0[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Origin {
    public Motor geometric_anti_quotient(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 / other.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4 * -1.0) * other.group0.xyz /  (float4(other.e4).xyz * float4(other.e4).xyz)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Plane> operator /(Plane lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Plane geometric_anti_quotient(AntiScalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(1.0 / other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Flector geometric_anti_quotient(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(1.0, 1.0, other.scalar /  (other.e1234 * other.e1234), 0.0) * float4((this.group0.xyz * float3(float2(other.scalar * -1.0 /  (other.e1234 * other.e1234)), 1.0) * float3(1.0, 1.0, -1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(1.0 / other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Motor geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3((geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g1[2] * this.e431), (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g1[0] * this.e412), (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g1[1] * this.e423)) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz)), geometric_anti_product_g1[0] * this.e423), 
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, (geometric_anti_product_g0[1] * this.e423) - (geometric_anti_product_g0[0] * this.e431), 0.0) + float4(((float3(geometric_anti_product_g1[3]) * this.group0.xyz) + float3(((geometric_anti_product_g0.zx * this.group0.yz) - (geometric_anti_product_g0.yz * this.group0.zx)), 0.0) - (float3(this.e321) * geometric_anti_product_g1.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Flector geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        float3 geometric_anti_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_anti_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((geometric_anti_product_g0 * float3(this.e321)) + (geometric_anti_product_g1.yzx * this.group0.zxy) - (geometric_anti_product_g1.zxy * this.group0.yzx)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((geometric_anti_product_g0.zxy * this.group0.yzx) - (geometric_anti_product_g0.yzx * this.group0.zxy)), geometric_anti_product_g1[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Flector geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g1[1] * this.e423), 0.0) + float4(((float3(this.e321) * geometric_anti_product_g0.xyz) + float3(((geometric_anti_product_g1.yz * this.group0.zx) - (geometric_anti_product_g1.zx * this.group0.yz)), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0, 0.0, geometric_anti_product_g0[0] * this.e431 * -1.0, 0.0) + (geometric_anti_product_g0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(float3(geometric_anti_product_g0[3]), geometric_anti_product_g1[0])) + float4(-(geometric_anti_product_g0.yz * this.group0.zx), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float3 geometric_anti_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_anti_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(-(geometric_anti_product_g1[0] * this.e423) - (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g1[2] * this.e412) - (geometric_anti_product_g1[3] * this.e321), (geometric_anti_product_g4[0] * this.e423) + (geometric_anti_product_g4[1] * this.e431) + (geometric_anti_product_g4[2] * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(((geometric_anti_product_g2 * float3(this.e321)) + (geometric_anti_product_g3.yzx * this.group0.zxy) - (float3(geometric_anti_product_g0[0]) * this.group0.xyz) - (geometric_anti_product_g3.zxy * this.group0.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[1] * this.e412) - (geometric_anti_product_g4[2] * this.e431), (geometric_anti_product_g4[2] * this.e423) - (geometric_anti_product_g4[0] * this.e412), (geometric_anti_product_g4[0] * this.e431) - (geometric_anti_product_g4[1] * this.e423)) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g1[2] * this.e431) - (geometric_anti_product_g1[1] * this.e412), (geometric_anti_product_g1[0] * this.e412) - (geometric_anti_product_g1[2] * this.e423), (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[0] * this.e431)) + (float3(geometric_anti_product_g4[3]) * this.group0.xyz) - (float3(this.e321) * geometric_anti_product_g4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product_g0[1]) * this.group0) + (this.group0.yzxx * float4(geometric_anti_product_g2.zxy, geometric_anti_product_g3[0])) + float4(-(geometric_anti_product_g2.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Motor geometric_anti_quotient(Origin other) {
        float geometric_anti_product_g0 = -1.0 / other.e4;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(geometric_anti_product_g0 * -1.0) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_anti_product_g0 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Motor geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((geometric_anti_product_g0.yz * this.group0.zx) - (geometric_anti_product_g0.zx * this.group0.yz)), (geometric_anti_product_g0[0] * this.e431) - (geometric_anti_product_g0[1] * this.e423), geometric_anti_product_g0[0] * this.e423), 
            /* e23, e31, e12, scalar */
            float4(((float3(geometric_anti_product_g0[3]) * this.group0.xyz) - (float3(this.e321) * geometric_anti_product_g0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Plane {
    public Motor geometric_anti_quotient(Point other) {
        float3 geometric_anti_product_g0_xyz = float3(-1.0 /  (other.e4 * other.e4)) * other.group0.xyz;
        float geometric_anti_product_g0_w = -1.0 / other.e4;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(geometric_anti_product_g0_w * -1.0) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((geometric_anti_product_g0_xyz.zxy * this.group0.yzx) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy)), geometric_anti_product_g0_w * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Point> operator /(Point lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiQuotientInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Point geometric_anti_quotient(AntiScalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(1.0 / other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Flector geometric_anti_quotient(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(1.0 / other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0 /  (other.e1234 * other.e1234))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Motor geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float4 geometric_anti_product_g0 = float4(other_g0 * -1.0) * other.group0;
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * -1.0) * float4(geometric_anti_product_g1.xyz, geometric_anti_product_g0[3]), 
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, geometric_anti_product_g1[1] * this.e1 * -1.0, 0.0) + (geometric_anti_product_g1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(geometric_anti_product_g0.xyz, geometric_anti_product_g1[0])) + float4((float3(-(geometric_anti_product_g1.zx * this.group0.yz), 0.0) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Flector geometric_anti_quotient(Line other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43;
        float3 geometric_anti_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(other_g0 * this.e4) * other.group1.xyz) + (geometric_anti_product_g0.zxy * this.group0.yzx) - (geometric_anti_product_g0.yzx * this.group0.zxy)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g0 * float4(this.e4).xyz), -(geometric_anti_product_g0[0] * this.e1) - (geometric_anti_product_g0[1] * this.e2) - (geometric_anti_product_g0[2] * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Flector geometric_anti_quotient(Motor other) {
        float other_g0 = other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e1234 * other.e1234;
        float4 geometric_anti_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_anti_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e2) - (geometric_anti_product_g0[1] * this.e3), (geometric_anti_product_g0[0] * this.e3) - (geometric_anti_product_g0[2] * this.e1), (geometric_anti_product_g0[1] * this.e1) - (geometric_anti_product_g0[0] * this.e2)) + (float3(geometric_anti_product_g0[3]) * this.group0.xyz) - (float3(this.e4) * geometric_anti_product_g1.xyz)), geometric_anti_product_g0[3] * this.e4), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g0.xyz * float4(this.e4).xyz), -(geometric_anti_product_g0[0] * this.e1) - (geometric_anti_product_g0[1] * this.e2) - (geometric_anti_product_g0[2] * this.e3) - (geometric_anti_product_g1[3] * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        float2 geometric_anti_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_anti_product_g1 = float4(other_g0 * -1.0) * other.group1;
        float3 geometric_anti_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float4 geometric_anti_product_g4 = float4(other_g0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g4[0] * this.e1) + (geometric_anti_product_g4[1] * this.e2) + (geometric_anti_product_g4[2] * this.e3) + (geometric_anti_product_g4[3] * this.e4), geometric_anti_product_g1[3] * this.e4 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0[1]) * this.group0.xyz) + (float3(other_g0 * this.e4) * other.group3.xyz) + (geometric_anti_product_g2.zxy * this.group0.yzx) - (geometric_anti_product_g2.yzx * this.group0.zxy)), geometric_anti_product_g0[1] * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * geometric_anti_product_g4.xyz, 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g4[1] * this.e3) - (geometric_anti_product_g4[2] * this.e2), (geometric_anti_product_g4[2] * this.e1) - (geometric_anti_product_g4[0] * this.e3), (geometric_anti_product_g4[0] * this.e2) - (geometric_anti_product_g4[1] * this.e1)) + (float3(this.e4) * geometric_anti_product_g1.xyz) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g2 * float4(this.e4).xyz), -(geometric_anti_product_g0[0] * this.e4) - (geometric_anti_product_g2[0] * this.e1) - (geometric_anti_product_g2[1] * this.e2) - (geometric_anti_product_g2[2] * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Motor geometric_anti_quotient(Origin other) {
        float geometric_anti_product_g0 = -1.0 / other.e4;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product_g0 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(geometric_anti_product_g0 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Motor geometric_anti_quotient(Plane other) {
        float4 geometric_anti_product_g0 = (float4(other.e431 * other.e431, other.e423 * other.e423, other.e423 * other.e423, other.e431 * other.e431) * other.group0) + (other.group0 * float4(float2(other.e412 * other.e412), other.e431 * other.e431, other.e412 * other.e412)) + (float4_pow(other.group0.xyzx, 2) * other.group0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((geometric_anti_product_g0.xyz * float4(this.e4).xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((geometric_anti_product_g0.yz * this.group0.zx) - (geometric_anti_product_g0.zx * this.group0.yz)), (geometric_anti_product_g0[0] * this.e2) - (geometric_anti_product_g0[1] * this.e1), geometric_anti_product_g0[0] * this.e1)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Point {
    public Motor geometric_anti_quotient(Point other) {
        float geometric_anti_product_g0_w = -1.0 / other.e4;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product_g0_w * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((-(float3(geometric_anti_product_g0_w) * this.group0.xyz) - (float3(this.e4 /  (other.e4 * other.e4)) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiQuotientInfixPartial<Scalar> operator /(Scalar lhs, GeometricAntiQuotientInfix rhs) {
    return GeometricAntiQuotientInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Scalar geometric_anti_quotient(AntiScalar other) {
        return Scalar(
            /* scalar */
            this.scalar / other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Scalar geometric_anti_quotient(DualNum other) {
        return Scalar(
            /* scalar */
            this.scalar / other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Flector geometric_anti_quotient(Flector other) {
        float other_g0 = other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other_g0 * this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other_g0 * other.e4 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Line geometric_anti_quotient(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            -(float3(other.e42 * other.e42 * this.scalar, other.e41 * other.e41 * this.scalar, other.e41 * other.e41 * this.scalar) * other.group0.xyz) - (other.group0.xyz * float3(float2(other.e43 * other.e43 * this.scalar), other.e42 * other.e42 * this.scalar)) - (float3_pow(other.group0.xyz, 3) * float3(this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Motor geometric_anti_quotient(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4((-(float3(this.scalar) * float3(pow(other.e41, 3), pow(other.e42, 3), pow(other.e43, 3))) - (float3(this.scalar) * float3(other.e42 * other.e42 * other.e41, other.e41 * other.e41 * other.e42, other.e41 * other.e41 * other.e43)) - (float3(this.scalar) * float3(other.e43 * other.e43 * other.e41, other.e43 * other.e43 * other.e42, other.e42 * other.e42 * other.e43)) - (float3(other.e1234 * other.e1234 * this.scalar) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public MultiVector geometric_anti_quotient(MultiVector other) {
        float other_g0 = other.e1234 * other.e1234 + other.e41 * other.e41 + other.e42 * other.e42 + other.e43 * other.e43 + other.e4 * other.e4 + other.e423 * other.e423 + other.e431 * other.e431 + other.e412 * other.e412;
        return MultiVector(
            /* scalar, e1234 */
            float2(other_g0 * other.e1234 * this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other_g0 * this.scalar) * other.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other_g0 * this.scalar * -1.0) * other.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other_g0 * other.e4 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, Origin rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Horizon geometric_anti_quotient(Origin other) {
        return Horizon(
            /* e321 */
            this.scalar * -1.0 / other.e4
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Point geometric_anti_quotient(Plane other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * float3(pow(other.e423, 3), pow(other.e431, 3), pow(other.e412, 3))) + (float3(this.scalar) * float3(other.e431 * other.e431 * other.e423, other.e423 * other.e423 * other.e431, other.e423 * other.e423 * other.e412)) + (float3(this.scalar) * float3(other.e412 * other.e412 * other.e423, other.e412 * other.e412 * other.e431, other.e431 * other.e431 * other.e412))), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiQuotientInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.geometric_anti_quotient(rhs);
}
public extension Scalar {
    public Horizon geometric_anti_quotient(Point other) {
        return Horizon(
            /* e321 */
            this.scalar * -1.0 / other.e4
        );
    }
}
