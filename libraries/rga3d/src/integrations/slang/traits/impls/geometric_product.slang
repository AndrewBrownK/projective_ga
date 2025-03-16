implementing rga3d;
// Fancy infix trick (first half)
public GeometricProductInfixPartial<AntiScalar> operator /(AntiScalar lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public AntiScalar geometric_product(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public Flector geometric_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public Origin geometric_product(Horizon other) {
        return Origin(
            /* e4 */
            this.e1234 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public Line geometric_product(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public Motor geometric_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234 * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public Origin geometric_product(Plane other) {
        return Origin(
            /* e4 */
            this.e1234 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricProductInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public Plane geometric_product(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension AntiScalar {
    public AntiScalar geometric_product(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<DualNum> operator /(DualNum lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public AntiScalar geometric_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricProductInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public DualNum geometric_product(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public Flector geometric_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.group0.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)) * float4(other.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group1.xyz) - (float3(this.e1234) * other.group0.xyz)), this.scalar * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public Flector geometric_product(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public Line geometric_product(Line other) {
        return Line(
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public Motor geometric_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public MultiVector geometric_product(MultiVector other) {
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
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public Origin geometric_product(Origin other) {
        return Origin(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public Flector geometric_product(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public Flector geometric_product(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(this.e1234 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricProductInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension DualNum {
    public DualNum geometric_product(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Flector> operator /(Flector lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Flector geometric_product(AntiScalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Flector geometric_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.group0.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)) * float4(this.group0.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz)), other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Motor geometric_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, other.e423 * this.e1) * float4(other.group0.xyx, 1.0)) + (float4(this.group1.zx, this.e4, other.e431 * this.e2) * float4(other.group0.yzz, 1.0)) - (other.group0.zxyx * this.group1.yzxx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, other.e4 * this.e321) * float4(other.group1.yzz, 1.0)) - (float4(this.group1.ww, this.e2, other.e3 * this.e412) * float4(other.group1.xyx, 1.0)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e3 * this.e321) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, other.e321 * this.e321) * float4(other.group0.xyx, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Motor geometric_product(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Flector geometric_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e3 * other.e31) + (this.e321 * other.e23), (this.e1 * other.e12) + (this.e321 * other.e31), (this.e2 * other.e23) + (this.e321 * other.e12), -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group1.zxy * this.group1.yzx), this.e2 * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Flector geometric_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e321 * other.e23, this.e321 * other.e31, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (this.group0.xxyw * other.group1.wzxw) + float4((this.group0.zyz * other.group1.yww), this.e321 * other.e1234) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e423 * other.scalar) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e431 * other.scalar) + (this.e321 * other.e42), (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e412 * other.scalar) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + float4((this.group0.xxy * other.group0.wzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - (other.group1.zxyy * float4(this.group1.yzx, this.e2))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group1.wx), 
            /* e1, e2, e3, e4 */
            float4(this.e321 * other.e23, this.e321 * other.e31, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (float4(other.scalar, other.e12, other.e23, other.scalar) * this.group0.xxyw) + (float4(other.e31, other.scalar, other.scalar, this.e321 * other.e1234) * float4(this.group0.zyz, 1.0)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e2, other.e321, other.e321) * this.group1.zyz) + (float3(other.e321, other.e3, other.e1) * this.group1.xxy) + (this.group0.yzx * other.group4.zxy) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e412, other.e423) * this.group0.xxy) - (float3(other.e431, other.e4, other.e4) * this.group0.zyz) - (this.group1.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (float3(this.e321) * other.group1.xyz) - (float3(other.e2, other.e321, other.e321) * this.group0.zyz) - (float3(other.e321, other.e3, other.e1) * this.group0.xxy), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e423 * other.scalar) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e431 * other.scalar) + (this.e321 * other.e42), (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e412 * other.scalar) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + (float4(other.e1234, other.e43, other.e41, this.e321 * other.scalar) * float4(this.group0.xxy, 1.0)) - (this.group0.yzxx * float4(other.group2.zxy, other.e23)) - float4((other.group3.zxy * this.group1.yzx), this.e2 * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Motor geometric_product(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * -1.0) * float4(this.group0.xyz, this.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Motor geometric_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * other.e431) - (this.e321 * other.e423), -(this.e1 * other.e412) - (this.e321 * other.e431), -(this.e2 * other.e423) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.yzxx * other.group0.zxyx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Motor geometric_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * other.e1) + (this.e412 * other.e2), (this.e4 * other.e2) + (this.e423 * other.e3), (this.e4 * other.e3) + (this.e431 * other.e1), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.yzxy * other.group0.zxyy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * other.e2) - (this.e321 * other.e1), -(this.e1 * other.e3) - (this.e321 * other.e2), -(this.e2 * other.e1) - (this.e321 * other.e3), (this.e2 * other.e2) + (this.e3 * other.e3)) + (this.group0.yzxx * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Flector {
    public Flector geometric_product(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Horizon> operator /(Horizon lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<Horizon> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Origin geometric_product(AntiScalar other) {
        return Origin(
            /* e4 */
            other.e1234 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Flector geometric_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Motor geometric_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e321 * -1.0) * float4(other.group1.xyz, other.e4), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricProductInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Scalar geometric_product(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Flector geometric_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Flector geometric_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group1.xyz, other.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group0.xyz, other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * -1.0) * float2(other.e321, other.e4), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group3.xyz, other.e1234), 
            /* e41, e42, e43 */
            float3(this.e321 * -1.0) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e321 * -1.0) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group2.xyz, other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<Horizon> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public AntiScalar geometric_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Motor geometric_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e321 * -1.0) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Motor geometric_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricProductInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Horizon {
    public Horizon geometric_product(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Line> operator /(Line lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Line geometric_product(AntiScalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Line geometric_product(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Flector geometric_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            (other.group0.yzxx * float4(this.group1.zxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e431 * this.e31) - (other.e412 * this.e12)) + float4((this.group1.xyz * other.group1.www), other.e2 * this.e42) - float4((this.group1.yzx * other.group0.zxy), other.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group0.xyz * other.group1.www), other.e3 * this.e12) - float4((this.group1.yzx * other.group1.zxy), other.e2 * this.e31)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Flector geometric_product(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Motor geometric_product(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e42 * this.e12) + (other.e31 * this.e43), (other.e43 * this.e23) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e23 * this.e42), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.zxy * this.group1.yzx), other.e41 * this.e23) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(other.e31 * this.e12, other.e12 * this.e23, other.e23 * this.e31, -(other.e31 * this.e31) - (other.e12 * this.e12)) - float4((other.group1.zxy * this.group1.yzx), other.e23 * this.e23)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Motor geometric_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.scalar) + (this.e43 * other.e31) + (this.e23 * other.e1234) + (this.e12 * other.e42), (this.e41 * other.e12) + (this.e42 * other.scalar) + (this.e23 * other.e43) + (this.e31 * other.e1234), (this.e42 * other.e23) + (this.e43 * other.scalar) + (this.e31 * other.e41) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.zxyx * float4(this.group0.yzx, this.e41)) - float4((this.group1.yzx * other.group0.zxy), this.e42 * other.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * other.scalar) + (this.e12 * other.e31), (this.e23 * other.e12) + (this.e31 * other.scalar), (this.e31 * other.e23) + (this.e12 * other.scalar), -(this.e31 * other.e31) - (this.e12 * other.e12)) - (other.group1.zxyx * float4(this.group1.yzx, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, this.e42 * other.e2) * float4(this.group1.zyz, 1.0)) + (float4(other.e321, other.e3, other.e1, this.e41 * other.e1) * float4(this.group1.xxy, 1.0)) + float4(float3(0.0), (this.e43 * other.e3) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e23 * other.e423), 
            /* e41, e42, e43 */
            (float3(other.scalar, other.e12, other.e23) * this.group0.xxy) + (float3(other.e1234, other.e43, other.e41) * this.group1.xxy) + (float3(other.e42, other.e1234, other.e1234) * this.group1.zyz) + (float3(other.e31, other.scalar, other.scalar) * this.group0.zyz) - (this.group0.yzx * other.group3.zxy) - (this.group1.yzx * other.group2.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar, other.e12, other.e23) * this.group1.xxy) + (float3(other.e31, other.scalar, other.scalar) * this.group1.zyz) - (this.group1.yzx * other.group3.zxy), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.e12 * other.e431), (this.e43 * other.e1) + (this.e23 * other.e412) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e31 * other.e423) + (this.e12 * other.e4), 0.0) - (float4(other.e2, other.e321, other.e321, this.e31 * other.e2) * float4(this.group0.zyz, 1.0)) - (float4(other.e321, other.e3, other.e1, this.e23 * other.e1) * float4(this.group0.xxy, 1.0)) - float4((this.group1.yzx * other.group4.zxy), this.e12 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricProductInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Plane geometric_product(Origin other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Flector geometric_product(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) * float4(this.group1.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((float3(other.e321) * this.group0.xyz), 0.0) - float4((this.group1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Flector geometric_product(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Line {
    public Line geometric_product(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Motor> operator /(Motor lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Motor geometric_product(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Motor geometric_product(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Flector geometric_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            (other.group0.xyxx * float4(this.group1.wwy, this.e41)) + (other.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (other.e4 * this.scalar) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((this.group1.xyz * other.group1.www), other.e3 * this.e43) - (this.group1.yzxx * float4(other.group0.zxy, other.e423)), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e431 * this.e12) - (other.e321 * this.e41), (other.e4 * this.e31) + (other.e431 * this.scalar) + (other.e412 * this.e23) - (other.e321 * this.e42), (other.e4 * this.e12) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e321 * this.e43), 0.0) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.xyxx * float4(this.group0.wwy, this.e23)) - (other.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(other.group1.zxy, other.e3))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Flector geometric_product(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4((this.group0.xyz * float3(-1.0)), this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Motor geometric_product(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.scalar) + (other.e42 * this.e12) + (other.e23 * this.e1234) + (other.e31 * this.e43), (other.e42 * this.scalar) + (other.e43 * this.e23) + (other.e31 * this.e1234) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e43 * this.scalar) + (other.e23 * this.e42) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.yzxx * float4(other.group0.zxy, other.e41)) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e23 * this.scalar) + (other.e31 * this.e12), (other.e31 * this.scalar) + (other.e12 * this.e23), (other.e23 * this.e31) + (other.e12 * this.scalar), -(other.e31 * this.e31) - (other.e12 * this.e12)) - (this.group1.yzxx * float4(other.group1.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Motor geometric_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e31 * this.e43) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e31 * this.e1234) + (other.e12 * this.e41) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e23 * this.e42) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xyxw * this.group1.wwyw) + float4((other.group0.yzz * this.group1.zxw), other.scalar * this.e1234) - (other.group0.zxyx * this.group1.yzxx) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e31 * this.e12) + (other.scalar * this.e23), (other.e12 * this.e23) + (other.scalar * this.e31), (other.e12 * this.scalar) + (other.scalar * this.e12), -(other.e31 * this.e31) - (other.e12 * this.e12)) + (other.group1.xyxw * this.group1.wwyw) - (other.group1.zxyx * this.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + (float2(other.scalar) * float2(this.scalar, this.e1234)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, this.e42 * other.e2) * float4(this.group1.zyz, 1.0)) + (float4(other.e321, other.e3, other.e1, this.e41 * other.e1) * float4(this.group1.xxy, 1.0)) + (other.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * other.e4) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e1234 * other.e321), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group3.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(other.scalar, other.e12, other.e23) * this.group0.xxy) + (float3(other.e1234, other.e43, other.e41) * this.group1.xxy) + (float3(other.e42, other.e1234, other.e1234) * this.group1.zyz) + (float3(other.e31, other.scalar, other.scalar) * this.group0.zyz) - (other.group2.zxy * this.group1.yzx) - (other.group3.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group3.xyz) + (float3(other.scalar, other.e12, other.e23) * this.group1.xxy) + (float3(other.e31, other.scalar, other.scalar) * this.group1.zyz) - (other.group3.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.e12 * other.e431) + (this.scalar * other.e423) - (this.e31 * other.e412), (this.e23 * other.e412) + (this.e31 * other.e4) + (this.scalar * other.e431) - (this.e12 * other.e423), (this.e31 * other.e423) + (this.e12 * other.e4) + (this.scalar * other.e412) - (this.e23 * other.e431), 0.0) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (float4(other.e2, other.e321, other.e321, this.e31 * other.e2) * float4(this.group0.zyz, 1.0)) - (float4(other.e321, other.e3, other.e1, this.e23 * other.e1) * float4(this.group0.xxy, 1.0)) - (other.group1.xyzz * float4(this.group0.www, this.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Flector geometric_product(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Flector geometric_product(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) * float4(this.group1.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), this.scalar * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Flector geometric_product(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e43 * other.e3) + (this.scalar * other.e4)) + (other.group0.xyzy * float4(this.group1.www, this.e42)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzy * float4(this.group0.www, this.e31)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Motor {
    public Motor geometric_product(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<MultiVector> operator /(MultiVector lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(DualNum other) {
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
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (other.group0.xyxx * float4(this.group0.xx, this.e31, this.e41)) + (other.group0.yzzy * float4(this.group3.xyz.zx, this.scalar, this.e42)) + float4(float3(0.0), (other.e4 * this.scalar) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((this.group3.xyz * other.group1.www), other.e3 * this.e43) - float4((this.group3.yzx * other.group0.zxy), other.e423 * this.e23), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * other.group0.xyx) + (float3(this.e412, this.e423, this.e4) * other.group0.yzz) + (other.group1.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group0.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group0.yzz) - (float3(this.e321, this.e321, this.e2) * other.group0.xyx), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e431 * this.e12) - (other.e321 * this.e41), (other.e4 * this.e31) + (other.e431 * this.scalar) + (other.e412 * this.e23) - (other.e321 * this.e42), (other.e4 * this.e12) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e321 * this.e43), 0.0) + float4((this.group2.yzx * other.group0.zxy), other.e321 * this.scalar) - (other.group0.xyxx * float4(this.group0.yy, this.e42, this.e23)) - (other.group0.yzzy * float4(this.group2.xyz.zx, this.e1234, this.e31)) - float4((this.group3.yzx * other.group1.zxy), other.e3 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group3.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(other.e321) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e321 * -1.0) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4((this.group2.xyz * float3(-1.0)), this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e23 * this.e321) + (other.e31 * this.e3), (other.e31 * this.e321) + (other.e12 * this.e1), (other.e23 * this.e2) + (other.e12 * this.e321), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (other.group0.xyx * float3(this.group0.xx, this.e31)) + (other.group0.yzz * float3(this.group3.xyz.zx, this.scalar)) + (other.group1.xyx * float3(this.group0.yy, this.e42)) + (other.group1.yzz * float3(this.group2.xyz.zx, this.e1234)) - (other.group0.zxy * this.group3.yzx) - (other.group1.zxy * this.group2.yzx), 
            /* e23, e31, e12 */
            (other.group1.xyx * float3(this.group0.xx, this.e31)) + (other.group1.yzz * float3(this.group3.xyz.zx, this.scalar)) - (other.group1.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423), (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4), other.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group1.zxy * this.group4.yzx), other.e31 * this.e2)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float2(this.scalar) * float2(other.scalar, other.e1234)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) + (other.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, other.e1234 * this.e321) * float4(other.group1.xyx, 1.0)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group3.xyz) + (float3(other.scalar) * this.group2.xyz) + (other.group0.xyx * float3(this.group0.xx, this.e31)) + (other.group0.yzz * float3(this.group3.xyz.zx, this.scalar)) + (other.group1.xyx * float3(this.group0.yy, this.e42)) + (other.group1.yzz * float3(this.group2.xyz.zx, this.e1234)) - (this.group2.yzx * other.group1.zxy) - (this.group3.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (other.group1.xyx * float3(this.group0.xx, this.e31)) + (other.group1.yzz * float3(this.group3.xyz.zx, this.scalar)) - (this.group3.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.scalar * this.e412), other.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, other.scalar * this.e321) * float4(other.group0.xyx, 1.0)) - (other.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float2(other.scalar) * this.group0.xy) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (other.group1.yzzx * float4(this.group3.xyz.zx, this.scalar, this.e41)) + (float4(this.group0.xx, this.e31, other.e1234 * this.e321) * float4(other.group1.xyx, 1.0)) + (float4(this.group1.zx, this.e321, other.e3 * this.e43) * float4(other.group3.yzz, 1.0)) + (float4(this.group4.ww, this.e2, other.e2 * this.e42) * float4(other.group3.xyx, 1.0)) + float4(float3(0.0), -(other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((this.group3.xyz * other.group4.www), other.e4 * this.scalar) - (this.group1.yzxy * float4(other.group3.zxy, other.e42)) - float4((this.group3.yzx * other.group1.zxy), other.e41 * this.e1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * other.group1.xyx) + (float3(this.e412, this.e423, this.e4) * other.group1.yzz) + (other.group2.xyx * float3(this.group0.xx, this.e31)) + (other.group2.yzz * float3(this.group3.xyz.zx, this.scalar)) + (other.group3.xyx * float3(this.group0.yy, this.e42)) + (other.group3.yzz * float3(this.group2.xyz.zx, this.e1234)) + (other.group4.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group4.yzz) - (float3(this.e321, this.e321, this.e2) * other.group4.xyx) - (other.group2.zxy * this.group3.yzx) - (other.group3.zxy * this.group2.yzx) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (other.group3.xyx * float3(this.group0.xx, this.e31)) + (other.group3.yzz * float3(this.group3.xyz.zx, this.scalar)) + (other.group1.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group3.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.e423 * this.scalar) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.e431 * this.scalar) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.e423 * this.e31) + (other.e412 * this.scalar), 0.0) + (float4(other.scalar) * this.group4) + (float4(other.group0.yy, other.e1234, other.e321 * this.scalar) * float4(this.group1.xyz, 1.0)) - (other.group1.xyxx * float4(this.group0.yy, this.e42, this.e23)) - (other.group1.yzzy * float4(this.group2.xyz.zx, this.e1234, this.e31)) - float4((this.group2.xyz * other.group4.www), other.e12 * this.e3) - float4((other.group2.zxy * this.group1.yzx), other.e3 * this.e12) - float4((other.group3.zxy * this.group4.yzx), other.e23 * this.e1) - float4((this.group3.yzx * other.group4.zxy), other.e31 * this.e2)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group3.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * other.e321, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) * float4(this.group3.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            float3(other.e321 * -1.0) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group3.zxy * other.group0.yzx) - (float3(other.e321) * this.group2.xyz) - (this.group3.yzx * other.group0.zxy)), this.scalar * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Point other) {
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
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_product(Scalar other) {
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
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Origin> operator /(Origin lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public Origin geometric_product(DualNum other) {
        return Origin(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public Motor geometric_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public AntiScalar geometric_product(Horizon other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricProductInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public Plane geometric_product(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public Flector geometric_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e321 * this.e4), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group3.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public AntiScalar geometric_product(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public Line geometric_product(Point other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Origin {
    public Origin geometric_product(Scalar other) {
        return Origin(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Plane> operator /(Plane lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Origin geometric_product(AntiScalar other) {
        return Origin(
            /* e4 */
            other.e1234 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Flector geometric_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Motor geometric_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e2 * this.e412) + (other.e321 * this.e423), (other.e3 * this.e423) + (other.e321 * this.e431), (other.e1 * this.e431) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Motor geometric_product(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Flector geometric_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, -(other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) * float4(other.group1.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - float4((other.group1.zxy * this.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Flector geometric_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) * float4(other.group1.xyz, 1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public MultiVector geometric_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e321 * this.e321, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) * float4(other.group3.xyz, 1.0), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (float3(this.e321) * other.group4.xyz) - (other.group1.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            float3(this.e321 * -1.0) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group2.xyz) + (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public AntiScalar geometric_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Motor geometric_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Motor geometric_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * other.e2, this.e423 * other.e3, this.e431 * other.e1, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.yzxx * other.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e321 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricProductInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Plane {
    public Plane geometric_product(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Point> operator /(Point lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(GeometricProductInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Plane geometric_product(AntiScalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Flector geometric_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4((float3(other.e1234) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Motor geometric_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e1) - (other.e431 * this.e3), -(other.e4 * this.e2) - (other.e412 * this.e1), -(other.e4 * this.e3) - (other.e423 * this.e2), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e1 * this.e2) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Motor geometric_product(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Flector geometric_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Flector geometric_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (other.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e1234 * this.e3) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public MultiVector geometric_product(MultiVector other) {
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
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Line geometric_product(Origin other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Motor geometric_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e431 * this.e3 * -1.0, other.e412 * this.e1 * -1.0, other.e423 * this.e2 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Motor geometric_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e2 * this.e3 * -1.0, other.e3 * this.e1 * -1.0, other.e1 * this.e2 * -1.0, (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricProductInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Point {
    public Point geometric_product(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public GeometricProductInfixPartial<Scalar> operator /(Scalar lhs, GeometricProductInfix rhs) {
    return GeometricProductInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricProductInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public AntiScalar geometric_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricProductInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public DualNum geometric_product(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricProductInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Flector geometric_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricProductInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Horizon geometric_product(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricProductInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Line geometric_product(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricProductInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Motor geometric_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricProductInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public MultiVector geometric_product(MultiVector other) {
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
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricProductInfixPartial<Scalar> lhs, Origin rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Origin geometric_product(Origin other) {
        return Origin(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricProductInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Plane geometric_product(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricProductInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Point geometric_product(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricProductInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.geometric_product(rhs);
}
public extension Scalar {
    public Scalar geometric_product(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
