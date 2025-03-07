implementing rga3d;
using traits;
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
            other.e1234 * this.e1234
        ).geometric_anti_product(this.anti_reverse());
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
            float2(this.e1234) * other.group0.xy
        ).geometric_anti_product(this.anti_reverse());
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
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        ).geometric_anti_product(this.anti_reverse());
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
            this.e1234 * other.e321
        ).geometric_anti_product(this.anti_reverse());
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
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        ).geometric_anti_product(this.anti_reverse());
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
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        ).geometric_anti_product(this.anti_reverse());
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
            float2(this.e1234) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        ).geometric_anti_product(this.anti_reverse());
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
            this.e1234 * other.e4
        ).geometric_anti_product(this.anti_reverse());
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
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
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
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
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
            this.e1234 * other.scalar
        ).geometric_anti_product(this.anti_reverse());
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
        return DualNum(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiSandwichInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public DualNum anti_sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Flector anti_sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group0.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0)
        ).geometric_anti_product(this.anti_reverse());
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
            this.e1234 * other.e321
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Line anti_sandwich(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Motor anti_sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group4.xyz) + (float3(this.e1234) * other.group1.xyz)), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group2.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.group0.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group4.xyz, 1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Flector anti_sandwich(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Flector anti_sandwich(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * float3(this.group0.xx, this.scalar)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension DualNum {
    public Flector anti_sandwich(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        ).geometric_anti_product(this.anti_reverse());
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
            this.e1234 * other.scalar
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Flector> operator /(Flector lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Motor anti_sandwich(AntiScalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Motor anti_sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(other.group0.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group1.xyz, 1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Flector anti_sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(other.e423 * this.e4) - (other.e412 * this.e431), -(other.e423 * this.e412) - (other.e431 * this.e4), -(other.e431 * this.e423) - (other.e412 * this.e4), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group1.zxyx) - (float4(other.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, this.e1) * float4(other.group0.xxy, other.e423)) + (float4(this.e431, this.e4, this.e4, this.e2) * float4(other.group0.zyz, other.e431)) + (other.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, this.e321) * float4(other.group1.zyz, other.e4)) - (float4(this.e321, this.e3, this.e1, this.e412) * float4(other.group1.xxy, other.e3)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Flector anti_sandwich(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Motor anti_sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - float4((other.group0.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group0.yzx * this.group1.zxy), this.e1 * other.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Motor anti_sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * other.group0.wwyw) - (this.group1.xyxz * float4(other.group1.wwy, other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((other.group1.xyz * this.group0.www), this.e431 * other.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((other.group0.xyz * this.group0.www), this.e423 * other.e23) - (other.group0.yzxx * float4(this.group1.zxy, this.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e4 * other.e31), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e4 * other.e12), 0.0) + (float4(other.e1234) * this.group0) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xxy, other.e43)) - (this.group1.xyzx * float4(other.group0.xx, other.scalar, other.e41)) - float4((other.group2.yzx * this.group0.zxy), this.e431 * other.e42), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (float3(this.e4) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group1.xyx) - (float3(other.e412, other.e423, other.e4) * this.group1.yzz), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e3, other.e1, other.e321) * this.group1.yzz) + (float3(other.e321, other.e321, other.e2) * this.group1.xyx) + (this.group0.zxy * other.group4.yzx) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group0.xyx) - (float3(other.e412, other.e423, other.e4) * this.group0.yzz) - (this.group1.zxy * other.group1.yzx), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group1) + (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group2.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group2.zyz, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)) - float4((other.group2.yzx * this.group1.zxy), this.e4 * other.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Flector anti_sandwich(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Flector anti_sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * other.e423) - (this.e431 * other.e412), -(this.e4 * other.e431) - (this.e412 * other.e423), -(this.e4 * other.e412) - (this.e423 * other.e431), (this.e431 * other.e431) + (this.e412 * other.e412)) + (this.group1.zxyx * other.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * other.e412) - (this.e321 * other.e423), -(this.e3 * other.e423) - (this.e321 * other.e431), -(this.e1 * other.e431) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.zxyx * other.group0.yzxx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Flector anti_sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group1.xyz, this.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * other.e1) + (this.e431 * other.e3), (this.e4 * other.e2) + (this.e412 * other.e1), (this.e4 * other.e3) + (this.e423 * other.e2), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.zxyy * other.group0.yzxy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Flector {
    public Motor anti_sandwich(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Horizon> operator /(Horizon lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Horizon {
    public Flector anti_sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Horizon {
    public Scalar anti_sandwich(Line other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Horizon {
    public Motor anti_sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Horizon {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiSandwichInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Horizon {
    public Point anti_sandwich(Plane other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Line> operator /(Line lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Motor anti_sandwich(AntiScalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Motor anti_sandwich(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Flector anti_sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e41) + (other.e412 * this.e42), (other.e4 * this.e42) + (other.e423 * this.e43), (other.e4 * this.e43) + (other.e431 * this.e41), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group0.zxy * other.group1.yzx), other.e1 * this.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Flector anti_sandwich(Horizon other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Motor anti_sandwich(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e43 * this.e42, other.e41 * this.e43, other.e42 * this.e41, -(other.e42 * this.e42) - (other.e43 * this.e43)) - float4((other.group0.yzx * this.group0.zxy), other.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((other.e43 * this.e31) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e23 * this.e43), (other.e42 * this.e23) + (other.e31 * this.e41), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e23) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Motor anti_sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.e1234) + (this.e42 * other.e43), (this.e42 * other.e1234) + (this.e43 * other.e41), (this.e41 * other.e42) + (this.e43 * other.e1234), -(this.e42 * other.e42) - (this.e43 * other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * other.scalar) + (this.e42 * other.e12) + (this.e23 * other.e1234) + (this.e31 * other.e43), (this.e42 * other.scalar) + (this.e43 * other.e23) + (this.e31 * other.e1234) + (this.e12 * other.e41), (this.e41 * other.e31) + (this.e43 * other.scalar) + (this.e23 * other.e42) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.yzxx * float4(this.group0.zxy, this.e41)) - float4((this.group1.zxy * other.group0.yzx), this.e42 * other.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e42 * other.e3) + (this.e23 * other.e4) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e43 * other.e1) + (this.e31 * other.e4) + (this.e12 * other.e423), (this.e41 * other.e2) + (this.e43 * other.e321) + (this.e23 * other.e431) + (this.e12 * other.e4), this.e43 * other.e412 * -1.0) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e423), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group0.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) + (this.group1.yzx * other.group2.zxy) - (this.group0.zxy * other.group3.yzx) - (this.group1.zxy * other.group2.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * other.e4) + (this.e42 * other.e412), (this.e42 * other.e4) + (this.e43 * other.e423), (this.e41 * other.e431) + (this.e43 * other.e4), -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group0.zxy * other.group4.yzx), this.e41 * other.e1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Flector anti_sandwich(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Flector anti_sandwich(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Flector anti_sandwich(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - float4((this.group0.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), -(this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Line {
    public Motor anti_sandwich(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Motor> operator /(Motor lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Motor anti_sandwich(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Motor anti_sandwich(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Flector anti_sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (other.group0.xxyw * this.group0.wzxw) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(other.group0.yzx, other.e423)), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(other.group1.yzx, other.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Flector anti_sandwich(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Motor anti_sandwich(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.e1234) + (other.e43 * this.e42), (other.e41 * this.e43) + (other.e42 * this.e1234), (other.e42 * this.e41) + (other.e43 * this.e1234), -(other.e42 * this.e42) - (other.e43 * this.e43)) - (this.group0.zxyx * float4(other.group0.yzx, other.e41)), 
            /* e23, e31, e12, scalar */
            float4((other.e41 * this.scalar) + (other.e43 * this.e31) + (other.e23 * this.e1234) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e42 * this.scalar) + (other.e23 * this.e43) + (other.e31 * this.e1234), (other.e42 * this.e23) + (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.zxyx * float4(other.group0.yzx, other.e41)) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Motor anti_sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e43 * this.e42) + (other.e1234 * this.e41), (other.e42 * this.e1234) + (other.e1234 * this.e42), (other.e43 * this.e1234) + (other.e1234 * this.e43), -(other.e42 * this.e42) - (other.e43 * this.e43)) + (other.group0.xxyw * this.group0.wzxw) - (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e12 * this.e42) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e23 * this.e43) + (other.e31 * this.e1234) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e31 * this.e41) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xxyw * this.group1.wzxw) + float4((other.group0.zyz * this.group1.yww), other.scalar * this.e1234) - (other.group0.yzxx * this.group1.zxyx) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12), 0.0) + (float2(this.e1234) * other.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * other.e3) + (this.e1234 * other.e1) + (this.e23 * other.e4) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e1234 * other.e2) + (this.e31 * other.e4) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e1234 * other.e3) + (this.e23 * other.e431) + (this.e12 * other.e4) + (this.scalar * other.e412), this.e43 * other.e412 * -1.0) + (this.group0.xyxw * float4(other.group4.ww, other.e2, other.e4)) - (this.group0.zxyx * float4(other.group1.yzx, other.e423)) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group0.xyz) + (other.group2.xxy * this.group0.wzx) + (other.group2.zyz * this.group0.yww) - (other.group2.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xxy * this.group1.wzx) + (other.group2.zyz * this.group1.yww) + (other.group3.xxy * this.group0.wzx) + (other.group3.zyz * this.group0.yww) - (other.group2.yzx * this.group1.zxy) - (other.group3.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.xyxw * float4(other.group1.ww, other.e431, other.e321)) + (float4(other.group4.zx, other.e4, other.e4) * float4(this.group0.yzz, this.scalar)) - (this.group0.zxyx * float4(other.group4.yzx, other.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Flector anti_sandwich(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group0.xyz, this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Flector anti_sandwich(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e42 * other.e321) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e23 * other.e431) + (this.scalar * other.e412), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.yzxw * other.group0.zxyw) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Flector anti_sandwich(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Motor {
    public Motor anti_sandwich(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0
        ).geometric_anti_product(this.anti_reverse());
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
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group1.xyz) - (float3(other.scalar) * this.group4.xyz)), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.group0.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group4.xyz, 1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group0.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (float4(this.group0.xx, this.scalar, this.e1234) * float4(other.group1.xyz, other.e4)) - (other.group1.yzxy * float4(this.group3.zxy, this.e42)) - float4((this.group2.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group1.xxy) - (float3(this.e431, this.e4, this.e4) * other.group1.zyz), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * other.group0.xxy) + (float3(this.e431, this.e4, this.e4) * other.group0.zyz) + (other.group1.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group1.zyz) - (float3(this.e321, this.e3, this.e1) * other.group1.xxy) - (other.group0.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e423 * this.e43, other.e4 * this.e43, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(this.group0.yy, this.e1234, this.scalar) * float4(other.group1.xyz, other.e4)) + (float4(other.group0.ww, other.e431, other.e321) * float4(this.group2.xyx, this.e1234)) - float4((this.group2.zxy * other.group1.yzx), other.e1 * this.e41)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group4.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412), (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423), (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e423), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group0.xyz) + (other.group0.zxy * this.group2.yzx) - (other.group0.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) + (other.group1.zxy * this.group2.yzx) - (other.group0.yzx * this.group3.zxy) - (other.group1.yzx * this.group2.zxy), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e423) * float4(other.group0.xxy, other.e23)) + (float4(this.e431, this.e4, this.e4, this.e431) * float4(other.group0.zyz, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3)) - float4((other.group0.yzx * this.group4.zxy), other.e41 * this.e1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12), 0.0) + (float2(other.e1234) * this.group0.xy) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e43 * this.e2) + (other.e1234 * this.e1) + (other.e31 * this.e412) - (other.scalar * this.e423), (other.e42 * this.e321) + (other.e1234 * this.e2) + (other.e12 * this.e423) - (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) - (other.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * other.group0.xxyw) - (float4(this.e4, this.e412, this.e423, this.e431) * float4(other.group1.xxy, other.e42)) - (float4(this.e431, this.e4, this.e4, this.e412) * float4(other.group1.zyz, other.e43)) - (other.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group0.xyz) + (this.group2.xyx * other.group0.wwy) + (this.group2.yzz * other.group0.zxw) - (this.group2.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xyx * other.group1.wwy) + (this.group2.yzz * other.group1.zxw) + (this.group3.xyx * other.group0.wwy) + (this.group3.yzz * other.group0.zxw) - (this.group2.zxy * other.group1.yzx) - (this.group3.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * other.group0.xxyw) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group0.zyz, other.e23)) + (this.group4.xyzy * float4(other.group0.www, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4)) - (other.group0.yzxx * float4(this.group4.zxy, this.e1))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * other.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1234) + (other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), 0.0) + (float4(other.e1234) * this.group1) + (float4(this.group0.xx, this.scalar, this.e1234) * float4(other.group4.xyz, other.e4)) - (float4(this.e4, this.e412, this.e423, this.e412) * float4(other.group3.xxy, other.e43)) - (float4(this.e431, this.e4, this.e4, other.e423) * float4(other.group3.zyz, this.e41)) - (other.group4.yzxz * float4(this.group3.zxy, this.e43)) - (this.group4.xyzx * float4(other.group0.xx, other.scalar, other.e41)) - float4((other.group2.yzx * this.group1.zxy), other.e42 * this.e431) - float4((this.group2.zxy * other.group1.yzx), other.e431 * this.e42), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group2.zxy * this.group2.yzx) + (other.group4.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group4.xxy) - (float3(this.e431, this.e4, this.e4) * other.group4.zyz) - (other.group2.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(this.e4, this.e412, this.e423) * other.group1.xxy) + (float3(this.e431, this.e4, this.e4) * other.group1.zyz) + (other.group2.zxy * this.group3.yzx) + (other.group3.zxy * this.group2.yzx) + (other.group4.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group4.zyz) - (float3(this.e321, this.e3, this.e1) * other.group4.xxy) - (other.group2.yzx * this.group3.zxy) - (other.group3.yzx * this.group2.zxy) - (other.group1.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e4, this.e412, this.e423, other.e321) * float4(other.group2.xxy, this.e1234)) + (float4(this.e431, this.e4, this.e4, this.e423) * float4(other.group2.zyz, other.e23)) + (float4(this.group0.yy, this.e1234, this.scalar) * float4(other.group4.xyz, other.e4)) + (float4(other.group1.ww, other.e431, this.e431) * float4(this.group2.xyx, other.e31)) + (float4(other.group4.zx, other.e4, this.e412) * float4(this.group2.yzz, other.e12)) + float4(float3(0.0), -(other.e1 * this.e41) - (other.e2 * this.e42) - (other.e3 * this.e43) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group2.yzx * this.group4.zxy), other.scalar * this.e4) - float4((this.group2.zxy * other.group4.yzx), other.e41 * this.e1)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e4) * float2(this.e321, this.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group3.xyz, this.e1234), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group2.xyz, this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e4 * other.e321, 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)), 
            /* e1, e2, e3, e4 */
            float4((this.scalar * other.e423) + (this.e41 * other.e321) + (this.e31 * other.e412), (this.scalar * other.e431) + (this.e42 * other.e321) + (this.e12 * other.e423), (this.scalar * other.e412) + (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (float3(this.e4) * other.group0.xyz) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(this.e321) * other.group0.xyz) - (this.group1.yzx * other.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (float4(this.e1234) * other.group0) - (other.group0.yzxx * float4(this.group2.zxy, this.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), this.e4 * other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group3.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group0.xyz) + (this.group4.yzx * other.group0.zxy) - (float3(other.e4) * this.group1.xyz) - (this.group4.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group2.xyz), (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension MultiVector {
    public MultiVector anti_sandwich(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e1234 * other.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group4.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
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
        return Origin(
            /* e4 */
            other.e1234 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Motor anti_sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Flector anti_sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(other.group0.xyz, other.e321)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Horizon anti_sandwich(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Motor anti_sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e4) * other.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Motor anti_sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group1.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e4) * float2(other.e321, other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e4) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group2.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiSandwichInfixPartial<Origin> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Origin anti_sandwich(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4 * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Flector anti_sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * other.e321)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Flector anti_sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * other.e4) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Origin {
    public Scalar anti_sandwich(Scalar other) {
        return Horizon(
            /* e321 */
            this.e4 * other.scalar * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Plane> operator /(Plane lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Motor anti_sandwich(AntiScalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Motor anti_sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz * float3(other.group0.xx, other.scalar) * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Flector anti_sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e423) - (other.e412 * this.e431), -(other.e4 * this.e431) - (other.e423 * this.e412), -(other.e4 * this.e412) - (other.e431 * this.e423), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e3 * this.e431) + (other.e321 * this.e423), (other.e1 * this.e412) + (other.e321 * this.e431), (other.e2 * this.e423) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Flector anti_sandwich(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Motor anti_sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Motor anti_sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(other.group1.www, other.e42)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(other.group0.www, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), (other.e423 * this.e423) + (other.e431 * this.e431) + (other.e412 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzx * float4(other.group0.xx, other.scalar, other.e41)) - (this.group0.yzxy * float4(other.group3.zxy, other.e42)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (float3(this.e321) * other.group4.xyz) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (float4(other.e1234) * this.group0) + (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Flector anti_sandwich(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e4 * this.e321) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Flector anti_sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e412 * this.e431 * -1.0, other.e423 * this.e412 * -1.0, other.e431 * this.e423 * -1.0, (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Flector anti_sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.group0.xyz * other.group0.www * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * other.e3, this.e412 * other.e1, this.e423 * other.e2, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.zxyx * other.group0.yzxx)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Plane {
    public Motor anti_sandwich(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Point> operator /(Point lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Motor anti_sandwich(AntiScalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Motor anti_sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Flector anti_sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(other.e4 * this.e1) - (other.e412 * this.e2), -(other.e4 * this.e2) - (other.e423 * this.e3), -(other.e4 * this.e3) - (other.e431 * this.e1), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Horizon anti_sandwich(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e4
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Motor anti_sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.group0.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * other.group1.xyz), 0.0) - float4((other.group0.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Motor anti_sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group0.zxy * this.group0.yzx) - (float3(this.e4) * other.group1.xyz) - (other.group0.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), other.e4 * this.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group2.zxy * this.group0.yzx) - (float3(this.e4) * other.group3.xyz) - (other.group2.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group2.xyz), -(other.scalar * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3))
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Flector anti_sandwich(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e4) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Flector anti_sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz * this.group0.www * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e412 * this.e2 * -1.0, other.e423 * this.e3 * -1.0, other.e431 * this.e1 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.yzxx * this.group0.zxyx)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Flector anti_sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiSandwichInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Point {
    public Scalar anti_sandwich(Scalar other) {
        return Horizon(
            /* e321 */
            this.e4 * other.scalar * -1.0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (first half)
public AntiSandwichInfixPartial<Scalar> operator /(Scalar lhs, AntiSandwichInfix rhs) {
    return AntiSandwichInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiSandwichInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Scalar {
    public Flector anti_sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiSandwichInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Scalar {
    public Line anti_sandwich(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiSandwichInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Scalar {
    public Motor anti_sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiSandwichInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Scalar {
    public MultiVector anti_sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e1234 * this.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        ).geometric_anti_product(this.anti_reverse());
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiSandwichInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.anti_sandwich(rhs);
}
public extension Scalar {
    public Horizon anti_sandwich(Plane other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        ).geometric_anti_product(this.anti_reverse());
    }
}
