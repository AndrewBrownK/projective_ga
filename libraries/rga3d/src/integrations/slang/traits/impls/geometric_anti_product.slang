implementing rga3d;
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<AntiScalar> operator /(AntiScalar lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public AntiScalar geometric_anti_product(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public DualNum geometric_anti_product(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Flector geometric_anti_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Horizon geometric_anti_product(Horizon other) {
        return Horizon(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Line geometric_anti_product(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Motor geometric_anti_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public MultiVector geometric_anti_product(MultiVector other) {
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
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Origin geometric_anti_product(Origin other) {
        return Origin(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Plane geometric_anti_product(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Point geometric_anti_product(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension AntiScalar {
    public Scalar geometric_anti_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<DualNum> operator /(DualNum lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricAntiProductInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public DualNum geometric_anti_product(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(GeometricAntiProductInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public DualNum geometric_anti_product(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Flector geometric_anti_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(this.e1234) * other.group0.xyz)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group0.yy, this.e1234, (this.scalar * other.e4) + (this.e1234 * other.e321)) * float4(other.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Horizon geometric_anti_product(Horizon other) {
        return Horizon(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Line geometric_anti_product(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Motor geometric_anti_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public MultiVector geometric_anti_product(MultiVector other) {
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
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Flector geometric_anti_product(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Flector geometric_anti_product(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Flector geometric_anti_product(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension DualNum {
    public Scalar geometric_anti_product(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Flector> operator /(Flector lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Flector geometric_anti_product(AntiScalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Flector geometric_anti_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(other.scalar) * this.group1.xyz)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(other.group0.yy, other.e1234, (other.e1234 * this.e321) - (other.scalar * this.e4)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Motor geometric_anti_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(other.e423 * this.e4) - (other.e412 * this.e431), -(other.e423 * this.e412) - (other.e431 * this.e4), -(other.e431 * this.e423) - (other.e412 * this.e4), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group1.zxyx) - (float4(other.e4) * float4(this.group1.xyz, this.e4)), 
            /* e23, e31, e12, scalar */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (float4(this.e4, this.e412, this.e423, other.e423 * this.e1) * float4(other.group0.xxy, 1.0)) + (float4(this.e431, this.e4, this.e4, other.e431 * this.e2) * float4(other.group0.zyz, 1.0)) + (other.group1.yzxz * this.group0.zxyz) - (float4(this.e2, this.e321, this.e321, other.e4 * this.e321) * float4(other.group1.zyz, 1.0)) - (float4(this.e321, this.e3, this.e1, other.e3 * this.e412) * float4(other.group1.xxy, 1.0)) - (other.group0.yzxx * this.group1.zxyx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Motor geometric_anti_product(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Flector geometric_anti_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e1 * other.e42) + (this.e431 * other.e23) + (this.e321 * other.e43), 0.0) - (this.group1.yzxz * float4(other.group1.zxy, other.e43)) - float4((other.group1.xyz * this.group0.www), this.e431 * other.e42) - float4((other.group0.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e423, e431, e412, e321 */
            (this.group1.yzxy * float4(other.group0.zxy, other.e31)) + float4(float3(0.0), (this.e412 * other.e12) - (this.e2 * other.e42) - (this.e3 * other.e43)) + float4((other.group0.xyz * this.group0.www), this.e423 * other.e23) - float4((other.group0.yzx * this.group1.zxy), this.e1 * other.e41)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Flector geometric_anti_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * other.group0.wwyw) - (this.group1.xyxz * float4(other.group1.wwy, other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e423)) - float4((other.group1.xyz * this.group0.www), this.e431 * other.e42), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.wwy, other.e31)) + (this.group1.yzzz * float4(other.group0.zxw, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((other.group0.xyz * this.group0.www), this.e423 * other.e23) - (other.group0.yzxx * float4(this.group1.zxy, this.e1))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((this.e2 * other.e43) + (this.e412 * other.e31) + (this.e321 * other.e41) - (this.e431 * other.e12), (this.e3 * other.e41) + (this.e423 * other.e12) + (this.e321 * other.e42) - (this.e412 * other.e23), (this.e3 * other.e1234) + (this.e431 * other.e23) + (this.e321 * other.e43) - (this.e412 * other.scalar), 0.0) + (this.group0.xyxw * float4(other.group0.yy, other.e42, other.e1234)) - (this.group1.xyxz * float4(other.group0.xx, other.e31, other.e43)) - float4((other.group3.xyz * this.group0.www), this.e431 * other.e42) - float4((other.group2.yzx * this.group0.zxy), this.e423 * other.e41), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (float3(this.e4) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group1.xyx) - (float3(other.e412, other.e423, other.e4) * this.group1.yzz), 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e3, other.e1, other.e321) * this.group1.yzz) + (float3(other.e321, other.e321, other.e2) * this.group1.xyx) + (this.group0.zxy * other.group4.yzx) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e4, other.e431) * this.group0.xyx) - (float3(other.e412, other.e423, other.e4) * this.group0.yzz) - (this.group1.zxy * other.group1.yzx), 
            /* e423, e431, e412, e321 */
            (this.group1.xyxy * float4(other.group0.yy, other.e42, other.e31)) + (this.group1.yzzz * float4(other.group2.xyz.zx, other.e1234, other.e12)) + float4(float3(0.0), (this.e321 * other.e1234) - (this.e2 * other.e42) - (this.e3 * other.e43) - (this.e4 * other.scalar)) + float4((other.group2.xyz * this.group0.www), this.e423 * other.e23) - float4((other.group2.yzx * this.group1.zxy), this.e1 * other.e41)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Motor geometric_anti_product(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * -1.0) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e4 * -1.0) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Motor geometric_anti_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(this.e4 * other.e423) - (this.e431 * other.e412), -(this.e4 * other.e431) - (this.e412 * other.e423), -(this.e4 * other.e412) - (this.e423 * other.e431), (this.e431 * other.e431) + (this.e412 * other.e412)) + (this.group1.zxyx * other.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(this.e2 * other.e412) - (this.e321 * other.e423), -(this.e3 * other.e423) - (this.e321 * other.e431), -(this.e1 * other.e431) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.zxyx * other.group0.yzxx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Motor geometric_anti_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * -1.0) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4((this.e4 * other.e1) + (this.e431 * other.e3), (this.e4 * other.e2) + (this.e412 * other.e1), (this.e4 * other.e3) + (this.e423 * other.e2), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.zxyy * other.group0.yzxy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Flector {
    public Flector geometric_anti_product(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar * -1.0) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Horizon> operator /(Horizon lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<Horizon> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Horizon geometric_anti_product(AntiScalar other) {
        return Horizon(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Horizon geometric_anti_product(DualNum other) {
        return Horizon(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Motor geometric_anti_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * -1.0) * float4(other.group1.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiProductInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Point geometric_anti_product(Line other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Flector geometric_anti_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * other.e4, 0.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321 * -1.0) * other.group4.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<Horizon> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Scalar geometric_anti_product(Origin other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Line geometric_anti_product(Plane other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321 * -1.0) * other.group0.xyz
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Horizon {
    public Scalar geometric_anti_product(Point other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Line> operator /(Line lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Line geometric_anti_product(AntiScalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Line geometric_anti_product(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Flector geometric_anti_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e41) + (other.e412 * this.e42), (other.e4 * this.e42) + (other.e423 * this.e43), (other.e4 * this.e43) + (other.e431 * this.e41), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group0.zxy * other.group1.yzx), other.e1 * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiProductInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Point geometric_anti_product(Horizon other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Motor geometric_anti_product(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e43 * this.e42, other.e41 * this.e43, other.e42 * this.e41, -(other.e42 * this.e42) - (other.e43 * this.e43)) - float4((other.group0.yzx * this.group0.zxy), other.e41 * this.e41), 
            /* e23, e31, e12, scalar */
            float4((other.e43 * this.e31) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e23 * this.e43), (other.e42 * this.e23) + (other.e31 * this.e41), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e23) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Motor geometric_anti_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.e1234) + (this.e42 * other.e43), (this.e42 * other.e1234) + (this.e43 * other.e41), (this.e41 * other.e42) + (this.e43 * other.e1234), -(this.e42 * other.e42) - (this.e43 * other.e43)) - (other.group0.yzxx * float4(this.group0.zxy, this.e41)), 
            /* e23, e31, e12, scalar */
            float4((this.e41 * other.scalar) + (this.e42 * other.e12) + (this.e23 * other.e1234) + (this.e31 * other.e43), (this.e42 * other.scalar) + (this.e43 * other.e23) + (this.e31 * other.e1234) + (this.e12 * other.e41), (this.e41 * other.e31) + (this.e43 * other.scalar) + (this.e23 * other.e42) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.yzxx * float4(this.group0.zxy, this.e41)) - float4((this.group1.zxy * other.group0.yzx), this.e42 * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e42 * other.e3) + (this.e23 * other.e4) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e43 * other.e1) + (this.e31 * other.e4) + (this.e12 * other.e423), (this.e41 * other.e2) + (this.e43 * other.e321) + (this.e23 * other.e431) + (this.e12 * other.e4), this.e43 * other.e412 * -1.0) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e423), 
            /* e41, e42, e43 */
            (this.group0.xyx * float3(other.group0.yy, other.e42)) + (this.group0.yzz * float3(other.group2.xyz.zx, other.e1234)) - (this.group0.zxy * other.group2.yzx), 
            /* e23, e31, e12 */
            (this.group0.xyx * float3(other.group0.xx, other.e31)) + (this.group0.yzz * float3(other.group3.xyz.zx, other.scalar)) + (this.group1.xyx * float3(other.group0.yy, other.e42)) + (this.group1.yzz * float3(other.group2.xyz.zx, other.e1234)) - (this.group0.zxy * other.group3.yzx) - (this.group1.zxy * other.group2.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e41 * other.e4) + (this.e42 * other.e412), (this.e42 * other.e4) + (this.e43 * other.e423), (this.e41 * other.e431) + (this.e43 * other.e4), -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group0.zxy * other.group4.yzx), this.e41 * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Flector geometric_anti_product(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Flector geometric_anti_product(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e42 * other.e412, this.e43 * other.e423, this.e41 * other.e431, -(this.e31 * other.e431) - (this.e12 * other.e412)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Flector geometric_anti_product(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - float4((this.group0.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, -(this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3)) * float4(this.group0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Line {
    public Line geometric_anti_product(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Motor> operator /(Motor lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Motor geometric_anti_product(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Motor geometric_anti_product(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Flector geometric_anti_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (other.group0.xxyw * this.group0.wzxw) - (other.group1.yzxy * float4(this.group1.zxy, this.e42)) - (this.group0.zxyx * float4(other.group0.yzx, other.e423)), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group0.xyz, this.scalar)) + (other.group1.xxyw * this.group0.wzxw) - (this.group0.zxyx * float4(other.group1.yzx, other.e1))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Flector geometric_anti_product(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Motor geometric_anti_product(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.e1234) + (other.e43 * this.e42), (other.e41 * this.e43) + (other.e42 * this.e1234), (other.e42 * this.e41) + (other.e43 * this.e1234), -(other.e42 * this.e42) - (other.e43 * this.e43)) - (this.group0.zxyx * float4(other.group0.yzx, other.e41)), 
            /* e23, e31, e12, scalar */
            float4((other.e41 * this.scalar) + (other.e43 * this.e31) + (other.e23 * this.e1234) + (other.e12 * this.e42), (other.e41 * this.e12) + (other.e42 * this.scalar) + (other.e23 * this.e43) + (other.e31 * this.e1234), (other.e42 * this.e23) + (other.e43 * this.scalar) + (other.e31 * this.e41) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.zxyx * float4(other.group0.yzx, other.e41)) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Motor geometric_anti_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.e43 * this.e42) + (other.e1234 * this.e41), (other.e42 * this.e1234) + (other.e1234 * this.e42), (other.e43 * this.e1234) + (other.e1234 * this.e43), -(other.e42 * this.e42) - (other.e43 * this.e43)) + (other.group0.xxyw * this.group0.wzxw) - (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e12 * this.e42) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e23 * this.e43) + (other.e31 * this.e1234) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e31 * this.e41) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xxyw * this.group1.wzxw) + float4((other.group0.zyz * this.group1.yww), other.scalar * this.e1234) - (other.group0.yzxx * this.group1.zxyx) - float4((other.group1.yzx * this.group0.zxy), other.e42 * this.e31)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0) + (float2(this.e1234) * other.group0.xy) - (float2(this.e41) * float2(other.e23, other.e41)) - (float2(this.e42) * float2(other.e31, other.e42)) - (float2(this.e43) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((this.e42 * other.e3) + (this.e1234 * other.e1) + (this.e23 * other.e4) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e43 * other.e1) + (this.e1234 * other.e2) + (this.e31 * other.e4) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e1234 * other.e3) + (this.e23 * other.e431) + (this.e12 * other.e4) + (this.scalar * other.e412), this.e43 * other.e412 * -1.0) + (this.group0.xyxw * float4(other.group4.ww, other.e2, other.e4)) - (this.group0.zxyx * float4(other.group1.yzx, other.e423)) - (other.group4.yzxy * float4(this.group1.zxy, this.e42)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group2.xyz) + (this.group0.xyx * float3(other.group0.yy, other.e42)) + (this.group0.yzz * float3(other.group2.xyz.zx, other.e1234)) - (other.group2.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group3.xyz) + (float3(this.scalar) * other.group2.xyz) + (this.group0.xyx * float3(other.group0.xx, other.e31)) + (this.group0.yzz * float3(other.group3.xyz.zx, other.scalar)) + (this.group1.xyx * float3(other.group0.yy, other.e42)) + (this.group1.yzz * float3(other.group2.xyz.zx, other.e1234)) - (other.group2.yzx * this.group1.zxy) - (other.group3.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e42 * other.e2) - (this.e43 * other.e3) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.xyxw * float4(other.group1.ww, other.e431, other.e321)) + (float4(other.group4.zx, other.e4, this.scalar * other.e4) * float4(this.group0.yzz, 1.0)) - (this.group0.zxyx * float4(other.group4.yzx, other.e1))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Flector geometric_anti_product(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group1.xyz, this.e1234), 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group0.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Flector geometric_anti_product(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412) + (this.scalar * other.e423), (this.e42 * other.e321) + (this.e12 * other.e423) + (this.scalar * other.e431), (this.e43 * other.e321) + (this.e23 * other.e431) + (this.scalar * other.e412), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234 * other.e423, this.e1234 * other.e431, this.e1234 * other.e412, -(this.e31 * other.e431) - (this.e12 * other.e412)) + (this.group0.yzxw * other.group0.zxyw) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Flector geometric_anti_product(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3)) * float4(this.group0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Motor {
    public Motor geometric_anti_product(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<MultiVector> operator /(MultiVector lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(AntiScalar other) {
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
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(DualNum other) {
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
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) - (float2(this.e423, this.e4) * other.group0.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), other.e412 * this.e43 * -1.0) + (float4(this.e1234, this.e43, this.e41, this.e1234) * other.group0.xxyw) - (other.group1.yzxy * float4(this.group3.zxy, this.e42)) - float4((this.group2.zxy * other.group0.yzx), other.e423 * this.e41), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group1.xxy) - (float3(this.e431, this.e4, this.e4) * other.group1.zyz), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e412, this.e423) * other.group0.xxy) + (float3(this.e431, this.e4, this.e4) * other.group0.zyz) + (other.group1.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group1.zyz) - (float3(this.e321, this.e3, this.e1) * other.group1.xxy) - (other.group0.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            float4(other.e412 * this.e42, other.e431 * this.e1234, other.e412 * this.e1234, -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (float4(other.e4) * float4(this.group2.xyz, this.scalar)) + (float4(this.e1234, this.e43, this.e41, this.e1234) * other.group1.xxyw) - float4((this.group2.zxy * other.group1.yzx), other.e1 * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group2.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group4.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412), (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423), (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431), 0.0) - (float4(this.e4, this.e412, this.e423, other.e42 * this.e431) * float4(other.group1.xxy, 1.0)) - (float4(this.e431, this.e4, this.e4, other.e43 * this.e412) * float4(other.group1.zyz, 1.0)) - float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e423), 
            /* e41, e42, e43 */
            (float3(this.e1234, this.e43, this.e41) * other.group0.xxy) + (float3(this.e42, this.e1234, this.e1234) * other.group0.zyz) - (other.group0.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar, this.e12, this.e23) * other.group0.xxy) + (float3(this.e1234, this.e43, this.e41) * other.group1.xxy) + (float3(this.e42, this.e1234, this.e1234) * other.group1.zyz) + (float3(this.e31, this.scalar, this.scalar) * other.group0.zyz) - (other.group0.yzx * this.group3.zxy) - (other.group1.yzx * this.group2.zxy), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, other.e23 * this.e423) * float4(other.group0.xxy, 1.0)) + (float4(this.e431, this.e4, this.e4, other.e31 * this.e431) * float4(other.group0.zyz, 1.0)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3)) - float4((other.group0.yzx * this.group4.zxy), other.e41 * this.e1)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) + (float2(other.e1234) * this.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e43 * this.e2) + (other.e1234 * this.e1) + (other.e31 * this.e412) - (other.scalar * this.e423), (other.e42 * this.e321) + (other.e1234 * this.e2) + (other.e12 * this.e423) - (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) - (other.scalar * this.e412), 0.0) + (float4(this.e321, this.e3, this.e1, this.e4) * other.group0.xxyw) - (float4(this.e4, this.e412, this.e423, other.e42 * this.e431) * float4(other.group1.xxy, 1.0)) - (float4(this.e431, this.e4, this.e4, other.e43 * this.e412) * float4(other.group1.zyz, 1.0)) - (other.group0.yzxx * float4(this.group1.zxy, this.e423)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234, this.e43, this.e41) * other.group0.xxy) + (float3(this.e42, this.e1234, this.e1234) * other.group0.zyz) - (this.group2.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3.xyz) + (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar, this.e12, this.e23) * other.group0.xxy) + (float3(this.e1234, this.e43, this.e41) * other.group1.xxy) + (float3(this.e42, this.e1234, this.e1234) * other.group1.zyz) + (float3(this.e31, this.scalar, this.scalar) * other.group0.zyz) - (this.group2.zxy * other.group1.yzx) - (this.group3.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            (float4(this.e4, this.e412, this.e423, this.e321) * other.group0.xxyw) + (float4(this.e431, this.e4, this.e4, other.e23 * this.e423) * float4(other.group0.zyz, 1.0)) + (this.group4.xyzy * float4(other.group0.www, other.e31)) + float4(float3(0.0), (other.e12 * this.e412) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4)) - (other.group0.yzxx * float4(this.group4.zxy, this.e1))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0) + (float2(other.e423) * float2(this.e1, this.e423)) + (float2(other.e431) * float2(this.e2, this.e431)) + (float2(other.e412) * float2(this.e3, this.e412)) + (float2(this.e1234) * other.group0.xy) - (float2(other.e41) * float2(this.e23, this.e41)) - (float2(other.e42) * float2(this.e31, this.e42)) - (float2(other.e43) * float2(this.e12, this.e43)) - (float2(this.e423, this.e4) * other.group1.xw), 
            /* e1, e2, e3, e4 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e43 * this.e2) + (other.e31 * this.e412) + (other.e423 * this.scalar) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e2 * this.e1234) + (other.e4 * this.e31) + (other.e41 * this.e3) + (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e431 * this.scalar) + (other.e321 * this.e42), (other.e3 * this.e1234) + (other.e4 * this.e12) + (other.e42 * this.e1) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e412 * this.scalar) + (other.e321 * this.e43), 0.0) + (float4(other.e1234) * this.group1) + (float4(this.e1234, this.e43, this.e41, this.e1234) * other.group1.xxyw) - (float4(this.e4, this.e412, this.e423, other.e423 * this.e41) * float4(other.group3.xxy, 1.0)) - (float4(this.e431, this.e4, this.e4, other.e431 * this.e42) * float4(other.group3.zyz, 1.0)) - (other.group4.yzxz * float4(this.group3.zxy, this.e43)) - (this.group4.xyzx * float4(other.group0.xx, other.scalar, other.e41)) - float4((other.group2.yzx * this.group1.zxy), other.e43 * this.e412) - float4((this.group2.zxy * other.group1.yzx), other.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234, this.e43, this.e41) * other.group2.xxy) + (float3(this.e42, this.e1234, this.e1234) * other.group2.zyz) + (other.group4.yzx * this.group4.zxy) - (float3(other.e4) * this.group4.xyz) - (float3(this.e4, this.e412, this.e423) * other.group4.xxy) - (float3(this.e431, this.e4, this.e4) * other.group4.zyz) - (other.group2.yzx * this.group2.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group2.xyz) + (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar, this.e12, this.e23) * other.group2.xxy) + (float3(this.e1234, this.e43, this.e41) * other.group3.xxy) + (float3(this.e4, this.e412, this.e423) * other.group1.xxy) + (float3(this.e42, this.e1234, this.e1234) * other.group3.zyz) + (float3(this.e31, this.scalar, this.scalar) * other.group2.zyz) + (float3(this.e431, this.e4, this.e4) * other.group1.zyz) + (other.group4.yzx * this.group1.zxy) - (float3(other.e4) * this.group1.xyz) - (float3(this.e2, this.e321, this.e321) * other.group4.zyz) - (float3(this.e321, this.e3, this.e1) * other.group4.xxy) - (other.group2.yzx * this.group3.zxy) - (other.group3.yzx * this.group2.zxy) - (other.group1.yzx * this.group4.zxy), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(other.e4) * float4(this.group2.xyz, this.scalar)) + (float4(this.e1234, this.e43, this.e41, other.e12 * this.e412) * float4(other.group4.xxy, 1.0)) + (float4(this.e4, this.e412, this.e423, other.e23 * this.e423) * float4(other.group2.xxy, 1.0)) + (float4(this.e42, this.e1234, this.e1234, this.e1234) * other.group4.zyzw) + (float4(this.e431, this.e4, this.e4, other.e31 * this.e431) * float4(other.group2.zyz, 1.0)) + float4(float3(0.0), -(other.e2 * this.e42) - (other.e3 * this.e43) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group2.yzx * this.group4.zxy), other.scalar * this.e4) - float4((this.group2.zxy * other.group4.yzx), other.e1 * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e4 * -1.0) * float2(this.e321, this.e4), 
            /* e1, e2, e3, e4 */
            float4(other.e4) * float4(this.group3.xyz, this.e1234), 
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e4 * -1.0) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.e4) * float4(this.group2.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Plane other) {
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
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), this.e4 * other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e4) * this.group3.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)), this.e1234 * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group0.xyz) + (this.group4.yzx * other.group0.zxy) - (float3(other.e4) * this.group1.xyz) - (this.group4.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, (this.scalar * other.e4) - (this.e41 * other.e1) - (this.e42 * other.e2) - (this.e43 * other.e3)) * float4(this.group2.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension MultiVector {
    public MultiVector geometric_anti_product(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e1234 * other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar * -1.0) * this.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e4 * other.scalar * -1.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Origin> operator /(Origin lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(GeometricAntiProductInfixPartial<Origin> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Origin geometric_anti_product(AntiScalar other) {
        return Origin(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Flector geometric_anti_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Motor geometric_anti_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * -1.0) * float4(other.group1.xyz, other.e4), 
            /* e23, e31, e12, scalar */
            float4(this.e4) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Scalar geometric_anti_product(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Flector geometric_anti_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e4 * -1.0) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Flector geometric_anti_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4((other.group1.xyz * float3(-1.0)), other.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group0.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e4) * float2(other.e321, other.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e4) * float4((other.group3.xyz * float3(-1.0)), other.e1234), 
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e4) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(this.e4) * float4(other.group2.xyz, other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(GeometricAntiProductInfixPartial<Origin> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public AntiScalar geometric_anti_product(Origin other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Motor geometric_anti_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4 * -1.0) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e4 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Motor geometric_anti_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e4 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e4) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Origin {
    public Horizon geometric_anti_product(Scalar other) {
        return Horizon(
            /* e321 */
            this.e4 * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Plane> operator /(Plane lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(GeometricAntiProductInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Plane geometric_anti_product(AntiScalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Flector geometric_anti_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar * -1.0) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Motor geometric_anti_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e423) - (other.e412 * this.e431), -(other.e4 * this.e431) - (other.e423 * this.e412), -(other.e4 * this.e412) - (other.e431 * this.e423), (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group1.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4((other.e3 * this.e431) + (other.e321 * this.e423), (other.e1 * this.e412) + (other.e321 * this.e431), (other.e2 * this.e423) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.yzxx * this.group0.zxyx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Line geometric_anti_product(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Flector geometric_anti_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Flector geometric_anti_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), other.e43 * this.e412 * -1.0) - (this.group0.xyzy * float4(other.group1.www, other.e42)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e42 * this.e412 * -1.0, other.e43 * this.e423 * -1.0, other.e41 * this.e431 * -1.0, (other.e31 * this.e431) + (other.e12 * this.e412)) + (other.group0.zxyw * this.group0.yzxw) + (this.group0.xyzx * float4(other.group0.www, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public MultiVector geometric_anti_product(MultiVector other) {
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
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Motor geometric_anti_product(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e4 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Motor geometric_anti_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e412 * this.e431 * -1.0, other.e423 * this.e412 * -1.0, other.e431 * this.e423 * -1.0, (other.e431 * this.e431) + (other.e412 * this.e412)) + (other.group0.yzxx * this.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Motor geometric_anti_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e431 * other.e3, this.e412 * other.e1, this.e423 * other.e2, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.zxyx * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiProductInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Plane {
    public Point geometric_anti_product(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Point> operator /(Point lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiProductInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Point geometric_anti_product(AntiScalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Flector geometric_anti_product(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Motor geometric_anti_product(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4 * -1.0) * float4(other.group1.xyz, other.e4), 
            /* e23, e31, e12, scalar */
            float4(-(other.e4 * this.e1) - (other.e412 * this.e2), -(other.e4 * this.e2) - (other.e423 * this.e3), -(other.e4 * this.e3) - (other.e431 * this.e1), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.yzxy * this.group0.zxyy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Scalar geometric_anti_product(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Flector geometric_anti_product(Line other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.group0.zxy * this.group0.yzx), 0.0) - float4((float3(this.e4) * other.group1.xyz), 0.0) - float4((other.group0.yzx * this.group0.zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3)) * float4(other.group0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Flector geometric_anti_product(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group0.zxy * this.group0.yzx) - (float3(this.e4) * other.group1.xyz) - (other.group0.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (other.scalar * this.e4)) * float4(other.group0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), other.e4 * this.e4) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.e1234) * this.group0.xyz) + (other.group2.zxy * this.group0.yzx) - (float3(this.e4) * other.group3.xyz) - (other.group2.yzx * this.group0.zxy)), other.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.e4) * other.group1.xyz) + (other.group4.yzx * this.group0.zxy) - (float3(other.e4) * this.group0.xyz) - (other.group4.zxy * this.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, -(other.scalar * this.e4) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3)) * float4(other.group2.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Motor geometric_anti_product(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Motor geometric_anti_product(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.e4 * -1.0) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e412 * this.e2 * -1.0, other.e423 * this.e3 * -1.0, other.e431 * this.e1 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.yzxx * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Motor geometric_anti_product(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e4 * this.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Point {
    public Horizon geometric_anti_product(Scalar other) {
        return Horizon(
            /* e321 */
            this.e4 * other.scalar * -1.0
        );
    }
}
// Fancy infix trick (first half)
public GeometricAntiProductInfixPartial<Scalar> operator /(Scalar lhs, GeometricAntiProductInfix rhs) {
    return GeometricAntiProductInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Scalar geometric_anti_product(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(GeometricAntiProductInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Scalar geometric_anti_product(DualNum other) {
        return Scalar(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(GeometricAntiProductInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Flector geometric_anti_product(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(GeometricAntiProductInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Line geometric_anti_product(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(GeometricAntiProductInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Motor geometric_anti_product(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(GeometricAntiProductInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public MultiVector geometric_anti_product(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e1234 * this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group2.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e4 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<Scalar> lhs, Origin rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Horizon geometric_anti_product(Origin other) {
        return Horizon(
            /* e321 */
            other.e4 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(GeometricAntiProductInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Point geometric_anti_product(Plane other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(GeometricAntiProductInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.geometric_anti_product(rhs);
}
public extension Scalar {
    public Horizon geometric_anti_product(Point other) {
        return Horizon(
            /* e321 */
            other.e4 * this.scalar
        );
    }
}
