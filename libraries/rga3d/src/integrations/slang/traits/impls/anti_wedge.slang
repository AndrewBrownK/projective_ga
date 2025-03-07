implementing rga3d;
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public DualNum anti_wedge(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.e1234) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Horizon anti_wedge(Horizon other) {
        return Horizon(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Line anti_wedge(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public MultiVector anti_wedge(MultiVector other) {
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
public Origin operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Origin rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Origin anti_wedge(Origin other) {
        return Origin(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Plane anti_wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Point anti_wedge(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<DualNum> operator /(DualNum lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public DualNum anti_wedge(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.e1234) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public DualNum anti_wedge(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiWedgeInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Horizon anti_wedge(Horizon other) {
        return Horizon(
            /* e321 */
            this.e1234 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Line anti_wedge(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.group0.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.scalar * other.e1234) + (this.e1234 * other.scalar), this.e1234 * other.e1234), 
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
public Origin operator /(AntiWedgeInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Origin anti_wedge(Origin other) {
        return Origin(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Plane anti_wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Point anti_wedge(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Flector> operator /(Flector lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Flector anti_wedge(AntiScalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Flector anti_wedge(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Motor anti_wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Motor anti_wedge(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group1.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Point anti_wedge(Line other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Flector anti_wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * other.group4.yzx) - (this.group1.yzx * other.group4.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Scalar anti_wedge(Origin other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Motor anti_wedge(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Scalar anti_wedge(Point other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Horizon> operator /(Horizon lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(AntiWedgeInfixPartial<Horizon> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Horizon anti_wedge(AntiScalar other) {
        return Horizon(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiWedgeInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Horizon anti_wedge(DualNum other) {
        return Horizon(
            /* e321 */
            other.e1234 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Motor anti_wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Point anti_wedge(Line other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Flector anti_wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public MultiVector anti_wedge(MultiVector other) {
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
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Horizon> lhs, Origin rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Scalar anti_wedge(Origin other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Line anti_wedge(Plane other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group0.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Horizon {
    public Scalar anti_wedge(Point other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Line> operator /(Line lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Line anti_wedge(AntiScalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Line anti_wedge(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Point anti_wedge(Flector other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Point anti_wedge(Horizon other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Scalar anti_wedge(Line other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((this.group0.xyz * other.group0.www), 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Point anti_wedge(Plane other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41))
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Motor> operator /(Motor lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.group0.yy, other.e1234, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Flector anti_wedge(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz * this.group0.www), 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e1234) * other.group0.xyz)), other.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43), this.e1234 * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(other.group4.www, other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group2.xyz) + (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group3.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiWedgeInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Origin anti_wedge(Origin other) {
        return Origin(
            /* e4 */
            this.e1234 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Flector anti_wedge(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Point anti_wedge(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<MultiVector> operator /(MultiVector lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiScalar other) {
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar), other.e1234 * this.e1234), 
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Horizon other) {
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
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43), other.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((this.group3.zxy * other.group4.yzx), other.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group4.yzx * this.group4.zxy) - (other.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(other.e1234) * this.group4) + (float4(this.e1234) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e4), 
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
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
public Scalar operator /(AntiWedgeInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Origin> operator /(Origin lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(AntiWedgeInfixPartial<Origin> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Origin {
    public Origin anti_wedge(AntiScalar other) {
        return Origin(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiWedgeInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Origin {
    public Origin anti_wedge(DualNum other) {
        return Origin(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Origin {
    public Scalar anti_wedge(Flector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Origin {
    public Scalar anti_wedge(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiWedgeInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Origin {
    public Origin anti_wedge(Motor other) {
        return Origin(
            /* e4 */
            other.e1234 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Origin {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e4), 
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
public Scalar operator /(AntiWedgeInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Origin {
    public Scalar anti_wedge(Plane other) {
        return Scalar(
            /* scalar */
            this.e4 * other.e321
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Plane> operator /(Plane lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Plane anti_wedge(AntiScalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Plane anti_wedge(DualNum other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Motor anti_wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Line anti_wedge(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group0.xyz
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Point anti_wedge(Line other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Flector anti_wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(-(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Scalar anti_wedge(Origin other) {
        return Scalar(
            /* scalar */
            other.e4 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Line anti_wedge(Plane other) {
        return Line(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Scalar anti_wedge(Point other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Point> operator /(Point lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Point> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Point {
    public Point anti_wedge(AntiScalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Point {
    public Point anti_wedge(DualNum other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Point {
    public Scalar anti_wedge(Flector other) {
        return Scalar(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Point {
    public Scalar anti_wedge(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiWedgeInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Point {
    public Point anti_wedge(Motor other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Point {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2((other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * this.group0, 
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
public Scalar operator /(AntiWedgeInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Point {
    public Scalar anti_wedge(Plane other) {
        return Scalar(
            /* scalar */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Scalar> operator /(Scalar lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(DualNum other) {
        return Scalar(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(Motor other) {
        return Scalar(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(MultiVector other) {
        return Scalar(
            /* scalar */
            other.e1234 * this.scalar
        );
    }
}
