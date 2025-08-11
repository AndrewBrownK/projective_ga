implementing rga3d;
// Fancy infix trick (first half)
public SandwichInfixPartial<DualNum> operator /(DualNum lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public AntiScalar sandwich(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.scalar * this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public DualNum sandwich(DualNum other) {
        float geometric_product_g0_x = other.scalar * this.scalar;
        return DualNum(
            /* scalar, e1234 */
            float2(geometric_product_g0_x * this.scalar, (geometric_product_g0_x * this.e1234) + (this.scalar * this.scalar * other.e1234) + (other.scalar * this.scalar * this.e1234))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Flector sandwich(Flector other) {
        float3 geometric_product_g0_xyz = float3(this.scalar) * other.group0.xyz;
        float geometric_product_g1_w = this.scalar * other.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((geometric_product_g0_xyz * float3(float2(this.scalar), this.scalar)), (geometric_product_g1_w * this.e1234) + (this.scalar * this.scalar * other.e4) - (this.scalar * this.e1234 * other.e321)), 
            /* e423, e431, e412, e321 */
            float4(((geometric_product_g0_xyz * float3(this.e1234)) + (float3(this.scalar * this.scalar) * other.group1.xyz) - (float3(this.scalar * this.e1234) * other.group0.xyz)), geometric_product_g1_w * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(SandwichInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Horizon sandwich(Horizon other) {
        return Horizon(
            /* e321 */
            this.scalar * this.scalar * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Line sandwich(Line other) {
        float3 geometric_product_g1 = float3(this.scalar) * other.group1.xyz;
        return Line(
            /* e41, e42, e43 */
            (geometric_product_g1 * float3(this.e1234)) + (float3(this.scalar * this.scalar) * other.group0.xyz) + (float3(this.scalar * this.e1234) * other.group1.xyz), 
            /* e23, e31, e12 */
            geometric_product_g1 * float3(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Motor sandwich(Motor other) {
        float4 geometric_product_g1 = float4(this.scalar) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            (geometric_product_g1 * float4(this.e1234)) + (float4(this.scalar * this.scalar) * other.group0) + (float4(this.scalar * this.e1234) * other.group1), 
            /* e23, e31, e12, scalar */
            geometric_product_g1 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public MultiVector sandwich(MultiVector other) {
        float geometric_product_g0_x = this.scalar * other.scalar;
        float3 geometric_product_g1_xyz = float3(this.scalar) * other.group1.xyz;
        float3 geometric_product_g3 = float3(this.scalar) * other.group3.xyz;
        float geometric_product_g4_w = this.scalar * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g0_x * this.scalar, (geometric_product_g0_x * this.e1234) + (this.scalar * this.scalar * other.e1234) + (this.scalar * this.e1234 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4((geometric_product_g1_xyz * float3(float2(this.scalar), this.scalar)), (geometric_product_g4_w * this.e1234) + (this.scalar * this.scalar * other.e4) - (this.scalar * this.e1234 * other.e321)), 
            /* e41, e42, e43 */
            (geometric_product_g3 * float3(this.e1234)) + (float3(this.scalar * this.scalar) * other.group2.xyz) + (float3(this.scalar * this.e1234) * other.group3.xyz), 
            /* e23, e31, e12 */
            geometric_product_g3 * float3(this.scalar), 
            /* e423, e431, e412, e321 */
            float4(((geometric_product_g1_xyz * float3(this.e1234)) + (float3(this.scalar * this.scalar) * other.group4.xyz) - (float3(this.scalar * this.e1234) * other.group1.xyz)), geometric_product_g4_w * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Origin sandwich(Origin other) {
        return Origin(
            /* e4 */
            this.scalar * this.scalar * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Plane sandwich(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar * this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Point sandwich(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar * this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public DualNum sandwich(Scalar other) {
        float2 geometric_product_g0 = float2(other.scalar) * this.group0.xy;
        return DualNum(
            /* scalar, e1234 */
            float2(geometric_product_g0[0] * this.scalar, (geometric_product_g0[0] * this.e1234) + (geometric_product_g0[1] * this.scalar))
        );
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Flector> operator /(Flector lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public AntiScalar sandwich(AntiScalar other) {
        float3 geometric_product_g1_xyz = float3(other.e1234) * this.group0.xyz;
        return AntiScalar(
            /* e1234 */
            -(geometric_product_g1_xyz[0] * this.e1) - (geometric_product_g1_xyz[1] * this.e2) - (geometric_product_g1_xyz[2] * this.e3) - (this.e321 * this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public DualNum sandwich(DualNum other) {
        float3 geometric_product_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float3 geometric_product_g1_xyz = (float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz);
        float geometric_product_g1_w = other.scalar * this.e321;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_product_g1_w * this.e321) + (geometric_product_g0_xyz[0] * this.e1) + (geometric_product_g0_xyz[1] * this.e2) + (geometric_product_g0_xyz[2] * this.e3), -(geometric_product_g1_w * this.e4) - (geometric_product_g0_xyz[0] * this.e423) - (geometric_product_g0_xyz[1] * this.e431) - (geometric_product_g0_xyz[2] * this.e412) - (geometric_product_g1_xyz[0] * this.e1) - (geometric_product_g1_xyz[1] * this.e2) - (geometric_product_g1_xyz[2] * this.e3) - (this.e321 * this.e321 * other.e1234) - (other.scalar * this.e4 * this.e321))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Flector sandwich(Flector other) {
        float4 geometric_product_g0 = (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyz * this.group0.yzxz) + float4(((float2(this.e4) * other.group0.xy) + (other.group0.yz * this.group1.zx) - (float2(this.e321) * other.group1.xy) - (other.group1.yz * this.group0.zx)), (other.e1 * this.e431) + (other.e3 * this.e4) - (other.e423 * this.e2) - (other.e412 * this.e321), (other.e423 * this.e1) + (other.e431 * this.e2) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.zxyx * this.group1.yzxx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431));
        float4 geometric_product_g1 = (other.group0.zxyx * this.group0.yzxx) + float4((float3((-(float2(this.e321) * other.group0.xy) - (other.group0.yz * this.group0.zx)), -(other.e1 * this.e2) - (other.e3 * this.e321)) - (float3(other.e321) * this.group0.xyz)), other.e321 * this.e321 * -1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, -(geometric_product_g1[0] * this.e2) - (geometric_product_g1[2] * this.e321), 0.0) + (this.group0.xyzy * float4(float3(geometric_product_g1[3]), geometric_product_g0[1])) + (this.group0.yzxx * float4(geometric_product_g1.zxy, geometric_product_g0[0])) + float4((-(float2(this.e321) * geometric_product_g1.xy) - (geometric_product_g1.yz * this.group0.zx)), 0.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3((geometric_product_g0[1] * this.e3) + (geometric_product_g1[1] * this.e412) - (geometric_product_g0[2] * this.e2) - (geometric_product_g1[2] * this.e431), (geometric_product_g0[2] * this.e1) + (geometric_product_g1[2] * this.e423) - (geometric_product_g0[0] * this.e3) - (geometric_product_g1[0] * this.e412), (geometric_product_g0[0] * this.e2) + (geometric_product_g0[2] * this.e321) + (geometric_product_g1[0] * this.e431) + (geometric_product_g1[2] * this.e4) - (geometric_product_g0[1] * this.e1) - (geometric_product_g1[1] * this.e423)) + float3(((float2(this.e4) * geometric_product_g1.xy) + (float2(this.e321) * geometric_product_g0.xy)), 0.0) - (float3(geometric_product_g0[3]) * this.group0.xyz) - (float3(geometric_product_g1[3]) * this.group1.xyz)), geometric_product_g1[3] * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Plane sandwich(Horizon other) {
        float4 geometric_product_g0 = float4(other.e321) * float4(this.group1.xyz, this.e4);
        float4 geometric_product_g1 = float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321);
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3((geometric_product_g0[1] * this.e3) + (geometric_product_g1[1] * this.e412) - (geometric_product_g0[2] * this.e2) - (geometric_product_g1[2] * this.e431), (geometric_product_g0[2] * this.e1) + (geometric_product_g1[2] * this.e423) - (geometric_product_g0[0] * this.e3) - (geometric_product_g1[0] * this.e412), (geometric_product_g0[0] * this.e2) + (geometric_product_g0[2] * this.e321) + (geometric_product_g1[0] * this.e431) + (geometric_product_g1[2] * this.e4) - (geometric_product_g0[1] * this.e1) - (geometric_product_g1[1] * this.e423)) + float3(((float2(this.e4) * geometric_product_g1.xy) + (float2(this.e321) * geometric_product_g0.xy)), 0.0) - (float3(geometric_product_g0[3]) * this.group0.xyz) - (float3(geometric_product_g1[3]) * this.group1.xyz)), geometric_product_g1[3] * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Line sandwich(Line other) {
        float3 geometric_product_g0_xyz = (other.group1.xyx * float3(float2(this.e321), this.e2)) + (other.group1.yzz * float3(this.group0.zx, this.e321)) - (other.group1.zxy * this.group0.yzx);
        float3 geometric_product_g1_xyz = (other.group0.xyx * float3(float2(this.e321), this.e2)) + (other.group0.yzz * float3(this.group0.zx, this.e321)) + (other.group1.xyx * float3(float2(this.e4), this.e431)) + (other.group1.yzz * float3(this.group1.zx, this.e4)) - (other.group0.zxy * this.group0.yzx) - (other.group1.zxy * this.group1.yzx);
        return Line(
            /* e41, e42, e43 */
            (geometric_product_g0_xyz.zxy * this.group1.yzx) + (geometric_product_g1_xyz.zxy * this.group0.yzx) - (geometric_product_g0_xyz.xyx * float3(float2(this.e4), this.e431)) - (geometric_product_g0_xyz.yzz * float3(this.group1.zx, this.e4)) - (geometric_product_g1_xyz.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g1_xyz.yzz * float3(this.group0.zx, this.e321)), 
            /* e23, e31, e12 */
            (geometric_product_g0_xyz.xyx * float3(float2(this.e321), this.e2)) + (geometric_product_g0_xyz.yzz * float3(this.group0.zx, this.e321)) - (geometric_product_g0_xyz.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Motor sandwich(Motor other) {
        float4 geometric_product_g0 = (this.group0.xxyw * other.group1.wzxw) + float4(((float3(this.e321) * other.group1.xyz) + float3(((this.group0.zy * other.group1.yw) - (this.group0.yz * other.group1.zx)), (this.e3 * other.scalar) - (this.e1 * other.e31))), this.e321 * other.e1234);
        float3 geometric_product_g1_xyz = float3((this.e3 * other.e42) + (this.e412 * other.e31) - (this.e2 * other.e43) - (this.e431 * other.e12), (this.e2 * other.e1234) + (this.e431 * other.scalar) - (this.e3 * other.e41) - (this.e412 * other.e23), (this.e2 * other.e41) + (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e412 * other.scalar) - (this.e1 * other.e42) - (this.e423 * other.e31)) + (float3(this.e4) * other.group1.xyz) + (float3(this.e321) * other.group0.xyz) + float3(((float2(this.e1) * other.group0.wz) + (float2(this.e423) * other.group1.wz)), 0.0);
        float geometric_product_g1_w = this.e321 * other.scalar;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0, 0.0, (geometric_product_g0[1] * this.e423) - (geometric_product_g0[0] * this.e431) - (geometric_product_g0[2] * this.e4), 0.0) + float4(((float3(geometric_product_g1_w) * this.group1.xyz) + (float3(geometric_product_g0[3]) * this.group0.xyz) + (geometric_product_g1_xyz.zxy * this.group0.yzx) + float3(((geometric_product_g0.zx * this.group1.yz) - (float2(this.e4) * geometric_product_g0.xy) - (geometric_product_g0.yz * this.group1.zx)), 0.0) - (geometric_product_g1_xyz.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g1_xyz.yzz * float3(this.group0.zx, this.e321))), 0.0), 
            /* e23, e31, e12, scalar */
            (geometric_product_g0.yzzx * float4(this.group0.zx, this.e321, this.e1)) + float4((float3(((float2(this.e321) * geometric_product_g0.xy) - (geometric_product_g0.zx * this.group0.yz)), (geometric_product_g0[0] * this.e2) - (geometric_product_g0[1] * this.e1)) - (float3(geometric_product_g1_w) * this.group0.xyz)), geometric_product_g1_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public MultiVector sandwich(MultiVector other) {
        float2 geometric_product_g0 = float2(this.e321 * other.e321 * -1.0, (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412));
        float4 geometric_product_g1 = (float4(other.scalar) * this.group0) + float4(((other.group3.xyx * float3(float2(this.e321), this.e2)) + (other.group3.yzz * float3(this.group0.zx, this.e321)) - (other.group3.zxy * this.group0.yzx)), other.e1234 * this.e321);
        float3 geometric_product_g2 = float3((this.e2 * other.e412) + (this.e412 * other.e2) - (this.e3 * other.e431) - (this.e431 * other.e3), (this.e3 * other.e423) + (this.e431 * other.e321) - (this.e2 * other.e4) - (this.e412 * other.e1), (this.e1 * other.e431) + (this.e431 * other.e1) + (this.e412 * other.e321) - (this.e2 * other.e423) - (this.e3 * other.e4) - (this.e423 * other.e2)) + (float3(this.e4) * other.group1.xyz) + float3(((float2(this.e423) * float2(other.e321, other.e3)) - (float2(this.e1) * float2(other.e4, other.e412))), 0.0) - (float3(this.e321) * other.group4.xyz);
        float3 geometric_product_g3 = float3((this.e2 * other.e3) - (this.e3 * other.e2), (this.e3 * other.e1) - (this.e2 * other.e321), (this.e1 * other.e2) - (this.e2 * other.e1) - (this.e3 * other.e321)) + float3(-(float2(this.e1) * float2(other.e321, other.e3)), 0.0) - (float3(this.e321) * other.group1.xyz);
        float3 geometric_product_g4_xyz = (float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz) + (other.group2.xyx * float3(float2(this.e321), this.e2)) + (other.group2.yzz * float3(this.group0.zx, this.e321)) + (other.group3.xyx * float3(float2(this.e4), this.e431)) + (other.group3.yzz * float3(this.group1.zx, this.e4)) - (other.group2.zxy * this.group0.yzx) - (other.group3.zxy * this.group1.yzx);
        float geometric_product_g4_w = other.scalar * this.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g4_w * this.e321) + (geometric_product_g1[0] * this.e1) + (geometric_product_g1[1] * this.e2) + (geometric_product_g1[2] * this.e3), -(geometric_product_g4_w * this.e4) - (geometric_product_g4_xyz[0] * this.e1) - (geometric_product_g4_xyz[1] * this.e2) - (geometric_product_g4_xyz[2] * this.e3) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412) - (geometric_product_g1[3] * this.e321)), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g0[0]) * this.group0) + float4(((geometric_product_g3.zxy * this.group0.yzx) - (geometric_product_g3.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g3.yzz * float3(this.group0.zx, this.e321))), geometric_product_g0[1] * this.e321), 
            /* e41, e42, e43 */
            float3((geometric_product_g1[2] * this.e431) - (geometric_product_g1[1] * this.e412), (geometric_product_g1[0] * this.e412) - (geometric_product_g1[2] * this.e423), (geometric_product_g1[1] * this.e423) - (geometric_product_g1[0] * this.e431) - (geometric_product_g1[2] * this.e4)) + (float3(geometric_product_g4_w) * this.group1.xyz) + (float3(geometric_product_g1[3]) * this.group0.xyz) + (geometric_product_g4_xyz.zxy * this.group0.yzx) + float3(-(float2(this.e4) * geometric_product_g1.xy), 0.0) - (geometric_product_g4_xyz.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g4_xyz.yzz * float3(this.group0.zx, this.e321)), 
            /* e23, e31, e12 */
            float3((geometric_product_g1[1] * this.e3) - (geometric_product_g1[2] * this.e2), (geometric_product_g1[2] * this.e1) - (geometric_product_g1[0] * this.e3), (geometric_product_g1[0] * this.e2) + (geometric_product_g1[2] * this.e321) - (geometric_product_g1[1] * this.e1)) + float3((float2(this.e321) * geometric_product_g1.xy), 0.0) - (float3(geometric_product_g4_w) * this.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(((geometric_product_g2.xyx * float3(float2(this.e321), this.e2)) + (geometric_product_g2.yzz * float3(this.group0.zx, this.e321)) + (geometric_product_g3.xyx * float3(float2(this.e4), this.e431)) + (geometric_product_g3.yzz * float3(this.group1.zx, this.e4)) - (float3(geometric_product_g0[0]) * this.group1.xyz) - (float3(geometric_product_g0[1]) * this.group0.xyz) - (geometric_product_g2.zxy * this.group0.yzx) - (geometric_product_g3.zxy * this.group1.yzx)), geometric_product_g0[0] * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Origin sandwich(Origin other) {
        float4 geometric_product_g0 = float4(other.e4 * -1.0) * float4(this.group0.xyz, this.e321);
        return Origin(
            /* e4 */
            (geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3) + (geometric_product_g0[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Plane sandwich(Plane other) {
        float4 geometric_product_g0 = float4(0.0, 0.0, this.e2 * other.e423 * -1.0, 0.0) + (this.group0.yzxx * other.group0.zxyx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2)) + float4((float3(-(this.group0.zx * other.group0.yz), 0.0) - (float3(this.e321) * other.group0.xyz)), 0.0);
        float4 geometric_product_g1 = float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321);
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3((geometric_product_g0[1] * this.e3) + (geometric_product_g1[1] * this.e412) - (geometric_product_g0[2] * this.e2) - (geometric_product_g1[2] * this.e431), (geometric_product_g0[2] * this.e1) + (geometric_product_g1[2] * this.e423) - (geometric_product_g0[0] * this.e3) - (geometric_product_g1[0] * this.e412), (geometric_product_g0[0] * this.e2) + (geometric_product_g0[2] * this.e321) + (geometric_product_g1[0] * this.e431) + (geometric_product_g1[2] * this.e4) - (geometric_product_g0[1] * this.e1) - (geometric_product_g1[1] * this.e423)) + float3(((float2(this.e4) * geometric_product_g1.xy) + (float2(this.e321) * geometric_product_g0.xy)), 0.0) - (float3(geometric_product_g0[3]) * this.group0.xyz) - (float3(geometric_product_g1[3]) * this.group1.xyz)), geometric_product_g1[3] * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Point sandwich(Point other) {
        float3 geometric_product_g1_xyz = float3((this.e2 * other.e3) - (this.e3 * other.e2), (this.e3 * other.e1) - (this.e1 * other.e3), (this.e1 * other.e2) - (this.e2 * other.e1)) - (float3(this.e321) * other.group0.xyz);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1 * other.e1) * this.group0.xyz) + (geometric_product_g1_xyz.zxy * this.group0.yzx) - (geometric_product_g1_xyz.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g1_xyz.yzz * float3(this.group0.zx, this.e321))), (geometric_product_g1_xyz[0] * this.e423) + (geometric_product_g1_xyz[1] * this.e431))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public DualNum sandwich(Scalar other) {
        float4 geometric_product_g0 = float4(other.scalar) * this.group0;
        float4 geometric_product_g1 = float4(other.scalar) * this.group1;
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3) + (geometric_product_g1[3] * this.e321), -(geometric_product_g0[0] * this.e423) - (geometric_product_g0[1] * this.e431) - (geometric_product_g0[2] * this.e412) - (geometric_product_g0[3] * this.e321) - (geometric_product_g1[0] * this.e1) - (geometric_product_g1[1] * this.e2) - (geometric_product_g1[2] * this.e3) - (geometric_product_g1[3] * this.e4))
        );
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Horizon> operator /(Horizon lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Horizon> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public AntiScalar sandwich(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e321 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public DualNum sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e321) * float2(other.scalar * this.e321, other.e1234 * this.e321) * float2(1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Flector sandwich(Flector other) {
        float4 geometric_product_g0 = float4(this.e321 * -1.0) * float4(other.group1.xyz, other.e4);
        float4 geometric_product_g1 = float4(this.e321 * -1.0) * float4(other.group0.xyz, other.e321);
        float reverse_g0 = this.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(reverse_g0) * float4(geometric_product_g1.xyz, geometric_product_g0[3]) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(reverse_g0) * float4((geometric_product_g0.xyz * float3(-1.0)), geometric_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(SandwichInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Horizon sandwich(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * this.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Line sandwich(Line other) {
        float reverse_g0 = this.e321 * -1.0;
        return Line(
            /* e41, e42, e43 */
            float3(reverse_g0 * this.e321) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(reverse_g0 * this.e321 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Motor sandwich(Motor other) {
        float4 geometric_product_g0 = float4(this.e321) * float4(other.group1.xyz, other.e1234);
        float4 geometric_product_g1 = float4(this.e321) * float4(other.group0.xyz, other.scalar);
        float reverse_g0 = this.e321 * -1.0;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(reverse_g0) * float4(geometric_product_g1.xyz, geometric_product_g0[3]), 
            /* e23, e31, e12, scalar */
            float4(reverse_g0 * -1.0) * float4(geometric_product_g0.xyz, geometric_product_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public MultiVector sandwich(MultiVector other) {
        float2 geometric_product_g0 = float2(this.e321 * -1.0) * float2(other.e321, other.e4);
        float4 geometric_product_g1 = float4(this.e321) * float4(other.group3.xyz, other.e1234);
        float4 geometric_product_g4 = float4(this.e321) * float4(other.group2.xyz, other.scalar);
        float reverse_g0 = this.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(reverse_g0) * float2(geometric_product_g4[3], geometric_product_g1[3]) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(reverse_g0 * -1.0) * float4((float3(this.e321) * other.group1.xyz), geometric_product_g0[1]), 
            /* e41, e42, e43 */
            float3(reverse_g0) * geometric_product_g4.xyz, 
            /* e23, e31, e12 */
            float3(reverse_g0 * -1.0) * geometric_product_g1.xyz, 
            /* e423, e431, e412, e321 */
            float4(reverse_g0) * float4((float3(this.e321) * other.group4.xyz), geometric_product_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Horizon> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Origin sandwich(Origin other) {
        return Origin(
            /* e4 */
            this.e321 * this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Plane sandwich(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e321 * this.e321) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Point sandwich(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e321 * this.e321) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(SandwichInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Scalar sandwich(Scalar other) {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Line> operator /(Line lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public AntiScalar sandwich(AntiScalar other) {
        float3 geometric_product_g0 = float3(other.e1234) * this.group1.xyz;
        return AntiScalar(
            /* e1234 */
            (geometric_product_g0[0] * this.e23) + (geometric_product_g0[1] * this.e31) + (geometric_product_g0[2] * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public DualNum sandwich(DualNum other) {
        float3 geometric_product_g0 = (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz);
        float3 geometric_product_g1 = float3(other.scalar) * this.group1.xyz;
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g1[0] * this.e41) + (geometric_product_g1[1] * this.e42) + (geometric_product_g1[2] * this.e43)) + (float2(this.e23) * float2(geometric_product_g1[0], geometric_product_g0[0])) + (float2(this.e31) * float2(geometric_product_g1[1], geometric_product_g0[1])) + (float2(this.e12) * float2(geometric_product_g1[2], geometric_product_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Flector sandwich(Flector other) {
        float3 geometric_product_g0_xyz = (float3(other.e2, other.e321, other.e321) * this.group1.zyz) + (float3(other.e321, other.e3, other.e1) * this.group1.xxy) - (this.group1.yzx * other.group0.zxy);
        float3 geometric_product_g1_xyz = (float3(other.e4, other.e412, other.e423) * this.group1.xxy) + (float3(other.e431, other.e4, other.e4) * this.group1.zyz) + (this.group0.yzx * other.group0.zxy) - (float3(other.e2, other.e321, other.e321) * this.group0.zyz) - (float3(other.e321, other.e3, other.e1) * this.group0.xxy) - (this.group1.yzx * other.group1.zxy);
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((geometric_product_g0_xyz.zxy * reverse_g1.yzx) - (geometric_product_g0_xyz.yzx * reverse_g1.zxy)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(((reverse_g1 * float3(this.e41 * other.e1)) + (reverse_g1 * float3(this.e42 * other.e2)) + (geometric_product_g0_xyz.zxy * reverse_g0.yzx) + (geometric_product_g1_xyz.zxy * reverse_g1.yzx) - (reverse_g1 * float3(this.e23 * other.e423)) - (reverse_g1 * float3(this.e31 * other.e431)) - (reverse_g1 * float3(this.e12 * other.e412)) - (geometric_product_g0_xyz.yzx * reverse_g0.zxy) - (geometric_product_g1_xyz.yzx * reverse_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Plane sandwich(Horizon other) {
        float3 geometric_product_g0_xyz = float3(other.e321) * this.group1.xyz;
        float3 geometric_product_g1_xyz = float3(other.e321 * -1.0) * this.group0.xyz;
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((geometric_product_g0_xyz.zxy * reverse_g0.yzx) + (geometric_product_g1_xyz.zxy * reverse_g1.yzx) - (geometric_product_g0_xyz.yzx * reverse_g0.zxy) - (geometric_product_g1_xyz.yzx * reverse_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Line sandwich(Line other) {
        float4 geometric_product_g0 = float4(((other.group0.yzx * this.group1.zxy) + (other.group1.yzx * this.group0.zxy) - (other.group0.zxy * this.group1.yzx) - (other.group1.zxy * this.group0.yzx)), 0.0);
        float4 geometric_product_g1 = float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0);
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return Line(
            /* e41, e42, e43 */
            (reverse_g0.xyx * geometric_product_g1.wwy) + (reverse_g0.yzz * geometric_product_g1.zxw) + (reverse_g1.xyx * geometric_product_g0.wwy) + (reverse_g1.yzz * geometric_product_g0.zxw) - (reverse_g0.zxy * geometric_product_g1.yzx) - (reverse_g1.zxy * geometric_product_g0.yzx), 
            /* e23, e31, e12 */
            (reverse_g1.xyx * geometric_product_g1.wwy) + (reverse_g1.yzz * geometric_product_g1.zxw) - (reverse_g1.zxy * geometric_product_g1.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Motor sandwich(Motor other) {
        float4 geometric_product_g0 = float4(((this.group0.xxy * other.group1.wzx) + (this.group0.zyz * other.group1.yww) + (this.group1.xxy * other.group0.wzx) + (this.group1.zyz * other.group0.yww) - (this.group0.yzx * other.group1.zxy) - (this.group1.yzx * other.group0.zxy)), 0.0);
        float4 geometric_product_g1 = float4(((this.group1.xxy * other.group1.wzx) + (this.group1.zyz * other.group1.yww) - (this.group1.yzx * other.group1.zxy)), 0.0);
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((reverse_g0.xyx * geometric_product_g1.wwy) + (reverse_g0.yzz * geometric_product_g1.zxw) + (reverse_g1.xyx * geometric_product_g0.wwy) + (reverse_g1.yzz * geometric_product_g0.zxw) - (reverse_g0.zxy * geometric_product_g1.yzx) - (reverse_g1.zxy * geometric_product_g0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((reverse_g1.xyx * geometric_product_g1.wwy) + (reverse_g1.yzz * geometric_product_g1.zxw) - (reverse_g1.zxy * geometric_product_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public MultiVector sandwich(MultiVector other) {
        float2 geometric_product_g0 = float2(0.0, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43));
        float3 geometric_product_g1_xyz = (float3(other.e2, other.e321, other.e321) * this.group1.zyz) + (float3(other.e321, other.e3, other.e1) * this.group1.xxy) - (this.group1.yzx * other.group1.zxy);
        float3 geometric_product_g2 = (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) + (this.group1.zxy * other.group2.yzx) - (this.group0.yzx * other.group3.zxy) - (this.group1.yzx * other.group2.zxy);
        float3 geometric_product_g3 = (float3(other.scalar) * this.group1.xyz) + (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy);
        float3 geometric_product_g4_xyz = (float3(other.e4, other.e412, other.e423) * this.group1.xxy) + (float3(other.e431, other.e4, other.e4) * this.group1.zyz) + (this.group0.yzx * other.group1.zxy) - (float3(other.e2, other.e321, other.e321) * this.group0.zyz) - (float3(other.e321, other.e3, other.e1) * this.group0.xxy) - (this.group1.yzx * other.group4.zxy);
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(geometric_product_g3[0] * reverse_g0[0]) - (geometric_product_g3[1] * reverse_g0[1]) - (geometric_product_g3[2] * reverse_g0[2])) - (float2(reverse_g1[0]) * float2(geometric_product_g3[0], geometric_product_g2[0])) - (float2(reverse_g1[1]) * float2(geometric_product_g3[1], geometric_product_g2[1])) - (float2(reverse_g1[2]) * float2(geometric_product_g3[2], geometric_product_g2[2])), 
            /* e1, e2, e3, e4 */
            float4(((geometric_product_g1_xyz.zxy * reverse_g1.yzx) - (geometric_product_g1_xyz.yzx * reverse_g1.zxy)), 0.0), 
            /* e41, e42, e43 */
            (reverse_g0 * float3(geometric_product_g0[0])) + (reverse_g1 * float3(geometric_product_g0[1])) + (geometric_product_g2.zxy * reverse_g1.yzx) + (geometric_product_g3.zxy * reverse_g0.yzx) - (geometric_product_g2.yzx * reverse_g1.zxy) - (geometric_product_g3.yzx * reverse_g0.zxy), 
            /* e23, e31, e12 */
            (reverse_g1 * float3(geometric_product_g0[0])) + (geometric_product_g3.zxy * reverse_g1.yzx) - (geometric_product_g3.yzx * reverse_g1.zxy), 
            /* e423, e431, e412, e321 */
            float4(((reverse_g1 * float3(this.e41 * other.e1)) + (reverse_g1 * float3(this.e42 * other.e2)) + (geometric_product_g1_xyz.zxy * reverse_g0.yzx) + (geometric_product_g4_xyz.zxy * reverse_g1.yzx) - (reverse_g1 * float3(this.e23 * other.e423)) - (reverse_g1 * float3(this.e31 * other.e431)) - (reverse_g1 * float3(this.e12 * other.e412)) - (geometric_product_g1_xyz.yzx * reverse_g0.zxy) - (geometric_product_g4_xyz.yzx * reverse_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Origin sandwich(Origin other) {
        float3 geometric_product_g0_xyz = float3(other.e4) * this.group1.xyz;
        return Origin(
            /* e4 */
            (geometric_product_g0_xyz[0] * this.e23) + (geometric_product_g0_xyz[1] * this.e31) + (geometric_product_g0_xyz[2] * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Plane sandwich(Plane other) {
        float3 geometric_product_g0_xyz = float3(other.e321) * this.group1.xyz;
        float3 geometric_product_g1_xyz = (this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy);
        float3 reverse_g0 = this.group0.xyz * float3(-1.0);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((geometric_product_g0_xyz.zxy * reverse_g0.yzx) + (geometric_product_g1_xyz.zxy * reverse_g1.yzx) - (reverse_g1 * float3(this.e23 * other.e423)) - (reverse_g1 * float3(this.e31 * other.e431)) - (reverse_g1 * float3(this.e12 * other.e412)) - (geometric_product_g0_xyz.yzx * reverse_g0.zxy) - (geometric_product_g1_xyz.yzx * reverse_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Point sandwich(Point other) {
        float3 geometric_product_g0_xyz = (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float3 reverse_g1 = this.group1.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((geometric_product_g0_xyz.zxy * reverse_g1.yzx) - (geometric_product_g0_xyz.yzx * reverse_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public DualNum sandwich(Scalar other) {
        float3 geometric_product_g0 = float3(other.scalar) * this.group0.xyz;
        float3 geometric_product_g1 = float3(other.scalar) * this.group1.xyz;
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g1[0] * this.e41) + (geometric_product_g1[1] * this.e42) + (geometric_product_g1[2] * this.e43)) + (float2(this.e23) * float2(geometric_product_g1[0], geometric_product_g0[0])) + (float2(this.e31) * float2(geometric_product_g1[1], geometric_product_g0[1])) + (float2(this.e12) * float2(geometric_product_g1[2], geometric_product_g0[2]))
        );
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Motor> operator /(Motor lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public AntiScalar sandwich(AntiScalar other) {
        float4 geometric_product_g0 = float4(other.e1234) * this.group1;
        return AntiScalar(
            /* e1234 */
            (geometric_product_g0[0] * this.e23) + (geometric_product_g0[1] * this.e31) + (geometric_product_g0[2] * this.e12) + (geometric_product_g0[3] * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public DualNum sandwich(DualNum other) {
        float4 geometric_product_g0 = (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1);
        float4 geometric_product_g1 = float4(other.scalar) * this.group1;
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g1[0] * this.e41) + (geometric_product_g1[1] * this.e42) + (geometric_product_g1[2] * this.e43) + (geometric_product_g1[3] * this.e1234)) + (float2(this.e23) * float2(geometric_product_g1[0], geometric_product_g0[0])) + (float2(this.e31) * float2(geometric_product_g1[1], geometric_product_g0[1])) + (float2(this.e12) * float2(geometric_product_g1[2], geometric_product_g0[2])) + (float2(this.scalar) * float2(geometric_product_g1[3], geometric_product_g0[3]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Flector sandwich(Flector other) {
        float4 geometric_product_g0 = (other.group0.xyxx * float4(this.group1.wwy, this.e41)) + (other.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4((float3(other.e321) * this.group1.xyz), (other.e3 * this.e43) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) - (this.group1.yzxx * float4(other.group0.zxy, other.e423));
        float3 geometric_product_g1_xyz = float3((other.e3 * this.e42) + (other.e431 * this.e12) - (other.e2 * this.e43) - (other.e412 * this.e31), (other.e1 * this.e43) + (other.e412 * this.e23) - (other.e3 * this.e41) - (other.e423 * this.e12), (other.e2 * this.e41) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e1 * this.e42) - (other.e3 * this.e1234) - (other.e431 * this.e23)) + (float3(other.e4) * this.group1.xyz) + float3(((float2(this.scalar) * other.group1.xy) - (float2(this.e1234) * other.group0.xy)), 0.0) - (float3(other.e321) * this.group0.xyz);
        float geometric_product_g1_w = other.e321 * this.scalar;
        return Flector(
            /* e1, e2, e3, e4 */
            (geometric_product_g0.yzzw * this.group1.zxww) + float4((float3(((float2(this.scalar) * geometric_product_g0.xy) - (geometric_product_g0.zx * this.group1.yz)), (geometric_product_g0[0] * this.e31) - (geometric_product_g0[1] * this.e23)) - (float3(geometric_product_g1_w) * this.group1.xyz)), geometric_product_g1_w * this.e1234), 
            /* e423, e431, e412, e321 */
            float4((float3((geometric_product_g0[1] * this.e43) - (geometric_product_g0[2] * this.e42), (geometric_product_g0[2] * this.e41) - (geometric_product_g0[0] * this.e43), (geometric_product_g0[0] * this.e42) + (geometric_product_g0[2] * this.e1234) - (geometric_product_g0[1] * this.e41)) + (geometric_product_g1_xyz.xyx * this.group1.wwy) + (geometric_product_g1_xyz.yzz * this.group1.zxw) + float3((float2(this.e1234) * geometric_product_g0.xy), 0.0) - (float3(geometric_product_g1_w) * this.group0.xyz) - (float3(geometric_product_g0[3]) * this.group1.xyz) - (geometric_product_g1_xyz.zxy * this.group1.yzx)), geometric_product_g1_w * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Plane sandwich(Horizon other) {
        float4 geometric_product_g0 = float4(other.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float4 geometric_product_g1 = float4(other.e321) * float4((this.group0.xyz * float3(-1.0)), this.scalar);
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3((geometric_product_g0[1] * this.e43) + (geometric_product_g1[1] * this.e12) - (geometric_product_g0[2] * this.e42) - (geometric_product_g1[2] * this.e31), (geometric_product_g0[2] * this.e41) + (geometric_product_g1[2] * this.e23) - (geometric_product_g0[0] * this.e43) - (geometric_product_g1[0] * this.e12), (geometric_product_g0[0] * this.e42) + (geometric_product_g0[2] * this.e1234) + (geometric_product_g1[0] * this.e31) + (geometric_product_g1[2] * this.scalar) - (geometric_product_g0[1] * this.e41) - (geometric_product_g1[1] * this.e23)) + float3(((float2(this.e1234) * geometric_product_g0.xy) + (float2(this.scalar) * geometric_product_g1.xy)), 0.0) - (float3(geometric_product_g0[3]) * this.group1.xyz) - (float3(geometric_product_g1[3]) * this.group0.xyz)), geometric_product_g1[3] * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Line sandwich(Line other) {
        float3 geometric_product_g0_xyz = (other.group0.xyx * this.group1.wwy) + (other.group0.yzz * this.group1.zxw) + (other.group1.xyx * this.group0.wwy) + (other.group1.yzz * this.group0.zxw) - (other.group0.zxy * this.group1.yzx) - (other.group1.zxy * this.group0.yzx);
        float3 geometric_product_g1_xyz = (other.group1.xyx * this.group1.wwy) + (other.group1.yzz * this.group1.zxw) - (other.group1.zxy * this.group1.yzx);
        return Line(
            /* e41, e42, e43 */
            (geometric_product_g0_xyz.xyx * this.group1.wwy) + (geometric_product_g0_xyz.yzz * this.group1.zxw) + (geometric_product_g1_xyz.xyx * this.group0.wwy) + (geometric_product_g1_xyz.yzz * this.group0.zxw) - (geometric_product_g0_xyz.zxy * this.group1.yzx) - (geometric_product_g1_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (geometric_product_g1_xyz.xyx * this.group1.wwy) + (geometric_product_g1_xyz.yzz * this.group1.zxw) - (geometric_product_g1_xyz.zxy * this.group1.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Motor sandwich(Motor other) {
        float4 geometric_product_g0 = (other.group0.xyxw * this.group1.wwyw) + float4(((float3(other.e1234) * this.group1.xyz) + (float3(other.scalar) * this.group0.xyz) + float3(((float2(this.e1234) * other.group1.xy) + (other.group0.yz * this.group1.zx) + (other.group1.yz * this.group0.zx) - (other.group0.zx * this.group1.yz) - (other.group1.zx * this.group0.yz)), (other.e43 * this.scalar) + (other.e23 * this.e42) + (other.e12 * this.e1234) - (other.e42 * this.e23) - (other.e31 * this.e41))), other.scalar * this.e1234);
        float3 geometric_product_g1_xyz = float3((other.e31 * this.e12) - (other.e12 * this.e31), (other.e12 * this.e23) - (other.e23 * this.e12), (other.e23 * this.e31) + (other.e12 * this.scalar) - (other.e31 * this.e23)) + (float3(other.scalar) * this.group1.xyz) + float3((float2(this.scalar) * other.group1.xy), 0.0);
        float geometric_product_g1_w = other.scalar * this.scalar;
        return Motor(
            /* e41, e42, e43, e1234 */
            (this.group0.wwyw * float4(geometric_product_g1_xyz.xyx, geometric_product_g1_w)) + float4(((geometric_product_g1_xyz.yzz * this.group0.zxw) + float3(((float2(this.scalar) * geometric_product_g0.xy) + (geometric_product_g0.yz * this.group1.zx) - (geometric_product_g0.zx * this.group1.yz)), (geometric_product_g0[0] * this.e31) + (geometric_product_g0[2] * this.scalar) - (geometric_product_g0[1] * this.e23)) - (float3(geometric_product_g1_w) * this.group0.xyz) - (float3(geometric_product_g0[3]) * this.group1.xyz) - (geometric_product_g1_xyz.zxy * this.group0.yzx)), geometric_product_g0[3] * this.scalar), 
            /* e23, e31, e12, scalar */
            float4(((geometric_product_g1_xyz.xyx * this.group1.wwy) + (geometric_product_g1_xyz.yzz * this.group1.zxw) - (float3(geometric_product_g1_w) * this.group1.xyz) - (geometric_product_g1_xyz.zxy * this.group1.yzx)), geometric_product_g1_w * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public MultiVector sandwich(MultiVector other) {
        float2 geometric_product_g0 = float2(other.e23 * this.e23 * -1.0, (other.e1234 * this.scalar) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float2(other.scalar) * float2(this.scalar, this.e1234)) - (float2(other.e31, other.e41) * this.group1.yx) - (float2(other.e12, other.e42) * this.group1.zy);
        float4 geometric_product_g1 = (other.group1.xyzz * float4(float3(this.scalar), this.e43)) + float4(((float2(this.e23) * float2(other.e321, other.e3)) + (float2(other.e2, other.e321) * this.group1.zy) - (this.group1.yz * other.group1.zx)), (this.e31 * other.e1) + (this.e12 * other.e321) - (this.e23 * other.e2), (this.e41 * other.e1) + (this.e42 * other.e2) - (this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412));
        float3 geometric_product_g2 = (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xyx * this.group1.wwy) + (other.group2.yzz * this.group1.zxw) + (other.group3.xyx * this.group0.wwy) + (other.group3.yzz * this.group0.zxw) - (other.group2.zxy * this.group1.yzx) - (other.group3.zxy * this.group0.yzx);
        float3 geometric_product_g3 = (float3(other.scalar) * this.group1.xyz) + (other.group3.xyx * this.group1.wwy) + (other.group3.yzz * this.group1.zxw) - (other.group3.zxy * this.group1.yzx);
        float3 geometric_product_g4_xyz = float3((this.e42 * other.e3) + (this.e12 * other.e431) - (this.e43 * other.e2) - (this.e31 * other.e412), (this.e43 * other.e1) + (this.e31 * other.e4) - (this.e42 * other.e321) - (this.e12 * other.e423), (this.e41 * other.e2) + (this.e31 * other.e423) + (this.e12 * other.e4) - (this.e42 * other.e1) - (this.e43 * other.e321) - (this.e23 * other.e431)) + (float3(this.scalar) * other.group4.xyz) + float3(((float2(this.e23) * float2(other.e4, other.e412)) - (float2(this.e41) * float2(other.e321, other.e3))), 0.0) - (float3(this.e1234) * other.group1.xyz);
        float geometric_product_g4_w = this.scalar * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g2[2] * this.e12) + (geometric_product_g3[0] * this.e41) + (geometric_product_g3[1] * this.e42) + (geometric_product_g3[2] * this.e43)) + (float2(geometric_product_g0[0]) * float2(this.scalar, this.e1234)) + (float2(geometric_product_g3[0], geometric_product_g0[1]) * this.group1.xw) + (float2(geometric_product_g3[1], geometric_product_g2[0]) * this.group1.yx) + (float2(geometric_product_g3[2], geometric_product_g2[1]) * this.group1.zy), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1.yzzw * this.group1.zxww) + float4((float3(((float2(this.scalar) * geometric_product_g1.xy) - (geometric_product_g1.zx * this.group1.yz)), (geometric_product_g1[0] * this.e31) - (geometric_product_g1[1] * this.e23)) - (float3(geometric_product_g4_w) * this.group1.xyz)), geometric_product_g4_w * this.e1234), 
            /* e41, e42, e43 */
            (geometric_product_g2.xyx * this.group1.wwy) + (geometric_product_g2.yzz * this.group1.zxw) + (geometric_product_g3.xyx * this.group0.wwy) + (geometric_product_g3.yzz * this.group0.zxw) - (float3(geometric_product_g0[0]) * this.group0.xyz) - (float3(geometric_product_g0[1]) * this.group1.xyz) - (geometric_product_g2.zxy * this.group1.yzx) - (geometric_product_g3.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (geometric_product_g3.xyx * this.group1.wwy) + (geometric_product_g3.yzz * this.group1.zxw) - (float3(geometric_product_g0[0]) * this.group1.xyz) - (geometric_product_g3.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3((geometric_product_g1[1] * this.e43) - (geometric_product_g1[2] * this.e42), (geometric_product_g1[2] * this.e41) - (geometric_product_g1[0] * this.e43), (geometric_product_g1[0] * this.e42) + (geometric_product_g1[2] * this.e1234) - (geometric_product_g1[1] * this.e41)) + (geometric_product_g4_xyz.xyx * this.group1.wwy) + (geometric_product_g4_xyz.yzz * this.group1.zxw) + float3((float2(this.e1234) * geometric_product_g1.xy), 0.0) - (float3(geometric_product_g4_w) * this.group0.xyz) - (float3(geometric_product_g1[3]) * this.group1.xyz) - (geometric_product_g4_xyz.zxy * this.group1.yzx)), geometric_product_g4_w * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Origin sandwich(Origin other) {
        float3 geometric_product_g1_xyz = float3(other.e4) * this.group1.xyz;
        return Origin(
            /* e4 */
            (geometric_product_g1_xyz[0] * this.e23) + (geometric_product_g1_xyz[1] * this.e31) + (geometric_product_g1_xyz[2] * this.e12) + (this.scalar * this.scalar * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Plane sandwich(Plane other) {
        float3 geometric_product_g0_xyz = float3(other.e321) * this.group1.xyz;
        float3 geometric_product_g1_xyz = float3((this.e12 * other.e431) - (this.e31 * other.e412), (this.e23 * other.e412) - (this.e12 * other.e423), (this.e31 * other.e423) - (this.e23 * other.e431)) + (float3(this.scalar) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz);
        float geometric_product_g1_w = this.scalar * other.e321;
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e23 * this.e23 * other.e423, this.e31 * this.e31 * other.e431, this.e12 * this.e12 * other.e412) + (float3(this.e23) * float3(this.e31 * other.e431, this.e31 * other.e423, this.e12 * other.e423)) + (float3(this.e12) * float3(this.e23 * other.e412, this.e31 * other.e412, this.e31 * other.e431)) + (float3(this.e1234 * other.e321) * this.group1.xyz) + (geometric_product_g0_xyz.xyx * this.group0.wwy) + (geometric_product_g0_xyz.yzz * this.group0.zxw) + (geometric_product_g1_xyz.xyx * this.group1.wwy) + (geometric_product_g1_xyz.yzz * this.group1.zxw) - (float3(geometric_product_g1_w) * this.group0.xyz) - (geometric_product_g0_xyz.zxy * this.group0.yzx) - (geometric_product_g1_xyz.zxy * this.group1.yzx)), geometric_product_g1_w * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Point sandwich(Point other) {
        float4 geometric_product_g0 = float4(0.0, 0.0, this.e23 * other.e2 * -1.0, 0.0) + (other.group0.xyzy * float4(float3(this.scalar), this.e42)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)) + float4(-(this.group1.yz * other.group0.zx), 0.0, 0.0);
        float4 geometric_product_g1 = float4(0.0, 0.0, (this.e41 * other.e2) - (this.e42 * other.e1), 0.0) + float4(((float3(other.e4) * this.group1.xyz) + float3(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), 0.0) - (float3(this.e1234) * other.group0.xyz)), 0.0);
        return Point(
            /* e1, e2, e3, e4 */
            (geometric_product_g0.xyxw * this.group1.wwyw) + float4((float3(((geometric_product_g0.yz * this.group1.zx) - (geometric_product_g0.zx * this.group1.yz)), (geometric_product_g0[2] * this.scalar) - (geometric_product_g0[1] * this.e23)) - (float3(geometric_product_g1[3]) * this.group1.xyz)), geometric_product_g1[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public DualNum sandwich(Scalar other) {
        float4 geometric_product_g0 = float4(other.scalar) * this.group0;
        float4 geometric_product_g1 = float4(other.scalar) * this.group1;
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g1[0] * this.e41) + (geometric_product_g1[1] * this.e42) + (geometric_product_g1[2] * this.e43) + (geometric_product_g1[3] * this.e1234)) + (float2(this.e23) * float2(geometric_product_g1[0], geometric_product_g0[0])) + (float2(this.e31) * float2(geometric_product_g1[1], geometric_product_g0[1])) + (float2(this.e12) * float2(geometric_product_g1[2], geometric_product_g0[2])) + (float2(this.scalar) * float2(geometric_product_g1[3], geometric_product_g0[3]))
        );
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<MultiVector> operator /(MultiVector lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(AntiScalar other) {
        float geometric_product_g0_y = this.scalar * other.e1234;
        float geometric_product_g1_w = this.e321 * other.e1234;
        float3 geometric_product_g2 = float3(other.e1234) * this.group3.xyz;
        float3 geometric_product_g4_xyz = float3(other.e1234) * this.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g0_y * this.scalar) + (geometric_product_g2[0] * this.e23) + (geometric_product_g2[1] * this.e31) + (geometric_product_g2[2] * this.e12) - (geometric_product_g1_w * this.e321) - (geometric_product_g4_xyz[0] * this.e1) - (geometric_product_g4_xyz[1] * this.e2) - (geometric_product_g4_xyz[2] * this.e3)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (geometric_product_g0_y * this.e321) + (geometric_product_g1_w * this.scalar) + (geometric_product_g2[0] * this.e1) + (geometric_product_g2[1] * this.e2) + (geometric_product_g2[2] * this.e3) + (geometric_product_g4_xyz[0] * this.e23) + (geometric_product_g4_xyz[1] * this.e31) + (geometric_product_g4_xyz[2] * this.e12)), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(DualNum other) {
        float geometric_product_g0_x = other.scalar * this.scalar;
        float geometric_product_g0_y = (other.scalar * this.e1234) + (other.e1234 * this.scalar);
        float3 geometric_product_g1_xyz = float3(other.scalar) * this.group1.xyz;
        float geometric_product_g1_w = (other.scalar * this.e4) + (other.e1234 * this.e321);
        float3 geometric_product_g2 = (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz);
        float3 geometric_product_g3 = float3(other.scalar) * this.group3.xyz;
        float3 geometric_product_g4_xyz = (float3(other.scalar) * this.group4.xyz) + (float3(other.e1234) * this.group1.xyz);
        float geometric_product_g4_w = other.scalar * this.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g4_w * this.e321) + (geometric_product_g1_xyz[0] * this.e1) + (geometric_product_g1_xyz[1] * this.e2) + (geometric_product_g1_xyz[2] * this.e3), (geometric_product_g0_y * this.scalar) + (geometric_product_g2[0] * this.e23) + (geometric_product_g2[1] * this.e31) + (geometric_product_g2[2] * this.e12) - (geometric_product_g1_w * this.e321) - (geometric_product_g4_w * this.e4) - (geometric_product_g1_xyz[0] * this.e423) - (geometric_product_g1_xyz[1] * this.e431) - (geometric_product_g1_xyz[2] * this.e412) - (geometric_product_g4_xyz[0] * this.e1) - (geometric_product_g4_xyz[1] * this.e2) - (geometric_product_g4_xyz[2] * this.e3)) + (float2(geometric_product_g0_x) * this.group0.xy) + (float2(geometric_product_g3[0]) * float2(this.e23, this.e41)) + (float2(geometric_product_g3[1]) * float2(this.e31, this.e42)) + (float2(geometric_product_g3[2]) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g0_x) * this.group1) + float4(((geometric_product_g1_xyz * float3(this.scalar)) + (geometric_product_g1_xyz.yzx * this.group3.zxy) + (geometric_product_g3.zxy * this.group1.yzx) - (float3(geometric_product_g4_w) * this.group3.xyz) - (geometric_product_g1_xyz.zxy * this.group3.yzx) - (geometric_product_g3.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g3.yzz * float3(this.group1.zx, this.e321))), (geometric_product_g0_y * this.e321) + (geometric_product_g1_w * this.scalar) + (geometric_product_g4_w * this.e1234)), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Flector other) {
        float2 geometric_product_g0 = float2(other.e321 * this.e321 * -1.0, (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412));
        float4 geometric_product_g1 = (float4(this.scalar) * other.group0) + float4(((float3(other.e2, other.e321, other.e321) * this.group3.zyz) + (float3(other.e321, other.e3, other.e1) * this.group3.xxy) - (this.group3.yzx * other.group0.zxy)), (this.e41 * other.e1) + (this.e42 * other.e2) - (this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412));
        float3 geometric_product_g2 = float3((other.e2 * this.e412) + (other.e412 * this.e2) - (other.e3 * this.e431) - (other.e431 * this.e3), (other.e3 * this.e423) + (other.e423 * this.e3) - (other.e1 * this.e412) - (other.e412 * this.e1), (other.e1 * this.e431) + (other.e3 * this.e4) + (other.e431 * this.e1) - (other.e2 * this.e423) - (other.e423 * this.e2) - (other.e412 * this.e321)) + (float3(other.e321) * this.group4.xyz) + float3(((float2(this.e4) * other.group0.xy) - (float2(this.e321) * other.group1.xy)), 0.0) - (float3(other.e4) * this.group1.xyz);
        float3 geometric_product_g3 = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2) - (other.e3 * this.e321)) + float3(-(float2(this.e321) * other.group0.xy), 0.0) - (float3(other.e321) * this.group1.xyz);
        float4 geometric_product_g4 = float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e4, other.e412, other.e423) * this.group3.xxy) + (float3(other.e431, other.e4, other.e4) * this.group3.zyz) + (this.group2.yzx * other.group0.zxy) - (float3(this.e1234) * other.group0.xyz) - (float3(other.e2, other.e321, other.e321) * this.group2.zyz) - (float3(other.e321, other.e3, other.e1) * this.group2.xxy) - (this.group3.yzx * other.group1.zxy)), this.scalar * other.e321);
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 reverse_g3 = this.group3.xyz * float3(-1.0);
        float4 reverse_g4 = this.group4 * float4(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g1[0] * this.e1) - (geometric_product_g3[0] * reverse_g3[0]) - (geometric_product_g3[1] * reverse_g3[1]) - (geometric_product_g4[3] * reverse_g4[3]), (geometric_product_g0[1] * this.scalar) + (geometric_product_g1[2] * reverse_g4[2]) + (geometric_product_g1[3] * reverse_g4[3]) - (geometric_product_g2[1] * reverse_g3[1]) - (geometric_product_g2[2] * reverse_g3[2]) - (geometric_product_g3[0] * reverse_g2[0]) - (geometric_product_g3[1] * reverse_g2[1]) - (geometric_product_g3[2] * reverse_g2[2]) - (geometric_product_g4[0] * this.e1) - (geometric_product_g4[1] * this.e2) - (geometric_product_g4[2] * this.e3) - (geometric_product_g4[3] * this.e4)) + (float2(geometric_product_g0[0]) * this.group0.xy) + (float2(this.e2, reverse_g4[0]) * geometric_product_g1.yx) + (float2(this.e3, reverse_g4[1]) * geometric_product_g1.zy) - (float2(geometric_product_g3[2], geometric_product_g2[0]) * reverse_g3.zx), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1 * float4(this.scalar)) + (float4(geometric_product_g0[0]) * this.group1) + float4(((float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g3.xxy) + (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g3.zyz) + (reverse_g3.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) + (reverse_g3.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3])) - (geometric_product_g3.yzx * this.group1.zxy) - (reverse_g3.zxy * geometric_product_g1.yzx)), (this.e1234 * geometric_product_g4[3]) + (geometric_product_g2[0] * this.e1) + (geometric_product_g2[1] * this.e2) + (geometric_product_g2[2] * this.e3) - (geometric_product_g0[1] * reverse_g4[3]) - (geometric_product_g3[0] * reverse_g4[0]) - (geometric_product_g3[1] * reverse_g4[1]) - (geometric_product_g3[2] * reverse_g4[2]) - (reverse_g2[0] * geometric_product_g1[0]) - (reverse_g2[1] * geometric_product_g1[1]) - (reverse_g2[2] * geometric_product_g1[2]) - (reverse_g3[0] * geometric_product_g4[0]) - (reverse_g3[1] * geometric_product_g4[1]) - (reverse_g3[2] * geometric_product_g4[2])), 
            /* e41, e42, e43 */
            float3((geometric_product_g1[1] * reverse_g4[2]) + (geometric_product_g4[2] * this.e2) - (geometric_product_g1[2] * reverse_g4[1]) - (geometric_product_g4[1] * this.e3), (geometric_product_g1[2] * reverse_g4[0]) + (geometric_product_g4[1] * reverse_g4[3]) - (geometric_product_g1[1] * this.e4) - (geometric_product_g4[2] * this.e1), (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * this.e1) + (geometric_product_g4[2] * reverse_g4[3]) - (geometric_product_g1[1] * reverse_g4[0]) - (geometric_product_g1[2] * this.e4) - (geometric_product_g4[0] * this.e2)) + (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (reverse_g2 * float3(geometric_product_g0[0])) + (reverse_g3 * float3(geometric_product_g0[1])) + (float3(geometric_product_g1[3]) * this.group1.xyz) + (geometric_product_g2.zxy * reverse_g3.yzx) + (geometric_product_g3.zxy * reverse_g2.yzx) + float3(((float2(geometric_product_g4[0]) * float2(reverse_g4[3], this.e3)) - (float2(geometric_product_g1[0]) * float2(this.e4, reverse_g4[2]))), 0.0) - (float3(geometric_product_g4[3]) * reverse_g4.xyz) - (geometric_product_g2.yzx * reverse_g3.zxy) - (geometric_product_g3.yzx * reverse_g2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_product_g1[1] * this.e3) - (geometric_product_g1[2] * this.e2), (geometric_product_g1[2] * this.e1) - (geometric_product_g1[1] * reverse_g4[3]), (geometric_product_g1[0] * this.e2) - (geometric_product_g1[1] * this.e1) - (geometric_product_g1[2] * reverse_g4[3])) + (geometric_product_g3 * float3(this.scalar)) + (reverse_g3 * float3(geometric_product_g0[0])) + (geometric_product_g3.zxy * reverse_g3.yzx) + float3(-(float2(geometric_product_g1[0]) * float2(reverse_g4[3], this.e3)), 0.0) - (float3(geometric_product_g4[3]) * this.group1.xyz) - (geometric_product_g3.yzx * reverse_g3.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_product_g4 * float4(this.scalar)) + (reverse_g4 * float4(geometric_product_g0[0])) + float4(((float3(this.e1234) * geometric_product_g1.xyz) + (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g3.zyz) + (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g3.xxy) + (geometric_product_g2.yzx * this.group1.zxy) + (reverse_g2.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) + (reverse_g2.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3])) + (reverse_g3.xyx * float3(float2(geometric_product_g1[3]), geometric_product_g4[1])) + (reverse_g3.yzz * float3(geometric_product_g4.zx, geometric_product_g1[3])) - (float3(geometric_product_g0[1]) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g2.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g2.zyz) - (geometric_product_g3.yzx * reverse_g4.zxy) - (reverse_g2.zxy * geometric_product_g1.yzx) - (reverse_g3.zxy * geometric_product_g4.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Horizon other) {
        float2 geometric_product_g0 = float2(other.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0);
        float4 geometric_product_g1 = float4(other.e321) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float3 geometric_product_g2 = float3(other.e321) * this.group4.xyz;
        float3 geometric_product_g3 = float3(other.e321 * -1.0) * this.group1.xyz;
        float4 geometric_product_g4 = float4(other.e321) * float4((this.group2.xyz * float3(-1.0)), this.scalar);
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 reverse_g3 = this.group3.xyz * float3(-1.0);
        float4 reverse_g4 = this.group4 * float4(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3((geometric_product_g1[1] * reverse_g4[2]) + (geometric_product_g4[2] * this.e2) - (geometric_product_g1[2] * reverse_g4[1]) - (geometric_product_g4[1] * this.e3), (geometric_product_g1[2] * reverse_g4[0]) + (geometric_product_g4[1] * reverse_g4[3]) - (geometric_product_g1[1] * this.e4) - (geometric_product_g4[2] * this.e1), (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * this.e1) + (geometric_product_g4[2] * reverse_g4[3]) - (geometric_product_g1[1] * reverse_g4[0]) - (geometric_product_g1[2] * this.e4) - (geometric_product_g4[0] * this.e2)) + (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (reverse_g2 * float3(geometric_product_g0[0])) + (reverse_g3 * float3(geometric_product_g0[1])) + (float3(geometric_product_g1[3]) * this.group1.xyz) + (geometric_product_g2.zxy * reverse_g3.yzx) + (geometric_product_g3.zxy * reverse_g2.yzx) + float3(((float2(geometric_product_g4[0]) * float2(reverse_g4[3], this.e3)) - (float2(geometric_product_g1[0]) * float2(this.e4, reverse_g4[2]))), 0.0) - (float3(geometric_product_g4[3]) * reverse_g4.xyz) - (geometric_product_g2.yzx * reverse_g3.zxy) - (geometric_product_g3.yzx * reverse_g2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_product_g1[1] * this.e3) - (geometric_product_g1[2] * this.e2), (geometric_product_g1[2] * this.e1) - (geometric_product_g1[1] * reverse_g4[3]), (geometric_product_g1[0] * this.e2) - (geometric_product_g1[1] * this.e1) - (geometric_product_g1[2] * reverse_g4[3])) + (geometric_product_g3 * float3(this.scalar)) + (reverse_g3 * float3(geometric_product_g0[0])) + (geometric_product_g3.zxy * reverse_g3.yzx) + float3(-(float2(geometric_product_g1[0]) * float2(reverse_g4[3], this.e3)), 0.0) - (float3(geometric_product_g4[3]) * this.group1.xyz) - (geometric_product_g3.yzx * reverse_g3.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_product_g4 * float4(this.scalar)) + (reverse_g4 * float4(geometric_product_g0[0])) + float4(((float3(this.e1234) * geometric_product_g1.xyz) + (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g3.zyz) + (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g3.xxy) + (geometric_product_g2.yzx * this.group1.zxy) + (reverse_g2.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) + (reverse_g2.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3])) + (reverse_g3.xyx * float3(float2(geometric_product_g1[3]), geometric_product_g4[1])) + (reverse_g3.yzz * float3(geometric_product_g4.zx, geometric_product_g1[3])) - (float3(geometric_product_g0[1]) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g2.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g2.zyz) - (geometric_product_g3.yzx * reverse_g4.zxy) - (reverse_g2.zxy * geometric_product_g1.yzx) - (reverse_g3.zxy * geometric_product_g4.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Line other) {
        float2 geometric_product_g0 = float2(0.0, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43));
        float3 geometric_product_g1_xyz = (other.group1.xyx * float3(float2(this.e321), this.e2)) + (other.group1.yzz * float3(this.group1.zx, this.e321)) - (other.group1.zxy * this.group1.yzx);
        float3 geometric_product_g2 = (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) + (other.group1.yzx * this.group2.zxy) - (other.group0.zxy * this.group3.yzx) - (other.group1.zxy * this.group2.yzx);
        float3 geometric_product_g3 = (float3(this.scalar) * other.group1.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx);
        float4 geometric_product_g4 = float4(((other.group0.xyx * float3(float2(this.e321), this.e2)) + (other.group0.yzz * float3(this.group1.zx, this.e321)) + (other.group1.xyx * float3(float2(this.e4), this.e431)) + (other.group1.yzz * float3(this.group4.zx, this.e4)) - (other.group0.zxy * this.group1.yzx) - (other.group1.zxy * this.group4.yzx)), 0.0);
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 reverse_g3 = this.group3.xyz * float3(-1.0);
        float4 reverse_g4 = this.group4 * float4(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3((geometric_product_g4[2] * this.e2) - (geometric_product_g4[1] * this.e3), (geometric_product_g4[1] * reverse_g4[3]) - (geometric_product_g4[2] * this.e1), (geometric_product_g4[1] * this.e1) + (geometric_product_g4[2] * reverse_g4[3]) - (geometric_product_g4[0] * this.e2)) + (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (reverse_g2 * float3(geometric_product_g0[0])) + (reverse_g3 * float3(geometric_product_g0[1])) + (geometric_product_g1_xyz.yzx * reverse_g4.zxy) + (geometric_product_g2.zxy * reverse_g3.yzx) + (geometric_product_g3.zxy * reverse_g2.yzx) + float3((float2(geometric_product_g4[0]) * float2(reverse_g4[3], this.e3)), 0.0) - (float3(geometric_product_g4[3]) * reverse_g4.xyz) - (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g1_xyz.zyz) - (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g1_xyz.xxy) - (geometric_product_g2.yzx * reverse_g3.zxy) - (geometric_product_g3.yzx * reverse_g2.zxy), 
            /* e23, e31, e12 */
            (geometric_product_g3 * float3(this.scalar)) + (reverse_g3 * float3(geometric_product_g0[0])) + (geometric_product_g1_xyz.yzx * this.group1.zxy) + (geometric_product_g3.zxy * reverse_g3.yzx) - (float3(geometric_product_g4[3]) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g1_xyz.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g1_xyz.zyz) - (geometric_product_g3.yzx * reverse_g3.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_product_g4 * float4(this.scalar)) + (reverse_g4 * float4(geometric_product_g0[0])) + float4(((geometric_product_g1_xyz * float3(this.e1234)) + (reverse_g2 * float3(geometric_product_g4[3])) + (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g3.zyz) + (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g3.xxy) + (geometric_product_g1_xyz.zxy * reverse_g2.yzx) + (geometric_product_g2.yzx * this.group1.zxy) + (reverse_g3.yzx * geometric_product_g4.zxy) - (float3(geometric_product_g0[1]) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g2.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g2.zyz) - (geometric_product_g1_xyz.yzx * reverse_g2.zxy) - (geometric_product_g3.yzx * reverse_g4.zxy) - (reverse_g3.zxy * geometric_product_g4.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Motor other) {
        float2 geometric_product_g0 = float2(this.e23 * other.e23 * -1.0, (this.e1234 * other.scalar) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + (float2(this.scalar) * float2(other.scalar, other.e1234)) - (float2(this.e31, this.e41) * other.group1.yx) - (float2(this.e12, this.e42) * other.group1.zy);
        float4 geometric_product_g1 = (other.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + float4(((float3(other.scalar) * this.group1.xyz) + float3(((float2(this.e321) * other.group1.xy) - (other.group1.zx * this.group1.yz)), (other.e23 * this.e2) - (other.e31 * this.e1))), other.e1234 * this.e321);
        float3 geometric_product_g2 = (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xxy * other.group1.wzx) + (this.group2.zyz * other.group1.yww) + (this.group3.xxy * other.group0.wzx) + (this.group3.zyz * other.group0.yww) - (this.group2.yzx * other.group1.zxy) - (this.group3.yzx * other.group0.zxy);
        float3 geometric_product_g3 = (float3(this.scalar) * other.group1.xyz) + (this.group3.xxy * other.group1.wzx) + (this.group3.zyz * other.group1.yww) - (this.group3.yzx * other.group1.zxy);
        float4 geometric_product_g4 = float4((float3((other.e42 * this.e3) + (other.e31 * this.e412) - (other.e43 * this.e2) - (other.e12 * this.e431), (other.e43 * this.e1) + (other.e12 * this.e423) - (other.e41 * this.e3) - (other.e23 * this.e412), (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4) - (other.e42 * this.e1) - (other.e31 * this.e423)) + (float3(other.e1234) * this.group1.xyz) + (float3(other.scalar) * this.group4.xyz) + float3(((float2(this.e4) * other.group1.xy) + (float2(this.e321) * other.group0.xy)), 0.0)), other.scalar * this.e321);
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 reverse_g3 = this.group3.xyz * float3(-1.0);
        float4 reverse_g4 = this.group4 * float4(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g1[0] * this.e1) - (geometric_product_g3[0] * reverse_g3[0]) - (geometric_product_g3[1] * reverse_g3[1]) - (geometric_product_g4[3] * reverse_g4[3]), (geometric_product_g0[1] * this.scalar) + (geometric_product_g1[2] * reverse_g4[2]) + (geometric_product_g1[3] * reverse_g4[3]) - (geometric_product_g2[1] * reverse_g3[1]) - (geometric_product_g2[2] * reverse_g3[2]) - (geometric_product_g3[0] * reverse_g2[0]) - (geometric_product_g3[1] * reverse_g2[1]) - (geometric_product_g3[2] * reverse_g2[2]) - (geometric_product_g4[0] * this.e1) - (geometric_product_g4[1] * this.e2) - (geometric_product_g4[2] * this.e3) - (geometric_product_g4[3] * this.e4)) + (float2(geometric_product_g0[0]) * this.group0.xy) + (float2(this.e2, reverse_g4[0]) * geometric_product_g1.yx) + (float2(this.e3, reverse_g4[1]) * geometric_product_g1.zy) - (float2(geometric_product_g3[2], geometric_product_g2[0]) * reverse_g3.zx), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1 * float4(this.scalar)) + (float4(geometric_product_g0[0]) * this.group1) + float4(((float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g3.xxy) + (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g3.zyz) + (reverse_g3.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) + (reverse_g3.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3])) - (geometric_product_g3.yzx * this.group1.zxy) - (reverse_g3.zxy * geometric_product_g1.yzx)), (this.e1234 * geometric_product_g4[3]) + (geometric_product_g2[0] * this.e1) + (geometric_product_g2[1] * this.e2) + (geometric_product_g2[2] * this.e3) - (geometric_product_g0[1] * reverse_g4[3]) - (geometric_product_g3[0] * reverse_g4[0]) - (geometric_product_g3[1] * reverse_g4[1]) - (geometric_product_g3[2] * reverse_g4[2]) - (reverse_g2[0] * geometric_product_g1[0]) - (reverse_g2[1] * geometric_product_g1[1]) - (reverse_g2[2] * geometric_product_g1[2]) - (reverse_g3[0] * geometric_product_g4[0]) - (reverse_g3[1] * geometric_product_g4[1]) - (reverse_g3[2] * geometric_product_g4[2])), 
            /* e41, e42, e43 */
            float3((geometric_product_g1[1] * reverse_g4[2]) + (geometric_product_g4[2] * this.e2) - (geometric_product_g1[2] * reverse_g4[1]) - (geometric_product_g4[1] * this.e3), (geometric_product_g1[2] * reverse_g4[0]) + (geometric_product_g4[1] * reverse_g4[3]) - (geometric_product_g1[1] * this.e4) - (geometric_product_g4[2] * this.e1), (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * this.e1) + (geometric_product_g4[2] * reverse_g4[3]) - (geometric_product_g1[1] * reverse_g4[0]) - (geometric_product_g1[2] * this.e4) - (geometric_product_g4[0] * this.e2)) + (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (reverse_g2 * float3(geometric_product_g0[0])) + (reverse_g3 * float3(geometric_product_g0[1])) + (float3(geometric_product_g1[3]) * this.group1.xyz) + (geometric_product_g2.zxy * reverse_g3.yzx) + (geometric_product_g3.zxy * reverse_g2.yzx) + float3(((float2(geometric_product_g4[0]) * float2(reverse_g4[3], this.e3)) - (float2(geometric_product_g1[0]) * float2(this.e4, reverse_g4[2]))), 0.0) - (float3(geometric_product_g4[3]) * reverse_g4.xyz) - (geometric_product_g2.yzx * reverse_g3.zxy) - (geometric_product_g3.yzx * reverse_g2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_product_g1[1] * this.e3) - (geometric_product_g1[2] * this.e2), (geometric_product_g1[2] * this.e1) - (geometric_product_g1[1] * reverse_g4[3]), (geometric_product_g1[0] * this.e2) - (geometric_product_g1[1] * this.e1) - (geometric_product_g1[2] * reverse_g4[3])) + (geometric_product_g3 * float3(this.scalar)) + (reverse_g3 * float3(geometric_product_g0[0])) + (geometric_product_g3.zxy * reverse_g3.yzx) + float3(-(float2(geometric_product_g1[0]) * float2(reverse_g4[3], this.e3)), 0.0) - (float3(geometric_product_g4[3]) * this.group1.xyz) - (geometric_product_g3.yzx * reverse_g3.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_product_g4 * float4(this.scalar)) + (reverse_g4 * float4(geometric_product_g0[0])) + float4(((float3(this.e1234) * geometric_product_g1.xyz) + (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g3.zyz) + (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g3.xxy) + (geometric_product_g2.yzx * this.group1.zxy) + (reverse_g2.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) + (reverse_g2.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3])) + (reverse_g3.xyx * float3(float2(geometric_product_g1[3]), geometric_product_g4[1])) + (reverse_g3.yzz * float3(geometric_product_g4.zx, geometric_product_g1[3])) - (float3(geometric_product_g0[1]) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g2.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g2.zyz) - (geometric_product_g3.yzx * reverse_g4.zxy) - (reverse_g2.zxy * geometric_product_g1.yzx) - (reverse_g3.zxy * geometric_product_g4.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(MultiVector other) {
        float2 geometric_product_g0 = float2((other.e1 * this.e1) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e321 * this.e321), (other.e1234 * this.scalar) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) + (float2(other.scalar) * this.group0.xy) + (float2(other.e2, other.e423) * this.group1.yx) + (float2(other.e3, other.e431) * this.group1.zy) - (float2(other.e12, other.e41) * this.group3.xyz.zx);
        float4 geometric_product_g1 = (float4(other.scalar) * this.group1) + float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e2, other.e321, other.e321) * this.group3.zyz) + (float3(other.e321, other.e3, other.e1) * this.group3.xxy) + (other.group3.xyx * float3(float2(this.e321), this.e2)) + (other.group3.yzz * float3(this.group1.zx, this.e321)) - (other.group3.zxy * this.group1.yzx) - (this.group3.yzx * other.group1.zxy)), (other.e1234 * this.e321) + (this.scalar * other.e4) + (this.e41 * other.e1) + (this.e42 * other.e2) + (this.e43 * other.e3) - (this.e1234 * other.e321) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412));
        float3 geometric_product_g2 = float3((other.e2 * this.e412) + (other.e412 * this.e2) - (other.e3 * this.e431) - (other.e431 * this.e3), (other.e3 * this.e423) + (other.e423 * this.e3) - (other.e1 * this.e412) - (other.e412 * this.e1), (other.e1 * this.e431) + (other.e3 * this.e4) + (other.e431 * this.e1) - (other.e2 * this.e423) - (other.e423 * this.e2) - (other.e412 * this.e321)) + (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (other.group2.yzx * this.group3.zxy) + (other.group3.yzx * this.group2.zxy) + float3(((float2(this.e4) * other.group1.xy) - (float2(this.e321) * other.group4.xy)), 0.0) - (float3(other.e4) * this.group1.xyz) - (other.group2.zxy * this.group3.yzx) - (other.group3.zxy * this.group2.yzx);
        float3 geometric_product_g3 = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2) - (other.e3 * this.e321)) + (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group3.yzx * this.group3.zxy) + float3(-(float2(this.e321) * other.group1.xy), 0.0) - (float3(other.e321) * this.group1.xyz) - (other.group3.zxy * this.group3.yzx);
        float4 geometric_product_g4 = (float4(other.scalar) * this.group4) + float4(((float3(other.e1234) * this.group1.xyz) + (float3(this.scalar) * other.group4.xyz) + (float3(other.e4, other.e412, other.e423) * this.group3.xxy) + (float3(other.e431, other.e4, other.e4) * this.group3.zyz) + (other.group2.xyx * float3(float2(this.e321), this.e2)) + (other.group2.yzz * float3(this.group1.zx, this.e321)) + (other.group3.xyx * float3(float2(this.e4), this.e431)) + (other.group3.yzz * float3(this.group4.zx, this.e4)) + (this.group2.yzx * other.group1.zxy) - (float3(this.e1234) * other.group1.xyz) - (float3(other.e2, other.e321, other.e321) * this.group2.zyz) - (float3(other.e321, other.e3, other.e1) * this.group2.xxy) - (other.group2.zxy * this.group1.yzx) - (other.group3.zxy * this.group4.yzx) - (this.group3.yzx * other.group4.zxy)), this.scalar * other.e321);
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 reverse_g3 = this.group3.xyz * float3(-1.0);
        float4 reverse_g4 = this.group4 * float4(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g1[0] * this.e1) - (geometric_product_g3[0] * reverse_g3[0]) - (geometric_product_g3[1] * reverse_g3[1]) - (geometric_product_g4[3] * reverse_g4[3]), (geometric_product_g0[1] * this.scalar) + (geometric_product_g1[2] * reverse_g4[2]) + (geometric_product_g1[3] * reverse_g4[3]) - (geometric_product_g2[1] * reverse_g3[1]) - (geometric_product_g2[2] * reverse_g3[2]) - (geometric_product_g3[0] * reverse_g2[0]) - (geometric_product_g3[1] * reverse_g2[1]) - (geometric_product_g3[2] * reverse_g2[2]) - (geometric_product_g4[0] * this.e1) - (geometric_product_g4[1] * this.e2) - (geometric_product_g4[2] * this.e3) - (geometric_product_g4[3] * this.e4)) + (float2(geometric_product_g0[0]) * this.group0.xy) + (float2(this.e2, reverse_g4[0]) * geometric_product_g1.yx) + (float2(this.e3, reverse_g4[1]) * geometric_product_g1.zy) - (float2(geometric_product_g3[2], geometric_product_g2[0]) * reverse_g3.zx), 
            /* e1, e2, e3, e4 */
            (geometric_product_g1 * float4(this.scalar)) + (float4(geometric_product_g0[0]) * this.group1) + float4(((float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g3.xxy) + (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g3.zyz) + (reverse_g3.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) + (reverse_g3.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3])) - (geometric_product_g3.yzx * this.group1.zxy) - (reverse_g3.zxy * geometric_product_g1.yzx)), (this.e1234 * geometric_product_g4[3]) + (geometric_product_g2[0] * this.e1) + (geometric_product_g2[1] * this.e2) + (geometric_product_g2[2] * this.e3) - (geometric_product_g0[1] * reverse_g4[3]) - (geometric_product_g3[0] * reverse_g4[0]) - (geometric_product_g3[1] * reverse_g4[1]) - (geometric_product_g3[2] * reverse_g4[2]) - (reverse_g2[0] * geometric_product_g1[0]) - (reverse_g2[1] * geometric_product_g1[1]) - (reverse_g2[2] * geometric_product_g1[2]) - (reverse_g3[0] * geometric_product_g4[0]) - (reverse_g3[1] * geometric_product_g4[1]) - (reverse_g3[2] * geometric_product_g4[2])), 
            /* e41, e42, e43 */
            float3((geometric_product_g1[1] * reverse_g4[2]) + (geometric_product_g4[2] * this.e2) - (geometric_product_g1[2] * reverse_g4[1]) - (geometric_product_g4[1] * this.e3), (geometric_product_g1[2] * reverse_g4[0]) + (geometric_product_g4[1] * reverse_g4[3]) - (geometric_product_g1[1] * this.e4) - (geometric_product_g4[2] * this.e1), (geometric_product_g1[0] * reverse_g4[1]) + (geometric_product_g4[1] * this.e1) + (geometric_product_g4[2] * reverse_g4[3]) - (geometric_product_g1[1] * reverse_g4[0]) - (geometric_product_g1[2] * this.e4) - (geometric_product_g4[0] * this.e2)) + (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (reverse_g2 * float3(geometric_product_g0[0])) + (reverse_g3 * float3(geometric_product_g0[1])) + (float3(geometric_product_g1[3]) * this.group1.xyz) + (geometric_product_g2.zxy * reverse_g3.yzx) + (geometric_product_g3.zxy * reverse_g2.yzx) + float3(((float2(geometric_product_g4[0]) * float2(reverse_g4[3], this.e3)) - (float2(geometric_product_g1[0]) * float2(this.e4, reverse_g4[2]))), 0.0) - (float3(geometric_product_g4[3]) * reverse_g4.xyz) - (geometric_product_g2.yzx * reverse_g3.zxy) - (geometric_product_g3.yzx * reverse_g2.zxy), 
            /* e23, e31, e12 */
            float3((geometric_product_g1[1] * this.e3) - (geometric_product_g1[2] * this.e2), (geometric_product_g1[2] * this.e1) - (geometric_product_g1[1] * reverse_g4[3]), (geometric_product_g1[0] * this.e2) - (geometric_product_g1[1] * this.e1) - (geometric_product_g1[2] * reverse_g4[3])) + (geometric_product_g3 * float3(this.scalar)) + (reverse_g3 * float3(geometric_product_g0[0])) + (geometric_product_g3.zxy * reverse_g3.yzx) + float3(-(float2(geometric_product_g1[0]) * float2(reverse_g4[3], this.e3)), 0.0) - (float3(geometric_product_g4[3]) * this.group1.xyz) - (geometric_product_g3.yzx * reverse_g3.zxy), 
            /* e423, e431, e412, e321 */
            (geometric_product_g4 * float4(this.scalar)) + (reverse_g4 * float4(geometric_product_g0[0])) + float4(((float3(this.e1234) * geometric_product_g1.xyz) + (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g3.zyz) + (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g3.xxy) + (geometric_product_g2.yzx * this.group1.zxy) + (reverse_g2.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) + (reverse_g2.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3])) + (reverse_g3.xyx * float3(float2(geometric_product_g1[3]), geometric_product_g4[1])) + (reverse_g3.yzz * float3(geometric_product_g4.zx, geometric_product_g1[3])) - (float3(geometric_product_g0[1]) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g2.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g2.zyz) - (geometric_product_g3.yzx * reverse_g4.zxy) - (reverse_g2.zxy * geometric_product_g1.yzx) - (reverse_g3.zxy * geometric_product_g4.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Origin other) {
        float geometric_product_g0_y = this.e321 * other.e4 * -1.0;
        float geometric_product_g1_w = this.scalar * other.e4;
        float3 geometric_product_g2 = float3(other.e4 * -1.0) * this.group1.xyz;
        float3 geometric_product_g4_xyz = float3(other.e4) * this.group3.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (geometric_product_g0_y * this.scalar) + (geometric_product_g2[0] * this.e23) + (geometric_product_g2[1] * this.e31) + (geometric_product_g2[2] * this.e12) - (geometric_product_g1_w * this.e321) - (geometric_product_g4_xyz[0] * this.e1) - (geometric_product_g4_xyz[1] * this.e2) - (geometric_product_g4_xyz[2] * this.e3)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (geometric_product_g0_y * this.e321) + (geometric_product_g1_w * this.scalar) + (geometric_product_g2[0] * this.e1) + (geometric_product_g2[1] * this.e2) + (geometric_product_g2[2] * this.e3) + (geometric_product_g4_xyz[0] * this.e23) + (geometric_product_g4_xyz[1] * this.e31) + (geometric_product_g4_xyz[2] * this.e12)), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Plane other) {
        float geometric_product_g0_x = this.e321 * other.e321 * -1.0;
        float geometric_product_g0_y = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321);
        float3 geometric_product_g1_xyz = float3(other.e321) * this.group3.xyz;
        float geometric_product_g1_w = -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float3 geometric_product_g2 = float3((this.e2 * other.e412) - (this.e3 * other.e431), (this.e3 * other.e423) - (this.e1 * other.e412), (this.e1 * other.e431) - (this.e2 * other.e423)) + (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 geometric_product_g3 = float3(other.e321 * -1.0) * this.group1.xyz;
        float3 geometric_product_g4_xyz = (float3(this.scalar) * other.group0.xyz) + (this.group3.zxy * other.group0.yzx) - (float3(other.e321) * this.group2.xyz) - (this.group3.yzx * other.group0.zxy);
        float geometric_product_g4_w = this.scalar * other.e321;
        float3 reverse_g2 = this.group2.xyz * float3(-1.0);
        float3 reverse_g3 = this.group3.xyz * float3(-1.0);
        float4 reverse_g4 = this.group4 * float4(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            (geometric_product_g2 * float3(this.scalar)) + (geometric_product_g3 * float3(this.e1234)) + (reverse_g2 * float3(geometric_product_g0_x)) + (reverse_g3 * float3(geometric_product_g0_y)) + (float3(geometric_product_g1_w) * this.group1.xyz) + (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g4_xyz.xxy) + (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g4_xyz.zyz) + (geometric_product_g1_xyz.yzx * reverse_g4.zxy) + (geometric_product_g2.zxy * reverse_g3.yzx) + (geometric_product_g3.zxy * reverse_g2.yzx) - (float3(geometric_product_g4_w) * reverse_g4.xyz) - (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g1_xyz.zyz) - (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g1_xyz.xxy) - (geometric_product_g2.yzx * reverse_g3.zxy) - (geometric_product_g3.yzx * reverse_g2.zxy) - (geometric_product_g4_xyz.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (geometric_product_g3 * float3(this.scalar)) + (reverse_g3 * float3(geometric_product_g0_x)) + (geometric_product_g1_xyz.yzx * this.group1.zxy) + (geometric_product_g3.zxy * reverse_g3.yzx) - (float3(geometric_product_g4_w) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g1_xyz.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g1_xyz.zyz) - (geometric_product_g3.yzx * reverse_g3.zxy), 
            /* e423, e431, e412, e321 */
            (reverse_g4 * float4(geometric_product_g0_x)) + float4(((geometric_product_g1_xyz * float3(this.e1234)) + (geometric_product_g4_xyz * float3(this.scalar)) + (reverse_g2 * float3(geometric_product_g4_w)) + (reverse_g3 * float3(geometric_product_g1_w)) + (float3(reverse_g4[1], this.e4, this.e4) * geometric_product_g3.zyz) + (float3(this.e4, reverse_g4[2], reverse_g4[0]) * geometric_product_g3.xxy) + (geometric_product_g1_xyz.zxy * reverse_g2.yzx) + (geometric_product_g2.yzx * this.group1.zxy) + (geometric_product_g4_xyz.zxy * reverse_g3.yzx) - (float3(geometric_product_g0_y) * this.group1.xyz) - (float3(reverse_g4[3], this.e3, this.e1) * geometric_product_g2.xxy) - (float3(this.e2, reverse_g4[3], reverse_g4[3]) * geometric_product_g2.zyz) - (geometric_product_g1_xyz.yzx * reverse_g2.zxy) - (geometric_product_g3.yzx * reverse_g4.zxy) - (geometric_product_g4_xyz.yzx * reverse_g3.zxy)), geometric_product_g4_w * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Point other) {
        float geometric_product_g0_x = (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3);
        float geometric_product_g0_y = -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 geometric_product_g1 = (float4(this.scalar) * other.group0) + (other.group0.yzxx * float4(this.group3.zxy, this.e41)) + float4(-(this.group3.yzx * other.group0.zxy), 0.0);
        float3 geometric_product_g2 = float3((this.e412 * other.e2) - (this.e431 * other.e3), (this.e423 * other.e3) - (this.e412 * other.e1), (this.e431 * other.e1) - (this.e423 * other.e2)) + (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 geometric_product_g3 = float3((this.e2 * other.e3) - (this.e3 * other.e2), (this.e3 * other.e1) - (this.e1 * other.e3), (this.e1 * other.e2) - (this.e2 * other.e1)) - (float3(this.e321) * other.group0.xyz);
        float3 geometric_product_g4_xyz = (float3(other.e4) * this.group3.xyz) + (this.group2.yzx * other.group0.zxy) - (float3(this.e1234) * other.group0.xyz) - (this.group2.zxy * other.group0.yzx);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g3[0] * this.e23) + (geometric_product_g1[0] * this.e1) + (geometric_product_g1[1] * this.e2) + (geometric_product_g1[2] * this.e3), (geometric_product_g0_y * this.scalar) + (geometric_product_g2[2] * this.e12) + (geometric_product_g3[0] * this.e41) + (geometric_product_g3[1] * this.e42) + (geometric_product_g3[2] * this.e43) - (geometric_product_g4_xyz[0] * this.e1) - (geometric_product_g4_xyz[1] * this.e2) - (geometric_product_g4_xyz[2] * this.e3) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412) - (geometric_product_g1[3] * this.e321)) + (float2(geometric_product_g0_x) * this.group0.xy) + (float2(geometric_product_g3[1], geometric_product_g2[0]) * this.group3.xyz.yx) + (float2(geometric_product_g3[2], geometric_product_g2[1]) * this.group3.xyz.zy), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g0_x) * this.group1) + float4(((float3(this.scalar) * geometric_product_g1.xyz) + (geometric_product_g3.zxy * this.group1.yzx) + (this.group3.zxy * geometric_product_g1.yzx) - (geometric_product_g3.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g3.yzz * float3(this.group1.zx, this.e321)) - (this.group3.yzx * geometric_product_g1.zxy)), (geometric_product_g0_y * this.e321) + (this.scalar * geometric_product_g1[3]) + (geometric_product_g2[0] * this.e1) + (this.e42 * geometric_product_g1[1]) + (this.e43 * geometric_product_g1[2])), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Scalar other) {
        float2 geometric_product_g0 = float2(other.scalar) * this.group0.xy;
        float4 geometric_product_g1 = float4(other.scalar) * this.group1;
        float3 geometric_product_g2 = float3(other.scalar) * this.group2.xyz;
        float3 geometric_product_g3 = float3(other.scalar) * this.group3.xyz;
        float4 geometric_product_g4 = float4(other.scalar) * this.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g3[0] * this.e23) + (geometric_product_g1[0] * this.e1) + (geometric_product_g1[1] * this.e2) + (geometric_product_g1[2] * this.e3) + (geometric_product_g4[3] * this.e321), (geometric_product_g0[1] * this.scalar) + (geometric_product_g2[2] * this.e12) + (geometric_product_g3[0] * this.e41) + (geometric_product_g3[1] * this.e42) + (geometric_product_g3[2] * this.e43) - (geometric_product_g1[0] * this.e423) - (geometric_product_g1[1] * this.e431) - (geometric_product_g1[2] * this.e412) - (geometric_product_g1[3] * this.e321) - (geometric_product_g4[0] * this.e1) - (geometric_product_g4[1] * this.e2) - (geometric_product_g4[2] * this.e3) - (geometric_product_g4[3] * this.e4)) + (float2(geometric_product_g0[0]) * this.group0.xy) + (float2(geometric_product_g3[1], geometric_product_g2[0]) * this.group3.xyz.yx) + (float2(geometric_product_g3[2], geometric_product_g2[1]) * this.group3.xyz.zy), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g0[0]) * this.group1) + float4(((float3(this.scalar) * geometric_product_g1.xyz) + (geometric_product_g3.zxy * this.group1.yzx) + (this.group3.zxy * geometric_product_g1.yzx) - (geometric_product_g3.xyx * float3(float2(this.e321), this.e2)) - (geometric_product_g3.yzz * float3(this.group1.zx, this.e321)) - (this.group3.xyx * float3(float2(geometric_product_g4[3]), geometric_product_g1[1])) - (this.group3.yzz * float3(geometric_product_g1.zx, geometric_product_g4[3]))), (geometric_product_g0[1] * this.e321) + (this.scalar * geometric_product_g1[3]) + (this.e1234 * geometric_product_g4[3])), 
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
public SandwichInfixPartial<Plane> operator /(Plane lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public AntiScalar sandwich(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.e321 * this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public DualNum sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e321 * this.e321) * other.group0.xy * float2(1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Flector sandwich(Flector other) {
        float4 geometric_product_g0 = float4(0.0, 0.0, (other.e1 * this.e431) - (other.e2 * this.e423), 0.0) + float4(((float3(other.e321) * this.group0.xyz) + float3(((other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz)), 0.0) - (float3(this.e321) * other.group1.xyz)), 0.0);
        float4 geometric_product_g1 = float4(this.e321 * -1.0) * float4(other.group0.xyz, other.e321);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((geometric_product_g1.xyz * float4(this.e321).xyz * float3(-1.0)), (geometric_product_g0[3] * this.e321) + (geometric_product_g1[0] * this.e423) + (geometric_product_g1[1] * this.e431) + (geometric_product_g1[2] * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3((geometric_product_g1[1] * this.e412) - (geometric_product_g1[2] * this.e431), (geometric_product_g1[2] * this.e423) - (geometric_product_g1[0] * this.e412), (geometric_product_g1[0] * this.e431) - (geometric_product_g1[1] * this.e423)) + (float3(this.e321) * geometric_product_g0.xyz) - (float3(geometric_product_g1[3]) * this.group0.xyz)), geometric_product_g1[3] * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Plane sandwich(Horizon other) {
        float geometric_product_g1_w = this.e321 * other.e321 * -1.0;
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((float3(this.e321 * other.e321) * this.group0.xyz) - (float3(geometric_product_g1_w) * this.group0.xyz)), geometric_product_g1_w * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Line sandwich(Line other) {
        float3 geometric_product_g0_xyz = float3(this.e321) * other.group1.xyz;
        return Line(
            /* e41, e42, e43 */
            (float3(this.e321) * float3(other.e12 * this.e431, other.e23 * this.e412, other.e31 * this.e423)) + (geometric_product_g0_xyz.zxy * this.group0.yzx) - (float3(this.e321) * float3(other.e31 * this.e412, other.e12 * this.e423, other.e23 * this.e431)) - (float3(this.e321 * this.e321) * other.group0.xyz) - (geometric_product_g0_xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            geometric_product_g0_xyz * float3(this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Motor sandwich(Motor other) {
        float3 geometric_product_g0_xyz = float3(this.e321) * other.group1.xyz;
        float geometric_product_g1_w = other.scalar * this.e321;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(geometric_product_g1_w) * this.group0.xyz) + (float3(this.e321) * float3(other.e12 * this.e431, other.e23 * this.e412, other.e31 * this.e423)) + (geometric_product_g0_xyz.zxy * this.group0.yzx) - (float3(this.e321) * float3(other.e31 * this.e412, other.e12 * this.e423, other.e23 * this.e431)) - (float3(this.e321 * this.e321) * other.group0.xyz) - (float3(other.scalar * this.e321) * this.group0.xyz) - (geometric_product_g0_xyz.yzx * this.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(geometric_product_g0_xyz, geometric_product_g1_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public MultiVector sandwich(MultiVector other) {
        float geometric_product_g0_x = other.e321 * this.e321 * -1.0;
        float3 geometric_product_g1_xyz = float3(this.e321) * other.group3.xyz;
        float3 geometric_product_g3 = float3(this.e321 * -1.0) * other.group1.xyz;
        float geometric_product_g4_w = other.scalar * this.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_product_g4_w * this.e321, (other.e23 * this.e423 * this.e321) + (other.e31 * this.e431 * this.e321) + (other.e12 * this.e412 * this.e321) - (geometric_product_g1_xyz[0] * this.e423) - (geometric_product_g1_xyz[1] * this.e431) - (geometric_product_g1_xyz[2] * this.e412) - (this.e321 * this.e321 * other.e1234)), 
            /* e1, e2, e3, e4 */
            float4((geometric_product_g3 * float4(this.e321).xyz * float3(-1.0)), (geometric_product_g3[0] * this.e423) + (geometric_product_g3[1] * this.e431) + (geometric_product_g3[2] * this.e412) - (this.e321 * this.e321 * other.e4) - (other.e1 * this.e423 * this.e321) - (other.e2 * this.e431 * this.e321) - (other.e3 * this.e412 * this.e321)), 
            /* e41, e42, e43 */
            (float3(geometric_product_g4_w) * this.group0.xyz) + (float3(this.e321) * float3(other.e12 * this.e431, other.e23 * this.e412, other.e31 * this.e423)) + (geometric_product_g1_xyz.zxy * this.group0.yzx) - (float3(this.e321) * float3(other.e31 * this.e412, other.e12 * this.e423, other.e23 * this.e431)) - (float3(this.e321 * this.e321) * other.group2.xyz) - (float3(other.scalar * this.e321) * this.group0.xyz) - (geometric_product_g1_xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            geometric_product_g1_xyz * float3(this.e321), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.e321) * float3(other.e2 * this.e412, other.e3 * this.e423, other.e1 * this.e431)) + (float3(other.e321 * this.e321) * this.group0.xyz) + (geometric_product_g3.yzx * this.group0.zxy) - (float3(geometric_product_g0_x) * this.group0.xyz) - (float3(this.e321) * float3(other.e3 * this.e431, other.e1 * this.e412, other.e2 * this.e423)) - (float3(this.e321 * this.e321) * other.group4.xyz) - (geometric_product_g3.zxy * this.group0.yzx)), geometric_product_g0_x * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Origin sandwich(Origin other) {
        return Origin(
            /* e4 */
            this.e321 * this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Plane sandwich(Plane other) {
        float geometric_product_g1_w = other.e321 * this.e321 * -1.0;
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((float3(other.e321 * this.e321) * this.group0.xyz) - (float3(geometric_product_g1_w) * this.group0.xyz) - (float3(this.e321 * this.e321) * other.group0.xyz)), geometric_product_g1_w * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Point sandwich(Point other) {
        float3 geometric_product_g1_xyz = float3(this.e321 * -1.0) * other.group0.xyz;
        return Point(
            /* e1, e2, e3, e4 */
            float4((geometric_product_g1_xyz * float4(this.e321).xyz * float3(-1.0)), (geometric_product_g1_xyz[0] * this.e423) + (geometric_product_g1_xyz[1] * this.e431) + (geometric_product_g1_xyz[2] * this.e412) - (this.e321 * this.e321 * other.e4))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(SandwichInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Scalar sandwich(Scalar other) {
        return Scalar(
            /* scalar */
            this.e321 * this.e321 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Point> operator /(Point lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public AntiScalar sandwich(AntiScalar other) {
        float3 geometric_product_g0_xyz = float3(other.e1234) * this.group0.xyz;
        return AntiScalar(
            /* e1234 */
            -(geometric_product_g0_xyz[0] * this.e1) - (geometric_product_g0_xyz[1] * this.e2) - (geometric_product_g0_xyz[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public DualNum sandwich(DualNum other) {
        float4 geometric_product_g0 = float4(other.scalar) * this.group0;
        float4 geometric_product_g1 = float4(1.0, 1.0, other.e1234, 0.0) * float4((this.group0.xyz * float3(float2(other.e1234), 1.0)), 0.0);
        return DualNum(
            /* scalar, e1234 */
            float2((geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3), -(geometric_product_g1[0] * this.e1) - (geometric_product_g1[1] * this.e2) - (geometric_product_g1[2] * this.e3) - (geometric_product_g1[3] * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Flector sandwich(Flector other) {
        float4 geometric_product_g0 = float4(0.0, 0.0, other.e423 * this.e2 * -1.0, 0.0) + (other.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4((float3(-(other.group1.yz * this.group0.zx), 0.0) - (float3(other.e4) * this.group0.xyz)), 0.0);
        float3 geometric_product_g1_xyz = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2)) - (float3(other.e321) * this.group0.xyz);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1 * this.e1 * other.e1, other.e1 * this.e1 * this.e2, other.e1 * this.e1 * this.e3, (geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2)) + float4(((geometric_product_g1_xyz.zxy * this.group0.yzx) - (geometric_product_g1_xyz.yzx * this.group0.zxy)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0, 0.0, (geometric_product_g0[0] * this.e2) - (geometric_product_g0[1] * this.e1), 0.0) + float4(((geometric_product_g1_xyz * float3(this.e4)) + float3(((geometric_product_g0.yz * this.group0.zx) - (geometric_product_g0.zx * this.group0.yz)), 0.0) - (float3(geometric_product_g0[3]) * this.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Plane sandwich(Horizon other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -2.0) * float4(this.e4).xyz * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Line sandwich(Line other) {
        float3 geometric_product_g0_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float3 geometric_product_g1_xyz = (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        return Line(
            /* e41, e42, e43 */
            (geometric_product_g1_xyz.zxy * this.group0.yzx) - (geometric_product_g0_xyz * float3(this.e4)) - (geometric_product_g1_xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (geometric_product_g0_xyz.yzx * this.group0.zxy) - (geometric_product_g0_xyz.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Motor sandwich(Motor other) {
        float3 geometric_product_g0_xyz = float3((other.e31 * this.e3) - (other.e12 * this.e2), (other.e12 * this.e1) - (other.e23 * this.e3), (other.e23 * this.e2) - (other.e31 * this.e1)) + (float3(other.scalar) * this.group0.xyz);
        float4 geometric_product_g1 = float4(0.0, 0.0, (other.e41 * this.e2) - (other.e42 * this.e1), 0.0) + float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e4) * other.group1.xyz) + float3(((other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz)), 0.0)), 0.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0, 0.0, (geometric_product_g1[1] * this.e1) - (geometric_product_g1[0] * this.e2), 0.0) + float4(((float3(other.scalar * this.e4) * this.group0.xyz) + float3(((geometric_product_g1.zx * this.group0.yz) - (geometric_product_g1.yz * this.group0.zx)), 0.0) - (geometric_product_g0_xyz * float3(this.e4))), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((geometric_product_g0_xyz.yzx * this.group0.zxy) - (float3(geometric_product_g1[3]) * this.group0.xyz) - (geometric_product_g0_xyz.zxy * this.group0.yzx)), geometric_product_g0_xyz[0] * this.e1)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public MultiVector sandwich(MultiVector other) {
        float2 geometric_product_g0 = float2(0.0, other.e321 * this.e4) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412));
        float3 geometric_product_g1_xyz = (float3(other.scalar) * this.group0.xyz) + (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx);
        float3 geometric_product_g2 = float3((other.e412 * this.e2) - (other.e431 * this.e3), (other.e423 * this.e3) - (other.e412 * this.e1), (other.e431 * this.e1) - (other.e423 * this.e2)) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 geometric_product_g3 = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2)) - (float3(other.e321) * this.group0.xyz);
        float3 geometric_product_g4_xyz = (float3(other.e1234) * this.group0.xyz) + (float3(this.e4) * other.group3.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx);
        return MultiVector(
            /* scalar, e1234 */
            float2((geometric_product_g1_xyz[0] * this.e1) + (geometric_product_g1_xyz[1] * this.e2) + (geometric_product_g1_xyz[2] * this.e3), -(geometric_product_g4_xyz[0] * this.e1) - (geometric_product_g4_xyz[1] * this.e2) - (geometric_product_g4_xyz[2] * this.e3)), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product_g0[0]) * this.group0) + (this.group0.yzxx * float4(geometric_product_g3.zxy, geometric_product_g2[0])) + float4(-(geometric_product_g3.yzx * this.group0.zxy), 0.0), 
            /* e41, e42, e43 */
            (float3(other.scalar * this.e4) * this.group0.xyz) + (geometric_product_g4_xyz.zxy * this.group0.yzx) - (geometric_product_g1_xyz * float3(this.e4)) - (geometric_product_g4_xyz.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (geometric_product_g1_xyz.yzx * this.group0.zxy) - (geometric_product_g1_xyz.zxy * this.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4(((geometric_product_g3 * float3(this.e4)) + (geometric_product_g2.yzx * this.group0.zxy) - (float3(geometric_product_g0[1]) * this.group0.xyz) - (geometric_product_g2.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Origin sandwich(Origin other) {
        float3 geometric_product_g0 = float3(other.e4 * -1.0) * this.group0.xyz;
        return Origin(
            /* e4 */
            (geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Plane sandwich(Plane other) {
        float2 geometric_product_g0_xy = (other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx);
        float geometric_product_g0_z = (other.e431 * this.e1) - (other.e423 * this.e2);
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3((this.e1 * this.e1 * other.e423 * -1.0), (other.e423 * this.e1 * this.e2 * -1.0), ((geometric_product_g0_xy[0] * this.e2) - (geometric_product_g0_xy[1] * this.e1) - (other.e423 * this.e1 * this.e3))) + float3(((float2(geometric_product_g0_xy[1], geometric_product_g0_z) * this.group0.zx) - (float2(geometric_product_g0_z, geometric_product_g0_xy[0]) * this.group0.yz)), 0.0) - (float3(other.e321 * this.e4) * this.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Point sandwich(Point other) {
        float3 geometric_product_g0_xyz = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz);
        float2 geometric_product_g1_xy = (other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx);
        float geometric_product_g1_z = (other.e2 * this.e1) - (other.e1 * this.e2);
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1 * this.e1 * other.e1, other.e1 * this.e1 * this.e2, (other.e1 * this.e1 * this.e3) - (geometric_product_g1_xy[0] * this.e2), geometric_product_g0_xyz[1] * this.e2) + (float4(geometric_product_g1_z, geometric_product_g1_xy[0], geometric_product_g1_xy[1], geometric_product_g0_xyz[0]) * this.group0.yzxx) + float4(-(float2(geometric_product_g1_xy[1], geometric_product_g1_z) * this.group0.zx), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(SandwichInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Scalar sandwich(Scalar other) {
        float4 geometric_product_g0 = float4(other.scalar) * this.group0;
        return Scalar(
            /* scalar */
            (geometric_product_g0[0] * this.e1) + (geometric_product_g0[1] * this.e2) + (geometric_product_g0[2] * this.e3)
        );
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Scalar> operator /(Scalar lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public AntiScalar sandwich(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public DualNum sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Flector sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar * this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar * this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(SandwichInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Horizon sandwich(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * this.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Line sandwich(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.scalar * this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar * this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Motor sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.scalar * this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar * this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar * this.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar * this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar * this.scalar) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar * this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.scalar * this.scalar) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Scalar> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Origin sandwich(Origin other) {
        return Origin(
            /* e4 */
            other.e4 * this.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Plane sandwich(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar * this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(SandwichInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Point sandwich(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar * this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(SandwichInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Scalar {
    public Scalar sandwich(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar * this.scalar
        );
    }
}
