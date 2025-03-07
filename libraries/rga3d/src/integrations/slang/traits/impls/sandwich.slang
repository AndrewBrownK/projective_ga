implementing rga3d;
using traits;
// Fancy infix trick (first half)
public SandwichInfixPartial<AntiScalar> operator /(AntiScalar lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension AntiScalar {
    public Flector sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension AntiScalar {
    public Line sandwich(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension AntiScalar {
    public Motor sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension AntiScalar {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, this.e1234 * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group1.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension AntiScalar {
    public Origin sandwich(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
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
            other.e1234 * this.scalar
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public DualNum sandwich(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Flector sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.group0.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group1.xyz) - (float3(this.e1234) * other.group0.xyz)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Flector sandwich(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Line operator /(SandwichInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Line sandwich(Line other) {
        return Line(
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Motor sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.group0.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group4.xyz) - (float3(this.e1234) * other.group1.xyz)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
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
            this.scalar * other.e4
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Flector sandwich(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public Flector sandwich(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.group0.xyz * float3(this.group0.yy, this.e1234) * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public DualNum operator /(SandwichInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension DualNum {
    public DualNum sandwich(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Flector> operator /(Flector lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Motor sandwich(AntiScalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Motor sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.group0.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Flector sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, this.e1) * float4(other.group0.xyx, other.e423)) + (float4(this.group1.zx, this.e4, this.e2) * float4(other.group0.yzz, other.e431)) - (other.group0.zxyx * this.group1.yzxx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, this.e321) * float4(other.group1.yzz, other.e4)) - (float4(this.group1.ww, this.e2, this.e412) * float4(other.group1.xyx, other.e3)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e3 * this.e321) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, this.e321) * float4(other.group0.xyx, other.e321))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Flector sandwich(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Motor sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e3 * other.e31) + (this.e321 * other.e23), (this.e1 * other.e12) + (this.e321 * other.e31), (this.e2 * other.e23) + (this.e321 * other.e12), -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group1.zxy * this.group1.yzx), this.e2 * other.e31)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Motor sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e321 * other.e23, this.e321 * other.e31, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (this.group0.xxyw * other.group1.wzxw) + float4((this.group0.zyz * other.group1.yww), this.e321 * other.e1234) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e423 * other.scalar) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e431 * other.scalar) + (this.e321 * other.e42), (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e412 * other.scalar) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + float4((this.group0.xxy * other.group0.wzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - (other.group1.zxyy * float4(this.group1.yzx, this.e2))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group1.wx), 
            /* e1, e2, e3, e4 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (float4(other.scalar) * this.group0) + (float4(this.group1.ww, this.e2, this.e321) * float4(other.group3.xyx, other.e1234)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e2, other.e321, other.e321) * this.group1.zyz) + (float3(other.e321, other.e3, other.e1) * this.group1.xxy) + (this.group0.yzx * other.group4.zxy) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e412, other.e423) * this.group0.xxy) - (float3(other.e431, other.e4, other.e4) * this.group0.zyz) - (this.group1.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (float3(this.e321) * other.group1.xyz) - (float3(other.e2, other.e321, other.e321) * this.group0.zyz) - (float3(other.e321, other.e3, other.e1) * this.group0.xxy), 
            /* e423, e431, e412, e321 */
            float4((this.e1 * other.e1234) + (this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.zxy, other.e23)) - float4((other.group3.zxy * this.group1.yzx), this.e2 * other.e31)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Flector sandwich(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Flector sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * other.e431) - (this.e321 * other.e423), -(this.e1 * other.e412) - (this.e321 * other.e431), -(this.e2 * other.e423) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.yzxx * other.group0.zxyx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Flector sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * other.e1) + (this.e412 * other.e2), (this.e4 * other.e2) + (this.e423 * other.e3), (this.e4 * other.e3) + (this.e431 * other.e1), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.yzxy * other.group0.zxyy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * other.e2) - (this.e321 * other.e1), -(this.e1 * other.e3) - (this.e321 * other.e2), -(this.e2 * other.e1) - (this.e321 * other.e3), (this.e2 * other.e2) + (this.e3 * other.e3)) + (this.group0.yzxx * other.group0.zxyx)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Flector {
    public Motor sandwich(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
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
        return Origin(
            /* e4 */
            other.e1234 * this.e321
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Motor sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Flector sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Horizon operator /(SandwichInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Horizon sandwich(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321 * -1.0
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Motor sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Motor sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group1.xyz, other.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group0.xyz, other.scalar)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321) * float2(other.e321, other.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group3.xyz, other.e1234), 
            /* e41, e42, e43 */
            float3(this.e321) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group2.xyz, other.scalar)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Horizon> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Origin sandwich(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Flector sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * other.e321) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Flector sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * other.e4) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321) * other.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Scalar operator /(SandwichInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Horizon {
    public Scalar sandwich(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 * other.scalar
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Line> operator /(Line lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Motor sandwich(AntiScalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Motor sandwich(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Flector sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group1.yzx * other.group0.zxy), other.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - float4((this.group1.yzx * other.group1.zxy), other.e3 * this.e12)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Flector sandwich(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Motor sandwich(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e42 * this.e12) + (other.e31 * this.e43), (other.e43 * this.e23) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e23 * this.e42), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.zxy * this.group1.yzx), other.e41 * this.e23) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(other.e31 * this.e12, other.e12 * this.e23, other.e23 * this.e31, -(other.e31 * this.e31) - (other.e12 * this.e12)) - float4((other.group1.zxy * this.group1.yzx), other.e23 * this.e23)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Motor sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.scalar) + (this.e43 * other.e31) + (this.e23 * other.e1234) + (this.e12 * other.e42), (this.e41 * other.e12) + (this.e42 * other.scalar) + (this.e23 * other.e43) + (this.e31 * other.e1234), (this.e42 * other.e23) + (this.e43 * other.scalar) + (this.e31 * other.e41) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.zxyx * float4(this.group0.yzx, this.e41)) - float4((this.group1.yzx * other.group0.zxy), this.e42 * other.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * other.scalar) + (this.e12 * other.e31), (this.e23 * other.e12) + (this.e31 * other.scalar), (this.e31 * other.e23) + (this.e12 * other.scalar), -(this.e31 * other.e31) - (this.e12 * other.e12)) - (other.group1.zxyx * float4(this.group1.yzx, this.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + float4(float3(0.0), (this.e43 * other.e3) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e23 * other.e423), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) + (this.group1.zxy * other.group2.yzx) - (this.group0.yzx * other.group3.zxy) - (this.group1.yzx * other.group2.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.e12 * other.e431), (this.e43 * other.e1) + (this.e23 * other.e412) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e31 * other.e423) + (this.e12 * other.e4), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - float4((this.group1.yzx * other.group4.zxy), this.e12 * other.e3)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Flector sandwich(Origin other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Flector sandwich(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((float3(other.e321) * this.group0.xyz), 0.0) - float4((this.group1.yzx * other.group0.zxy), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Flector sandwich(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Line {
    public Motor sandwich(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Motor> operator /(Motor lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Motor sandwich(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Motor sandwich(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Flector sandwich(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            (other.group0.xyxx * float4(this.group1.wwy, this.e41)) + (other.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (other.e4 * this.scalar) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((this.group1.xyz * other.group1.www), other.e3 * this.e43) - (this.group1.yzxx * float4(other.group0.zxy, other.e423)), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e431 * this.e12) - (other.e321 * this.e41), (other.e4 * this.e31) + (other.e431 * this.scalar) + (other.e412 * this.e23) - (other.e321 * this.e42), (other.e4 * this.e12) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e321 * this.e43), 0.0) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.xyxx * float4(this.group0.wwy, this.e23)) - (other.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(other.group1.zxy, other.e3))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Flector sandwich(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Motor sandwich(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.scalar) + (other.e42 * this.e12) + (other.e23 * this.e1234) + (other.e31 * this.e43), (other.e42 * this.scalar) + (other.e43 * this.e23) + (other.e31 * this.e1234) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e43 * this.scalar) + (other.e23 * this.e42) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.yzxx * float4(other.group0.zxy, other.e41)) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e23 * this.scalar) + (other.e31 * this.e12), (other.e31 * this.scalar) + (other.e12 * this.e23), (other.e23 * this.e31) + (other.e12 * this.scalar), -(other.e31 * this.e31) - (other.e12 * this.e12)) - (this.group1.yzxx * float4(other.group1.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Motor sandwich(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e31 * this.e43) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e31 * this.e1234) + (other.e12 * this.e41) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e23 * this.e42) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xyxw * this.group1.wwyw) + float4((other.group0.yzz * this.group1.zxw), other.scalar * this.e1234) - (other.group0.zxyx * this.group1.yzxx) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e31 * this.e12) + (other.scalar * this.e23), (other.e12 * this.e23) + (other.scalar * this.e31), (other.e12 * this.scalar) + (other.scalar * this.e12), -(other.e31 * this.e31) - (other.e12 * this.e12)) + (other.group1.xyxw * this.group1.wwyw) - (other.group1.zxyx * this.group1.yzxx)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)) + (float2(other.scalar) * float2(this.scalar, this.e1234)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + (other.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * other.e4) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e1234 * other.e321), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xyx * this.group1.wwy) + (other.group2.yzz * this.group1.zxw) + (other.group3.xyx * this.group0.wwy) + (other.group3.yzz * this.group0.zxw) - (other.group2.zxy * this.group1.yzx) - (other.group3.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (other.group3.xyx * this.group1.wwy) + (other.group3.yzz * this.group1.zxw) - (other.group3.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.e12 * other.e431) + (this.scalar * other.e423) - (this.e31 * other.e412), (this.e23 * other.e412) + (this.e31 * other.e4) + (this.scalar * other.e431) - (this.e12 * other.e423), (this.e31 * other.e423) + (this.e12 * other.e4) + (this.scalar * other.e412) - (this.e23 * other.e431), 0.0) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - (other.group1.xyzz * float4(this.group0.www, this.e12))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Flector sandwich(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Flector sandwich(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Flector sandwich(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e43 * other.e3) + (this.scalar * other.e4)) + (other.group0.xyzy * float4(this.group1.www, this.e42)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzy * float4(this.group0.www, this.e31)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Motor {
    public Motor sandwich(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        ).geometric_product(this.reverse());
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
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, other.e1234 * this.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.group0.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group1.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group4.xyz) + (float3(other.e1234) * this.group1.xyz)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * other.group0) + (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group3.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group3.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group3.yzx * other.group0.zxy), other.e321 * this.e1234), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * other.group0.xyx) + (float3(this.e412, this.e423, this.e4) * other.group0.yzz) + (other.group1.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group0.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group0.yzz) - (float3(this.e321, this.e321, this.e2) * other.group0.xyx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12) - (other.e412 * this.e31), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23) - (other.e423 * this.e12), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31) - (other.e431 * this.e23), 0.0) + (float4(this.scalar) * other.group1) - (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group2.zyz, this.e12)) - (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group2.xxy, this.e31)) - (other.group0.xyzx * float4(this.group0.yy, this.e1234, this.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(other.e321) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group2.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e23 * this.e321) + (other.e31 * this.e3), (other.e31 * this.e321) + (other.e12 * this.e1), (other.e23 * this.e2) + (other.e12 * this.e321), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) + (other.group1.yzx * this.group2.zxy) - (other.group0.zxy * this.group3.yzx) - (other.group1.zxy * this.group2.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423), (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4), other.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group1.zxy * this.group4.yzx), other.e31 * this.e2)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)) + (float2(this.scalar) * float2(other.scalar, other.e1234)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) + (other.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group1.xyx, other.e1234)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xxy * other.group1.wzx) + (this.group2.zyz * other.group1.yww) + (this.group3.xxy * other.group0.wzx) + (this.group3.zyz * other.group0.yww) - (this.group2.yzx * other.group1.zxy) - (this.group3.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (this.group3.xxy * other.group1.wzx) + (this.group3.zyz * other.group1.yww) - (this.group3.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.scalar * this.e412), other.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group0.xyx, other.scalar)) - (other.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float2(other.scalar) * this.group0.xy) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group3.zyz, this.e43)) + (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group3.xxy, this.e42)) + (float4(this.group0.xx, this.scalar, other.e1234) * float4(other.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, other.e1) * float4(other.group3.yzz, this.e41)) + (float4(this.group4.ww, this.e2, other.e4) * float4(other.group3.xyx, this.scalar)) + float4(float3(0.0), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group3.zxy * this.group1.yzx), other.e321 * this.e1234) - float4((this.group3.yzx * other.group1.zxy), other.e41 * this.e1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e1234) * other.group3.xyz) + (float3(this.e4, this.e4, this.e431) * other.group1.xyx) + (float3(this.e412, this.e423, this.e4) * other.group1.yzz) + (other.group2.yzx * this.group3.zxy) + (other.group3.yzx * this.group2.zxy) + (other.group4.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group4.yzz) - (float3(this.e321, this.e321, this.e2) * other.group4.xyx) - (other.group2.zxy * this.group3.yzx) - (other.group3.zxy * this.group2.yzx) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group3.yzx * this.group3.zxy) + (other.group1.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group3.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.e423 * this.scalar) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.e431 * this.scalar) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.e423 * this.e31) + (other.e412 * this.scalar), 0.0) + (float4(other.scalar) * this.group4) + (float4(other.group0.yy, other.e1234, this.scalar) * float4(this.group1.xyz, other.e321)) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group2.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group2.xxy, this.e23)) - (this.group1.yzxy * float4(other.group2.zxy, other.e31)) - (float4(this.group0.yy, this.e1234, other.e23) * float4(other.group1.xyz, this.e1)) - float4((other.group3.zxy * this.group4.yzx), other.e12 * this.e3) - float4((this.group3.yzx * other.group4.zxy), other.e3 * this.e12)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group3.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * other.e321, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group3.xyz), -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group3.zxy * other.group0.yzx) - (float3(other.e321) * this.group2.xyz) - (this.group3.yzx * other.group0.zxy)), this.scalar * other.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3), -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (float4(this.scalar) * other.group0) + (other.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) + (this.group4.zxy * other.group0.yzx) - (float3(other.e4) * this.group1.xyz) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzx * float4(this.group0.yy, this.e1234, this.e23)) - (other.group0.yzxy * float4(this.group2.zxy, this.e31))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension MultiVector {
    public MultiVector sandwich(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Origin> operator /(Origin lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Origin {
    public Flector sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(SandwichInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Origin {
    public AntiScalar sandwich(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Origin {
    public Motor sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Origin {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group3.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Plane operator /(SandwichInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Origin {
    public Plane sandwich(Point other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
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
        return Origin(
            /* e4 */
            other.e1234 * this.e321
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Motor sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Flector sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e2 * this.e412) + (other.e321 * this.e423), (other.e3 * this.e423) + (other.e321 * this.e431), (other.e1 * this.e431) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Flector sandwich(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Motor sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), -(other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - float4((other.group1.zxy * this.group0.yzx), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Motor sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e321 * this.e321, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group3.xyz), (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (float3(this.e321) * other.group4.xyz) - (other.group1.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group2.xyz) + (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), other.scalar * this.e321)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Origin operator /(SandwichInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Origin sandwich(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Flector sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321) * float4(0.0, 0.0, 0.0, -1.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Flector sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * other.e2, this.e423 * other.e3, this.e431 * other.e1, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.yzxx * other.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * this.group0.www * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Plane {
    public Motor sandwich(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (first half)
public SandwichInfixPartial<Point> operator /(Point lhs, SandwichInfix rhs) {
    return SandwichInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Motor sandwich(AntiScalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Motor sandwich(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((this.group0.xyz * float3(other.group0.yy, other.e1234)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Flector sandwich(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e1) - (other.e431 * this.e3), -(other.e4 * this.e2) - (other.e412 * this.e1), -(other.e4 * this.e3) - (other.e423 * this.e2), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e1 * this.e2) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Flector sandwich(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e321) * this.group0.xyz * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Motor sandwich(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Motor sandwich(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (other.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e1234 * this.e3) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(SandwichInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public MultiVector sandwich(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e321 * this.e4) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)), 
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (float4(other.scalar) * this.group0) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) + (other.group4.zxy * this.group0.yzx) - (float3(other.e4) * this.group0.xyz) - (other.group4.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e1234 * this.e1) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1234 * this.e2) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e1234 * this.e3) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Flector sandwich(Origin other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Flector sandwich(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e431 * this.e3 * -1.0, other.e412 * this.e1 * -1.0, other.e423 * this.e2 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz * other.group0.www * float3(-1.0)), 0.0)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Flector operator /(SandwichInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Flector sandwich(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e2 * this.e3 * -1.0, other.e3 * this.e1 * -1.0, other.e1 * this.e2 * -1.0, (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        ).geometric_product(this.reverse());
    }
}
// Fancy infix trick (second half)
public Motor operator /(SandwichInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.sandwich(rhs);
}
public extension Point {
    public Motor sandwich(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        ).geometric_product(this.reverse());
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
            other.e1234 * this.scalar
        ).geometric_product(this.reverse());
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
            float2(this.scalar) * other.group0.xy
        ).geometric_product(this.reverse());
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
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
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
            other.e321 * this.scalar
        ).geometric_product(this.reverse());
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
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        ).geometric_product(this.reverse());
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
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        ).geometric_product(this.reverse());
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
            float2(this.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        ).geometric_product(this.reverse());
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
            other.e4 * this.scalar
        ).geometric_product(this.reverse());
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
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
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
            float4(this.scalar) * other.group0
        ).geometric_product(this.reverse());
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
            other.scalar * this.scalar
        ).geometric_product(this.reverse());
    }
}
