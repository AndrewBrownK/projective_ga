implementing rga3d;
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_orthogonally_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor anti_reject_orthogonally_from(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234 * other.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0 = this.e1234 * other.scalar;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, wedge_g0 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge_g0 * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(wedge_g0 * -1.0) * other.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge_g0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiRejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<DualNum> operator /(DualNum lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * other.scalar * this.scalar, (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual_g1_xyz.yzx * wedge_g1.zxy) - (right_anti_dual_g1_xyz.zxy * wedge_g1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g1_xyz[0] * wedge_g1[3] * -1.0, right_anti_dual_g1_xyz[1] * wedge_g1[3] * -1.0, right_anti_dual_g1_xyz[2] * wedge_g1[3] * -1.0, (right_anti_dual_g1_xyz[0] * wedge_g0[0]) + (right_anti_dual_g1_xyz[1] * wedge_g0[1]) + (right_anti_dual_g1_xyz[2] * wedge_g0[2]) + (wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Horizon other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Line other) {
        float3 wedge_g1 = float3(this.scalar) * other.group1.xyz;
        return Scalar(
            /* scalar */
            (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_orthogonally_from(Motor other) {
        float wedge_g0_w = (this.scalar * other.e1234) + (this.e1234 * other.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.scalar * other.scalar) * other.group0.xyz) - (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * wedge_g1.xyz), (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12) + (wedge_g1[3] * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.scalar * other.e1234) + (this.e1234 * other.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float3 wedge_g3 = float3(this.scalar) * other.group3.xyz;
        float4 wedge_g4 = float4(this.scalar) * other.group4;
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3]) + (this.scalar * other.scalar * other.scalar) - (right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]) - (right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[1] * right_anti_dual_g4[1]) - (wedge_g2[2] * right_anti_dual_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0) + float4((wedge_g2 * float3(right_anti_dual_g4[3])), 0.0) + float4((wedge_g3.yzx * right_anti_dual_g4.zxy), 0.0) - (right_anti_dual_g4.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_orthogonally_from(Point other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_orthogonally_from(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Flector> operator /(Flector lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = (float4(this.e4) * float4(other.group0.xyz, other.e321)) + float4(float3(0.0), (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - (float4(other.e4) * float4(this.group0.xyz, this.e321));
        float3 wedge_g1_xyz = (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0);
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * wedge_g0[1]) - (right_anti_dual_g1_xyz[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)) + float4((right_anti_dual_g1_xyz.zxy * wedge_g1_xyz.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * wedge_g1_xyz.zxy), right_anti_dual_g1_xyz[0] * wedge_g0[0]), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual_g1_xyz * float3(wedge_g0[3])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Origin anti_reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            this.e4 * other.e321 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Point anti_reject_orthogonally_from(Line other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx);
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = (other.group1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(other.scalar) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_anti_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            (right_anti_dual_g0 * float4(float3(wedge_g1[3]), wedge_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) + float4((float3(right_anti_dual_g0[3]) * wedge_g0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g3 = (this.group0.yzx * other.group1.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group0.zx * other.group1.yz), 0.0);
        float4 wedge_g4 = (float4(other.scalar) * this.group1) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx);
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]) - (right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[1] * right_anti_dual_g4[1]) - (wedge_g2[2] * right_anti_dual_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0) + float4((wedge_g2 * float3(right_anti_dual_g4[3])), 0.0) + float4((wedge_g3.yzx * right_anti_dual_g4.zxy), 0.0) - (right_anti_dual_g4.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Origin anti_reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            -(other.e321 * other.e321 * this.e4) - (this.e1 * other.e423 * other.e321) - (this.e2 * other.e431 * other.e321) - (this.e3 * other.e412 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Point other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)) + float4((float3(this.e4) * other.group0.xyz), 0.0) - (float4(other.e4) * float4(this.group0.xyz, this.e321));
        float3 wedge_g1_xyz = (this.group0.yzx * other.group0.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group0.zx * other.group0.yz), 0.0);
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * wedge_g0[1]) - (right_anti_dual_g0_xyz[2] * wedge_g0[2])) + float4((right_anti_dual_g0_xyz.zxy * wedge_g1_xyz.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * wedge_g1_xyz.zxy), right_anti_dual_g0_xyz[0] * wedge_g0[0]), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual_g0_xyz * float3(wedge_g0[3])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector anti_reject_orthogonally_from(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Horizon> operator /(Horizon lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Horizon anti_reject_orthogonally_from(DualNum other) {
        return Horizon(
            /* e321 */
            other.scalar * other.scalar * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Flector anti_reject_orthogonally_from(Flector other) {
        float wedge_g0 = other.e4 * this.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge_g0 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge_g0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Flector anti_reject_orthogonally_from(Motor other) {
        float wedge_g0 = this.e321 * other.scalar;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0 * -1.0) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge_g0 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = this.e321 * other.e4 * -1.0;
        float4 wedge_g4 = float4(float3(0.0), this.e321 * other.scalar);
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Plane anti_reject_orthogonally_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiRejectOrthogonallyFromInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Horizon anti_reject_orthogonally_from(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Line> operator /(Line lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx);
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual_g1_xyz.yzx * wedge_g0.zxy) - (right_anti_dual_g1_xyz.zxy * wedge_g0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g1_xyz[0] * wedge_g0[3] * -1.0, right_anti_dual_g1_xyz[1] * wedge_g0[3] * -1.0, right_anti_dual_g1_xyz[2] * wedge_g0[3] * -1.0, wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(Line other) {
        return Line(
            /* e41, e42, e43 */
            (float3(other.e41 * this.e23) * other.group1.xyz) + (float3(other.e42 * this.e31) * other.group1.xyz) + (float3(other.e43 * this.e12) * other.group1.xyz) + (float3(other.e31 * this.e42, other.e23 * this.e41, other.e23 * this.e41) * other.group1.xyz) + (other.group1.xyz * float3(float2(other.e12 * this.e43), other.e31 * this.e42)) + (other.group1.xyz * other.group1.xyz * this.group0.xyz), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor anti_reject_orthogonally_from(Motor other) {
        float wedge_g0_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float3 wedge_g1_xyz = float3(other.scalar) * this.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.scalar * other.scalar) * this.group0.xyz) - (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1_xyz * float3(other.scalar)), (wedge_g1_xyz[0] * other.e23) + (wedge_g1_xyz[1] * other.e31) + (wedge_g1_xyz[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float3 wedge_g2 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g3 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g4 = float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group1.yzxx);
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]) - (right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[1] * right_anti_dual_g4[1]) - (wedge_g2[2] * right_anti_dual_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0) + float4((wedge_g2 * float3(right_anti_dual_g4[3])), 0.0) + float4((wedge_g3.yzx * right_anti_dual_g4.zxy), 0.0) - (right_anti_dual_g4.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(Point other) {
        float4 wedge_g0 = float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx);
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz.yzx * wedge_g0.zxy) - (right_anti_dual_g0_xyz.zxy * wedge_g0.yzx), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0_xyz[0] * wedge_g0[3] * -1.0, right_anti_dual_g0_xyz[1] * wedge_g0[3] * -1.0, right_anti_dual_g0_xyz[2] * wedge_g0[3] * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectOrthogonallyFromInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Line {
    public Line anti_reject_orthogonally_from(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Motor> operator /(Motor lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(other.scalar) * float3((float2(other.scalar) * this.group0.xy), other.scalar * this.e43)), (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = (this.group1 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual_g1_xyz.yzx * wedge_g1.zxy) - (right_anti_dual_g1_xyz.zxy * wedge_g1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g1_xyz[0] * wedge_g1[3] * -1.0, right_anti_dual_g1_xyz[1] * wedge_g1[3] * -1.0, right_anti_dual_g1_xyz[2] * wedge_g1[3] * -1.0, (right_anti_dual_g1_xyz[0] * wedge_g0[0]) + (right_anti_dual_g1_xyz[1] * wedge_g0[1]) + (right_anti_dual_g1_xyz[2] * wedge_g0[2]) + (wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Scalar anti_reject_orthogonally_from(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Line other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group1.xyz;
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            -float4((right_anti_dual_g0 * float3(other.e41 * this.e23)), 0.0) - float4((right_anti_dual_g0 * float3(other.e42 * this.e31)), 0.0) - float4((right_anti_dual_g0 * float3(other.e43 * this.e12)), 0.0) - float4((right_anti_dual_g0 * float3(other.e23 * this.e41)), 0.0) - float4((right_anti_dual_g0 * float3(other.e31 * this.e42)), 0.0) - float4((right_anti_dual_g0 * float3(other.e12 * this.e43)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g1_xyz[0]) - (right_anti_dual_g0[1] * wedge_g1_xyz[1]) - (right_anti_dual_g0[2] * wedge_g1_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        float3 wedge_g1_xyz = (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.scalar) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1_xyz * float3(other.scalar)), (wedge_g1_xyz[0] * other.e23) + (wedge_g1_xyz[1] * other.e31) + (wedge_g1_xyz[2] * other.e12) + (other.scalar * other.scalar * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = (float3(this.scalar) * other.group2.xyz) + (float3(other.scalar) * this.group0.xyz);
        float3 wedge_g3 = (float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz);
        float4 wedge_g4 = (this.group1 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(this.scalar) * other.group4.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3]) + (this.scalar * other.scalar * other.scalar) - (right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]) - (right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[1] * right_anti_dual_g4[1]) - (wedge_g2[2] * right_anti_dual_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0) + float4((wedge_g2 * float3(right_anti_dual_g4[3])), 0.0) + float4((wedge_g3.yzx * right_anti_dual_g4.zxy), 0.0) - (right_anti_dual_g4.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Point other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual_g0_xyz.yzx * wedge_g1.zxy) - (right_anti_dual_g0_xyz.zxy * wedge_g1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g0_xyz[0] * wedge_g1[3] * -1.0, right_anti_dual_g0_xyz[1] * wedge_g1[3] * -1.0, right_anti_dual_g0_xyz[2] * wedge_g1[3] * -1.0, (right_anti_dual_g0_xyz[0] * wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor anti_reject_orthogonally_from(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<MultiVector> operator /(MultiVector lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * other.scalar * this.scalar, (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Flector other) {
        float wedge_g0_y = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g3 = (other.group0.zxy * this.group1.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group1.zx), 0.0);
        float4 wedge_g4 = (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx);
        float4 right_anti_dual_g0 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g1_xyz[0] * wedge_g1[0]) + (right_anti_dual_g1_xyz[1] * wedge_g1[1]) + (right_anti_dual_g1_xyz[2] * wedge_g1[2]) - (right_anti_dual_g0[0] * wedge_g4[0]) - (right_anti_dual_g0[1] * wedge_g4[1]) - (right_anti_dual_g0[2] * wedge_g4[2]) - (right_anti_dual_g0[3] * wedge_g4[3]), 0.0), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g0 * float4(wedge_g0_y)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * wedge_g2[1]) - (right_anti_dual_g1_xyz[2] * wedge_g2[2])) + float4((right_anti_dual_g1_xyz.zxy * wedge_g3.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * wedge_g3.zxy), right_anti_dual_g1_xyz[0] * wedge_g2[0]), 
            /* e41, e42, e43 */
            (right_anti_dual_g1_xyz.yzx * wedge_g4.zxy) - (right_anti_dual_g1_xyz.zxy * wedge_g4.yzx), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g1_xyz[0] * wedge_g4[3] * -1.0, right_anti_dual_g1_xyz[1] * wedge_g4[3] * -1.0, right_anti_dual_g1_xyz[2] * wedge_g4[3] * -1.0), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual_g1_xyz * float3(wedge_g0_y)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Horizon other) {
        float right_anti_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_anti_dual_g0 * other.e321 * this.scalar * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0 * other.e321 * this.e4), 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Line other) {
        float3 wedge_g3 = float3(this.scalar) * other.group1.xyz;
        float4 wedge_g4 = float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group1.yzxx);
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_anti_dual_g0[0] * wedge_g3[0]) - (right_anti_dual_g0[1] * wedge_g3[1]) - (right_anti_dual_g0[2] * wedge_g3[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0 * float3(wedge_g4[3])), -(right_anti_dual_g0[0] * wedge_g4[0]) - (right_anti_dual_g0[1] * wedge_g4[1]) - (right_anti_dual_g0[2] * wedge_g4[2])), 
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Motor other) {
        float wedge_g0_y = (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g3 = (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz);
        float4 wedge_g4 = (other.group1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(other.scalar) * this.group4.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_anti_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g0[3] * other.scalar * this.scalar) - (wedge_g3[0] * right_anti_dual_g0[0]) - (wedge_g3[1] * right_anti_dual_g0[1]) - (wedge_g3[2] * right_anti_dual_g0[2]), wedge_g0_y * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g0 * float4(float3(wedge_g4[3]), wedge_g1[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g4[0]) - (right_anti_dual_g0[1] * wedge_g4[1]) - (right_anti_dual_g0[2] * wedge_g4[2])) + float4((float3(right_anti_dual_g0[3]) * wedge_g1.xyz), 0.0), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3] * other.scalar) * this.group2.xyz) + (float3(right_anti_dual_g0[3] * this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            wedge_g3 * float3(right_anti_dual_g0[3]), 
            /* e423, e431, e412, e321 */
            wedge_g4 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float3 wedge_g2 = (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g3 = (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + float3(float2(0.0), (other.e2 * this.e1) - (other.e1 * this.e2)) + float3((other.group1.zx * this.group1.yz), 0.0) - float3((other.group1.yz * this.group1.zx), 0.0);
        float4 wedge_g4 = (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12) - (other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((other.group2.yzx * this.group1.zxy), 0.0) + float4((this.group2.yzx * other.group1.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group1.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group1.yzxx);
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3]) + (other.scalar * other.scalar * this.scalar) - (right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]) - (right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[1] * right_anti_dual_g4[1]) - (wedge_g2[2] * right_anti_dual_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0) + float4((wedge_g2 * float3(right_anti_dual_g4[3])), 0.0) + float4((wedge_g3.yzx * right_anti_dual_g4.zxy), 0.0) - (right_anti_dual_g4.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Plane other) {
        float right_anti_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_anti_dual_g0 * this.scalar * other.e321 * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (right_anti_dual_g0 * this.e1 * other.e423) + (right_anti_dual_g0 * this.e2 * other.e431) + (right_anti_dual_g0 * this.e3 * other.e412) + (right_anti_dual_g0 * this.e4 * other.e321)), 
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
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Point other) {
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g3 = (this.group1.yzx * other.group0.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group1.zx * other.group0.yz), 0.0);
        float4 wedge_g4 = float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx);
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g0_xyz[0] * wedge_g1[0]) + (right_anti_dual_g0_xyz[1] * wedge_g1[1]) + (right_anti_dual_g0_xyz[2] * wedge_g1[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * wedge_g2[1]) - (right_anti_dual_g0_xyz[2] * wedge_g2[2])) + float4((right_anti_dual_g0_xyz.zxy * wedge_g3.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * wedge_g3.zxy), right_anti_dual_g0_xyz[0] * wedge_g2[0]), 
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz.yzx * wedge_g4.zxy) - (right_anti_dual_g0_xyz.zxy * wedge_g4.yzx), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0_xyz[0] * wedge_g4[3] * -1.0, right_anti_dual_g0_xyz[1] * wedge_g4[3] * -1.0, right_anti_dual_g0_xyz[2] * wedge_g4[3] * -1.0), 
            /* e423, e431, e412, e321 */
            -float4((right_anti_dual_g0_xyz * float3(this.e423 * other.e1)), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e431 * other.e2)), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e412 * other.e3)), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e321 * other.e4)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_orthogonally_from(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar * other.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group4
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Origin> operator /(Origin lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(DualNum other) {
        return Origin(
            /* e4 */
            other.scalar * other.scalar * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Flector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4(this.e4) * float4(other.group0.xyz, other.e321);
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * wedge_g0[0]) - (right_anti_dual_g1_xyz[1] * wedge_g0[1]) - (right_anti_dual_g1_xyz[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual_g1_xyz * float3(wedge_g0[3])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Line other) {
        float3 wedge_g0_xyz = float3(this.e4) * other.group1.xyz;
        return Origin(
            /* e4 */
            (wedge_g0_xyz[0] * other.e23) + (wedge_g0_xyz[1] * other.e31) + (wedge_g0_xyz[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Flector anti_reject_orthogonally_from(Motor other) {
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (wedge_g1_xyz[0] * other.e23) + (wedge_g1_xyz[1] * other.e31) + (wedge_g1_xyz[2] * other.e12) + (other.scalar * other.scalar * this.e4)), 
            /* e423, e431, e412, e321 */
            float4((wedge_g1_xyz * float3(other.scalar)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = other.e321 * this.e4;
        float3 wedge_g2 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g4_xyz = float3(this.e4) * other.group3.xyz;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (other.scalar * other.scalar * this.e4) - (wedge_g0_y * other.e321) - (right_anti_dual_g2[0] * wedge_g4_xyz[0]) - (right_anti_dual_g2[1] * wedge_g4_xyz[1]) - (right_anti_dual_g2[2] * wedge_g4_xyz[2]) - (right_anti_dual_g4_xyz[0] * wedge_g2[0]) - (right_anti_dual_g4_xyz[1] * wedge_g2[1]) - (right_anti_dual_g4_xyz[2] * wedge_g2[2])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (right_anti_dual_g4_xyz.yzx * wedge_g4_xyz.zxy) - (right_anti_dual_g4_xyz.zxy * wedge_g4_xyz.yzx), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((right_anti_dual_g4_xyz * float3(wedge_g0_y)) + (wedge_g4_xyz * float3(other.scalar))), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            this.e4 * other.e321 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Point other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Origin(
            /* e4 */
            -(right_anti_dual_g0_xyz[0] * wedge_g0[0]) - (right_anti_dual_g0_xyz[1] * wedge_g0[1]) - (right_anti_dual_g0_xyz[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin anti_reject_orthogonally_from(Scalar other) {
        return Origin(
            /* e4 */
            this.e4 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Plane> operator /(Plane lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(DualNum other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector anti_reject_orthogonally_from(Flector other) {
        float wedge_g0 = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge_g0 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge_g0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0[3] * -1.0) * other.group1.xyz), (wedge_g0[0] * other.e23) + (wedge_g0[1] * other.e31) + (wedge_g0[2] * other.e12)), 
            /* e423, e431, e412, e321 */
            wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 wedge_g4 = float4(other.scalar) * this.group0;
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            -float4((float3(this.e321 * other.e4) * other.group0.xyz), 0.0) - float4((other.group0.xyz * other.group0.xyz * this.group0.xyz), 0.0) - float4((float3(other.e1) * this.group0.yxx * other.group0.yyz), 0.0) - float4((float3(other.e3) * this.group0.zzy * other.group0.xyy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectOrthogonallyFromInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane anti_reject_orthogonally_from(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Point> operator /(Point lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(DualNum other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Flector anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = (float4(this.e4) * float4(other.group0.xyz, other.e321)) + float4(float3(0.0), (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((float3(other.e4) * this.group0.xyz), 0.0);
        float3 wedge_g1_xyz = (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0);
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * wedge_g0[1]) - (right_anti_dual_g1_xyz[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)) + float4((right_anti_dual_g1_xyz.zxy * wedge_g1_xyz.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * wedge_g1_xyz.zxy), right_anti_dual_g1_xyz[0] * wedge_g0[0]), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual_g1_xyz * float3(wedge_g0[3])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Origin anti_reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(Line other) {
        float4 wedge_g0 = float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx);
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0 * float3(wedge_g0[3])), -(right_anti_dual_g0[0] * wedge_g0[0]) - (right_anti_dual_g0[1] * wedge_g0[1]) - (right_anti_dual_g0[2] * wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Flector anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_anti_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            (right_anti_dual_g0 * float4(float3(wedge_g1[3]), wedge_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * wedge_g1[0]) - (right_anti_dual_g0[1] * wedge_g1[1]) - (right_anti_dual_g0[2] * wedge_g1[2])) + float4((float3(right_anti_dual_g0[3]) * wedge_g0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float wedge_g0_y = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g3 = (other.group1.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group1.yz * this.group0.zx), 0.0);
        float4 wedge_g4 = float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx);
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]) - (right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[1] * right_anti_dual_g4[1]) - (wedge_g2[2] * right_anti_dual_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0) + float4((wedge_g2 * float3(right_anti_dual_g4[3])), 0.0) + float4((wedge_g3.yzx * right_anti_dual_g4.zxy), 0.0) - (right_anti_dual_g4.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Origin anti_reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            -(other.e321 * other.e321 * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(Point other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g1 = (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0);
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Point(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * wedge_g0[1]) - (right_anti_dual_g0_xyz[2] * wedge_g0[2])) + float4((right_anti_dual_g0_xyz.zxy * wedge_g1.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * wedge_g1.zxy), right_anti_dual_g0_xyz[0] * wedge_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectOrthogonallyFromInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Point {
    public Point anti_reject_orthogonally_from(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectOrthogonallyFromInfixPartial<Scalar> operator /(Scalar lhs, AntiRejectOrthogonallyFromInfix rhs) {
    return AntiRejectOrthogonallyFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public DualNum anti_reject_orthogonally_from(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Motor anti_reject_orthogonally_from(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual_g1_xyz.yzx * wedge_g1.zxy) - (right_anti_dual_g1_xyz.zxy * wedge_g1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g1_xyz[0] * wedge_g1[3] * -1.0, right_anti_dual_g1_xyz[1] * wedge_g1[3] * -1.0, right_anti_dual_g1_xyz[2] * wedge_g1[3] * -1.0, (right_anti_dual_g1_xyz[0] * wedge_g0[0]) + (right_anti_dual_g1_xyz[1] * wedge_g0[1]) + (right_anti_dual_g1_xyz[2] * wedge_g0[2]) + (wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Line other) {
        float3 wedge_g1 = float3(this.scalar) * other.group1.xyz;
        return Scalar(
            /* scalar */
            (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Motor anti_reject_orthogonally_from(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.scalar) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * wedge_g1.xyz), (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12) + (wedge_g1[3] * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public MultiVector anti_reject_orthogonally_from(MultiVector other) {
        float2 wedge_g0 = float2(this.scalar) * other.group0.xy;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float3 wedge_g3 = float3(this.scalar) * other.group3.xyz;
        float4 wedge_g4 = float4(this.scalar) * other.group4;
        float4 right_anti_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0[0] * other.scalar) + (right_anti_dual_g4[0] * wedge_g1[0]) + (right_anti_dual_g4[1] * wedge_g1[1]) + (right_anti_dual_g4[2] * wedge_g1[2]) + (right_anti_dual_g4[3] * wedge_g1[3]) - (right_anti_dual_g2[0] * wedge_g3[0]) - (right_anti_dual_g2[1] * wedge_g3[1]) - (right_anti_dual_g2[2] * wedge_g3[2]) - (right_anti_dual_g1[0] * wedge_g4[0]) - (right_anti_dual_g1[1] * wedge_g4[1]) - (right_anti_dual_g1[2] * wedge_g4[2]) - (right_anti_dual_g1[3] * wedge_g4[3]), wedge_g0[1] * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(wedge_g0[1])) + (wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(right_anti_dual_g2[0] * wedge_g4[0]) - (right_anti_dual_g2[1] * wedge_g4[1]) - (right_anti_dual_g2[2] * wedge_g4[2]) - (wedge_g2[1] * right_anti_dual_g4[1]) - (wedge_g2[2] * right_anti_dual_g4[2])) + float4((right_anti_dual_g2 * float3(wedge_g4[3])), 0.0) + float4((wedge_g2 * float3(right_anti_dual_g4[3])), 0.0) + float4((wedge_g3.yzx * right_anti_dual_g4.zxy), 0.0) - (right_anti_dual_g4.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(wedge_g0[1])) + (wedge_g2 * float3(other.scalar)) + float3(float2(0.0), (right_anti_dual_g4[0] * wedge_g4[1]) - (right_anti_dual_g4[1] * wedge_g4[0])) + float3((right_anti_dual_g4.yz * wedge_g4.zx), 0.0) - float3((right_anti_dual_g4.zx * wedge_g4.yz), 0.0), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_anti_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_anti_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_anti_dual_g4 * float4(wedge_g0[1])) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Point other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectOrthogonallyFromInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_orthogonally_from(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
