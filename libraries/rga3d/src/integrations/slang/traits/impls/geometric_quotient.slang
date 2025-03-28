implementing rga3d;
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<AntiScalar> operator /(AntiScalar lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public AntiScalar geometric_quotient(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 / other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public Flector geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other_g0 * other.e321 * this.e1234), 
            /* e423, e431, e412, e321 */
            float4((float3(other_g0 * this.e1234 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public Origin geometric_quotient(Horizon other) {
        return Origin(
            /* e4 */
            this.e1234 / other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public Line geometric_quotient(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(-(other.e23 * other.e23 * this.e1234) - (other.e31 * other.e31 * this.e1234) - (other.e12 * other.e12 * this.e1234)) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public Motor geometric_quotient(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e23 * other.e23 * this.e1234) + (other.e31 * other.e31 * this.e1234) + (other.e12 * other.e12 * this.e1234) + (other.scalar * other.scalar * this.e1234)) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other_g0 * other.scalar * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other_g0 * other.e321 * this.e1234), 
            /* e41, e42, e43 */
            float3(other_g0 * this.e1234 * -1.0) * other.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other_g0 * this.e1234 * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public Origin geometric_quotient(Plane other) {
        return Origin(
            /* e4 */
            this.e1234 / other.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public Plane geometric_quotient(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            -float4((float3_pow(other.group0.xyz, 3) * float3(this.e1234)), 0.0) - float4((float3(other.e1 * this.e1234) * other.group0.yxx * other.group0.yyz), 0.0) - float4((float3(other.e3 * this.e1234) * other.group0.xyy * other.group0.zzy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricQuotientInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension AntiScalar {
    public AntiScalar geometric_quotient(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 / other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<DualNum> operator /(DualNum lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricQuotientInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public DualNum geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return DualNum(
            /* scalar, e1234 */
            float2(geometric_product_g0[0] * this.scalar, (geometric_product_g0[0] * this.e1234) + (geometric_product_g0[1] * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * geometric_product_g0.xyz), (this.scalar * geometric_product_g0[3]) - (this.e1234 * geometric_product_g1[3])), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product_g1.xyz) - (float3(this.e1234) * geometric_product_g0.xyz)), this.scalar * geometric_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_quotient(Horizon other) {
        float geometric_product_g0 = -1.0 / other.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product_g0 * this.e1234 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_product_g0 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricQuotientInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public Line geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        float3 geometric_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Line(
            /* e41, e42, e43 */
            (geometric_product_g1 * float3(this.e1234)) - (float3(other_g0 * this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            geometric_product_g1 * float3(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public Motor geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (geometric_product_g1 * float4(this.e1234)) + float4(float3(0.0), other_g0 * this.scalar * other.e1234) - float4((float3(other_g0 * this.scalar) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            geometric_product_g1 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float3 geometric_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g0[0] * this.scalar, (geometric_product_g0[0] * this.e1234) + (geometric_product_g0[1] * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * geometric_product_g1.xyz), (this.scalar * geometric_product_g1[3]) - (this.e1234 * geometric_product_g4[3])), 
            /* e41, e42, e43 */
            (geometric_product_g3 * float3(this.e1234)) - (float3(other_g0 * this.scalar) * other.group2.xyz), 
            /* e23, e31, e12 */
            geometric_product_g3 * float3(this.scalar), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product_g4.xyz) - (float3(this.e1234) * geometric_product_g1.xyz)), this.scalar * geometric_product_g4[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * geometric_product_g0[3] * -1.0), 
            /* e423, e431, e412, e321 */
            geometric_product_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public Flector geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return Flector(
            /* e1, e2, e3, e4 */
            geometric_product_g0 * float4(this.scalar), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234 * -1.0) * geometric_product_g0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricQuotientInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension DualNum {
    public DualNum geometric_quotient(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(1.0 / other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Flector> operator /(Flector lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Flector geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product_g0[0]) * this.group0.xyz), (geometric_product_g0[0] * this.e4) + (geometric_product_g0[1] * this.e321)), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product_g0[0]) * this.group1.xyz) + (float3(geometric_product_g0[1]) * this.group0.xyz)), geometric_product_g0[0] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Motor geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(this.e4) * float4(geometric_product_g0.xyz, geometric_product_g1[3])) + (geometric_product_g1.zxyx * this.group0.yzxx) + (geometric_product_g1.wwwy * float4(this.group1.xyz, this.e2)) + float4(float3(0.0), (geometric_product_g1[2] * this.e3) - (geometric_product_g0[2] * this.e412)) + float4((geometric_product_g0.yzx * this.group1.zxy), 0.0) - (float4(geometric_product_g0[3]) * float4(this.group0.xyz, this.e321)) - (geometric_product_g0.zxyx * this.group1.yzxx) - (this.group1.wwwy * float4(geometric_product_g1.xyz, geometric_product_g0[1])) - float4((geometric_product_g1.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, scalar */
            (geometric_product_g0.zxyx * this.group0.yzxx) + float4(float3(0.0), (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3)) - (float4(this.e321) * float4(geometric_product_g0.xyz, geometric_product_g1[3])) - float4((float3(geometric_product_g1[3]) * this.group0.xyz), 0.0) - float4((geometric_product_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Motor geometric_quotient(Horizon other) {
        float geometric_product_g0 = -1.0 / other.e321;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_product_g0) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(geometric_product_g0 * -1.0) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Flector geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        float3 geometric_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(geometric_product_g0[1] * this.e2) - (geometric_product_g0[2] * this.e3) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)) + float4((geometric_product_g1.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g1.yzz * float3(this.group0.zx, this.e321)), 0.0) - (this.group0.yzxx * float4(geometric_product_g1.zxy, geometric_product_g0[0])), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), (geometric_product_g1[2] * this.e3) * -1.0) + float4((geometric_product_g0.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g0.yzz * float3(this.group0.zx, this.e321)), 0.0) + float4((geometric_product_g1.xyx * float3(float2(this.e4), this.e431)), 0.0) + float4((geometric_product_g1.yzz * float3(this.group1.zx, this.e4)), 0.0) - (this.group0.yzxx * float4(geometric_product_g0.zxy, geometric_product_g1[0])) - float4((geometric_product_g1.zxy * this.group1.yzx), geometric_product_g1[1] * this.e2)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Flector geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            (geometric_product_g1 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(geometric_product_g0[1] * this.e2) - (geometric_product_g0[2] * this.e3) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)) + float4((float3(geometric_product_g1[3]) * this.group0.xyz), 0.0) + float4((geometric_product_g1.yzx * this.group0.zxy), geometric_product_g0[3] * this.e321) - (this.group0.yzxx * float4(geometric_product_g1.zxy, geometric_product_g0[0])), 
            /* e423, e431, e412, e321 */
            (geometric_product_g1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), (geometric_product_g1[2] * this.e3) * -1.0) + float4((float3(geometric_product_g0[3]) * this.group0.xyz), 0.0) + float4((float3(geometric_product_g1[3]) * this.group1.xyz), 0.0) + float4((geometric_product_g0.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g0.yzz * float3(this.group0.zx, this.e321)), 0.0) + float4((geometric_product_g1.yzx * this.group1.zxy), 0.0) - (geometric_product_g1.zxyy * float4(this.group1.yzx, this.e2)) - (this.group0.yzxx * float4(geometric_product_g0.zxy, geometric_product_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float3 geometric_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g4[3] * this.e4) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)) + (float2(this.e1) * float2(geometric_product_g1[0], geometric_product_g4[0])) + (float2(this.e2) * float2(geometric_product_g1[1], geometric_product_g4[1])) + (float2(this.e3) * float2(geometric_product_g1[2], geometric_product_g4[2])) - (float2(this.e321) * float2(geometric_product_g4[3], geometric_product_g1[3])), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g0[0]) * this.group0) + (float4(this.e321) * float4(geometric_product_g3, geometric_product_g0[1])) + float4(float3(0.0), -(geometric_product_g2[1] * this.e2) - (geometric_product_g2[2] * this.e3) - (geometric_product_g3[0] * this.e423) - (geometric_product_g3[1] * this.e431) - (geometric_product_g3[2] * this.e412)) + float4((geometric_product_g3.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(geometric_product_g3.zxy, geometric_product_g2[0])), 
            /* e41, e42, e43 */
            (float3(geometric_product_g4[3]) * this.group1.xyz) + (float3(this.e4) * geometric_product_g1.xyz) + (geometric_product_g1.yzx * this.group1.zxy) + float3(float2(0.0), (geometric_product_g4[1] * this.e1) - (geometric_product_g1[1] * this.e423) - (geometric_product_g4[0] * this.e2)) + float3((geometric_product_g4.zx * this.group0.yz), 0.0) - (float3(geometric_product_g1[3]) * this.group0.xyz) - (float3(this.e321) * geometric_product_g4.xyz) - float3((geometric_product_g1.zx * this.group1.yz), 0.0) - float3((geometric_product_g4.yz * this.group0.zx), 0.0), 
            /* e23, e31, e12 */
            (geometric_product_g1.zxy * this.group0.yzx) + float3(float2(0.0), (geometric_product_g1[0] * this.e2) * -1.0) - (float3(geometric_product_g4[3]) * this.group0.xyz) - (float3(this.e321) * geometric_product_g1.xyz) - float3((geometric_product_g1.yz * this.group0.zx), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(geometric_product_g0[0]) * this.group1) + float4(float3(0.0), (geometric_product_g3[2] * this.e3) * -1.0) + float4((float3(geometric_product_g0[1]) * this.group0.xyz), 0.0) + float4((geometric_product_g2.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g2.yzz * float3(this.group0.zx, this.e321)), 0.0) + float4((geometric_product_g3.xyx * float3(float2(this.e4), this.e431)), 0.0) + float4((geometric_product_g3.yzz * float3(this.group1.zx, this.e4)), 0.0) - (this.group0.yzxx * float4(geometric_product_g2.zxy, geometric_product_g3[0])) - float4((geometric_product_g3.zxy * this.group1.yzx), geometric_product_g3[1] * this.e2)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Motor geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product_g0[3]) * float4(this.group1.xyz, this.e4)) + (geometric_product_g0.zxyx * this.group0.yzxx) + float4(float3(0.0), (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3)) - float4((geometric_product_g0.xyx * float3(float2(this.e321), this.e2)), 0.0) - float4((geometric_product_g0.yzz * float3(this.group0.zx, this.e321)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_product_g0[3] * -1.0) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Motor geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), -(geometric_product_g0[1] * this.e431) - (geometric_product_g0[2] * this.e412)) + float4((geometric_product_g0.xyx * float3(float2(this.e4), this.e431)), 0.0) + float4((geometric_product_g0.yzz * float3(this.group1.zx, this.e4)), 0.0) - (float4(geometric_product_g0[3]) * float4(this.group0.xyz, this.e321)) - (geometric_product_g0.zxyx * this.group1.yzxx), 
            /* e23, e31, e12, scalar */
            (geometric_product_g0.zxyx * this.group0.yzxx) + float4(float3(0.0), (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3)) - float4((geometric_product_g0.xyx * float3(float2(this.e321), this.e2)), 0.0) - float4((geometric_product_g0.yzz * float3(this.group0.zx, this.e321)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Flector {
    public Flector geometric_quotient(Scalar other) {
        float geometric_product_g0 = 1.0 / other.scalar;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(geometric_product_g0) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_product_g0) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Horizon> operator /(Horizon lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Flector geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product_g0[1] * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_product_g0[0] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Motor geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e321 * -1.0) * float4(geometric_product_g1.xyz, geometric_product_g0[3]), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * -1.0) * float4(geometric_product_g0.xyz, geometric_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricQuotientInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Scalar geometric_quotient(Horizon other) {
        return Scalar(
            /* scalar */
            this.e321 / other.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Flector geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other_g0 * this.e321 * -1.0) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other_g0 * this.e321 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Flector geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(geometric_product_g1.xyz, geometric_product_g0[3]), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(geometric_product_g0.xyz, geometric_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * -1.0) * float2(geometric_product_g4[3], geometric_product_g1[3]), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4((float3(other_g0 * -1.0) * other.group3.xyz), geometric_product_g0[1]), 
            /* e41, e42, e43 */
            float3(this.e321 * -1.0) * geometric_product_g4.xyz, 
            /* e23, e31, e12 */
            float3(this.e321 * -1.0) * geometric_product_g1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4((float3(other_g0 * -1.0) * other.group2.xyz), geometric_product_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Motor geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321 * -1.0) * geometric_product_g0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product_g0[3] * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Motor geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_product_g0[3] * this.e321 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321 * -1.0) * geometric_product_g0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricQuotientInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Horizon {
    public Horizon geometric_quotient(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 / other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Line> operator /(Line lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(GeometricQuotientInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Line geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return Line(
            /* e41, e42, e43 */
            (float3(geometric_product_g0[0]) * this.group0.xyz) + (float3(geometric_product_g0[1]) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3(geometric_product_g0[0]) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Flector geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return Flector(
            /* e1, e2, e3, e4 */
            (geometric_product_g0.yzxx * float4(this.group1.zxy, this.e41)) + float4(float3(0.0), (this.e43 * geometric_product_g0[2]) - (this.e31 * geometric_product_g1[1]) - (this.e12 * geometric_product_g1[2])) + float4((float3(geometric_product_g1[3]) * this.group1.xyz), this.e42 * geometric_product_g0[1]) - float4((this.group1.yzx * geometric_product_g0.zxy), this.e23 * geometric_product_g1[0]), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_product_g0[3], geometric_product_g1[2], geometric_product_g1[0]) * this.group1.xxy), 0.0) + float4((float3(geometric_product_g1[1], geometric_product_g0[3], geometric_product_g0[3]) * this.group1.zyz), 0.0) + float4((this.group0.yzx * geometric_product_g0.zxy), 0.0) - (geometric_product_g0.yzxx * float4(this.group0.zxy, this.e23)) - float4((float3(geometric_product_g1[3]) * this.group0.xyz), this.e31 * geometric_product_g0[1]) - float4((this.group1.yzx * geometric_product_g1.zxy), this.e12 * geometric_product_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Flector geometric_quotient(Horizon other) {
        float geometric_product_g0 = -1.0 / other.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product_g0) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_product_g0 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Motor geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        float3 geometric_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), -(geometric_product_g0[2] * this.e12) - (geometric_product_g1[0] * this.e41) - (geometric_product_g1[1] * this.e42) - (geometric_product_g1[2] * this.e43)) + float4((geometric_product_g0.yzx * this.group1.zxy), 0.0) + float4((geometric_product_g1.yzx * this.group0.zxy), 0.0) - float4((geometric_product_g0.zxy * this.group1.yzx), geometric_product_g0[0] * this.e23) - float4((geometric_product_g1.zxy * this.group0.yzx), geometric_product_g0[1] * this.e31), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(geometric_product_g1[1] * this.e31) - (geometric_product_g1[2] * this.e12)) + float4((geometric_product_g1.yzx * this.group1.zxy), 0.0) - float4((geometric_product_g1.zxy * this.group1.yzx), geometric_product_g1[0] * this.e23)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Motor geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), -(this.e42 * geometric_product_g1[1]) - (this.e43 * geometric_product_g1[2]) - (this.e31 * geometric_product_g0[1]) - (this.e12 * geometric_product_g0[2])) + float4((this.group0.xxy * geometric_product_g1.wzx), 0.0) + float4((this.group0.zyz * geometric_product_g1.yww), 0.0) + float4((this.group1.xxy * geometric_product_g0.wzx), 0.0) + float4((this.group1.zyz * geometric_product_g0.yww), 0.0) - (this.group0.xyz.yzxx * geometric_product_g1.zxyx) - (this.group1.xyz.yzxx * geometric_product_g0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e31 * geometric_product_g1[1]) - (this.e12 * geometric_product_g1[2])) + float4((this.group1.xxy * geometric_product_g1.wzx), 0.0) + float4((this.group1.zyz * geometric_product_g1.yww), 0.0) - (this.group1.xyz.yzxx * geometric_product_g1.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float3 geometric_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(geometric_product_g3[0] * this.e41) - (geometric_product_g3[1] * this.e42) - (geometric_product_g3[2] * this.e43)) - (float2(this.e23) * float2(geometric_product_g3[0], geometric_product_g2[0])) - (float2(this.e31) * float2(geometric_product_g3[1], geometric_product_g2[1])) - (float2(this.e12) * float2(geometric_product_g3[2], geometric_product_g2[2])), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1.yzxx * float4(this.group1.zxy, this.e41)) + float4(float3(0.0), (this.e43 * geometric_product_g1[2]) - (this.e31 * geometric_product_g4[1]) - (this.e12 * geometric_product_g4[2])) + float4((float3(geometric_product_g4[3]) * this.group1.xyz), this.e42 * geometric_product_g1[1]) - float4((this.group1.yzx * geometric_product_g1.zxy), this.e23 * geometric_product_g4[0]), 
            /* e41, e42, e43 */
            (float3(geometric_product_g0[0]) * this.group0.xyz) + (float3(geometric_product_g0[1]) * this.group1.xyz) + (geometric_product_g2.yzx * this.group1.zxy) + (geometric_product_g3.yzx * this.group0.zxy) - (geometric_product_g2.zxy * this.group1.yzx) - (geometric_product_g3.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(geometric_product_g0[0]) * this.group1.xyz) + (geometric_product_g3.yzx * this.group1.zxy) - (geometric_product_g3.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_product_g1[3], geometric_product_g4[2], geometric_product_g4[0]) * this.group1.xxy), 0.0) + float4((float3(geometric_product_g4[1], geometric_product_g1[3], geometric_product_g1[3]) * this.group1.zyz), 0.0) + float4((this.group0.yzx * geometric_product_g1.zxy), 0.0) - (geometric_product_g1.yzxx * float4(this.group0.zxy, this.e23)) - float4((float3(geometric_product_g4[3]) * this.group0.xyz), this.e31 * geometric_product_g1[1]) - float4((this.group1.yzx * geometric_product_g4.zxy), this.e12 * geometric_product_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Flector geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product_g0[3]) * this.group1.xyz), -(this.e23 * geometric_product_g0[0]) - (this.e31 * geometric_product_g0[1]) - (this.e12 * geometric_product_g0[2])), 
            /* e423, e431, e412, e321 */
            float4((this.group1.zxy * geometric_product_g0.yzx), 0.0) - float4((float3(geometric_product_g0[3]) * this.group0.xyz), 0.0) - float4((this.group1.yzx * geometric_product_g0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Flector geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return Flector(
            /* e1, e2, e3, e4 */
            (geometric_product_g0.yzxx * float4(this.group1.zxy, this.e41)) + float4(float3(0.0), (this.e42 * geometric_product_g0[1]) + (this.e43 * geometric_product_g0[2])) - float4((this.group1.yzx * geometric_product_g0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(this.e31 * geometric_product_g0[1]) - (this.e12 * geometric_product_g0[2])) + float4((float3(geometric_product_g0[3]) * this.group1.xyz), 0.0) + float4((this.group0.yzx * geometric_product_g0.zxy), 0.0) - (geometric_product_g0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricQuotientInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Line {
    public Line geometric_quotient(Scalar other) {
        float geometric_product_g0 = 1.0 / other.scalar;
        return Line(
            /* e41, e42, e43 */
            float3(geometric_product_g0) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(geometric_product_g0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Motor> operator /(Motor lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Motor geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product_g0[0]) * this.group0) + (float4(geometric_product_g0[1]) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(geometric_product_g0[0]) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Flector geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return Flector(
            /* e1, e2, e3, e4 */
            (geometric_product_g0 * float4(this.scalar)) + (geometric_product_g0.yzxx * float4(this.group1.zxy, this.e41)) + float4(float3(0.0), (geometric_product_g0[1] * this.e42) + (geometric_product_g0[2] * this.e43) - (geometric_product_g1[1] * this.e31) - (geometric_product_g1[2] * this.e12) - (geometric_product_g1[3] * this.e1234)) + float4((float3(geometric_product_g1[3]) * this.group1.xyz), 0.0) - (this.group1.yzxx * float4(geometric_product_g0.zxy, geometric_product_g1[0])), 
            /* e423, e431, e412, e321 */
            (this.group1 * float4(float3(geometric_product_g0[3]), geometric_product_g1[3])) + float4((geometric_product_g0.zxy * this.group0.yzx), 0.0) + float4((geometric_product_g1.xyx * this.group1.wwy), 0.0) + float4((geometric_product_g1.yzz * this.group1.zxw), 0.0) - (geometric_product_g0.xyxx * float4(this.group0.wwy, this.e23)) - (geometric_product_g0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(geometric_product_g1.zxy, geometric_product_g0[2])) - float4((float3(geometric_product_g1[3]) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Flector geometric_quotient(Horizon other) {
        float geometric_product_g0 = -1.0 / other.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(geometric_product_g0) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product_g0) * float4((this.group0.xyz * float3(-1.0)), this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Motor geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        float3 geometric_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), -(geometric_product_g0[1] * this.e31) - (geometric_product_g0[2] * this.e12) - (geometric_product_g1[1] * this.e42) - (geometric_product_g1[2] * this.e43)) + float4((geometric_product_g0.xyx * this.group1.wwy), 0.0) + float4((geometric_product_g0.yzz * this.group1.zxw), 0.0) + float4((geometric_product_g1.xyx * this.group0.wwy), 0.0) + float4((geometric_product_g1.yzz * this.group0.zxw), 0.0) - (geometric_product_g0.zxyx * this.group1.yzxx) - (geometric_product_g1.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(geometric_product_g1[1] * this.e31) - (geometric_product_g1[2] * this.e12)) + float4((geometric_product_g1.xyx * this.group1.wwy), 0.0) + float4((geometric_product_g1.yzz * this.group1.zxw), 0.0) - (geometric_product_g1.zxyx * this.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Motor geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (geometric_product_g0 * float4(this.scalar)) + (geometric_product_g1 * float4(this.e1234)) + float4(float3(0.0), -(geometric_product_g0[1] * this.e31) - (geometric_product_g0[2] * this.e12) - (geometric_product_g1[1] * this.e42) - (geometric_product_g1[2] * this.e43)) + float4((float3(geometric_product_g0[3]) * this.group1.xyz), 0.0) + float4((float3(geometric_product_g1[3]) * this.group0.xyz), 0.0) + float4((geometric_product_g0.yzx * this.group1.zxy), 0.0) + float4((geometric_product_g1.yzx * this.group0.zxy), 0.0) - (geometric_product_g0.zxyx * this.group1.yzxx) - (geometric_product_g1.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            (geometric_product_g1 * float4(this.scalar)) + float4(float3(0.0), -(geometric_product_g1[1] * this.e31) - (geometric_product_g1[2] * this.e12)) + float4((float3(geometric_product_g1[3]) * this.group1.xyz), 0.0) + float4((geometric_product_g1.yzx * this.group1.zxy), 0.0) - (geometric_product_g1.zxyx * this.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float3 geometric_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g0[0] * this.e1234) - (geometric_product_g3[0] * this.e41) - (geometric_product_g3[1] * this.e42) - (geometric_product_g3[2] * this.e43)) + (geometric_product_g0 * float2(this.scalar)) - (float2(this.e23) * float2(geometric_product_g3[0], geometric_product_g2[0])) - (float2(this.e31) * float2(geometric_product_g3[1], geometric_product_g2[1])) - (float2(this.e12) * float2(geometric_product_g3[2], geometric_product_g2[2])), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1 * float4(this.scalar)) + (geometric_product_g1.yzxx * float4(this.group1.zxy, this.e41)) + float4(float3(0.0), (geometric_product_g1[1] * this.e42) + (geometric_product_g1[2] * this.e43) - (geometric_product_g4[1] * this.e31) - (geometric_product_g4[2] * this.e12) - (geometric_product_g4[3] * this.e1234)) + float4((float3(geometric_product_g4[3]) * this.group1.xyz), 0.0) - (this.group1.yzxx * float4(geometric_product_g1.zxy, geometric_product_g4[0])), 
            /* e41, e42, e43 */
            (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (float3(geometric_product_g0[0]) * this.group0.xyz) + (float3(geometric_product_g0[1]) * this.group1.xyz) + (geometric_product_g2.yzx * this.group1.zxy) + (geometric_product_g3.yzx * this.group0.zxy) - (geometric_product_g2.zxy * this.group1.yzx) - (geometric_product_g3.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (geometric_product_g3 * float3(this.scalar)) + (float3(geometric_product_g0[0]) * this.group1.xyz) + (geometric_product_g3.yzx * this.group1.zxy) - (geometric_product_g3.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            (this.group1 * float4(float3(geometric_product_g1[3]), geometric_product_g4[3])) + float4((geometric_product_g1.zxy * this.group0.yzx), 0.0) + float4((geometric_product_g4.xyx * this.group1.wwy), 0.0) + float4((geometric_product_g4.yzz * this.group1.zxw), 0.0) - (geometric_product_g1.xyxx * float4(this.group0.wwy, this.e23)) - (geometric_product_g1.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(geometric_product_g4.zxy, geometric_product_g1[2])) - float4((float3(geometric_product_g4[3]) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Flector geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product_g0[3]) * this.group1.xyz), -(geometric_product_g0[0] * this.e23) - (geometric_product_g0[1] * this.e31) - (geometric_product_g0[2] * this.e12) - (geometric_product_g0[3] * this.e1234)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product_g0.xyz) + (geometric_product_g0.yzx * this.group1.zxy) + float3(float2(0.0), geometric_product_g0[1] * this.e23 * -1.0) - (float3(geometric_product_g0[3]) * this.group0.xyz) - float3((geometric_product_g0.zx * this.group1.yz), 0.0)), geometric_product_g0[3] * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Flector geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return Flector(
            /* e1, e2, e3, e4 */
            (geometric_product_g0 * float4(this.scalar)) + (geometric_product_g0.yzxx * float4(this.group1.zxy, this.e41)) + float4(float3(0.0), (geometric_product_g0[1] * this.e42) + (geometric_product_g0[2] * this.e43)) - float4((geometric_product_g0.zxy * this.group1.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), (geometric_product_g0[2] * this.e12) * -1.0) + float4((float3(geometric_product_g0[3]) * this.group1.xyz), 0.0) + float4((geometric_product_g0.zxy * this.group0.yzx), 0.0) - (geometric_product_g0.xyxx * float4(this.group0.wwy, this.e23)) - (geometric_product_g0.yzzy * float4(this.group0.zxw, this.e31))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Motor {
    public Motor geometric_quotient(Scalar other) {
        float geometric_product_g0 = 1.0 / other.scalar;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_product_g0) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_product_g0) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<MultiVector> operator /(MultiVector lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g0[0] * this.scalar, (geometric_product_g0[0] * this.e1234) + (geometric_product_g0[1] * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product_g0[0]) * this.group1.xyz), (geometric_product_g0[0] * this.e4) + (geometric_product_g0[1] * this.e321)), 
            /* e41, e42, e43 */
            (float3(geometric_product_g0[0]) * this.group2.xyz) + (float3(geometric_product_g0[1]) * this.group3.xyz), 
            /* e23, e31, e12 */
            float3(geometric_product_g0[0]) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product_g0[0]) * this.group4.xyz) + (float3(geometric_product_g0[1]) * this.group1.xyz)), geometric_product_g0[0] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g1[3] * this.e4) - (geometric_product_g0[0] * this.e423) - (geometric_product_g0[1] * this.e431) - (geometric_product_g0[2] * this.e412)) + (float2(this.e1) * float2(geometric_product_g0[0], geometric_product_g1[0])) + (float2(this.e2) * float2(geometric_product_g0[1], geometric_product_g1[1])) + (float2(this.e3) * float2(geometric_product_g0[2], geometric_product_g1[2])) - (float2(this.e321) * float2(geometric_product_g1[3], geometric_product_g0[3])), 
            /* e1, e2, e3, e4 */
            (geometric_product_g0 * float4(this.scalar)) + (geometric_product_g0.yzxx * float4(this.group3.zxy, this.e41)) + float4(float3(0.0), (this.e43 * geometric_product_g0[2]) - (this.e23 * geometric_product_g1[0]) - (this.e31 * geometric_product_g1[1]) - (this.e12 * geometric_product_g1[2])) + float4((float3(geometric_product_g1[3]) * this.group3.xyz), this.e42 * geometric_product_g0[1]) - float4((this.group3.yzx * geometric_product_g0.zxy), this.e1234 * geometric_product_g1[3]), 
            /* e41, e42, e43 */
            (float3(geometric_product_g1[3]) * this.group4.xyz) + (float3(this.e4) * geometric_product_g0.xyz) + (geometric_product_g0.yzx * this.group4.zxy) + float3(float2(0.0), (geometric_product_g1[1] * this.e1) - (geometric_product_g0[1] * this.e423) - (geometric_product_g1[0] * this.e2)) + float3((geometric_product_g1.zx * this.group1.yz), 0.0) - (float3(geometric_product_g0[3]) * this.group1.xyz) - (float3(this.e321) * geometric_product_g1.xyz) - float3((geometric_product_g0.zx * this.group4.yz), 0.0) - float3((geometric_product_g1.yz * this.group1.zx), 0.0), 
            /* e23, e31, e12 */
            (geometric_product_g0.zxy * this.group1.yzx) + float3(float2(0.0), (geometric_product_g0[0] * this.e2) * -1.0) - (float3(geometric_product_g1[3]) * this.group1.xyz) - (float3(this.e321) * geometric_product_g0.xyz) - float3((geometric_product_g0.yz * this.group1.zx), 0.0), 
            /* e423, e431, e412, e321 */
            (geometric_product_g1 * float4(this.scalar)) + float4((float3(geometric_product_g0[3], geometric_product_g1[2], geometric_product_g1[0]) * this.group3.xxy), 0.0) + float4((float3(geometric_product_g1[1], geometric_product_g0[3], geometric_product_g0[3]) * this.group3.zyz), 0.0) + float4((this.group2.yzx * geometric_product_g0.zxy), 0.0) - (geometric_product_g0.xyzx * float4(float3(this.e1234), this.e23)) - (geometric_product_g0.yzxy * float4(this.group2.zxy, this.e31)) - float4((float3(geometric_product_g1[3]) * this.group2.xyz), this.e12 * geometric_product_g0[2]) - float4((this.group3.yzx * geometric_product_g1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(Horizon other) {
        float geometric_product_g0 = -1.0 / other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g0) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product_g0) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(geometric_product_g0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(geometric_product_g0 * -1.0) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_product_g0) * float4((this.group2.xyz * float3(-1.0)), this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        float3 geometric_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(geometric_product_g1[0] * this.e41) - (geometric_product_g1[1] * this.e42) - (geometric_product_g1[2] * this.e43)) - (float2(this.e23) * float2(geometric_product_g1[0], geometric_product_g0[0])) - (float2(this.e31) * float2(geometric_product_g1[1], geometric_product_g0[1])) - (float2(this.e12) * float2(geometric_product_g1[2], geometric_product_g0[2])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(geometric_product_g0[1] * this.e2) - (geometric_product_g0[2] * this.e3) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)) + float4((geometric_product_g1.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g1.yzz * float3(this.group1.zx, this.e321)), 0.0) - (this.group1.yzxx * float4(geometric_product_g1.zxy, geometric_product_g0[0])), 
            /* e41, e42, e43 */
            (geometric_product_g0 * float3(this.scalar)) + (geometric_product_g1 * float3(this.e1234)) + (geometric_product_g0.yzx * this.group3.zxy) + (geometric_product_g1.yzx * this.group2.zxy) - (geometric_product_g0.zxy * this.group3.yzx) - (geometric_product_g1.zxy * this.group2.yzx), 
            /* e23, e31, e12 */
            (geometric_product_g1 * float3(this.scalar)) + (geometric_product_g1.yzx * this.group3.zxy) - (geometric_product_g1.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), (geometric_product_g1[2] * this.e3) * -1.0) + float4((geometric_product_g0.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g0.yzz * float3(this.group1.zx, this.e321)), 0.0) + float4((geometric_product_g1.xyx * float3(float2(this.e4), this.e431)), 0.0) + float4((geometric_product_g1.yzz * float3(this.group4.zx, this.e4)), 0.0) - (this.group1.yzxx * float4(geometric_product_g0.zxy, geometric_product_g1[0])) - float4((geometric_product_g1.zxy * this.group4.yzx), geometric_product_g1[1] * this.e2)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * geometric_product_g0[3]) - (this.e23 * geometric_product_g0[0]) - (this.e31 * geometric_product_g0[1]) - (this.e12 * geometric_product_g0[2])) + (float2(geometric_product_g1[3]) * this.group0.xy) - (float2(geometric_product_g1[0]) * float2(this.e23, this.e41)) - (float2(geometric_product_g1[1]) * float2(this.e31, this.e42)) - (float2(geometric_product_g1[2]) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(geometric_product_g0[1] * this.e2) - (geometric_product_g0[2] * this.e3) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)) + float4((float3(geometric_product_g1[3]) * this.group1.xyz), 0.0) + float4((geometric_product_g1.yzx * this.group1.zxy), geometric_product_g0[3] * this.e321) - (this.group1.yzxx * float4(geometric_product_g1.zxy, geometric_product_g0[0])), 
            /* e41, e42, e43 */
            (float3(this.scalar) * geometric_product_g0.xyz) + (float3(this.e1234) * geometric_product_g1.xyz) + (float3(geometric_product_g0[3]) * this.group3.xyz) + (float3(geometric_product_g1[3]) * this.group2.xyz) + (this.group2.zxy * geometric_product_g1.yzx) + (this.group3.zxy * geometric_product_g0.yzx) - (this.group2.yzx * geometric_product_g1.zxy) - (this.group3.yzx * geometric_product_g0.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * geometric_product_g1.xyz) + (float3(geometric_product_g1[3]) * this.group3.xyz) + (this.group3.zxy * geometric_product_g1.yzx) - (this.group3.yzx * geometric_product_g1.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_product_g1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), (geometric_product_g1[2] * this.e3) * -1.0) + float4((float3(geometric_product_g0[3]) * this.group1.xyz), 0.0) + float4((float3(geometric_product_g1[3]) * this.group4.xyz), 0.0) + float4((geometric_product_g0.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g0.yzz * float3(this.group1.zx, this.e321)), 0.0) + float4((geometric_product_g1.yzx * this.group4.zxy), 0.0) - (geometric_product_g1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(geometric_product_g0.zxy, geometric_product_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float3 geometric_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g0[1] * this.scalar) + (geometric_product_g4[3] * this.e4) - (geometric_product_g3[0] * this.e41) - (geometric_product_g3[1] * this.e42) - (geometric_product_g3[2] * this.e43) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)) + (float2(geometric_product_g0[0]) * this.group0.xy) + (float2(this.e1) * float2(geometric_product_g1[0], geometric_product_g4[0])) + (float2(this.e2) * float2(geometric_product_g1[1], geometric_product_g4[1])) + (float2(this.e3) * float2(geometric_product_g1[2], geometric_product_g4[2])) - (float2(this.e23) * float2(geometric_product_g3[0], geometric_product_g2[0])) - (float2(this.e31) * float2(geometric_product_g3[1], geometric_product_g2[1])) - (float2(this.e12) * float2(geometric_product_g3[2], geometric_product_g2[2])) - (float2(this.e321) * float2(geometric_product_g4[3], geometric_product_g1[3])), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1 * float4(this.scalar)) + (float4(geometric_product_g0[0]) * this.group1) + (float4(this.e321) * float4(geometric_product_g3, geometric_product_g0[1])) + (geometric_product_g1.yzxx * float4(this.group3.zxy, this.e41)) + float4(float3(0.0), -(geometric_product_g2[1] * this.e2) - (geometric_product_g2[2] * this.e3) - (geometric_product_g3[0] * this.e423) - (geometric_product_g3[1] * this.e431) - (geometric_product_g3[2] * this.e412) - (this.e23 * geometric_product_g4[0]) - (this.e31 * geometric_product_g4[1]) - (this.e12 * geometric_product_g4[2])) + float4((float3(geometric_product_g4[3]) * this.group3.xyz), this.e43 * geometric_product_g1[2]) + float4((geometric_product_g3.yzx * this.group1.zxy), this.e42 * geometric_product_g1[1]) - (this.group1.yzxx * float4(geometric_product_g3.zxy, geometric_product_g2[0])) - float4((this.group3.yzx * geometric_product_g1.zxy), this.e1234 * geometric_product_g4[3]), 
            /* e41, e42, e43 */
            (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (float3(geometric_product_g0[0]) * this.group2.xyz) + (float3(geometric_product_g0[1]) * this.group3.xyz) + (float3(geometric_product_g4[3]) * this.group4.xyz) + (float3(this.e4) * geometric_product_g1.xyz) + (geometric_product_g2.yzx * this.group3.zxy) + (geometric_product_g3.yzx * this.group2.zxy) + float3(float2(0.0), (geometric_product_g1[0] * this.e431) + (geometric_product_g4[1] * this.e1) - (geometric_product_g1[1] * this.e423) - (geometric_product_g4[0] * this.e2)) + float3((geometric_product_g1.yz * this.group4.zx), 0.0) + float3((geometric_product_g4.zx * this.group1.yz), 0.0) - (float3(geometric_product_g1[3]) * this.group1.xyz) - (float3(this.e321) * geometric_product_g4.xyz) - (geometric_product_g2.zxy * this.group3.yzx) - (geometric_product_g3.zxy * this.group2.yzx) - float3((geometric_product_g1.zx * this.group4.yz), 0.0) - float3((geometric_product_g4.yz * this.group1.zx), 0.0), 
            /* e23, e31, e12 */
            (geometric_product_g3 * float3(this.scalar)) + (float3(geometric_product_g0[0]) * this.group3.xyz) + (geometric_product_g3.yzx * this.group3.zxy) + float3(float2(0.0), (geometric_product_g1[1] * this.e1) - (geometric_product_g1[0] * this.e2)) + float3((geometric_product_g1.zx * this.group1.yz), 0.0) - (float3(geometric_product_g4[3]) * this.group1.xyz) - (float3(this.e321) * geometric_product_g1.xyz) - (geometric_product_g3.zxy * this.group3.yzx) - float3((geometric_product_g1.yz * this.group1.zx), 0.0), 
            /* e423, e431, e412, e321 */
            (geometric_product_g4 * float4(this.scalar)) + (float4(geometric_product_g0[0]) * this.group4) + float4((float3(geometric_product_g0[1]) * this.group1.xyz), 0.0) + float4((float3(geometric_product_g1[3], geometric_product_g4[2], geometric_product_g4[0]) * this.group3.xxy), 0.0) + float4((float3(geometric_product_g4[1], geometric_product_g1[3], geometric_product_g1[3]) * this.group3.zyz), 0.0) + float4((geometric_product_g2.xyx * float3(float2(this.e321), this.e2)), 0.0) + float4((geometric_product_g2.yzz * float3(this.group1.zx, this.e321)), 0.0) + float4((geometric_product_g3.xyx * float3(float2(this.e4), this.e431)), 0.0) + float4((geometric_product_g3.yzz * float3(this.group4.zx, this.e4)), 0.0) + float4((this.group2.yzx * geometric_product_g1.zxy), 0.0) - (geometric_product_g1.xyzx * float4(float3(this.e1234), this.e23)) - (geometric_product_g1.yzxy * float4(this.group2.zxy, this.e31)) - (this.group1.yzxx * float4(geometric_product_g2.zxy, geometric_product_g3[0])) - float4((float3(geometric_product_g4[3]) * this.group2.xyz), geometric_product_g3[2] * this.e3) - float4((geometric_product_g3.zxy * this.group4.yzx), geometric_product_g3[1] * this.e2) - float4((this.group3.yzx * geometric_product_g4.zxy), this.e12 * geometric_product_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g0[3] * this.e321 * -1.0, (geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3) + (geometric_product_g0[3] * this.e4)), 
            /* e1, e2, e3, e4 */
            float4((float3(geometric_product_g0[3]) * this.group3.xyz), -(this.e1234 * geometric_product_g0[3]) - (this.e23 * geometric_product_g0[0]) - (this.e31 * geometric_product_g0[1]) - (this.e12 * geometric_product_g0[2])), 
            /* e41, e42, e43 */
            (float3(geometric_product_g0[3]) * this.group4.xyz) + (geometric_product_g0.zxy * this.group1.yzx) + float3(float2(0.0), (geometric_product_g0[0] * this.e2) * -1.0) - (float3(this.e321) * geometric_product_g0.xyz) - float3((geometric_product_g0.yz * this.group1.zx), 0.0), 
            /* e23, e31, e12 */
            float3(geometric_product_g0[3] * -1.0) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * geometric_product_g0.xyz) + (this.group3.zxy * geometric_product_g0.yzx) - (float3(geometric_product_g0[3]) * this.group2.xyz) - (this.group3.yzx * geometric_product_g0.zxy)), this.scalar * geometric_product_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3), -(geometric_product_g0[0] * this.e423) - (geometric_product_g0[1] * this.e431) - (geometric_product_g0[2] * this.e412) - (geometric_product_g0[3] * this.e321)), 
            /* e1, e2, e3, e4 */
            (geometric_product_g0 * float4(this.scalar)) + (geometric_product_g0.yzxx * float4(this.group3.zxy, this.e41)) + float4(float3(0.0), (this.e42 * geometric_product_g0[1]) + (this.e43 * geometric_product_g0[2])) - float4((this.group3.yzx * geometric_product_g0.zxy), 0.0), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product_g0.xyz) + (geometric_product_g0.yzx * this.group4.zxy) + float3(float2(0.0), (geometric_product_g0[1] * this.e423) * -1.0) - (float3(geometric_product_g0[3]) * this.group1.xyz) - float3((geometric_product_g0.zx * this.group4.yz), 0.0), 
            /* e23, e31, e12 */
            (geometric_product_g0.zxy * this.group1.yzx) + float3(float2(0.0), (geometric_product_g0[0] * this.e2) * -1.0) - (float3(this.e321) * geometric_product_g0.xyz) - float3((geometric_product_g0.yz * this.group1.zx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), (this.e12 * geometric_product_g0[2]) * -1.0) + float4((float3(geometric_product_g0[3]) * this.group3.xyz), 0.0) + float4((this.group2.yzx * geometric_product_g0.zxy), 0.0) - (geometric_product_g0.xyzx * float4(float3(this.e1234), this.e23)) - (geometric_product_g0.yzxy * float4(this.group2.zxy, this.e31))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension MultiVector {
    public MultiVector geometric_quotient(Scalar other) {
        float geometric_product_g0 = 1.0 / other.scalar;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_product_g0) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_product_g0) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(geometric_product_g0) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_product_g0) * this.group4
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Origin> operator /(Origin lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(GeometricQuotientInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public Origin geometric_quotient(DualNum other) {
        return Origin(
            /* e4 */
            this.e4 / other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public Motor geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4((float4(other_g0) * other.group0).xyz, other_g0 * other.e321 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricQuotientInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public AntiScalar geometric_quotient(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * -1.0 / other.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricQuotientInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public Plane geometric_quotient(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(-(other.e23 * other.e23 * this.e4) - (other.e31 * other.e31 * this.e4) - (other.e12 * other.e12 * this.e4)) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public Flector geometric_quotient(Motor other) {
        float4 geometric_product_g1 = float4(other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product_g1[3] * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * geometric_product_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other_g0 * other.e321 * this.e4 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other_g0 * other.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(other_g0 * this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other_g0 * this.e4 * -1.0) * other.group3.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricQuotientInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public AntiScalar geometric_quotient(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * -1.0 / other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricQuotientInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public Line geometric_quotient(Point other) {
        return Line(
            /* e41, e42, e43 */
            (float3(this.e4) * float3(pow(other.e1, 3), pow(other.e2, 3), pow(other.e3, 3))) + (float3(other.e1 * this.e4) * float3(other.e2 * other.e2, other.e1 * other.e2, other.e1 * other.e3)) + (float3(other.e3 * this.e4) * float3(other.e1 * other.e3, other.e2 * other.e3, other.e2 * other.e2)), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricQuotientInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Origin {
    public Origin geometric_quotient(Scalar other) {
        return Origin(
            /* e4 */
            this.e4 / other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Plane> operator /(Plane lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Flector geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product_g0[1] * this.e321), 
            /* e423, e431, e412, e321 */
            float4(geometric_product_g0[0]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Motor geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), -(geometric_product_g0[1] * this.e431) - (geometric_product_g0[2] * this.e412)) + float4((float3(geometric_product_g1[3]) * this.group0.xyz), 0.0) + float4((geometric_product_g0.yzx * this.group0.zxy), 0.0) - (float4(this.e321) * float4(geometric_product_g1.xyz, geometric_product_g0[3])) - (geometric_product_g0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * -1.0) * float4(geometric_product_g0.xyz, geometric_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Motor geometric_quotient(Horizon other) {
        float geometric_product_g0 = -1.0 / other.e321;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(geometric_product_g0) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product_g0 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Flector geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        float3 geometric_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((geometric_product_g1 * float3(this.e321)), -(geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((geometric_product_g1.yzx * this.group0.zxy), 0.0) - float4((float3(other_g0 * this.e321) * other.group0.xyz), 0.0) - float4((geometric_product_g1.zxy * this.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Flector geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * geometric_product_g1.xyz), (geometric_product_g0[3] * this.e321) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product_g1[3]) * this.group0.xyz) + (float3(this.e321) * geometric_product_g0.xyz) + float3(float2(0.0), (geometric_product_g1[0] * this.e431) - (geometric_product_g1[1] * this.e423)) + float3((geometric_product_g1.yz * this.group0.zx), 0.0) - float3((geometric_product_g1.zx * this.group0.yz), 0.0)), geometric_product_g1[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float3 geometric_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g4[3] * this.e321 * -1.0, -(geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412) - (geometric_product_g1[3] * this.e321)), 
            /* e1, e2, e3, e4 */
            float4((geometric_product_g3 * float3(this.e321)), (geometric_product_g0[1] * this.e321) - (geometric_product_g3[0] * this.e423) - (geometric_product_g3[1] * this.e431) - (geometric_product_g3[2] * this.e412)), 
            /* e41, e42, e43 */
            (float3(geometric_product_g4[3]) * this.group0.xyz) + (geometric_product_g1.yzx * this.group0.zxy) + float3(float2(0.0), (geometric_product_g1[1] * this.e423) * -1.0) - (float3(this.e321) * geometric_product_g4.xyz) - float3((geometric_product_g1.zx * this.group0.yz), 0.0), 
            /* e23, e31, e12 */
            float3(this.e321 * -1.0) * geometric_product_g1.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product_g0[0]) * this.group0.xyz) + (geometric_product_g3.yzx * this.group0.zxy) - (float3(other_g0 * this.e321) * other.group2.xyz) - (geometric_product_g3.zxy * this.group0.yzx)), geometric_product_g0[0] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Motor geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(geometric_product_g0[3]) * this.group0.xyz) - (float3(this.e321) * geometric_product_g0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product_g0[3] * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Motor geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), -(geometric_product_g0[1] * this.e431) - (geometric_product_g0[2] * this.e412) - (geometric_product_g0[3] * this.e321)) + float4((geometric_product_g0.yzx * this.group0.zxy), 0.0) - (geometric_product_g0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321 * -1.0) * geometric_product_g0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricQuotientInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Plane {
    public Plane geometric_quotient(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(1.0 / other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Point> operator /(Point lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Flector geometric_quotient(DualNum other) {
        float2 geometric_product_g0 = float2(1.0 / other.scalar) * float2(1.0, other.e1234 / other.scalar);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(geometric_product_g0[0]) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(geometric_product_g0[1]) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Motor geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float4 geometric_product_g0 = float4(other_g0) * other.group0;
        float4 geometric_product_g1 = float4(other_g0 * -1.0) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(this.e4) * float4(geometric_product_g0.xyz, geometric_product_g1[3])) + (geometric_product_g1.zxyx * this.group0.yzxx) + float4(float3(0.0), (geometric_product_g1[1] * this.e2) + (geometric_product_g1[2] * this.e3)) - float4((float3(geometric_product_g0[3]) * this.group0.xyz), 0.0) - float4((geometric_product_g1.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, scalar */
            (geometric_product_g0.zxyx * this.group0.yzxx) + float4(float3(0.0), (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3)) - float4((float3(geometric_product_g1[3]) * this.group0.xyz), 0.0) - float4((geometric_product_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Motor geometric_quotient(Horizon other) {
        float geometric_product_g0 = -1.0 / other.e321;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_product_g0 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(geometric_product_g0 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Flector geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        float3 geometric_product_g0 = float3(other_g0 * -1.0) * other.group0.xyz;
        float3 geometric_product_g1 = float3(other_g0 * -1.0) * other.group1.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(geometric_product_g0[1] * this.e2) - (geometric_product_g0[2] * this.e3)) + float4((geometric_product_g1.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(geometric_product_g1.zxy, geometric_product_g0[0])), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(geometric_product_g1[1] * this.e2) - (geometric_product_g1[2] * this.e3)) + float4((geometric_product_g1 * float3(this.e4)), 0.0) + float4((geometric_product_g0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(geometric_product_g0.zxy, geometric_product_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Flector geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        float4 geometric_product_g0 = float4(other_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 geometric_product_g1 = float4(other_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g1[3]) * this.group0) + float4(float3(0.0), -(geometric_product_g0[1] * this.e2) - (geometric_product_g0[2] * this.e3)) + float4((geometric_product_g1.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(geometric_product_g1.zxy, geometric_product_g0[0])), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(geometric_product_g1[1] * this.e2) - (geometric_product_g1[2] * this.e3)) + float4((float3(geometric_product_g0[3]) * this.group0.xyz), 0.0) + float4((float3(this.e4) * geometric_product_g1.xyz), 0.0) + float4((geometric_product_g0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(geometric_product_g0.zxy, geometric_product_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        float2 geometric_product_g0 = float2(other_g0) * other.group0.xy;
        float4 geometric_product_g1 = float4(other_g0) * other.group1;
        float3 geometric_product_g2 = float3(other_g0 * -1.0) * other.group2.xyz;
        float3 geometric_product_g3 = float3(other_g0 * -1.0) * other.group3.xyz;
        float4 geometric_product_g4 = float4(other_g0 * -1.0) * other.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, geometric_product_g4[3] * this.e4) + (float2(this.e1) * float2(geometric_product_g1[0], geometric_product_g4[0])) + (float2(this.e2) * float2(geometric_product_g1[1], geometric_product_g4[1])) + (float2(this.e3) * float2(geometric_product_g1[2], geometric_product_g4[2])), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g0[0]) * this.group0) + float4(float3(0.0), -(geometric_product_g2[1] * this.e2) - (geometric_product_g2[2] * this.e3)) + float4((geometric_product_g3.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(geometric_product_g3.zxy, geometric_product_g2[0])), 
            /* e41, e42, e43 */
            (float3(this.e4) * geometric_product_g1.xyz) + float3(float2(0.0), (geometric_product_g4[1] * this.e1) - (geometric_product_g4[0] * this.e2)) + float3((geometric_product_g4.zx * this.group0.yz), 0.0) - (float3(geometric_product_g1[3]) * this.group0.xyz) - float3((geometric_product_g4.yz * this.group0.zx), 0.0), 
            /* e23, e31, e12 */
            (geometric_product_g1.zxy * this.group0.yzx) + float3(float2(0.0), (geometric_product_g1[0] * this.e2) * -1.0) - (float3(geometric_product_g4[3]) * this.group0.xyz) - float3((geometric_product_g1.yz * this.group0.zx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(geometric_product_g3[1] * this.e2) - (geometric_product_g3[2] * this.e3)) + float4((geometric_product_g3 * float3(this.e4)), 0.0) + float4((float3(geometric_product_g0[1]) * this.group0.xyz), 0.0) + float4((geometric_product_g2.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(geometric_product_g2.zxy, geometric_product_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Motor geometric_quotient(Plane other) {
        float4 geometric_product_g0 = float4(-1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (geometric_product_g0.zxyx * this.group0.yzxx) + float4(float3(0.0), (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3) + (geometric_product_g0[3] * this.e4)) - float4((geometric_product_g0.yzx * this.group0.zxy), 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(geometric_product_g0[3] * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Motor geometric_quotient(Point other) {
        float4 geometric_product_g0 = (float4(other.e1 * other.e1, other.e2 * other.e2, other.e3 * other.e3, other.e1 * other.e1) * other.group0) + (float4(other.e2 * other.e2, other.e1 * other.e1, other.e1 * other.e1, other.e2 * other.e2) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3), other.e2 * other.e2, other.e3 * other.e3));
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * geometric_product_g0.xyz) - (float3(geometric_product_g0[3]) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            (geometric_product_g0.zxyx * this.group0.yzxx) + float4(float3(0.0), (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3)) - float4((geometric_product_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricQuotientInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Point {
    public Point geometric_quotient(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(1.0 / other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public GeometricQuotientInfixPartial<Scalar> operator /(Scalar lhs, GeometricQuotientInfix rhs) {
    return GeometricQuotientInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricQuotientInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public DualNum geometric_quotient(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar / other.scalar) * float2(1.0, other.e1234 / other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricQuotientInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public Flector geometric_quotient(Flector other) {
        float other_g0 = other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other_g0 * this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(other_g0 * this.scalar * -1.0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricQuotientInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public Horizon geometric_quotient(Horizon other) {
        return Horizon(
            /* e321 */
            this.scalar * -1.0 / other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricQuotientInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public Line geometric_quotient(Line other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12;
        return Line(
            /* e41, e42, e43 */
            float3(other_g0 * this.scalar * -1.0) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * this.scalar * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricQuotientInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public Motor geometric_quotient(Motor other) {
        float other_g0 = other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.scalar * other.scalar;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other_g0 * this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other_g0 * this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricQuotientInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public MultiVector geometric_quotient(MultiVector other) {
        float other_g0 = other.scalar * other.scalar + other.e23 * other.e23 + other.e31 * other.e31 + other.e12 * other.e12 + other.e1 * other.e1 + other.e2 * other.e2 + other.e3 * other.e3 + other.e321 * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(other_g0 * this.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_g0 * this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(other_g0 * this.scalar * -1.0) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * this.scalar * -1.0) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_g0 * this.scalar * -1.0) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricQuotientInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public Plane geometric_quotient(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar * -1.0 / other.e321) * float4((float3(1.0 / other.e321) * other.group0.xyz), 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricQuotientInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public Point geometric_quotient(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            (float4(other.e1 * other.e1 * this.scalar, other.e2 * other.e2 * this.scalar, other.e3 * other.e3 * this.scalar, other.e1 * other.e1 * this.scalar) * other.group0) + (float4(other.e2 * other.e2 * this.scalar, other.e1 * other.e1 * this.scalar, other.e1 * other.e1 * this.scalar, other.e2 * other.e2 * this.scalar) * other.group0) + (other.group0 * float4(float2(other.e3 * other.e3 * this.scalar), other.e2 * other.e2 * this.scalar, other.e3 * other.e3 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricQuotientInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.geometric_quotient(rhs);
}
public extension Scalar {
    public Scalar geometric_quotient(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar / other.scalar
        );
    }
}
