implementing rga3d;
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiSandwichInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_sandwich(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public DualNum anti_sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e1234 * this.e1234) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Flector anti_sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234 * this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Horizon anti_sandwich(Horizon other) {
        return Horizon(
            /* e321 */
            this.e1234 * this.e1234 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Line anti_sandwich(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234 * this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234 * this.e1234) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Motor anti_sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234 * this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234 * this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e1234 * this.e1234) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e1234 * this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234 * this.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234 * this.e1234) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * this.e1234) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Origin anti_sandwich(Origin other) {
        return Origin(
            /* e4 */
            this.e1234 * this.e1234 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Plane anti_sandwich(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e1234 * this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Point anti_sandwich(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234 * this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension AntiScalar {
    public Scalar anti_sandwich(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<DualNum> operator /(DualNum lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public DualNum anti_sandwich(AntiScalar other) {
        float2 geometric_anti_product_g0 = float2(other.e1234) * this.group0.xy;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[0] * this.e1234) + (geometric_anti_product_g0[1] * this.scalar), geometric_anti_product_g0[1] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public DualNum anti_sandwich(DualNum other) {
        float geometric_anti_product_g0_y = other.e1234 * this.e1234;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0_y * this.scalar) + (this.e1234 * this.e1234 * other.scalar) + (other.e1234 * this.scalar * this.e1234), geometric_anti_product_g0_y * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Flector anti_sandwich(Flector other) {
        float geometric_anti_product_g0_w = this.e1234 * other.e4;
        float3 geometric_anti_product_g1_xyz = float3(this.e1234) * other.group1.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234 * this.e1234) * other.group0.xyz) + (float3(this.scalar * this.e1234) * other.group1.xyz) - (geometric_anti_product_g1_xyz * float3(this.scalar))), geometric_anti_product_g0_w * this.e1234), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g1_xyz * float3(float2(this.e1234), this.e1234)), (this.e1234 * this.e1234 * other.e321) + (this.scalar * this.e1234 * other.e4) - (geometric_anti_product_g0_w * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Horizon anti_sandwich(Horizon other) {
        return Horizon(
            /* e321 */
            this.e1234 * this.e1234 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Line anti_sandwich(Line other) {
        float3 geometric_anti_product_g0 = float3(this.e1234) * other.group0.xyz;
        return Line(
            /* e41, e42, e43 */
            geometric_anti_product_g0 * float3(this.e1234), 
            /* e23, e31, e12 */
            (geometric_anti_product_g0 * float3(this.scalar)) + (float3(this.e1234 * this.e1234) * other.group1.xyz) + (float3(this.scalar * this.e1234) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Motor anti_sandwich(Motor other) {
        float4 geometric_anti_product_g0 = float4(this.e1234) * other.group0;
        return Motor(
            /* e41, e42, e43, e1234 */
            geometric_anti_product_g0 * float4(this.e1234), 
            /* e23, e31, e12, scalar */
            (geometric_anti_product_g0 * float4(this.scalar)) + (float4(this.e1234 * this.e1234) * other.group1) + (float4(this.scalar * this.e1234) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public MultiVector anti_sandwich(MultiVector other) {
        float geometric_anti_product_g0_y = this.e1234 * other.e1234;
        float geometric_anti_product_g1_w = this.e1234 * other.e4;
        float3 geometric_anti_product_g2 = float3(this.e1234) * other.group2.xyz;
        float3 geometric_anti_product_g4_xyz = float3(this.e1234) * other.group4.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0_y * this.scalar) + (this.e1234 * this.e1234 * other.scalar) + (this.scalar * this.e1234 * other.e1234), geometric_anti_product_g0_y * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234 * this.e1234) * other.group1.xyz) + (float3(this.scalar * this.e1234) * other.group4.xyz) - (geometric_anti_product_g4_xyz * float3(this.scalar))), geometric_anti_product_g1_w * this.e1234), 
            /* e41, e42, e43 */
            geometric_anti_product_g2 * float3(this.e1234), 
            /* e23, e31, e12 */
            (geometric_anti_product_g2 * float3(this.scalar)) + (float3(this.e1234 * this.e1234) * other.group3.xyz) + (float3(this.scalar * this.e1234) * other.group2.xyz), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g4_xyz * float3(float2(this.e1234), this.e1234)), (this.e1234 * this.e1234 * other.e321) + (this.scalar * this.e1234 * other.e4) - (geometric_anti_product_g1_w * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiSandwichInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Origin anti_sandwich(Origin other) {
        return Origin(
            /* e4 */
            this.e1234 * this.e1234 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Plane anti_sandwich(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e1234 * this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Point anti_sandwich(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234 * this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Scalar anti_sandwich(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Flector> operator /(Flector lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public DualNum anti_sandwich(AntiScalar other) {
        float4 geometric_anti_product_g0 = float4(other.e1234) * this.group0;
        float4 geometric_anti_product_g1 = float4(other.e1234) * this.group1;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1[0] * this.e1) + (geometric_anti_product_g1[1] * this.e2) + (geometric_anti_product_g1[2] * this.e3) + (geometric_anti_product_g1[3] * this.e4), 0.0) + (float2(geometric_anti_product_g0[1], geometric_anti_product_g1[0]) * this.group1.yx) + (float2(geometric_anti_product_g0[2], geometric_anti_product_g1[1]) * this.group1.zy) + (float2(geometric_anti_product_g0[3], geometric_anti_product_g1[2]) * this.group1.wz) + (float2(this.e423, this.e4) * geometric_anti_product_g0.xw)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public DualNum anti_sandwich(DualNum other) {
        float3 geometric_anti_product_g0_xyz = (float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz);
        float3 geometric_anti_product_g1_xyz = float3(other.e1234) * this.group1.xyz;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1_xyz[0] * this.e1) + (geometric_anti_product_g1_xyz[1] * this.e2) + (geometric_anti_product_g1_xyz[2] * this.e3) - (this.e4 * this.e4 * other.scalar), 0.0) + (float2(this.e4) * float2(other.e1234 * this.e321, other.e1234 * this.e4)) + (float2(this.e423) * float2(geometric_anti_product_g0_xyz[0], geometric_anti_product_g1_xyz[0])) + (float2(this.e431) * float2(geometric_anti_product_g0_xyz[1], geometric_anti_product_g1_xyz[1])) + (float2(this.e412) * float2(geometric_anti_product_g0_xyz[2], geometric_anti_product_g1_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Flector anti_sandwich(Flector other) {
        float4 geometric_anti_product_g0 = float4(0.0, 0.0, -(other.e431 * this.e423) - (other.e412 * this.e4), 0.0) + (other.group1.yzxx * this.group1.zxyx) + float4((-(float2(other.e423) * float2(this.e4, this.e412)) - (float2(this.e431, this.e4) * other.group1.zy)), 0.0, 0.0) - (float4(other.e4) * float4(this.group1.xyz, this.e4));
        float4 geometric_anti_product_g1 = (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.yzxz * this.group0.zxyz) + float4(((float2(other.e1) * float2(this.e4, this.e412)) + (float2(this.e431, this.e4) * other.group0.zy) - (float2(other.e423) * float2(this.e321, this.e3)) - (float2(this.e2, this.e321) * other.group1.zy)), (other.e2 * this.e423) + (other.e3 * this.e4) - (other.e431 * this.e1) - (other.e412 * this.e321), (other.e423 * this.e1) + (other.e431 * this.e2) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431));
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e2) + (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g0[1] * this.e3) - (geometric_anti_product_g1[2] * this.e431), (geometric_anti_product_g0[1] * this.e321) + (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g0[2] * this.e1) - (geometric_anti_product_g1[1] * this.e4), (geometric_anti_product_g0[1] * this.e1) + (geometric_anti_product_g0[2] * this.e321) + (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g0[0] * this.e2) - (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[2] * this.e4)) + (float3(geometric_anti_product_g1[3]) * this.group1.xyz) + float3(((float2(geometric_anti_product_g0[0]) * float2(this.e321, this.e3)) - (float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412))), 0.0) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz)), geometric_anti_product_g0[3] * this.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0, 0.0, -(geometric_anti_product_g0[1] * this.e423) - (geometric_anti_product_g0[2] * this.e4), 0.0) + (geometric_anti_product_g0.yzxw * this.group1.zxyw) + float4(((float3(geometric_anti_product_g0[3]) * this.group1.xyz) + float3((-(float2(geometric_anti_product_g0[0]) * float2(this.e4, this.e412)) - (float2(this.e431, this.e4) * geometric_anti_product_g0.zy)), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Horizon anti_sandwich(Horizon other) {
        float4 geometric_anti_product_g1 = float4(other.e321) * float4(this.group1.xyz, this.e4);
        return Horizon(
            /* e321 */
            -(geometric_anti_product_g1[0] * this.e423) - (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g1[2] * this.e412) - (geometric_anti_product_g1[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Line anti_sandwich(Line other) {
        float3 geometric_anti_product_g0_xyz = (float3(this.e2, this.e321, this.e321) * other.group0.zyz) + (float3(this.e321, this.e3, this.e1) * other.group0.xxy) + (other.group1.yzx * this.group1.zxy) - (float3(this.e4, this.e412, this.e423) * other.group1.xxy) - (float3(this.e431, this.e4, this.e4) * other.group1.zyz) - (other.group0.yzx * this.group0.zxy);
        float3 geometric_anti_product_g1_xyz = (float3(this.e4, this.e412, this.e423) * other.group0.xxy) + (float3(this.e431, this.e4, this.e4) * other.group0.zyz) - (other.group0.yzx * this.group1.zxy);
        return Line(
            /* e41, e42, e43 */
            (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1_xyz.zyz) - (geometric_anti_product_g1_xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (float3(other.e41 * this.e1) * this.group1.xyz) + (float3(other.e42 * this.e2) * this.group1.xyz) + (float3(other.e43 * this.e3) * this.group1.xyz) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g1_xyz.zyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g0_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g0_xyz.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g1_xyz.xxy) + float3(-(float2(this.e423 * this.e423, this.e431 * this.e431) * other.group1.xyz.xy), other.e23 * this.e423 * this.e412 * -1.0) - (geometric_anti_product_g0_xyz.yzx * this.group1.zxy) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy) - (other.group1.yxy * float3(float2(this.e423 * this.e431), this.e431 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Motor anti_sandwich(Motor other) {
        float3 geometric_anti_product_g0_xyz = float3((this.e2 * other.e43) + (this.e412 * other.e31) - (this.e3 * other.e42) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) - (this.e1 * other.e43) - (this.e412 * other.e23), (this.e1 * other.e42) + (this.e3 * other.e1234) + (this.e431 * other.e23) - (this.e2 * other.e41) - (this.e423 * other.e31) - (this.e412 * other.scalar)) + (float3(this.e321) * other.group0.xyz) + float3(((float2(other.e1234) * this.group0.xy) - (float2(other.scalar) * this.group1.xy)), 0.0) - (float3(this.e4) * other.group1.xyz);
        float geometric_anti_product_g0_w = this.e4 * other.e1234;
        float4 geometric_anti_product_g1 = (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4((float3(this.e4) * other.group0.xyz), (this.e423 * other.e23) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) - (other.group0.yzxx * float4(this.group1.zxy, this.e1));
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(this.e431, this.e4, this.e4, this.e423) * geometric_anti_product_g1.zyzx) + float4((float3(((float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412)) - (geometric_anti_product_g1.yz * this.group1.zx)), (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[0] * this.e431)) - (float3(geometric_anti_product_g0_w) * this.group1.xyz)), geometric_anti_product_g0_w * this.e4), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4, this.e412, this.e423) * geometric_anti_product_g0_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g0_xyz.zyz) + float3(((float2(geometric_anti_product_g1[0]) * float2(this.e321, this.e3)) + (float2(this.e2, this.e321) * geometric_anti_product_g1.zy) - (geometric_anti_product_g1.yz * this.group0.zx)), (geometric_anti_product_g1[1] * this.e1) + (geometric_anti_product_g1[2] * this.e321) - (geometric_anti_product_g1[0] * this.e2)) - (float3(geometric_anti_product_g0_w) * this.group0.xyz) - (float3(geometric_anti_product_g1[3]) * this.group1.xyz) - (geometric_anti_product_g0_xyz.yzx * this.group1.zxy)), (geometric_anti_product_g0_w * this.e321) + (geometric_anti_product_g0_xyz[0] * this.e423) + (geometric_anti_product_g0_xyz[1] * this.e431) + (geometric_anti_product_g0_xyz[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public MultiVector anti_sandwich(MultiVector other) {
        float2 geometric_anti_product_g0 = float2((this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), this.e4 * other.e4 * -1.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412));
        float3 geometric_anti_product_g1_xyz = (float3(other.e1234) * this.group0.xyz) + (float3(this.e2, this.e321, this.e321) * other.group2.zyz) + (float3(this.e321, this.e3, this.e1) * other.group2.xxy) + (other.group3.yzx * this.group1.zxy) - (float3(other.scalar) * this.group1.xyz) - (float3(this.e4, this.e412, this.e423) * other.group3.xxy) - (float3(this.e431, this.e4, this.e4) * other.group3.zyz) - (other.group2.yzx * this.group0.zxy);
        float geometric_anti_product_g1_w = other.e1234 * this.e4;
        float3 geometric_anti_product_g2 = float3((this.e412 * other.e431) - (this.e431 * other.e412), (this.e423 * other.e412) - (this.e412 * other.e423), (this.e431 * other.e423) - (this.e423 * other.e431) - (this.e412 * other.e4)) + float3(-(float2(other.e4) * this.group1.xy), 0.0) - (float3(this.e4) * other.group4.xyz);
        float3 geometric_anti_product_g3 = float3((this.e3 * other.e431) + (this.e431 * other.e3) - (this.e2 * other.e412) - (this.e412 * other.e2), (this.e1 * other.e412) + (this.e412 * other.e1) - (this.e3 * other.e423) - (this.e423 * other.e3), (this.e2 * other.e423) + (this.e423 * other.e2) + (this.e412 * other.e321) - (this.e1 * other.e431) - (this.e3 * other.e4) - (this.e431 * other.e1)) + (float3(this.e4) * other.group1.xyz) + float3(((float2(other.e321) * this.group1.xy) - (float2(other.e4) * this.group0.xy)), 0.0) - (float3(this.e321) * other.group4.xyz);
        float4 geometric_anti_product_g4 = (float4(other.e1234) * this.group1) + float4(((float3(this.e4, this.e412, this.e423) * other.group2.xxy) + (float3(this.e431, this.e4, this.e4) * other.group2.zyz) - (other.group2.yzx * this.group1.zxy)), (other.e23 * this.e423) + (other.e31 * this.e431) - (other.scalar * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3));
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1_xyz[0] * this.e423) + (geometric_anti_product_g1_xyz[1] * this.e431) + (geometric_anti_product_g1_xyz[2] * this.e412) + (geometric_anti_product_g4[0] * this.e1) + (geometric_anti_product_g4[1] * this.e2) + (geometric_anti_product_g4[2] * this.e3) + (geometric_anti_product_g4[3] * this.e4), (geometric_anti_product_g4[0] * this.e423) + (geometric_anti_product_g4[1] * this.e431) + (geometric_anti_product_g4[2] * this.e412)) + (float2(geometric_anti_product_g1_w) * float2(this.e321, this.e4)), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0[0]) * this.group1.xyz) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (geometric_anti_product_g3.yzx * this.group1.zxy) - (float3(geometric_anti_product_g0[1]) * this.group0.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (geometric_anti_product_g2.yzx * this.group0.zxy)), geometric_anti_product_g0[1] * this.e4 * -1.0), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[2] * this.e431) - (geometric_anti_product_g4[1] * this.e412), (geometric_anti_product_g4[1] * this.e4) - (geometric_anti_product_g4[2] * this.e423), (geometric_anti_product_g4[1] * this.e423) + (geometric_anti_product_g4[2] * this.e4) - (geometric_anti_product_g4[0] * this.e431)) + float3((float2(geometric_anti_product_g4[0]) * float2(this.e4, this.e412)), 0.0) - (float3(geometric_anti_product_g1_w) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g4[2] * this.e2) - (geometric_anti_product_g4[1] * this.e3), (geometric_anti_product_g4[1] * this.e321) - (geometric_anti_product_g4[2] * this.e1), (geometric_anti_product_g4[1] * this.e1) + (geometric_anti_product_g4[2] * this.e321) - (geometric_anti_product_g4[0] * this.e2)) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1_xyz.zyz) + float3((float2(geometric_anti_product_g4[0]) * float2(this.e321, this.e3)), 0.0) - (float3(geometric_anti_product_g1_w) * this.group0.xyz) - (float3(geometric_anti_product_g4[3]) * this.group1.xyz) - (geometric_anti_product_g1_xyz.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product_g0[1]) * this.group1) + float4(((geometric_anti_product_g2.yzx * this.group1.zxy) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Point anti_sandwich(Origin other) {
        float4 geometric_anti_product_g0 = float4(other.e4 * -1.0) * float4(this.group1.xyz, this.e4);
        float4 geometric_anti_product_g1 = float4(other.e4 * -1.0) * float4(this.group0.xyz, this.e321);
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e2) + (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g0[1] * this.e3) - (geometric_anti_product_g1[2] * this.e431), (geometric_anti_product_g0[1] * this.e321) + (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g0[2] * this.e1) - (geometric_anti_product_g1[1] * this.e4), (geometric_anti_product_g0[1] * this.e1) + (geometric_anti_product_g0[2] * this.e321) + (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g0[0] * this.e2) - (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[2] * this.e4)) + (float3(geometric_anti_product_g1[3]) * this.group1.xyz) + float3(((float2(geometric_anti_product_g0[0]) * float2(this.e321, this.e3)) - (float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412))), 0.0) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz)), geometric_anti_product_g0[3] * this.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Plane anti_sandwich(Plane other) {
        float3 geometric_anti_product_g0_xyz = float3((this.e412 * other.e431) - (this.e431 * other.e412), (this.e423 * other.e412) - (this.e412 * other.e423), (this.e431 * other.e423) - (this.e423 * other.e431)) - (float3(this.e4) * other.group0.xyz);
        float geometric_anti_product_g0_w = this.e423 * other.e423;
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_anti_product_g0_w) * this.group1.xyz) + (geometric_anti_product_g0_xyz.yzx * this.group1.zxy) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g0_xyz.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g0_xyz.zyz)), geometric_anti_product_g0_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Point anti_sandwich(Point other) {
        float4 geometric_anti_product_g0 = float4(other.e4 * -1.0) * float4(this.group1.xyz, this.e4);
        float4 geometric_anti_product_g1 = float4(0.0, 0.0, (this.e423 * other.e2) - (this.e431 * other.e1), 0.0) + float4(((float3(this.e4) * other.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0) - (float3(other.e4) * this.group0.xyz)), 0.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e2) + (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g0[1] * this.e3) - (geometric_anti_product_g1[2] * this.e431), (geometric_anti_product_g0[1] * this.e321) + (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g0[2] * this.e1) - (geometric_anti_product_g1[1] * this.e4), (geometric_anti_product_g0[1] * this.e1) + (geometric_anti_product_g0[2] * this.e321) + (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g0[0] * this.e2) - (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[2] * this.e4)) + (float3(geometric_anti_product_g1[3]) * this.group1.xyz) + float3(((float2(geometric_anti_product_g0[0]) * float2(this.e321, this.e3)) - (float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412))), 0.0) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz)), geometric_anti_product_g0[3] * this.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Scalar anti_sandwich(Scalar other) {
        float3 geometric_anti_product_g0_xyz = float3(other.scalar * -1.0) * this.group1.xyz;
        return Scalar(
            /* scalar */
            (geometric_anti_product_g0_xyz[0] * this.e423) + (geometric_anti_product_g0_xyz[1] * this.e431) + (geometric_anti_product_g0_xyz[2] * this.e412) - (this.e4 * this.e4 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Line> operator /(Line lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public DualNum anti_sandwich(AntiScalar other) {
        float3 geometric_anti_product_g0 = float3(other.e1234) * this.group0.xyz;
        float3 geometric_anti_product_g1 = float3(other.e1234) * this.group1.xyz;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1[0] * this.e41) + (geometric_anti_product_g1[1] * this.e42) + (geometric_anti_product_g1[2] * this.e43), 0.0) + (float2(geometric_anti_product_g0[0]) * float2(this.e23, this.e41)) + (float2(geometric_anti_product_g0[1]) * float2(this.e31, this.e42)) + (float2(geometric_anti_product_g0[2]) * float2(this.e12, this.e43))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public DualNum anti_sandwich(DualNum other) {
        float3 geometric_anti_product_g0 = float3(other.e1234) * this.group0.xyz;
        float3 geometric_anti_product_g1 = (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz);
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1[0] * this.e41) + (geometric_anti_product_g1[1] * this.e42) + (geometric_anti_product_g1[2] * this.e43), 0.0) + (float2(geometric_anti_product_g0[0]) * float2(this.e23, this.e41)) + (float2(geometric_anti_product_g0[1]) * float2(this.e31, this.e42)) + (float2(geometric_anti_product_g0[2]) * float2(this.e12, this.e43))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Flector anti_sandwich(Flector other) {
        float3 geometric_anti_product_g0_xyz = (this.group0.xyx * float3(float2(other.e321), other.e2)) + (this.group0.yzz * float3(other.group0.zx, other.e321)) + (this.group1.xyx * float3(float2(other.e4), other.e431)) + (this.group1.yzz * float3(other.group1.zx, other.e4)) - (this.group0.zxy * other.group0.yzx) - (this.group1.zxy * other.group1.yzx);
        float3 geometric_anti_product_g1_xyz = (this.group0.xyx * float3(float2(other.e4), other.e431)) + (this.group0.yzz * float3(other.group1.zx, other.e4)) - (this.group0.zxy * other.group1.yzx);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((geometric_anti_product_g0_xyz.zxy * this.group0.yzx) + (geometric_anti_product_g1_xyz.yzx * this.group1.zxy) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy) - (geometric_anti_product_g1_xyz.zxy * this.group1.yzx)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((geometric_anti_product_g1_xyz.zxy * this.group0.yzx) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy)), (geometric_anti_product_g0_xyz[0] * this.e41) - (geometric_anti_product_g1_xyz[0] * this.e23) - (geometric_anti_product_g1_xyz[1] * this.e31))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Horizon anti_sandwich(Horizon other) {
        float3 geometric_anti_product_g0_xyz = float3(other.e321) * this.group0.xyz;
        return Horizon(
            /* e321 */
            (geometric_anti_product_g0_xyz[0] * this.e41) + (geometric_anti_product_g0_xyz[1] * this.e42) + (geometric_anti_product_g0_xyz[2] * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Line anti_sandwich(Line other) {
        float4 geometric_anti_product_g0 = float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0);
        float4 geometric_anti_product_g1 = float4(((other.group0.zxy * this.group1.yzx) + (other.group1.zxy * this.group0.yzx) - (other.group0.yzx * this.group1.zxy) - (other.group1.yzx * this.group0.zxy)), 0.0);
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return Line(
            /* e41, e42, e43 */
            (anti_reverse_g0.xxy * geometric_anti_product_g0.wzx) + (anti_reverse_g0.zyz * geometric_anti_product_g0.yww) - (anti_reverse_g0.yzx * geometric_anti_product_g0.zxy), 
            /* e23, e31, e12 */
            (anti_reverse_g0.xxy * geometric_anti_product_g1.wzx) + (anti_reverse_g0.zyz * geometric_anti_product_g1.yww) + (anti_reverse_g1.xxy * geometric_anti_product_g0.wzx) + (anti_reverse_g1.zyz * geometric_anti_product_g0.yww) - (anti_reverse_g0.yzx * geometric_anti_product_g1.zxy) - (anti_reverse_g1.yzx * geometric_anti_product_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Motor anti_sandwich(Motor other) {
        float4 geometric_anti_product_g0 = float4(((this.group0.xyx * other.group0.wwy) + (this.group0.yzz * other.group0.zxw) - (this.group0.zxy * other.group0.yzx)), 0.0);
        float4 geometric_anti_product_g1 = float4(((this.group0.xyx * other.group1.wwy) + (this.group0.yzz * other.group1.zxw) + (this.group1.xyx * other.group0.wwy) + (this.group1.yzz * other.group0.zxw) - (this.group0.zxy * other.group1.yzx) - (this.group1.zxy * other.group0.yzx)), 0.0);
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((anti_reverse_g0.xxy * geometric_anti_product_g0.wzx) + (anti_reverse_g0.zyz * geometric_anti_product_g0.yww) - (anti_reverse_g0.yzx * geometric_anti_product_g0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((anti_reverse_g0.xxy * geometric_anti_product_g1.wzx) + (anti_reverse_g0.zyz * geometric_anti_product_g1.yww) + (anti_reverse_g1.xxy * geometric_anti_product_g0.wzx) + (anti_reverse_g1.zyz * geometric_anti_product_g0.yww) - (anti_reverse_g0.yzx * geometric_anti_product_g1.zxy) - (anti_reverse_g1.yzx * geometric_anti_product_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public MultiVector anti_sandwich(MultiVector other) {
        float2 geometric_anti_product_g0 = float2(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43));
        float3 geometric_anti_product_g1_xyz = (this.group0.xyx * float3(float2(other.e321), other.e2)) + (this.group0.yzz * float3(other.group1.zx, other.e321)) + (this.group1.xyx * float3(float2(other.e4), other.e431)) + (this.group1.yzz * float3(other.group4.zx, other.e4)) - (this.group0.zxy * other.group1.yzx) - (this.group1.zxy * other.group4.yzx);
        float3 geometric_anti_product_g2 = (float3(other.e1234) * this.group0.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx);
        float3 geometric_anti_product_g3 = (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) + (this.group1.yzx * other.group2.zxy) - (this.group0.zxy * other.group3.yzx) - (this.group1.zxy * other.group2.yzx);
        float3 geometric_anti_product_g4_xyz = (this.group0.xyx * float3(float2(other.e4), other.e431)) + (this.group0.yzz * float3(other.group4.zx, other.e4)) - (this.group0.zxy * other.group4.yzx);
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(anti_reverse_g1[0] * geometric_anti_product_g2[0]) - (anti_reverse_g1[1] * geometric_anti_product_g2[1]) - (anti_reverse_g1[2] * geometric_anti_product_g2[2]), 0.0) - (float2(anti_reverse_g0[0]) * float2(geometric_anti_product_g3[0], geometric_anti_product_g2[0])) - (float2(anti_reverse_g0[1]) * float2(geometric_anti_product_g3[1], geometric_anti_product_g2[1])) - (float2(anti_reverse_g0[2]) * float2(geometric_anti_product_g3[2], geometric_anti_product_g2[2])), 
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g0.zxy * geometric_anti_product_g1_xyz.yzx) + (anti_reverse_g1.yzx * geometric_anti_product_g4_xyz.zxy) - (anti_reverse_g0.yzx * geometric_anti_product_g1_xyz.zxy) - (anti_reverse_g1.zxy * geometric_anti_product_g4_xyz.yzx)), 0.0), 
            /* e41, e42, e43 */
            (anti_reverse_g0 * float3(geometric_anti_product_g0[1])) + (anti_reverse_g0.zxy * geometric_anti_product_g2.yzx) - (anti_reverse_g0.yzx * geometric_anti_product_g2.zxy), 
            /* e23, e31, e12 */
            (anti_reverse_g0 * float3(geometric_anti_product_g0[0])) + (anti_reverse_g1 * float3(geometric_anti_product_g0[1])) + (anti_reverse_g0.zxy * geometric_anti_product_g3.yzx) + (anti_reverse_g1.zxy * geometric_anti_product_g2.yzx) - (anti_reverse_g0.yzx * geometric_anti_product_g3.zxy) - (anti_reverse_g1.yzx * geometric_anti_product_g2.zxy), 
            /* e423, e431, e412, e321 */
            float4(((anti_reverse_g0.zxy * geometric_anti_product_g4_xyz.yzx) - (anti_reverse_g0.yzx * geometric_anti_product_g4_xyz.zxy)), (anti_reverse_g1[0] * geometric_anti_product_g4_xyz[0]) - (anti_reverse_g0[0] * geometric_anti_product_g1_xyz[0]) - (anti_reverse_g0[1] * geometric_anti_product_g1_xyz[1]) - (anti_reverse_g0[2] * geometric_anti_product_g1_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Point anti_sandwich(Origin other) {
        float3 geometric_anti_product_g0_xyz = float3(other.e4) * this.group1.xyz;
        float3 geometric_anti_product_g1_xyz = float3(other.e4) * this.group0.xyz;
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g0.zxy * geometric_anti_product_g0_xyz.yzx) + (anti_reverse_g1.yzx * geometric_anti_product_g1_xyz.zxy) - (anti_reverse_g0.yzx * geometric_anti_product_g0_xyz.zxy) - (anti_reverse_g1.zxy * geometric_anti_product_g1_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Plane anti_sandwich(Plane other) {
        float3 geometric_anti_product_g1_xyz = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((geometric_anti_product_g1_xyz.zxy * this.group0.yzx) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy)), (this.e41 * this.e41 * other.e321) + (this.e41 * this.e31 * other.e412) - (geometric_anti_product_g1_xyz[0] * this.e23) - (geometric_anti_product_g1_xyz[1] * this.e31) - (this.e41 * this.e12 * other.e431))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Point anti_sandwich(Point other) {
        float3 geometric_anti_product_g0_xyz = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float3 geometric_anti_product_g1_xyz = float3(other.e4) * this.group0.xyz;
        float3 anti_reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 anti_reverse_g1 = this.group1.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g0.zxy * geometric_anti_product_g0_xyz.yzx) + (anti_reverse_g1.yzx * geometric_anti_product_g1_xyz.zxy) - (anti_reverse_g0 * float3(this.e41 * other.e1)) - (anti_reverse_g0 * float3(this.e42 * other.e2)) - (anti_reverse_g0 * float3(this.e43 * other.e3)) - (anti_reverse_g0.yzx * geometric_anti_product_g0_xyz.zxy) - (anti_reverse_g1.zxy * geometric_anti_product_g1_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Scalar anti_sandwich(Scalar other) {
        float3 geometric_anti_product_g1 = float3(other.scalar) * this.group0.xyz;
        return Scalar(
            /* scalar */
            (geometric_anti_product_g1[0] * this.e41) + (geometric_anti_product_g1[1] * this.e42) + (geometric_anti_product_g1[2] * this.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Motor> operator /(Motor lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public DualNum anti_sandwich(AntiScalar other) {
        float4 geometric_anti_product_g0 = float4(other.e1234) * this.group0;
        float4 geometric_anti_product_g1 = float4(other.e1234) * this.group1;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1[0] * this.e41) + (geometric_anti_product_g1[1] * this.e42) + (geometric_anti_product_g1[2] * this.e43) + (geometric_anti_product_g1[3] * this.e1234), 0.0) + (float2(geometric_anti_product_g0[0]) * float2(this.e23, this.e41)) + (float2(geometric_anti_product_g0[1]) * float2(this.e31, this.e42)) + (float2(geometric_anti_product_g0[2]) * float2(this.e12, this.e43)) + (float2(geometric_anti_product_g0[3]) * float2(this.scalar, this.e1234))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public DualNum anti_sandwich(DualNum other) {
        float4 geometric_anti_product_g0 = float4(other.e1234) * this.group0;
        float4 geometric_anti_product_g1 = (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1);
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1[0] * this.e41) + (geometric_anti_product_g1[1] * this.e42) + (geometric_anti_product_g1[2] * this.e43) + (geometric_anti_product_g1[3] * this.e1234), 0.0) + (float2(geometric_anti_product_g0[0]) * float2(this.e23, this.e41)) + (float2(geometric_anti_product_g0[1]) * float2(this.e31, this.e42)) + (float2(geometric_anti_product_g0[2]) * float2(this.e12, this.e43)) + (float2(geometric_anti_product_g0[3]) * float2(this.scalar, this.e1234))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Flector anti_sandwich(Flector other) {
        float3 geometric_anti_product_g0_xyz = float3((other.e3 * this.e42) + (other.e412 * this.e31) - (other.e2 * this.e43) - (other.e431 * this.e12), (other.e2 * this.e1234) + (other.e431 * this.scalar) - (other.e3 * this.e41) - (other.e412 * this.e23), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e431 * this.e23) + (other.e412 * this.scalar) - (other.e1 * this.e42) - (other.e423 * this.e31)) + (float3(other.e4) * this.group1.xyz) + (float3(other.e321) * this.group0.xyz) + float3(((float2(other.e1) * this.group0.wz) + (float2(other.e423) * this.group1.wz)), 0.0);
        float geometric_anti_product_g0_w = other.e4 * this.e1234;
        float4 geometric_anti_product_g1 = float4(0.0, 0.0, (other.e412 * this.e1234) - (other.e423 * this.e42), 0.0) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) + float4(((other.group1.zy * this.group0.yw) - (other.group1.yz * this.group0.zx)), 0.0, 0.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g1[1] * this.e12) - (geometric_anti_product_g1[2] * this.e31), (geometric_anti_product_g1[2] * this.e23) - (geometric_anti_product_g1[1] * this.scalar), (geometric_anti_product_g1[0] * this.e31) - (geometric_anti_product_g1[1] * this.e23) - (geometric_anti_product_g1[2] * this.scalar)) + (float3(geometric_anti_product_g0_w) * this.group1.xyz) + (geometric_anti_product_g0_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g0_xyz.zyz * this.group0.yww) + float3(-(float2(geometric_anti_product_g1[0]) * this.group1.wz), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy)), geometric_anti_product_g0_w * this.e1234), 
            /* e423, e431, e412, e321 */
            float4(((float2(geometric_anti_product_g1[0]) * this.group0.wz) + (geometric_anti_product_g1.zy * this.group0.yw)), (geometric_anti_product_g1[1] * this.e41) + (geometric_anti_product_g1[2] * this.e1234), (geometric_anti_product_g0_xyz[0] * this.e41) + (geometric_anti_product_g0_xyz[1] * this.e42) - (geometric_anti_product_g1[1] * this.e31) - (geometric_anti_product_g1[2] * this.e12)) - (float4(geometric_anti_product_g0_w) * float4(this.group0.xyz, this.scalar)) - (geometric_anti_product_g1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Horizon anti_sandwich(Horizon other) {
        float3 geometric_anti_product_g0_xyz = float3(other.e321) * this.group0.xyz;
        return Horizon(
            /* e321 */
            (geometric_anti_product_g0_xyz[0] * this.e41) + (geometric_anti_product_g0_xyz[1] * this.e42) + (geometric_anti_product_g0_xyz[2] * this.e43) + (this.e1234 * this.e1234 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Line anti_sandwich(Line other) {
        float3 geometric_anti_product_g0_xyz = (other.group0.xxy * this.group0.wzx) + (other.group0.zyz * this.group0.yww) - (other.group0.yzx * this.group0.zxy);
        float3 geometric_anti_product_g1_xyz = (other.group0.xxy * this.group1.wzx) + (other.group0.zyz * this.group1.yww) + (other.group1.xxy * this.group0.wzx) + (other.group1.zyz * this.group0.yww) - (other.group0.yzx * this.group1.zxy) - (other.group1.yzx * this.group0.zxy);
        return Line(
            /* e41, e42, e43 */
            (geometric_anti_product_g0_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g0_xyz.zyz * this.group0.yww) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (geometric_anti_product_g0_xyz.xxy * this.group1.wzx) + (geometric_anti_product_g0_xyz.zyz * this.group1.yww) + (geometric_anti_product_g1_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g1_xyz.zyz * this.group0.yww) - (geometric_anti_product_g0_xyz.yzx * this.group1.zxy) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Motor anti_sandwich(Motor other) {
        float3 geometric_anti_product_g0_xyz = float3((other.e43 * this.e42) - (other.e42 * this.e43), (other.e42 * this.e1234) - (other.e43 * this.e41), (other.e42 * this.e41) + (other.e43 * this.e1234) - (other.e41 * this.e42)) + (float3(other.e1234) * this.group0.xyz) + float3((float2(other.e41) * this.group0.wz), 0.0);
        float geometric_anti_product_g0_w = other.e1234 * this.e1234;
        float4 geometric_anti_product_g1 = (other.group0.xxyw * this.group1.wzxw) + float4(((float3(other.e1234) * this.group1.xyz) + (float3(other.scalar) * this.group0.xyz) + float3(((float2(other.e23) * this.group0.wz) + (other.group0.zy * this.group1.yw) + (other.group1.zy * this.group0.yw) - (other.group0.yz * this.group1.zx) - (other.group1.yz * this.group0.zx)), (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234) - (other.e41 * this.e31) - (other.e23 * this.e42))), other.scalar * this.e1234);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((geometric_anti_product_g0_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g0_xyz.zyz * this.group0.yww) - (float3(geometric_anti_product_g0_w) * this.group0.xyz) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy)), geometric_anti_product_g0_w * this.e1234), 
            /* e23, e31, e12, scalar */
            (this.group1.wzxw * float4(geometric_anti_product_g0_xyz.xxy, geometric_anti_product_g0_w)) + float4(((geometric_anti_product_g0_xyz.zyz * this.group1.yww) + float3(((float2(geometric_anti_product_g1[0]) * this.group0.wz) + (geometric_anti_product_g1.zy * this.group0.yw) - (geometric_anti_product_g1.yz * this.group0.zx)), (geometric_anti_product_g1[1] * this.e41) + (geometric_anti_product_g1[2] * this.e1234) - (geometric_anti_product_g1[0] * this.e42)) - (float3(geometric_anti_product_g0_w) * this.group1.xyz) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz) - (geometric_anti_product_g0_xyz.yzx * this.group1.zxy)), geometric_anti_product_g1[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public MultiVector anti_sandwich(MultiVector other) {
        float2 geometric_anti_product_g0 = float2((other.e1234 * this.scalar) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e41 * this.e41 * -1.0) + (float2(this.e1234) * other.group0.xy) - (float2(this.e23, this.e42) * other.group2.xyz.xy) - (float2(this.e31, this.e43) * other.group2.xyz.yz);
        float3 geometric_anti_product_g1_xyz = float3((this.e42 * other.e3) + (this.e31 * other.e412) - (this.e43 * other.e2) - (this.e12 * other.e431), (this.e43 * other.e1) + (this.e12 * other.e423) - (this.e41 * other.e3) - (this.e23 * other.e412), (this.e41 * other.e2) + (this.e43 * other.e321) + (this.e23 * other.e431) + (this.e12 * other.e4) - (this.e42 * other.e1) - (this.e31 * other.e423)) + (float3(this.e1234) * other.group1.xyz) + (float3(this.scalar) * other.group4.xyz) + float3(((float2(other.e4) * this.group1.xy) + (float2(other.e321) * this.group0.xy)), 0.0);
        float geometric_anti_product_g1_w = this.e1234 * other.e4;
        float3 geometric_anti_product_g2 = (float3(other.e1234) * this.group0.xyz) + (other.group2.xxy * this.group0.wzx) + (other.group2.zyz * this.group0.yww) - (other.group2.yzx * this.group0.zxy);
        float3 geometric_anti_product_g3 = (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xxy * this.group1.wzx) + (other.group2.zyz * this.group1.yww) + (other.group3.xxy * this.group0.wzx) + (other.group3.zyz * this.group0.yww) - (other.group2.yzx * this.group1.zxy) - (other.group3.yzx * this.group0.zxy);
        float4 geometric_anti_product_g4 = (this.group0.xyxw * float4(float2(other.e4), other.e431, other.e321)) + float4(((float3(this.e1234) * other.group4.xyz) + float3(((this.group0.yz * other.group4.zx) - (this.group0.zx * other.group4.yz)), (this.e43 * other.e4) - (this.e42 * other.e423))), this.scalar * other.e4);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[1] * this.scalar) + (geometric_anti_product_g2[2] * this.e12) + (geometric_anti_product_g3[0] * this.e41) + (geometric_anti_product_g3[1] * this.e42) + (geometric_anti_product_g3[2] * this.e43), geometric_anti_product_g2[0] * this.e41) + (geometric_anti_product_g0 * float2(this.e1234)) + (float2(this.e23, this.e42) * geometric_anti_product_g2.xy) + (float2(this.e31, this.e43) * geometric_anti_product_g2.yz), 
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g4[1] * this.e12) - (geometric_anti_product_g4[2] * this.e31), (geometric_anti_product_g4[2] * this.e23) - (geometric_anti_product_g4[1] * this.scalar), (geometric_anti_product_g4[0] * this.e31) - (geometric_anti_product_g4[1] * this.e23) - (geometric_anti_product_g4[2] * this.scalar)) + (float3(geometric_anti_product_g1_w) * this.group1.xyz) + (geometric_anti_product_g1_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g1_xyz.zyz * this.group0.yww) + float3(-(float2(geometric_anti_product_g4[0]) * this.group1.wz), 0.0) - (float3(geometric_anti_product_g4[3]) * this.group0.xyz) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy)), geometric_anti_product_g1_w * this.e1234), 
            /* e41, e42, e43 */
            (geometric_anti_product_g2.xxy * this.group0.wzx) + (geometric_anti_product_g2.zyz * this.group0.yww) - (float3(geometric_anti_product_g0[1]) * this.group0.xyz) - (geometric_anti_product_g2.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (geometric_anti_product_g2.xxy * this.group1.wzx) + (geometric_anti_product_g2.zyz * this.group1.yww) + (geometric_anti_product_g3.xxy * this.group0.wzx) + (geometric_anti_product_g3.zyz * this.group0.yww) - (float3(geometric_anti_product_g0[0]) * this.group0.xyz) - (float3(geometric_anti_product_g0[1]) * this.group1.xyz) - (geometric_anti_product_g2.yzx * this.group1.zxy) - (geometric_anti_product_g3.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(((float2(geometric_anti_product_g4[0]) * this.group0.wz) + (geometric_anti_product_g4.zy * this.group0.yw)), (geometric_anti_product_g4[1] * this.e41) + (geometric_anti_product_g4[2] * this.e1234), (geometric_anti_product_g1_xyz[0] * this.e41) + (geometric_anti_product_g1_xyz[1] * this.e42) - (geometric_anti_product_g4[1] * this.e31)) - (float4(geometric_anti_product_g1_w) * float4(this.group0.xyz, this.scalar)) - (geometric_anti_product_g4.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Point anti_sandwich(Origin other) {
        float4 geometric_anti_product_g0 = float4(other.e4) * float4(this.group1.xyz, this.e1234);
        float4 geometric_anti_product_g1 = float4(other.e4) * float4(this.group0.xyz, this.scalar);
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e42) + (geometric_anti_product_g1[1] * this.e12) - (geometric_anti_product_g0[1] * this.e43) - (geometric_anti_product_g1[2] * this.e31), (geometric_anti_product_g0[1] * this.e1234) + (geometric_anti_product_g1[2] * this.e23) - (geometric_anti_product_g0[2] * this.e41) - (geometric_anti_product_g1[1] * this.scalar), (geometric_anti_product_g0[1] * this.e41) + (geometric_anti_product_g0[2] * this.e1234) + (geometric_anti_product_g1[0] * this.e31) - (geometric_anti_product_g0[0] * this.e42) - (geometric_anti_product_g1[1] * this.e23) - (geometric_anti_product_g1[2] * this.scalar)) + (float3(geometric_anti_product_g0[3]) * this.group1.xyz) + float3(((float2(geometric_anti_product_g0[0]) * this.group0.wz) - (float2(geometric_anti_product_g1[0]) * this.group1.wz)), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz)), geometric_anti_product_g0[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Plane anti_sandwich(Plane other) {
        float4 geometric_anti_product_g0 = float4(0.0, 0.0, (this.e23 * other.e431) - (this.e31 * other.e423), 0.0) + float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.e321) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0)), 0.0);
        float3 geometric_anti_product_g1_xyz = float3((this.e42 * other.e412) - (this.e43 * other.e431), (this.e43 * other.e423) - (this.e41 * other.e412), (this.e41 * other.e431) - (this.e42 * other.e423)) + (float3(this.e1234) * other.group0.xyz);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((geometric_anti_product_g1_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g1_xyz.zyz * this.group0.yww) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy)), (geometric_anti_product_g0[0] * this.e41) + (geometric_anti_product_g0[1] * this.e42) - (geometric_anti_product_g1_xyz[0] * this.e23) - (geometric_anti_product_g1_xyz[1] * this.e31) - (geometric_anti_product_g1_xyz[2] * this.e12) - (geometric_anti_product_g0[3] * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Point anti_sandwich(Point other) {
        float3 geometric_anti_product_g0_xyz = float3((this.e42 * other.e3) - (this.e43 * other.e2), (this.e43 * other.e1) - (this.e41 * other.e3), (this.e41 * other.e2) - (this.e42 * other.e1)) + (float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz);
        float geometric_anti_product_g0_w = this.e1234 * other.e4;
        float3 geometric_anti_product_g1_xyz = float3(other.e4) * this.group0.xyz;
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.e41 * this.e41 * other.e1, this.e42 * this.e42 * other.e2, this.e43 * this.e43 * other.e3) + (float3(geometric_anti_product_g0_w) * this.group1.xyz) + (float3(this.e41) * float3(this.e42 * other.e2, this.e42 * other.e1, this.e43 * other.e1)) + (float3(this.e43) * float3(this.e41 * other.e3, this.e42 * other.e3, this.e42 * other.e2)) + (geometric_anti_product_g0_xyz.xxy * this.group0.wzx) + (geometric_anti_product_g0_xyz.zyz * this.group0.yww) + (geometric_anti_product_g1_xyz.yzx * this.group1.zxy) - (float3(this.scalar * other.e4) * this.group0.xyz) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy) - (geometric_anti_product_g1_xyz.xxy * this.group1.wzx) - (geometric_anti_product_g1_xyz.zyz * this.group1.yww)), geometric_anti_product_g0_w * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Scalar anti_sandwich(Scalar other) {
        float4 geometric_anti_product_g1 = float4(other.scalar) * this.group0;
        return Scalar(
            /* scalar */
            (geometric_anti_product_g1[0] * this.e41) + (geometric_anti_product_g1[1] * this.e42) + (geometric_anti_product_g1[2] * this.e43) + (geometric_anti_product_g1[3] * this.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<MultiVector> operator /(MultiVector lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(AntiScalar other) {
        float2 geometric_anti_product_g0 = float2(other.e1234) * this.group0.xy;
        float4 geometric_anti_product_g1 = float4(other.e1234) * this.group1;
        float3 geometric_anti_product_g2 = float3(other.e1234) * this.group2.xyz;
        float3 geometric_anti_product_g3 = float3(other.e1234) * this.group3.xyz;
        float4 geometric_anti_product_g4 = float4(other.e1234) * this.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[1] * this.scalar) + (geometric_anti_product_g2[2] * this.e12) + (geometric_anti_product_g3[0] * this.e41) + (geometric_anti_product_g3[1] * this.e42) + (geometric_anti_product_g3[2] * this.e43) + (geometric_anti_product_g1[0] * this.e423) + (geometric_anti_product_g1[1] * this.e431) + (geometric_anti_product_g1[2] * this.e412) + (geometric_anti_product_g1[3] * this.e321) + (geometric_anti_product_g4[0] * this.e1) + (geometric_anti_product_g4[1] * this.e2) + (geometric_anti_product_g4[2] * this.e3) + (geometric_anti_product_g4[3] * this.e4), (geometric_anti_product_g2[0] * this.e41) + (geometric_anti_product_g1[3] * this.e4) + (geometric_anti_product_g4[0] * this.e423) + (geometric_anti_product_g4[1] * this.e431) + (geometric_anti_product_g4[2] * this.e412)) + (geometric_anti_product_g0 * float2(this.e1234)) + (float2(this.e23, this.e42) * geometric_anti_product_g2.xy) + (float2(this.e31, this.e43) * geometric_anti_product_g2.yz), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            (geometric_anti_product_g4 * float4(this.e1234)) + (float4(geometric_anti_product_g0[1]) * this.group4) + float4(((geometric_anti_product_g2.yzx * this.group4.zxy) + (this.group2.yzx * geometric_anti_product_g4.zxy) - (float3(geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]) * this.group2.xxy) - (float3(geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]) * this.group2.zyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz)), (geometric_anti_product_g2[0] * this.e1) + (geometric_anti_product_g2[1] * this.e2) - (geometric_anti_product_g0[0] * this.e4) - (this.scalar * geometric_anti_product_g1[3]) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412) - (this.e23 * geometric_anti_product_g4[0]) - (this.e31 * geometric_anti_product_g4[1]) - (this.e12 * geometric_anti_product_g4[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(DualNum other) {
        float geometric_anti_product_g0_x = (other.scalar * this.e1234) + (other.e1234 * this.scalar);
        float geometric_anti_product_g0_y = other.e1234 * this.e1234;
        float3 geometric_anti_product_g1_xyz = (float3(other.e1234) * this.group1.xyz) - (float3(other.scalar) * this.group4.xyz);
        float geometric_anti_product_g1_w = other.e1234 * this.e4;
        float3 geometric_anti_product_g2 = float3(other.e1234) * this.group2.xyz;
        float3 geometric_anti_product_g3 = (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz);
        float3 geometric_anti_product_g4_xyz = float3(other.e1234) * this.group4.xyz;
        float geometric_anti_product_g4_w = (other.e1234 * this.e321) - (other.scalar * this.e4);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0_y * this.scalar) + (geometric_anti_product_g1_w * this.e321) + (geometric_anti_product_g4_w * this.e4) + (geometric_anti_product_g1_xyz[0] * this.e423) + (geometric_anti_product_g2[0] * this.e23) + (geometric_anti_product_g2[1] * this.e31) + (geometric_anti_product_g2[2] * this.e12) + (geometric_anti_product_g3[0] * this.e41) + (geometric_anti_product_g3[1] * this.e42) + (geometric_anti_product_g3[2] * this.e43) + (geometric_anti_product_g4_xyz[0] * this.e1) + (geometric_anti_product_g4_xyz[1] * this.e2) + (geometric_anti_product_g4_xyz[2] * this.e3), (geometric_anti_product_g1_w * this.e4) + (geometric_anti_product_g2[0] * this.e41) + (geometric_anti_product_g2[1] * this.e42) + (geometric_anti_product_g2[2] * this.e43) + (geometric_anti_product_g4_xyz[2] * this.e412)) + (float2(this.e1234) * float2(geometric_anti_product_g0_x, geometric_anti_product_g0_y)) + (float2(geometric_anti_product_g1_xyz[1], geometric_anti_product_g4_xyz[0]) * this.group4.yx) + (float2(geometric_anti_product_g1_xyz[2], geometric_anti_product_g4_xyz[1]) * this.group4.zy), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product_g0_y) * this.group4) + (float4(this.e1234) * float4(geometric_anti_product_g4_xyz, geometric_anti_product_g4_w)) + float4(((geometric_anti_product_g2.yzx * this.group4.zxy) + (geometric_anti_product_g4_xyz.zxy * this.group2.yzx) - (float3(geometric_anti_product_g1_w) * this.group2.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz) - (geometric_anti_product_g4_xyz.yzx * this.group2.zxy)), (geometric_anti_product_g1_xyz[0] * this.e41) + (geometric_anti_product_g1_xyz[1] * this.e42) - (geometric_anti_product_g0_x * this.e4) - (geometric_anti_product_g1_w * this.scalar) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412) - (geometric_anti_product_g4_xyz[0] * this.e23) - (geometric_anti_product_g4_xyz[1] * this.e31) - (geometric_anti_product_g4_xyz[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Flector other) {
        float2 geometric_anti_product_g0 = float2((other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), other.e4 * this.e4 * -1.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412));
        float3 geometric_anti_product_g1_xyz = (float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz) + (this.group2.xyx * float3(float2(other.e321), other.e2)) + (this.group2.yzz * float3(other.group0.zx, other.e321)) + (this.group3.xyx * float3(float2(other.e4), other.e431)) + (this.group3.yzz * float3(other.group1.zx, other.e4)) - (this.group2.zxy * other.group0.yzx) - (this.group3.zxy * other.group1.yzx);
        float geometric_anti_product_g1_w = this.e1234 * other.e4;
        float3 geometric_anti_product_g2 = float3((other.e431 * this.e412) - (other.e412 * this.e431), (other.e412 * this.e423) - (other.e431 * this.e4), (other.e423 * this.e431) - (other.e431 * this.e423) - (other.e412 * this.e4)) + float3(-(float2(other.e423) * float2(this.e4, this.e412)), 0.0) - (float3(other.e4) * this.group4.xyz);
        float3 geometric_anti_product_g3 = float3((other.e3 * this.e431) + (other.e431 * this.e3) - (other.e2 * this.e412) - (other.e412 * this.e2), (other.e2 * this.e4) + (other.e412 * this.e1) - (other.e3 * this.e423) - (other.e431 * this.e321), (other.e2 * this.e423) + (other.e3 * this.e4) + (other.e423 * this.e2) - (other.e1 * this.e431) - (other.e431 * this.e1) - (other.e412 * this.e321)) + (float3(other.e321) * this.group4.xyz) + float3(((float2(other.e1) * float2(this.e4, this.e412)) - (float2(other.e423) * float2(this.e321, this.e3))), 0.0) - (float3(other.e4) * this.group1.xyz);
        float3 geometric_anti_product_g4_xyz = (float3(this.e1234) * other.group1.xyz) + (this.group2.xyx * float3(float2(other.e4), other.e431)) + (this.group2.yzz * float3(other.group1.zx, other.e4)) - (this.group2.zxy * other.group1.yzx);
        float geometric_anti_product_g4_w = (this.scalar * other.e4) + (this.e1234 * other.e321);
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 anti_reverse_g3 = this.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g4_w * this.e4) + (geometric_anti_product_g0[0] * this.e1234) + (geometric_anti_product_g0[1] * this.scalar) + (geometric_anti_product_g1_xyz[1] * this.e431) + (geometric_anti_product_g1_xyz[2] * this.e412) + (geometric_anti_product_g4_xyz[0] * this.e1) + (geometric_anti_product_g4_xyz[1] * this.e2) + (geometric_anti_product_g4_xyz[2] * this.e3) - (anti_reverse_g2[0] * geometric_anti_product_g3[0]) - (anti_reverse_g2[1] * geometric_anti_product_g3[1]) - (anti_reverse_g2[2] * geometric_anti_product_g3[2]) - (anti_reverse_g3[0] * geometric_anti_product_g2[0]) - (anti_reverse_g3[1] * geometric_anti_product_g2[1]) - (anti_reverse_g3[2] * geometric_anti_product_g2[2]), (geometric_anti_product_g0[1] * this.e1234) + (geometric_anti_product_g4_xyz[0] * this.e423) + (geometric_anti_product_g4_xyz[1] * this.e431) - (anti_reverse_g2[0] * geometric_anti_product_g2[0]) - (anti_reverse_g2[1] * geometric_anti_product_g2[1]) - (anti_reverse_g2[2] * geometric_anti_product_g2[2])) + (float2(geometric_anti_product_g1_w) * float2(this.e321, this.e4)) + (float2(geometric_anti_product_g1_xyz[0], geometric_anti_product_g4_xyz[2]) * this.group4.xz), 
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g2 * float3(geometric_anti_product_g4_w)) + (geometric_anti_product_g1_xyz * float3(this.e1234)) + (float3(geometric_anti_product_g0[0]) * this.group4.xyz) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (anti_reverse_g2.zxy * geometric_anti_product_g1_xyz.yzx) + (anti_reverse_g3.yzx * geometric_anti_product_g4_xyz.zxy) + (geometric_anti_product_g3.yzx * this.group4.zxy) - (anti_reverse_g3 * float3(geometric_anti_product_g1_w)) - (geometric_anti_product_g4_xyz * float3(this.scalar)) - (float3(geometric_anti_product_g0[1]) * this.group1.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g1_xyz.zxy) - (anti_reverse_g3.zxy * geometric_anti_product_g4_xyz.yzx) - (geometric_anti_product_g2.yzx * this.group1.zxy)), geometric_anti_product_g1_w * this.e1234), 
            /* e41, e42, e43 */
            (anti_reverse_g2 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.e1234)) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g4_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g4_xyz.zyz) + (anti_reverse_g2.zxy * geometric_anti_product_g2.yzx) - (float3(geometric_anti_product_g1_w) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g2.zxy) - (geometric_anti_product_g4_xyz.yzx * this.group4.zxy), 
            /* e23, e31, e12 */
            (anti_reverse_g2 * float3(geometric_anti_product_g0[0])) + (anti_reverse_g3 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.scalar)) + (geometric_anti_product_g3 * float3(this.e1234)) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g4_xyz.zyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1_xyz.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g4_xyz.xxy) + (anti_reverse_g2.zxy * geometric_anti_product_g3.yzx) + (anti_reverse_g3.zxy * geometric_anti_product_g2.yzx) - (float3(geometric_anti_product_g1_w) * this.group1.xyz) - (float3(geometric_anti_product_g4_w) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g3.zxy) - (anti_reverse_g3.yzx * geometric_anti_product_g2.zxy) - (geometric_anti_product_g1_xyz.yzx * this.group4.zxy) - (geometric_anti_product_g4_xyz.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product_g0[1]) * this.group4) + float4(((anti_reverse_g2 * float3(geometric_anti_product_g1_w)) + (geometric_anti_product_g4_xyz * float3(this.e1234)) + (anti_reverse_g2.zxy * geometric_anti_product_g4_xyz.yzx) + (geometric_anti_product_g2.yzx * this.group4.zxy) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g4_xyz.zxy)), (geometric_anti_product_g4_w * this.e1234) + (anti_reverse_g3[0] * geometric_anti_product_g4_xyz[0]) + (anti_reverse_g3[1] * geometric_anti_product_g4_xyz[1]) + (anti_reverse_g3[2] * geometric_anti_product_g4_xyz[2]) - (geometric_anti_product_g1_w * this.scalar) - (geometric_anti_product_g0[0] * this.e4) - (anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0]) - (anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1]) - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2]) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Horizon other) {
        float geometric_anti_product_g0_x = this.e4 * other.e321;
        float3 geometric_anti_product_g1_xyz = float3(other.e321) * this.group2.xyz;
        float3 geometric_anti_product_g3 = float3(other.e321) * this.group4.xyz;
        float geometric_anti_product_g4_w = this.e1234 * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0_x * this.e1234) + (geometric_anti_product_g4_w * this.e4) + (geometric_anti_product_g1_xyz[0] * this.e423) + (geometric_anti_product_g1_xyz[1] * this.e431) + (geometric_anti_product_g1_xyz[2] * this.e412) + (geometric_anti_product_g3[0] * this.e41) + (geometric_anti_product_g3[1] * this.e42) + (geometric_anti_product_g3[2] * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), (geometric_anti_product_g4_w * this.e1234) + (geometric_anti_product_g1_xyz[0] * this.e41) + (geometric_anti_product_g1_xyz[1] * this.e42) + (geometric_anti_product_g1_xyz[2] * this.e43) - (geometric_anti_product_g0_x * this.e4) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Line other) {
        float2 geometric_anti_product_g0 = float2(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43));
        float3 geometric_anti_product_g1_xyz = (float3(this.e2, this.e321, this.e321) * other.group0.zyz) + (float3(this.e321, this.e3, this.e1) * other.group0.xxy) + (other.group1.yzx * this.group4.zxy) - (float3(this.e4, this.e412, this.e423) * other.group1.xxy) - (float3(this.e431, this.e4, this.e4) * other.group1.zyz) - (other.group0.yzx * this.group1.zxy);
        float3 geometric_anti_product_g2 = (float3(this.e1234) * other.group0.xyz) + (other.group0.zxy * this.group2.yzx) - (other.group0.yzx * this.group2.zxy);
        float3 geometric_anti_product_g3 = (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) + (other.group1.zxy * this.group2.yzx) - (other.group0.yzx * this.group3.zxy) - (other.group1.yzx * this.group2.zxy);
        float3 geometric_anti_product_g4_xyz = (float3(this.e4, this.e412, this.e423) * other.group0.xxy) + (float3(this.e431, this.e4, this.e4) * other.group0.zyz) - (other.group0.yzx * this.group4.zxy);
        float geometric_anti_product_g4_w = (other.e23 * this.e423) + (other.e31 * this.e431) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3);
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 anti_reverse_g3 = this.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g2 * float3(geometric_anti_product_g4_w)) + (geometric_anti_product_g1_xyz * float3(this.e1234)) + (float3(geometric_anti_product_g0[0]) * this.group4.xyz) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (anti_reverse_g2.zxy * geometric_anti_product_g1_xyz.yzx) + (anti_reverse_g3.yzx * geometric_anti_product_g4_xyz.zxy) + (geometric_anti_product_g3.yzx * this.group4.zxy) - (geometric_anti_product_g4_xyz * float3(this.scalar)) - (float3(geometric_anti_product_g0[1]) * this.group1.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g1_xyz.zxy) - (anti_reverse_g3.zxy * geometric_anti_product_g4_xyz.yzx) - (geometric_anti_product_g2.yzx * this.group1.zxy)), geometric_anti_product_g0[1] * this.e4 * -1.0), 
            /* e41, e42, e43 */
            (anti_reverse_g2 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.e1234)) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g4_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g4_xyz.zyz) + (anti_reverse_g2.zxy * geometric_anti_product_g2.yzx) - (anti_reverse_g2.yzx * geometric_anti_product_g2.zxy) - (geometric_anti_product_g4_xyz.yzx * this.group4.zxy), 
            /* e23, e31, e12 */
            (anti_reverse_g2 * float3(geometric_anti_product_g0[0])) + (anti_reverse_g3 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.scalar)) + (geometric_anti_product_g3 * float3(this.e1234)) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g4_xyz.zyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1_xyz.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g4_xyz.xxy) + (anti_reverse_g2.zxy * geometric_anti_product_g3.yzx) + (anti_reverse_g3.zxy * geometric_anti_product_g2.yzx) - (float3(geometric_anti_product_g4_w) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g3.zxy) - (anti_reverse_g3.yzx * geometric_anti_product_g2.zxy) - (geometric_anti_product_g1_xyz.yzx * this.group4.zxy) - (geometric_anti_product_g4_xyz.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Motor other) {
        float2 geometric_anti_product_g0 = float2((this.e1234 * other.scalar) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), this.e41 * other.e41 * -1.0) + (float2(other.e1234) * this.group0.xy) - (float2(other.e23, other.e42) * this.group2.xyz.xy) - (float2(other.e31, other.e43) * this.group2.xyz.yz);
        float3 geometric_anti_product_g1_xyz = float3((other.e43 * this.e2) + (other.e31 * this.e412) - (other.e42 * this.e3) - (other.e12 * this.e431), (other.e42 * this.e321) + (other.e12 * this.e423) - (other.e43 * this.e1) - (other.e31 * this.e4), (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431) - (other.e41 * this.e2) - (other.e31 * this.e423) - (other.e12 * this.e4)) + (float3(other.e1234) * this.group1.xyz) + float3(((float2(other.e41) * float2(this.e321, this.e3)) - (float2(other.e23) * float2(this.e4, this.e412))), 0.0) - (float3(other.scalar) * this.group4.xyz);
        float geometric_anti_product_g1_w = other.e1234 * this.e4;
        float3 geometric_anti_product_g2 = (float3(this.e1234) * other.group0.xyz) + (this.group2.xyx * other.group0.wwy) + (this.group2.yzz * other.group0.zxw) - (this.group2.zxy * other.group0.yzx);
        float3 geometric_anti_product_g3 = (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xyx * other.group1.wwy) + (this.group2.yzz * other.group1.zxw) + (this.group3.xyx * other.group0.wwy) + (this.group3.yzz * other.group0.zxw) - (this.group2.zxy * other.group1.yzx) - (this.group3.zxy * other.group0.yzx);
        float4 geometric_anti_product_g4 = (float4(this.e4, this.e412, this.e423, this.e321) * other.group0.xxyw) + (this.group4.xyzy * float4(float3(other.e1234), other.e31)) + float4((float2(this.e431, this.e4) * other.group0.zy), other.e43 * this.e4, (other.e23 * this.e423) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4)) - (other.group0.yzxx * float4(this.group4.zxy, this.e1));
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 anti_reverse_g3 = this.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[1] * this.scalar) + (geometric_anti_product_g1_xyz[0] * this.e423) + (geometric_anti_product_g1_xyz[1] * this.e431) + (geometric_anti_product_g1_xyz[2] * this.e412) + (geometric_anti_product_g4[0] * this.e1) + (geometric_anti_product_g4[1] * this.e2) + (geometric_anti_product_g4[2] * this.e3) + (geometric_anti_product_g4[3] * this.e4) - (anti_reverse_g2[0] * geometric_anti_product_g3[0]) - (anti_reverse_g2[1] * geometric_anti_product_g3[1]) - (anti_reverse_g2[2] * geometric_anti_product_g3[2]) - (anti_reverse_g3[0] * geometric_anti_product_g2[0]) - (anti_reverse_g3[1] * geometric_anti_product_g2[1]) - (anti_reverse_g3[2] * geometric_anti_product_g2[2]), (geometric_anti_product_g4[0] * this.e423) + (geometric_anti_product_g4[1] * this.e431) + (geometric_anti_product_g4[2] * this.e412) - (anti_reverse_g2[0] * geometric_anti_product_g2[0]) - (anti_reverse_g2[1] * geometric_anti_product_g2[1]) - (anti_reverse_g2[2] * geometric_anti_product_g2[2])) + (geometric_anti_product_g0 * float2(this.e1234)) + (float2(geometric_anti_product_g1_w) * float2(this.e321, this.e4)), 
            /* e1, e2, e3, e4 */
            float4(((geometric_anti_product_g1_xyz * float3(this.e1234)) + (float3(geometric_anti_product_g0[0]) * this.group4.xyz) + (float3(geometric_anti_product_g1_xyz[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]) * anti_reverse_g2.zyz) + (float3(geometric_anti_product_g4[3], geometric_anti_product_g1_xyz[2], geometric_anti_product_g1_xyz[0]) * anti_reverse_g2.xxy) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (anti_reverse_g3.yzx * geometric_anti_product_g4.zxy) + (geometric_anti_product_g3.yzx * this.group4.zxy) - (anti_reverse_g3 * float3(geometric_anti_product_g1_w)) - (float3(geometric_anti_product_g0[1]) * this.group1.xyz) - (float3(this.scalar) * geometric_anti_product_g4.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g1_xyz.zxy) - (anti_reverse_g3.zxy * geometric_anti_product_g4.yzx) - (geometric_anti_product_g2.yzx * this.group1.zxy)), geometric_anti_product_g1_w * this.e1234), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[2] * this.e431) - (geometric_anti_product_g4[1] * this.e412), (geometric_anti_product_g4[1] * this.e4) - (geometric_anti_product_g4[2] * this.e423), (geometric_anti_product_g4[1] * this.e423) + (geometric_anti_product_g4[2] * this.e4) - (geometric_anti_product_g4[0] * this.e431)) + (anti_reverse_g2 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.e1234)) + (anti_reverse_g2.zxy * geometric_anti_product_g2.yzx) + float3((float2(geometric_anti_product_g4[0]) * float2(this.e4, this.e412)), 0.0) - (float3(geometric_anti_product_g1_w) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g4[2] * this.e2) - (geometric_anti_product_g4[1] * this.e3), (geometric_anti_product_g4[1] * this.e321) - (geometric_anti_product_g4[2] * this.e1), (geometric_anti_product_g4[1] * this.e1) + (geometric_anti_product_g4[2] * this.e321) - (geometric_anti_product_g4[0] * this.e2)) + (anti_reverse_g2 * float3(geometric_anti_product_g0[0])) + (anti_reverse_g3 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.scalar)) + (geometric_anti_product_g3 * float3(this.e1234)) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1_xyz.zyz) + (anti_reverse_g2.zxy * geometric_anti_product_g3.yzx) + (anti_reverse_g3.zxy * geometric_anti_product_g2.yzx) + float3((float2(geometric_anti_product_g4[0]) * float2(this.e321, this.e3)), 0.0) - (float3(geometric_anti_product_g1_w) * this.group1.xyz) - (float3(geometric_anti_product_g4[3]) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g3.zxy) - (anti_reverse_g3.yzx * geometric_anti_product_g2.zxy) - (geometric_anti_product_g1_xyz.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_anti_product_g4.xyzx * float4(float3(this.e1234), anti_reverse_g3[0])) + (geometric_anti_product_g4.yzxy * float4(anti_reverse_g2.zxy, anti_reverse_g3[1])) + float4(((anti_reverse_g2 * float3(geometric_anti_product_g1_w)) + (float3(geometric_anti_product_g0[1]) * this.group4.xyz) + (geometric_anti_product_g2.yzx * this.group4.zxy) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g4.zxy)), (geometric_anti_product_g0[1] * this.e321) + (this.e1234 * geometric_anti_product_g4[3]) + (anti_reverse_g3[2] * geometric_anti_product_g4[2]) - (geometric_anti_product_g1_w * this.scalar) - (geometric_anti_product_g0[0] * this.e4) - (anti_reverse_g2[0] * geometric_anti_product_g1_xyz[0]) - (anti_reverse_g2[1] * geometric_anti_product_g1_xyz[1]) - (anti_reverse_g2[2] * geometric_anti_product_g1_xyz[2]) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(MultiVector other) {
        float2 geometric_anti_product_g0 = float2((other.e1234 * this.scalar) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), (other.e423 * this.e423) - (other.e41 * this.e41) - (other.e42 * this.e42) - (other.e4 * this.e4)) + (float2(this.e1234) * other.group0.xy) + (float2(this.e1, this.e431) * other.group4.xy) + (float2(this.e2, this.e412) * other.group4.yz) - (float2(this.e23, this.e43) * other.group2.xyz.xz);
        float4 geometric_anti_product_g1 = (float4(other.e1234) * this.group1) + float4(((float3(this.scalar) * other.group4.xyz) + (float3(this.e1234) * other.group1.xyz) + (float3(this.e2, this.e321, this.e321) * other.group2.zyz) + (float3(this.e321, this.e3, this.e1) * other.group2.xxy) + (other.group3.yzx * this.group4.zxy) + (this.group2.xyx * float3(float2(other.e321), other.e2)) + (this.group2.yzz * float3(other.group1.zx, other.e321)) + (this.group3.xyx * float3(float2(other.e4), other.e431)) + (this.group3.yzz * float3(other.group4.zx, other.e4)) - (float3(other.scalar) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group3.xxy) - (float3(this.e431, this.e4, this.e4) * other.group3.zyz) - (other.group2.yzx * this.group1.zxy) - (this.group2.zxy * other.group1.yzx) - (this.group3.zxy * other.group4.yzx)), this.e1234 * other.e4);
        float3 geometric_anti_product_g2 = float3((other.e431 * this.e412) - (other.e412 * this.e431), (other.e412 * this.e423) - (other.e431 * this.e4), (other.e423 * this.e431) - (other.e431 * this.e423) - (other.e412 * this.e4)) + (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group2.zxy * this.group2.yzx) + float3(-(float2(other.e423) * float2(this.e4, this.e412)), 0.0) - (float3(other.e4) * this.group4.xyz) - (other.group2.yzx * this.group2.zxy);
        float3 geometric_anti_product_g3 = float3((other.e3 * this.e431) + (other.e431 * this.e3) - (other.e2 * this.e412) - (other.e412 * this.e2), (other.e2 * this.e4) + (other.e412 * this.e1) - (other.e3 * this.e423) - (other.e431 * this.e321), (other.e2 * this.e423) + (other.e3 * this.e4) + (other.e423 * this.e2) - (other.e1 * this.e431) - (other.e431 * this.e1) - (other.e412 * this.e321)) + (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (other.group2.zxy * this.group3.yzx) + (other.group3.zxy * this.group2.yzx) + float3(((float2(other.e1) * float2(this.e4, this.e412)) - (float2(other.e423) * float2(this.e321, this.e3))), 0.0) - (float3(other.e4) * this.group1.xyz) - (other.group2.yzx * this.group3.zxy) - (other.group3.yzx * this.group2.zxy);
        float4 geometric_anti_product_g4 = (float4(other.e1234) * this.group4) + float4(((float3(this.e1234) * other.group4.xyz) + (float3(this.e4, this.e412, this.e423) * other.group2.xxy) + (float3(this.e431, this.e4, this.e4) * other.group2.zyz) + (this.group2.xyx * float3(float2(other.e4), other.e431)) + (this.group2.yzz * float3(other.group4.zx, other.e4)) - (other.group2.yzx * this.group4.zxy) - (this.group2.zxy * other.group4.yzx)), (this.scalar * other.e4) + (this.e1234 * other.e321) + (other.e23 * this.e423) + (other.e31 * this.e431) + (other.e12 * this.e412) - (other.scalar * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412));
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 anti_reverse_g3 = this.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0[1] * this.scalar) + (geometric_anti_product_g1[3] * this.e321) + (geometric_anti_product_g4[0] * this.e1) + (geometric_anti_product_g4[1] * this.e2) + (geometric_anti_product_g4[2] * this.e3) + (geometric_anti_product_g4[3] * this.e4) - (anti_reverse_g2[0] * geometric_anti_product_g3[0]) - (anti_reverse_g2[1] * geometric_anti_product_g3[1]) - (anti_reverse_g2[2] * geometric_anti_product_g3[2]) - (anti_reverse_g3[0] * geometric_anti_product_g2[0]) - (anti_reverse_g3[1] * geometric_anti_product_g2[1]) - (anti_reverse_g3[2] * geometric_anti_product_g2[2]), (geometric_anti_product_g1[3] * this.e4) - (anti_reverse_g2[0] * geometric_anti_product_g2[0]) - (anti_reverse_g2[1] * geometric_anti_product_g2[1]) - (anti_reverse_g2[2] * geometric_anti_product_g2[2])) + (geometric_anti_product_g0 * float2(this.e1234)) + (float2(this.e423) * float2(geometric_anti_product_g1[0], geometric_anti_product_g4[0])) + (float2(this.e431) * float2(geometric_anti_product_g1[1], geometric_anti_product_g4[1])) + (float2(this.e412) * float2(geometric_anti_product_g1[2], geometric_anti_product_g4[2])), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0[0]) * this.group4.xyz) + (float3(this.e1234) * geometric_anti_product_g1.xyz) + (float3(geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]) * anti_reverse_g2.zyz) + (float3(geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]) * anti_reverse_g2.xxy) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (anti_reverse_g3.yzx * geometric_anti_product_g4.zxy) + (geometric_anti_product_g3.yzx * this.group4.zxy) - (float3(geometric_anti_product_g0[1]) * this.group1.xyz) - (float3(this.scalar) * geometric_anti_product_g4.xyz) - (float3(geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]) * anti_reverse_g3.xxy) - (float3(geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]) * anti_reverse_g3.zyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g1.zxy) - (geometric_anti_product_g2.yzx * this.group1.zxy)), this.e1234 * geometric_anti_product_g1[3]), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[2] * this.e431) - (geometric_anti_product_g4[1] * this.e412), (geometric_anti_product_g4[1] * this.e4) - (geometric_anti_product_g4[2] * this.e423), (geometric_anti_product_g4[1] * this.e423) + (geometric_anti_product_g4[2] * this.e4) - (geometric_anti_product_g4[0] * this.e431)) + (anti_reverse_g2 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.e1234)) + (anti_reverse_g2.zxy * geometric_anti_product_g2.yzx) + float3((float2(geometric_anti_product_g4[0]) * float2(this.e4, this.e412)), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g1[2] * this.e431) + (geometric_anti_product_g4[2] * this.e2) - (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g4[1] * this.e3), (geometric_anti_product_g1[1] * this.e4) + (geometric_anti_product_g4[1] * this.e321) - (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g4[2] * this.e1), (geometric_anti_product_g1[1] * this.e423) + (geometric_anti_product_g1[2] * this.e4) + (geometric_anti_product_g4[1] * this.e1) + (geometric_anti_product_g4[2] * this.e321) - (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g4[0] * this.e2)) + (anti_reverse_g2 * float3(geometric_anti_product_g0[0])) + (anti_reverse_g3 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.scalar)) + (geometric_anti_product_g3 * float3(this.e1234)) + (anti_reverse_g2.zxy * geometric_anti_product_g3.yzx) + (anti_reverse_g3.zxy * geometric_anti_product_g2.yzx) + float3(((float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412)) + (float2(geometric_anti_product_g4[0]) * float2(this.e321, this.e3))), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group1.xyz) - (float3(geometric_anti_product_g4[3]) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g3.zxy) - (anti_reverse_g3.yzx * geometric_anti_product_g2.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_anti_product_g4 * float4(this.e1234)) + (float4(geometric_anti_product_g0[1]) * this.group4) + float4(((float3(geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]) * anti_reverse_g2.xxy) + (float3(geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]) * anti_reverse_g2.zyz) + (geometric_anti_product_g2.yzx * this.group4.zxy) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g4.zxy)), (anti_reverse_g3[0] * geometric_anti_product_g4[0]) + (anti_reverse_g3[1] * geometric_anti_product_g4[1]) + (anti_reverse_g3[2] * geometric_anti_product_g4[2]) - (geometric_anti_product_g0[0] * this.e4) - (this.scalar * geometric_anti_product_g1[3]) - (anti_reverse_g2[0] * geometric_anti_product_g1[0]) - (anti_reverse_g2[1] * geometric_anti_product_g1[1]) - (anti_reverse_g2[2] * geometric_anti_product_g1[2]) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Origin other) {
        float2 geometric_anti_product_g0 = float2(other.e4 * -1.0) * float2(this.e321, this.e4);
        float4 geometric_anti_product_g1 = float4(other.e4) * float4(this.group3.xyz, this.e1234);
        float3 geometric_anti_product_g2 = float3(other.e4 * -1.0) * this.group4.xyz;
        float3 geometric_anti_product_g3 = float3(other.e4 * -1.0) * this.group1.xyz;
        float4 geometric_anti_product_g4 = float4(other.e4) * float4(this.group2.xyz, this.scalar);
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 anti_reverse_g3 = this.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product_g0[0]) * this.group4.xyz) + (float3(this.e1234) * geometric_anti_product_g1.xyz) + (float3(geometric_anti_product_g1[1], geometric_anti_product_g4[3], geometric_anti_product_g4[3]) * anti_reverse_g2.zyz) + (float3(geometric_anti_product_g4[3], geometric_anti_product_g1[2], geometric_anti_product_g1[0]) * anti_reverse_g2.xxy) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (anti_reverse_g3.yzx * geometric_anti_product_g4.zxy) + (geometric_anti_product_g3.yzx * this.group4.zxy) - (float3(geometric_anti_product_g0[1]) * this.group1.xyz) - (float3(this.scalar) * geometric_anti_product_g4.xyz) - (float3(geometric_anti_product_g1[3], geometric_anti_product_g4[2], geometric_anti_product_g4[0]) * anti_reverse_g3.xxy) - (float3(geometric_anti_product_g4[1], geometric_anti_product_g1[3], geometric_anti_product_g1[3]) * anti_reverse_g3.zyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g1.zxy) - (geometric_anti_product_g2.yzx * this.group1.zxy)), this.e1234 * geometric_anti_product_g1[3]), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[2] * this.e431) - (geometric_anti_product_g4[1] * this.e412), (geometric_anti_product_g4[1] * this.e4) - (geometric_anti_product_g4[2] * this.e423), (geometric_anti_product_g4[1] * this.e423) + (geometric_anti_product_g4[2] * this.e4) - (geometric_anti_product_g4[0] * this.e431)) + (anti_reverse_g2 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.e1234)) + (anti_reverse_g2.zxy * geometric_anti_product_g2.yzx) + float3((float2(geometric_anti_product_g4[0]) * float2(this.e4, this.e412)), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_anti_product_g1[2] * this.e431) + (geometric_anti_product_g4[2] * this.e2) - (geometric_anti_product_g1[1] * this.e412) - (geometric_anti_product_g4[1] * this.e3), (geometric_anti_product_g1[1] * this.e4) + (geometric_anti_product_g4[1] * this.e321) - (geometric_anti_product_g1[2] * this.e423) - (geometric_anti_product_g4[2] * this.e1), (geometric_anti_product_g1[1] * this.e423) + (geometric_anti_product_g1[2] * this.e4) + (geometric_anti_product_g4[1] * this.e1) + (geometric_anti_product_g4[2] * this.e321) - (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g4[0] * this.e2)) + (anti_reverse_g2 * float3(geometric_anti_product_g0[0])) + (anti_reverse_g3 * float3(geometric_anti_product_g0[1])) + (geometric_anti_product_g2 * float3(this.scalar)) + (geometric_anti_product_g3 * float3(this.e1234)) + (anti_reverse_g2.zxy * geometric_anti_product_g3.yzx) + (anti_reverse_g3.zxy * geometric_anti_product_g2.yzx) + float3(((float2(geometric_anti_product_g1[0]) * float2(this.e4, this.e412)) + (float2(geometric_anti_product_g4[0]) * float2(this.e321, this.e3))), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group1.xyz) - (float3(geometric_anti_product_g4[3]) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g3.zxy) - (anti_reverse_g3.yzx * geometric_anti_product_g2.zxy), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Plane other) {
        float2 geometric_anti_product_g0 = float2(this.e4 * other.e321, 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412));
        float3 geometric_anti_product_g1_xyz = (float3(this.scalar) * other.group0.xyz) + (float3(other.e321) * this.group2.xyz) + (this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx);
        float3 geometric_anti_product_g2 = float3((this.e412 * other.e431) - (this.e431 * other.e412), (this.e423 * other.e412) - (this.e412 * other.e423), (this.e431 * other.e423) - (this.e423 * other.e431)) - (float3(this.e4) * other.group0.xyz);
        float3 geometric_anti_product_g3 = float3((this.e3 * other.e431) - (this.e2 * other.e412), (this.e1 * other.e412) - (this.e3 * other.e423), (this.e2 * other.e423) - (this.e1 * other.e431)) + (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 geometric_anti_product_g4_xyz = (float3(this.e1234) * other.group0.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float geometric_anti_product_g4_w = this.e1234 * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g4_w * this.e4) + (geometric_anti_product_g0[0] * this.e1234) + (geometric_anti_product_g0[1] * this.scalar) + (geometric_anti_product_g1_xyz[0] * this.e423) + (geometric_anti_product_g2[0] * this.e23) + (geometric_anti_product_g2[1] * this.e31) + (geometric_anti_product_g2[2] * this.e12) + (geometric_anti_product_g3[0] * this.e41) + (geometric_anti_product_g3[1] * this.e42) + (geometric_anti_product_g3[2] * this.e43) + (geometric_anti_product_g4_xyz[0] * this.e1) + (geometric_anti_product_g4_xyz[1] * this.e2) + (geometric_anti_product_g4_xyz[2] * this.e3), (geometric_anti_product_g0[1] * this.e1234) + (geometric_anti_product_g2[0] * this.e41) + (geometric_anti_product_g2[1] * this.e42) + (geometric_anti_product_g2[2] * this.e43) + (geometric_anti_product_g4_xyz[2] * this.e412)) + (float2(geometric_anti_product_g1_xyz[1], geometric_anti_product_g4_xyz[0]) * this.group4.yx) + (float2(geometric_anti_product_g1_xyz[2], geometric_anti_product_g4_xyz[1]) * this.group4.zy), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((geometric_anti_product_g4_xyz * float3(this.e1234)) + (float3(geometric_anti_product_g0[1]) * this.group4.xyz) + (geometric_anti_product_g2.yzx * this.group4.zxy) + (geometric_anti_product_g4_xyz.zxy * this.group2.yzx) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g2.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g2.zyz) - (geometric_anti_product_g4_xyz.yzx * this.group2.zxy)), (geometric_anti_product_g4_w * this.e1234) + (geometric_anti_product_g0[1] * this.e321) + (geometric_anti_product_g1_xyz[0] * this.e41) + (geometric_anti_product_g1_xyz[1] * this.e42) - (geometric_anti_product_g0[0] * this.e4) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412) - (geometric_anti_product_g4_xyz[0] * this.e23) - (geometric_anti_product_g4_xyz[1] * this.e31) - (geometric_anti_product_g4_xyz[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Point other) {
        float geometric_anti_product_g0_x = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float geometric_anti_product_g0_y = this.e4 * other.e4 * -1.0;
        float3 geometric_anti_product_g1_xyz = (float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group3.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float geometric_anti_product_g1_w = this.e1234 * other.e4;
        float3 geometric_anti_product_g2 = float3(other.e4 * -1.0) * this.group4.xyz;
        float3 geometric_anti_product_g3 = float3((this.e431 * other.e3) - (this.e412 * other.e2), (this.e412 * other.e1) - (this.e423 * other.e3), (this.e423 * other.e2) - (this.e431 * other.e1)) + (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 geometric_anti_product_g4_xyz = float3(other.e4) * this.group2.xyz;
        float geometric_anti_product_g4_w = (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3);
        float3 anti_reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 anti_reverse_g3 = this.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(((anti_reverse_g2 * float3(geometric_anti_product_g4_w)) + (geometric_anti_product_g1_xyz * float3(this.e1234)) + (float3(geometric_anti_product_g0_x) * this.group4.xyz) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g2.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g2.xxy) + (anti_reverse_g2.zxy * geometric_anti_product_g1_xyz.yzx) + (anti_reverse_g3.yzx * geometric_anti_product_g4_xyz.zxy) + (geometric_anti_product_g3.yzx * this.group4.zxy) - (anti_reverse_g3 * float3(geometric_anti_product_g1_w)) - (geometric_anti_product_g4_xyz * float3(this.scalar)) - (float3(geometric_anti_product_g0_y) * this.group1.xyz) - (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g3.xxy) - (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g3.zyz) - (anti_reverse_g2.yzx * geometric_anti_product_g1_xyz.zxy) - (anti_reverse_g3.zxy * geometric_anti_product_g4_xyz.yzx) - (geometric_anti_product_g2.yzx * this.group1.zxy)), geometric_anti_product_g1_w * this.e1234), 
            /* e41, e42, e43 */
            (anti_reverse_g2 * float3(geometric_anti_product_g0_y)) + (geometric_anti_product_g2 * float3(this.e1234)) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g4_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g4_xyz.zyz) + (anti_reverse_g2.zxy * geometric_anti_product_g2.yzx) - (float3(geometric_anti_product_g1_w) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g2.zxy) - (geometric_anti_product_g4_xyz.yzx * this.group4.zxy), 
            /* e23, e31, e12 */
            (anti_reverse_g2 * float3(geometric_anti_product_g0_x)) + (anti_reverse_g3 * float3(geometric_anti_product_g0_y)) + (geometric_anti_product_g2 * float3(this.scalar)) + (geometric_anti_product_g3 * float3(this.e1234)) + (float3(this.e2, this.e321, this.e321) * geometric_anti_product_g4_xyz.zyz) + (float3(this.e4, this.e412, this.e423) * geometric_anti_product_g1_xyz.xxy) + (float3(this.e431, this.e4, this.e4) * geometric_anti_product_g1_xyz.zyz) + (float3(this.e321, this.e3, this.e1) * geometric_anti_product_g4_xyz.xxy) + (anti_reverse_g2.zxy * geometric_anti_product_g3.yzx) + (anti_reverse_g3.zxy * geometric_anti_product_g2.yzx) - (float3(geometric_anti_product_g1_w) * this.group1.xyz) - (float3(geometric_anti_product_g4_w) * this.group4.xyz) - (anti_reverse_g2.yzx * geometric_anti_product_g3.zxy) - (anti_reverse_g3.yzx * geometric_anti_product_g2.zxy) - (geometric_anti_product_g1_xyz.yzx * this.group4.zxy) - (geometric_anti_product_g4_xyz.yzx * this.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Scalar other) {
        float geometric_anti_product_g0_x = this.e1234 * other.scalar;
        float3 geometric_anti_product_g1_xyz = float3(other.scalar * -1.0) * this.group4.xyz;
        float3 geometric_anti_product_g3 = float3(other.scalar) * this.group2.xyz;
        float geometric_anti_product_g4_w = this.e4 * other.scalar * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g0_x * this.e1234) + (geometric_anti_product_g4_w * this.e4) + (geometric_anti_product_g1_xyz[0] * this.e423) + (geometric_anti_product_g1_xyz[1] * this.e431) + (geometric_anti_product_g1_xyz[2] * this.e412) + (geometric_anti_product_g3[0] * this.e41) + (geometric_anti_product_g3[1] * this.e42) + (geometric_anti_product_g3[2] * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), (geometric_anti_product_g4_w * this.e1234) + (geometric_anti_product_g1_xyz[0] * this.e41) + (geometric_anti_product_g1_xyz[1] * this.e42) + (geometric_anti_product_g1_xyz[2] * this.e43) - (geometric_anti_product_g0_x * this.e4) - (geometric_anti_product_g3[0] * this.e423) - (geometric_anti_product_g3[1] * this.e431) - (geometric_anti_product_g3[2] * this.e412))
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Origin> operator /(Origin lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiSandwichInfixPartial<Origin> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public AntiScalar anti_sandwich(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e4 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public DualNum anti_sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e4) * float2(other.scalar * this.e4 * -1.0, other.e1234 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Flector anti_sandwich(Flector other) {
        float4 geometric_anti_product_g0 = float4(this.e4 * -1.0) * float4(other.group1.xyz, other.e4);
        float4 geometric_anti_product_g1 = float4(this.e4) * float4(other.group0.xyz, other.e321);
        float anti_reverse_g0 = this.e4 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_reverse_g0) * float4(geometric_anti_product_g1.xyz, geometric_anti_product_g0[3]), 
            /* e423, e431, e412, e321 */
            float4(anti_reverse_g0) * float4(geometric_anti_product_g0.xyz, geometric_anti_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Horizon anti_sandwich(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * this.e4 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Line anti_sandwich(Line other) {
        float anti_reverse_g0 = this.e4 * -1.0;
        return Line(
            /* e41, e42, e43 */
            float3(anti_reverse_g0 * this.e4 * -1.0) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_reverse_g0 * this.e4) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Motor anti_sandwich(Motor other) {
        float4 geometric_anti_product_g0 = float4(this.e4) * float4((other.group1.xyz * float3(-1.0)), other.e1234);
        float4 geometric_anti_product_g1 = float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_reverse_g0 = this.e4 * -1.0;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(anti_reverse_g0 * -1.0) * float4(geometric_anti_product_g1.xyz, geometric_anti_product_g0[3]), 
            /* e23, e31, e12, scalar */
            float4(anti_reverse_g0 * -1.0) * float4(geometric_anti_product_g0.xyz, geometric_anti_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public MultiVector anti_sandwich(MultiVector other) {
        float2 geometric_anti_product_g0 = float2(this.e4) * float2(other.e321, other.e4) * float2(1.0, -1.0);
        float4 geometric_anti_product_g1 = float4(this.e4) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float4 geometric_anti_product_g4 = float4(this.e4) * float4(other.group2.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_reverse_g0 = this.e4 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_reverse_g0 * -1.0) * float2(geometric_anti_product_g4[3], geometric_anti_product_g1[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_reverse_g0) * float4((float3(this.e4) * other.group1.xyz), geometric_anti_product_g0[1]), 
            /* e41, e42, e43 */
            float3(anti_reverse_g0 * -1.0) * geometric_anti_product_g4.xyz, 
            /* e23, e31, e12 */
            float3(anti_reverse_g0 * -1.0) * geometric_anti_product_g1.xyz, 
            /* e423, e431, e412, e321 */
            float4(anti_reverse_g0) * float4((float3(this.e4 * -1.0) * other.group4.xyz), geometric_anti_product_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiSandwichInfixPartial<Origin> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Origin anti_sandwich(Origin other) {
        return Origin(
            /* e4 */
            other.e4 * this.e4 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Plane anti_sandwich(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e4 * this.e4) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Point anti_sandwich(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e4 * this.e4) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Scalar anti_sandwich(Scalar other) {
        return Scalar(
            /* scalar */
            this.e4 * this.e4 * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Plane> operator /(Plane lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiSandwichInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public AntiScalar anti_sandwich(AntiScalar other) {
        float4 geometric_anti_product_g0 = float4(other.e1234) * this.group0;
        return AntiScalar(
            /* e1234 */
            (geometric_anti_product_g0[0] * this.e423) + (geometric_anti_product_g0[1] * this.e431) + (geometric_anti_product_g0[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public DualNum anti_sandwich(DualNum other) {
        float4 geometric_anti_product_g0 = float4(1.0, 1.0, other.scalar, 0.0) * float4((this.group0.xyz * float3(float2(other.scalar * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0);
        float4 geometric_anti_product_g1 = float4(other.e1234) * this.group0;
        return DualNum(
            /* scalar, e1234 */
            float2(geometric_anti_product_g0[3] * this.e321, 0.0) + (float2(this.e423) * float2(geometric_anti_product_g0[0], geometric_anti_product_g1[0])) + (float2(this.e431) * float2(geometric_anti_product_g0[1], geometric_anti_product_g1[1])) + (float2(this.e412) * float2(geometric_anti_product_g0[2], geometric_anti_product_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Flector anti_sandwich(Flector other) {
        float3 geometric_anti_product_g0_xyz = float3((other.e431 * this.e412) - (other.e412 * this.e431), (other.e412 * this.e423) - (other.e423 * this.e412), (other.e423 * this.e431) - (other.e431 * this.e423)) - (float3(other.e4) * this.group0.xyz);
        float geometric_anti_product_g0_w = other.e423 * this.e423;
        float4 geometric_anti_product_g1 = float4(0.0, 0.0, (other.e2 * this.e423) - (other.e1 * this.e431), 0.0) + float4(((float3(other.e321) * this.group0.xyz) + float3(((other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx)), 0.0) - (float3(this.e321) * other.group1.xyz)), 0.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, (geometric_anti_product_g1[0] * this.e431) - (geometric_anti_product_g1[1] * this.e423), 0.0) + float4(((geometric_anti_product_g0_xyz * float3(this.e321)) + (float3(geometric_anti_product_g1[3]) * this.group0.xyz) + float3(((geometric_anti_product_g1.yz * this.group0.zx) - (geometric_anti_product_g1.zx * this.group0.yz)), 0.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_anti_product_g0_w) * this.group0.xyz) + (geometric_anti_product_g0_xyz.yzx * this.group0.zxy) - (geometric_anti_product_g0_xyz.zxy * this.group0.yzx)), geometric_anti_product_g0_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Horizon anti_sandwich(Horizon other) {
        float3 geometric_anti_product_g1 = float3(other.e321) * this.group0.xyz;
        return Horizon(
            /* e321 */
            -(geometric_anti_product_g1[0] * this.e423) - (geometric_anti_product_g1[1] * this.e431) - (geometric_anti_product_g1[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Line anti_sandwich(Line other) {
        float3 geometric_anti_product_g0_xyz = (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float3 geometric_anti_product_g1_xyz = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        return Line(
            /* e41, e42, e43 */
            (geometric_anti_product_g1_xyz.zxy * this.group0.yzx) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (geometric_anti_product_g1_xyz * float3(this.e321)) + (geometric_anti_product_g0_xyz.zxy * this.group0.yzx) - (float3(this.e423) * float3(other.e23 * this.e423, other.e23 * this.e431, other.e23 * this.e412)) - (geometric_anti_product_g0_xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Motor anti_sandwich(Motor other) {
        float4 geometric_anti_product_g0 = float4(0.0, 0.0, (other.e23 * this.e431) - (other.e31 * this.e423), 0.0) + float4(((float3(this.e321) * other.group0.xyz) + float3(((other.group1.yz * this.group0.zx) - (other.group1.zx * this.group0.yz)), 0.0) - (float3(other.scalar) * this.group0.xyz)), 0.0);
        float4 geometric_anti_product_g1 = float4(0.0, 0.0, other.e41 * this.e431 * -1.0, 0.0) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(float3(other.e1234), other.e23)) + float4(-(other.group0.yz * this.group0.zx), 0.0, 0.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3((geometric_anti_product_g1[2] * this.e431) - (geometric_anti_product_g1[1] * this.e412), (geometric_anti_product_g1[0] * this.e412) - (geometric_anti_product_g1[2] * this.e423), (geometric_anti_product_g1[1] * this.e423) - (geometric_anti_product_g1[0] * this.e431)) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz)), geometric_anti_product_g1[0] * this.e423), 
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, geometric_anti_product_g0[0] * this.e431 * -1.0, 0.0) + (geometric_anti_product_g0.zxyx * this.group0.yzxx) + (this.group0.wwwy * float4(geometric_anti_product_g1.xyz, geometric_anti_product_g0[1])) + float4((float3(-(geometric_anti_product_g0.yz * this.group0.zx), 0.0) - (float3(geometric_anti_product_g1[3]) * this.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public MultiVector anti_sandwich(MultiVector other) {
        float geometric_anti_product_g0_y = (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412);
        float3 geometric_anti_product_g1_xyz = (float3(this.e321) * other.group2.xyz) + (other.group3.yzx * this.group0.zxy) - (float3(other.scalar) * this.group0.xyz) - (other.group3.zxy * this.group0.yzx);
        float3 geometric_anti_product_g2 = float3((other.e431 * this.e412) - (other.e412 * this.e431), (other.e412 * this.e423) - (other.e423 * this.e412), (other.e423 * this.e431) - (other.e431 * this.e423)) - (float3(other.e4) * this.group0.xyz);
        float3 geometric_anti_product_g3 = float3((other.e3 * this.e431) - (other.e2 * this.e412), (other.e1 * this.e412) - (other.e3 * this.e423), (other.e2 * this.e423) - (other.e1 * this.e431)) + (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz);
        float4 geometric_anti_product_g4 = (float4(other.e1234) * this.group0) + (this.group0.yzxx * float4(other.group2.zxy, other.e23)) + float4(-(other.group2.yzx * this.group0.zxy), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g1_xyz[0] * this.e423) + (geometric_anti_product_g1_xyz[1] * this.e431) + (geometric_anti_product_g1_xyz[2] * this.e412), (geometric_anti_product_g4[0] * this.e423) + (geometric_anti_product_g4[1] * this.e431) + (geometric_anti_product_g4[2] * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, this.e412 * this.e412 * other.e3 * -1.0, 0.0) + float4(((geometric_anti_product_g2 * float3(this.e321)) + (geometric_anti_product_g3.yzx * this.group0.zxy) + float3(-(this.group0.xy * this.group0.xy * other.group1.xy), 0.0) - (float3(this.e423) * float3(other.e2 * this.e431, other.e1 * this.e431, other.e1 * this.e412)) - (float3(this.e412) * float3(other.e3 * this.e423, other.e3 * this.e431, other.e2 * this.e431)) - (float3(other.e4 * this.e321) * this.group0.xyz) - (geometric_anti_product_g3.zxy * this.group0.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((geometric_anti_product_g4[2] * this.e431) - (geometric_anti_product_g4[1] * this.e412), (geometric_anti_product_g4[0] * this.e412) - (geometric_anti_product_g4[2] * this.e423), (geometric_anti_product_g4[1] * this.e423) - (geometric_anti_product_g4[0] * this.e431)), 
            /* e23, e31, e12 */
            (float3(this.e321) * geometric_anti_product_g4.xyz) + (geometric_anti_product_g1_xyz.zxy * this.group0.yzx) - (float3(geometric_anti_product_g4[3]) * this.group0.xyz) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_anti_product_g0_y) * this.group0.xyz) + (geometric_anti_product_g2.yzx * this.group0.zxy) - (geometric_anti_product_g2.zxy * this.group0.yzx)), geometric_anti_product_g0_y * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Point anti_sandwich(Origin other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4 * -2.0) * float4(this.e321).xyz * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Plane anti_sandwich(Plane other) {
        float2 geometric_anti_product_g0_xy = (other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz);
        float geometric_anti_product_g0_z = (other.e423 * this.e431) - (other.e431 * this.e423);
        float geometric_anti_product_g0_w = other.e423 * this.e423;
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3((geometric_anti_product_g0_xy[1] * this.e412) - (geometric_anti_product_g0_z * this.e431), (geometric_anti_product_g0_z * this.e423) - (geometric_anti_product_g0_xy[0] * this.e412), (geometric_anti_product_g0_xy[0] * this.e431) - (geometric_anti_product_g0_xy[1] * this.e423)) + (float3(geometric_anti_product_g0_w) * this.group0.xyz)), geometric_anti_product_g0_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Point anti_sandwich(Point other) {
        float2 geometric_anti_product_g1_xy = (this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz);
        float geometric_anti_product_g1_z = (this.e423 * other.e2) - (this.e431 * other.e1);
        return Point(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, (geometric_anti_product_g1_xy[0] * this.e431) - (geometric_anti_product_g1_xy[1] * this.e423), 0.0) + float4((float3(((float2(geometric_anti_product_g1_xy[1], geometric_anti_product_g1_z) * this.group0.zx) - (float2(geometric_anti_product_g1_z, geometric_anti_product_g1_xy[0]) * this.group0.yz)), 0.0) - float3(2.0)*(float3(this.e321 * other.e4) * this.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Scalar anti_sandwich(Scalar other) {
        float3 geometric_anti_product_g0_xyz = float3(other.scalar * -1.0) * this.group0.xyz;
        return Scalar(
            /* scalar */
            (geometric_anti_product_g0_xyz[0] * this.e423) + (geometric_anti_product_g0_xyz[1] * this.e431) + (geometric_anti_product_g0_xyz[2] * this.e412)
        );
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Point> operator /(Point lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiSandwichInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public AntiScalar anti_sandwich(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * this.e4 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public DualNum anti_sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e4 * this.e4) * other.group0.xy * float2(-1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Flector anti_sandwich(Flector other) {
        float4 geometric_anti_product_g0 = float4(this.e4 * -1.0) * float4(other.group1.xyz, other.e4);
        float4 geometric_anti_product_g1 = float4(0.0, 0.0, other.e431 * this.e1 * -1.0, 0.0) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4((float3(-(other.group1.zx * this.group0.yz), 0.0) - (float3(other.e4) * this.group0.xyz)), 0.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3((geometric_anti_product_g0[2] * this.e2) - (geometric_anti_product_g0[1] * this.e3), (geometric_anti_product_g0[0] * this.e3) - (geometric_anti_product_g0[2] * this.e1), (geometric_anti_product_g0[1] * this.e1) - (geometric_anti_product_g0[0] * this.e2)) - (float3(geometric_anti_product_g0[3]) * this.group0.xyz) - (float3(this.e4) * geometric_anti_product_g1.xyz)), geometric_anti_product_g0[3] * this.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g0.xyz * float4(this.e4).xyz * float3(-1.0)), (geometric_anti_product_g0[0] * this.e1) + (geometric_anti_product_g0[1] * this.e2) + (geometric_anti_product_g0[2] * this.e3) - (geometric_anti_product_g1[3] * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Horizon anti_sandwich(Horizon other) {
        return Horizon(
            /* e321 */
            this.e4 * this.e4 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Line anti_sandwich(Line other) {
        float3 geometric_anti_product_g1_xyz = float3(this.e4) * other.group0.xyz;
        return Line(
            /* e41, e42, e43 */
            geometric_anti_product_g1_xyz * float3(this.e4), 
            /* e23, e31, e12 */
            (float3(this.e4) * float3(other.e43 * this.e2, other.e41 * this.e3, other.e42 * this.e1)) + (geometric_anti_product_g1_xyz.zxy * this.group0.yzx) - (float3(this.e4) * float3(other.e42 * this.e3, other.e43 * this.e1, other.e41 * this.e2)) - (float3(this.e4 * this.e4) * other.group1.xyz) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Motor anti_sandwich(Motor other) {
        float geometric_anti_product_g0_w = other.e1234 * this.e4;
        float3 geometric_anti_product_g1_xyz = float3(this.e4) * other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(geometric_anti_product_g1_xyz, geometric_anti_product_g0_w), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * float3(other.e43 * this.e2, other.e41 * this.e3, other.e42 * this.e1)) + (float3(other.e1234 * this.e4) * this.group0.xyz) + (geometric_anti_product_g1_xyz.zxy * this.group0.yzx) - (float3(geometric_anti_product_g0_w) * this.group0.xyz) - (float3(this.e4) * float3(other.e42 * this.e3, other.e43 * this.e1, other.e41 * this.e2)) - (float3(this.e4 * this.e4) * other.group1.xyz) - (geometric_anti_product_g1_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public MultiVector anti_sandwich(MultiVector other) {
        float geometric_anti_product_g0_y = other.e4 * this.e4 * -1.0;
        float geometric_anti_product_g1_w = other.e1234 * this.e4;
        float3 geometric_anti_product_g2 = float3(this.e4 * -1.0) * other.group4.xyz;
        float3 geometric_anti_product_g4_xyz = float3(this.e4) * other.group2.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_anti_product_g4_xyz[0] * this.e1) + (geometric_anti_product_g4_xyz[1] * this.e2) + (geometric_anti_product_g4_xyz[2] * this.e3) - (this.e4 * this.e4 * other.scalar) - (other.e41 * this.e1 * this.e4) - (other.e42 * this.e2 * this.e4) - (other.e43 * this.e3 * this.e4), geometric_anti_product_g1_w * this.e4), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e4) * float3(other.e412 * this.e2, other.e423 * this.e3, other.e431 * this.e1)) + (float3(other.e4 * this.e4) * this.group0.xyz) + (geometric_anti_product_g2.zxy * this.group0.yzx) - (float3(geometric_anti_product_g0_y) * this.group0.xyz) - (float3(this.e4) * float3(other.e431 * this.e3, other.e412 * this.e1, other.e423 * this.e2)) - (float3(this.e4 * this.e4) * other.group1.xyz) - (geometric_anti_product_g2.yzx * this.group0.zxy)), geometric_anti_product_g0_y * this.e4 * -1.0), 
            /* e41, e42, e43 */
            geometric_anti_product_g4_xyz * float3(this.e4), 
            /* e23, e31, e12 */
            (float3(this.e4) * float3(other.e43 * this.e2, other.e41 * this.e3, other.e42 * this.e1)) + (float3(other.e1234 * this.e4) * this.group0.xyz) + (geometric_anti_product_g4_xyz.zxy * this.group0.yzx) - (float3(geometric_anti_product_g1_w) * this.group0.xyz) - (float3(this.e4) * float3(other.e42 * this.e3, other.e43 * this.e1, other.e41 * this.e2)) - (float3(this.e4 * this.e4) * other.group3.xyz) - (geometric_anti_product_g4_xyz.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g2 * float4(this.e4).xyz * float3(-1.0)), (geometric_anti_product_g2[0] * this.e1) + (geometric_anti_product_g2[1] * this.e2) + (geometric_anti_product_g2[2] * this.e3) - (this.e4 * this.e4 * other.e321) - (other.e423 * this.e1 * this.e4) - (other.e431 * this.e2 * this.e4) - (other.e412 * this.e3 * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Point anti_sandwich(Origin other) {
        float geometric_anti_product_g0_w = this.e4 * other.e4 * -1.0;
        return Point(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e4 * other.e4) * this.group0.xyz) - (float3(geometric_anti_product_g0_w) * this.group0.xyz)), geometric_anti_product_g0_w * this.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiSandwichInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Plane anti_sandwich(Plane other) {
        float3 geometric_anti_product_g0_xyz = float3(this.e4 * -1.0) * other.group0.xyz;
        return Plane(
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product_g0_xyz * float4(this.e4).xyz * float3(-1.0)), (geometric_anti_product_g0_xyz[0] * this.e1) + (geometric_anti_product_g0_xyz[1] * this.e2) + (geometric_anti_product_g0_xyz[2] * this.e3) - (other.e423 * this.e1 * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Point anti_sandwich(Point other) {
        float geometric_anti_product_g0_w = other.e4 * this.e4 * -1.0;
        return Point(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e4 * this.e4) * this.group0.xyz) - (float3(geometric_anti_product_g0_w) * this.group0.xyz) - (float3(this.e4 * this.e4) * other.group0.xyz)), geometric_anti_product_g0_w * this.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Scalar anti_sandwich(Scalar other) {
        return Scalar(
            /* scalar */
            this.e4 * this.e4 * other.scalar * -1.0
        );
    }
}
