implementing rga3d;
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiScalar> operator /(AntiScalar lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(Motor other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(MultiVector other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<DualNum> operator /(DualNum lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WedgeInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Flector wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(WedgeInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Horizon wedge(Horizon other) {
        return Horizon(
            /* e321 */
            this.scalar * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Line wedge(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * other.group0.xyz), (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
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
public Origin operator /(WedgeInfixPartial<DualNum> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Origin wedge(Origin other) {
        return Origin(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Plane wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(WedgeInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Point wedge(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WedgeInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public DualNum wedge(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Flector> operator /(Flector lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(this.e4) * float4(other.group0.xyz, other.e321)) + float4(float3(0.0), (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - (float4(other.e4) * float4(this.group0.xyz, this.e321)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public AntiScalar wedge(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Plane wedge(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            (other.group1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(other.scalar) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group0.zx * other.group1.yz), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(Origin other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * -1.0) * float4(this.group0.xyz, this.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(Point other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)) + float4((float3(this.e4) * other.group0.xyz), 0.0) - (float4(other.e4) * float4(this.group0.xyz, this.e321)), 
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Horizon> operator /(Horizon lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(WedgeInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Horizon {
    public Horizon wedge(DualNum other) {
        return Horizon(
            /* e321 */
            other.scalar * this.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Horizon {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(WedgeInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Horizon {
    public Horizon wedge(Motor other) {
        return Horizon(
            /* e321 */
            other.scalar * this.e321
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Horizon {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e4 * this.e321 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Horizon> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension Horizon {
    public AntiScalar wedge(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Horizon {
    public AntiScalar wedge(Point other) {
        return AntiScalar(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(WedgeInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Horizon {
    public Horizon wedge(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Line> operator /(Line lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Line wedge(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(Flector other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(Origin other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Line wedge(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Motor> operator /(Motor lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            (this.group1 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(WedgeInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Horizon wedge(Horizon other) {
        return Horizon(
            /* e321 */
            this.scalar * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group3.xyz), 
            /* e423, e431, e412, e321 */
            (this.group1 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(this.scalar) * other.group4.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(Origin other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(other.e4) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Plane wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(Point other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<MultiVector> operator /(MultiVector lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
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
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group1.zx), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group2.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e4 * other.e321), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group2.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group3.xyz), 
            /* e423, e431, e412, e321 */
            (other.group1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(other.scalar) * this.group4.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + float3(float2(0.0), (other.e2 * this.e1) - (other.e1 * this.e2)) + float3((other.group1.zx * this.group1.yz), 0.0) - float3((other.group1.yz * this.group1.zx), 0.0), 
            /* e423, e431, e412, e321 */
            (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3) - (this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((other.group2.yzx * this.group1.zxy), 0.0) + float4((this.group2.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group2.zxy, this.e23)) - (this.group1.yzxx * float4(other.group2.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e321 * other.e4 * -1.0), 
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
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group1.zx * other.group0.yz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group2.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Scalar other) {
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
public WedgeInfixPartial<Origin> operator /(Origin lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(WedgeInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public Origin wedge(DualNum other) {
        return Origin(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public Motor wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public AntiScalar wedge(Horizon other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public Plane wedge(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public Flector wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e4) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public MultiVector wedge(MultiVector other) {
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
public AntiScalar operator /(WedgeInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public Line wedge(Point other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(WedgeInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Origin {
    public Origin wedge(Scalar other) {
        return Origin(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Plane> operator /(Plane lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(DualNum other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(Motor other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(Origin other) {
        return AntiScalar(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(Point other) {
        return AntiScalar(
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Point> operator /(Point lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(WedgeInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public Point wedge(DualNum other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public Motor wedge(Flector other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(this.e4) * float4(other.group0.xyz, other.e321)) + float4(float3(0.0), (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((float3(other.e4) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public AntiScalar wedge(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public Plane wedge(Line other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public Flector wedge(Motor other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group1.yz * this.group0.zx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Point> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public Line wedge(Origin other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public Line wedge(Point other) {
        return Line(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(WedgeInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Point {
    public Point wedge(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Scalar> operator /(Scalar lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WedgeInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Flector wedge(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(WedgeInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Horizon wedge(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Line wedge(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Motor wedge(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public MultiVector wedge(MultiVector other) {
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
public Origin operator /(WedgeInfixPartial<Scalar> lhs, Origin rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Origin wedge(Origin other) {
        return Origin(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Plane wedge(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(WedgeInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Point wedge(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WedgeInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Scalar wedge(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
