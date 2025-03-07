implementing rga3d;
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_via_horizon_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public Motor anti_reject_via_horizon_from(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.e1234 * other.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0 = this.e1234 * other.scalar;
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, wedge_g0 * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge_g0 * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge_g0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_via_horizon_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<DualNum> operator /(DualNum lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_via_horizon_from(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(other.group0.xyz, 0.0).wwwx * float4(wedge_g1.xyz, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (right_dual_g0[3] * wedge_g1[3])) - (wedge_g1.wwwx * float4(other.group0.xyz, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_via_horizon_from(Horizon other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_via_horizon_from(Line other) {
        float3 wedge_g1 = float3(this.scalar) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(this.group0.xx, this.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group0.xyz, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_dual_g0.xyz)), right_dual_g0[3] * wedge_g0[3]), 
            /* e23, e31, e12, scalar */
            (wedge_g1 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.scalar * other.e1234) + (this.e1234 * other.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float3 wedge_g3 = float3(this.scalar) * other.group3.xyz;
        float4 wedge_g4 = float4(this.scalar) * other.group4;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (this.scalar * pow(other.scalar, 2)) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other.e3), (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other.e1), (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other.e2), -(right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) + (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((wedge_g3.zxy * other.group1.yzx), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) - (float3(wedge_g4[3]) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0_y) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Scalar anti_reject_via_horizon_from(Point other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public DualNum anti_reject_via_horizon_from(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * float2(this.scalar * other.scalar, this.e1234 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Flector> operator /(Flector lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group0, 
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423));
        float3 wedge_g1_xyz = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0)) + float4((wedge_g1_xyz.yzx * other.group0.zxy), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((wedge_g1_xyz.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.group0.xyz * wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Origin anti_reject_via_horizon_from(Horizon other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Point anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0 * float3(wedge_g0[3])), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) - (wedge_g0.yzxx * float4(float3(0.0), right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g1 = float4((this.e4 * other.e23) + (this.e423 * other.scalar), (this.e4 * other.e31) + (this.e431 * other.scalar), (this.e4 * other.e12) + (this.e412 * other.scalar), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((this.group0.zxy * other.group0.yzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(right_dual_g0[3]) * float4(other.scalar) * this.group0) + float4((float3(wedge_g1[3]) * right_dual_g0.xyz), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) - (wedge_g1.yzxx * float4(float3(0.0), right_dual_g0[0])), 
            /* e423, e431, e412, e321 */
            wedge_g1 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g3 = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 wedge_g4 = float4((this.e3 * other.e42) + (this.e4 * other.e23), (this.e1 * other.e43) + (this.e4 * other.e31), (this.e2 * other.e41) + (this.e4 * other.e12), -(this.e2 * other.e31) - (this.e3 * other.e12)) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.zxy, other.e23));
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other.e3), (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other.e1), (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other.e2), -(right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) + (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((wedge_g3.zxy * other.group1.yzx), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) - (float3(wedge_g4[3]) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0_y) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Origin anti_reject_via_horizon_from(Plane other) {
        return Origin(
            /* e4 */
            -(other.e321 * other.e321 * this.e4) - (this.e1 * other.e423 * other.e321) - (this.e2 * other.e431 * other.e321) - (this.e3 * other.e412 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(Point other) {
        float4 wedge_g0 = float4((float3(this.e4) * other.group0.xyz), -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423));
        float3 wedge_g1_xyz = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((wedge_g1_xyz.yzx * other.group0.zxy) - (wedge_g1_xyz.zxy * other.group0.yzx)), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)), 
            /* e423, e431, e412, e321 */
            float4((other.group0.xyz * wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group0, 
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Horizon> operator /(Horizon lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(AntiRejectViaHorizonFromInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Horizon {
    public Horizon anti_reject_via_horizon_from(DualNum other) {
        return Horizon(
            /* e321 */
            other.scalar * other.scalar * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Horizon {
    public Flector anti_reject_via_horizon_from(Flector other) {
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
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Horizon {
    public Flector anti_reject_via_horizon_from(Motor other) {
        float wedge_g0 = this.e321 * other.scalar;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0) * right_dual_g0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge_g0 * right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Horizon {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float2 wedge_g0 = float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0);
        float4 wedge_g4 = float4(float3(0.0), this.e321 * other.scalar);
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0[0] * other.scalar) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0[1] * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0[1])) + float4((right_dual_g2 * float3(wedge_g4[3])), right_dual_g2[2] * wedge_g4[2] * -1.0) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4(float3(0.0), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0[1])) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            float3(wedge_g4[3]) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0[1]) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectViaHorizonFromInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Horizon {
    public Plane anti_reject_via_horizon_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiRejectViaHorizonFromInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Horizon {
    public Horizon anti_reject_via_horizon_from(Scalar other) {
        return Horizon(
            /* e321 */
            this.e321 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Line> operator /(Line lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3_pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3_pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2]) - (right_dual_g0[3] * wedge_g0[3])) - (wedge_g0.wwwx * float4(other.group0.xyz, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Motor anti_reject_via_horizon_from(Motor other) {
        float wedge_g0_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g1 = float4((this.group1.xyz * other.group1.www), 0.0);
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(wedge_g0_w) * right_dual_g0.xyz) + (float3(right_dual_g0[3]) * float3(other.scalar) * this.group0.xyz)), wedge_g0_w * right_dual_g0[3]), 
            /* e23, e31, e12, scalar */
            (wedge_g1 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float3 wedge_g2 = float3(other.scalar) * this.group0.xyz;
        float3 wedge_g3 = float3(other.scalar) * this.group1.xyz;
        float4 wedge_g4 = float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group1.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other.e3), (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other.e1), (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other.e2), -(right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) + (right_dual_g1 * float4(wedge_g0_y)) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((wedge_g3.zxy * other.group1.yzx), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) - (float3(wedge_g4[3]) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0_y) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(Point other) {
        float4 wedge_g0 = float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return Line(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            float3(wedge_g0[3]) * other.group0.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3_pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3_pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Motor> operator /(Motor lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * float4(other.group0.xx, other.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            float4_pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4((other.e4 * this.e23) + (other.e423 * this.scalar), (other.e4 * this.e31) + (other.e431 * this.scalar), (other.e4 * this.e12) + (other.e412 * this.scalar), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(other.group0.xyz, 0.0).wwwx * float4(wedge_g1.xyz, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (right_dual_g0[3] * wedge_g1[3])) - (wedge_g1.wwwx * float4(other.group0.xyz, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Scalar anti_reject_via_horizon_from(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Line other) {
        float3 wedge_g1_xyz = other.group1.xyz * this.group1.www;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((right_dual_g0 * float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual_g0[0] * wedge_g1_xyz[0]) - (right_dual_g0[1] * wedge_g1_xyz[1]) - (right_dual_g0[2] * wedge_g1_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = (float4(other.scalar) * this.group0) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        float4 wedge_g1 = float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.scalar);
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_dual_g0.xyz)), right_dual_g0[3] * wedge_g0[3]), 
            /* e23, e31, e12, scalar */
            (wedge_g1 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = (float3(this.scalar) * other.group2.xyz) + (float3(other.scalar) * this.group0.xyz);
        float3 wedge_g3 = (float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz);
        float4 wedge_g4 = float4((this.e23 * other.e4) + (this.scalar * other.e423), (this.e31 * other.e4) + (this.scalar * other.e431), (this.e12 * other.e4) + (this.scalar * other.e412), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (other.group1.yzxx * float4(this.group0.zxy, this.e23));
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (this.scalar * pow(other.scalar, 2)) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other.e3), (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other.e1), (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other.e2), -(right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) + (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((wedge_g3.zxy * other.group1.yzx), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) - (float3(wedge_g4[3]) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0_y) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        return Scalar(
            /* scalar */
            this.scalar * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Point other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23));
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(other.group0.xyz, 0.0).wwwx * float4(wedge_g1.xyz, wedge_g0[0])) + float4((other.group0.xyz * wedge_g1.www * float3(-1.0)), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4_pow(float4(other.scalar), 2) * this.group0, 
            /* e23, e31, e12, scalar */
            float4_pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<MultiVector> operator /(MultiVector lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group1, 
            /* e41, e42, e43 */
            float3_pow(float3(other.scalar), 2) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3_pow(float3(other.scalar), 2) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Flector other) {
        float wedge_g0_y = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g3 = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float4 wedge_g4 = float4((other.e3 * this.e42) + (other.e4 * this.e23), (other.e1 * this.e43) + (other.e4 * this.e31), (other.e2 * this.e41) + (other.e4 * this.e12), -(other.e2 * this.e31) - (other.e3 * this.e12)) + (float4(this.scalar) * other.group1) - (other.group0.yzxx * float4(this.group2.zxy, this.e23));
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_dual_g0[0] * wedge_g4[0]) - (right_dual_g0[1] * wedge_g4[1]) - (right_dual_g0[2] * wedge_g4[2]) - (right_dual_g0[3] * wedge_g4[3]), 0.0), 
            /* e1, e2, e3, e4 */
            (right_dual_g0 * float4(wedge_g0_y)) + float4((wedge_g3.yzx * other.group0.zxy), -(wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) - float4((wedge_g3.zxy * other.group0.yzx), 0.0), 
            /* e41, e42, e43 */
            (wedge_g4.zxy * other.group0.yzx) - (wedge_g4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            float3(wedge_g4[3]) * other.group0.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(wedge_g0_y) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Horizon other) {
        float right_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_dual_g0 * other.e321 * this.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual_g0 * other.e321 * this.e4), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Line other) {
        float3 wedge_g3 = float3(this.scalar) * other.group1.xyz;
        float4 wedge_g4 = float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0 * float3(wedge_g4[3])), -(right_dual_g0[1] * wedge_g4[1]) - (right_dual_g0[2] * wedge_g4[2])) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g0[0])), 
            /* e41, e42, e43 */
            right_dual_g0 * float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Motor other) {
        float wedge_g0_y = (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g3 = (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz);
        float4 wedge_g4 = float4((other.e23 * this.e4) + (other.scalar * this.e423), (other.e31 * this.e4) + (other.scalar * this.e431), (other.e12 * this.e4) + (other.scalar * this.e412), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((other.group0.yzx * this.group1.zxy), other.scalar * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_dual_g0[3] * other.scalar * this.scalar) - (wedge_g3[0] * right_dual_g0[0]) - (wedge_g3[1] * right_dual_g0[1]) - (wedge_g3[2] * right_dual_g0[2]), wedge_g0_y * right_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            (right_dual_g0 * float4(wedge_g4.www, wedge_g1[3])) + (float4(wedge_g1[0], wedge_g1[1], wedge_g1[2], 1.0) * float4(right_dual_g0.www, -(right_dual_g0[1] * wedge_g4[1]) - (right_dual_g0[2] * wedge_g4[2]))) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g0[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * right_dual_g0.xyz) + (float3(right_dual_g0[3]) * ((float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group0.xyz))), 
            /* e23, e31, e12 */
            wedge_g3 * float3(right_dual_g0[3]), 
            /* e423, e431, e412, e321 */
            wedge_g4 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float3 wedge_g2 = (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g3 = (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g4 = float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12) - (other.e12 * this.e3)) + (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) - (this.group1.yzxx * float4(other.group2.zxy, other.e23)) - float4((this.group2.zxy * other.group1.yzx), other.e31 * this.e2);
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (other.scalar * other.scalar * this.scalar) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other.e3), (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other.e1), (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other.e2), -(right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) + (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((wedge_g3.zxy * other.group1.yzx), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) - (float3(wedge_g4[3]) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0_y) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Plane other) {
        float right_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_dual_g0 * this.scalar * other.e321, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (right_dual_g0 * this.e1 * other.e423) + (right_dual_g0 * this.e2 * other.e431) + (right_dual_g0 * this.e3 * other.e412) + (right_dual_g0 * this.e4 * other.e321)), 
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Point other) {
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g3 = (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float4 wedge_g4 = float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group2.zxy, this.e23));
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g3.yzx * other.group0.zxy) - (wedge_g3.zxy * other.group0.yzx)), -(wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)), 
            /* e41, e42, e43 */
            (wedge_g4.zxy * other.group0.yzx) - (wedge_g4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            float3(wedge_g4[3]) * other.group0.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2_pow(float2(other.scalar), 2) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group1, 
            /* e41, e42, e43 */
            float3_pow(float3(other.scalar), 2) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3_pow(float3(other.scalar), 2) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group4
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Origin> operator /(Origin lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Origin anti_reject_via_horizon_from(DualNum other) {
        return Origin(
            /* e4 */
            other.scalar * other.scalar * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Flector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(this.e4) * float4(other.group0.xyz, other.e321);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3) - (wedge_g0[3] * other.e321)), 
            /* e423, e431, e412, e321 */
            float4((other.group0.xyz * wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Origin anti_reject_via_horizon_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Origin anti_reject_via_horizon_from(Line other) {
        float3 wedge_g0_xyz = float3(this.e4) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Origin(
            /* e4 */
            -(right_dual_g0[0] * wedge_g0_xyz[0]) - (right_dual_g0[1] * wedge_g0_xyz[1]) - (right_dual_g0[2] * wedge_g0_xyz[2])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Flector anti_reject_via_horizon_from(Motor other) {
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (right_dual_g0[3] * other.scalar * this.e4) - (wedge_g1_xyz[0] * right_dual_g0[0]) - (wedge_g1_xyz[1] * right_dual_g0[1]) - (wedge_g1_xyz[2] * right_dual_g0[2])), 
            /* e423, e431, e412, e321 */
            float4((wedge_g1_xyz * right_dual_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float2 wedge_g0 = float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0);
        float3 wedge_g2 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g4_xyz = float3(this.e4) * other.group3.xyz;
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, wedge_g0[1] * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (other.scalar * other.scalar * this.e4) - (wedge_g0[1] * other.e321) - (right_dual_g2[0] * wedge_g4_xyz[0]) - (right_dual_g2[1] * wedge_g4_xyz[1]) - (right_dual_g2[2] * wedge_g4_xyz[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0[1])) + (wedge_g2 * float3(other.scalar)) + (wedge_g4_xyz.zxy * other.group1.yzx) - (wedge_g4_xyz.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((wedge_g4_xyz * float3(other.scalar)) + (float3(wedge_g0[1]) * other.group1.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Origin anti_reject_via_horizon_from(Plane other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Origin anti_reject_via_horizon_from(Point other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        return Origin(
            /* e4 */
            -(wedge_g0[0] * other.e1) - (wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public Origin anti_reject_via_horizon_from(Scalar other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Plane> operator /(Plane lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Plane anti_reject_via_horizon_from(DualNum other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(other.scalar * this.e423, other.scalar * this.e431, other.scalar * this.e412, other.scalar * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Flector anti_reject_via_horizon_from(Flector other) {
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
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Flector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0[3]) * right_dual_g0.xyz), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) - (wedge_g0.yzxx * float4(float3(0.0), right_dual_g0[0])), 
            /* e423, e431, e412, e321 */
            wedge_g0 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 wedge_g4 = float4(other.scalar) * this.group0;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0_y)) + float4((right_dual_g2 * float3(wedge_g4[3])), right_dual_g2[2] * wedge_g4[2] * -1.0) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4(float3(0.0), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            float3(wedge_g4[3]) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0_y) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Plane anti_reject_via_horizon_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiRejectViaHorizonFromInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Plane {
    public Plane anti_reject_via_horizon_from(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(this.e423 * other.scalar, this.e431 * other.scalar, this.e412 * other.scalar, this.e321 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Point> operator /(Point lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Point anti_reject_via_horizon_from(DualNum other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, other.scalar * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Flector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = (float4((float3(other.e4) * this.group0.xyz), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.wwwx * float4(other.group0.xyz, other.e423));
        float3 wedge_g1_xyz = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0)) + float4((wedge_g1_xyz.yzx * other.group0.zxy), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3)) - float4((wedge_g1_xyz.zxy * other.group0.yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4((other.group0.xyz * wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Origin anti_reject_via_horizon_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Point anti_reject_via_horizon_from(Line other) {
        float4 wedge_g0 = float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0 * float3(wedge_g0[3])), -(right_dual_g0[1] * wedge_g0[1]) - (right_dual_g0[2] * wedge_g0[2])) - (wedge_g0.yzxx * float4(float3(0.0), right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Flector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g1 = float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23));
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(right_dual_g0[3]) * float4(other.scalar) * this.group0) + float4((float3(wedge_g1[3]) * right_dual_g0.xyz), -(right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])) - (wedge_g1.yzxx * float4(float3(0.0), right_dual_g0[0])), 
            /* e423, e431, e412, e321 */
            wedge_g1 * float4(right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 wedge_g1 = float4(other.scalar) * this.group0;
        float3 wedge_g2 = (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g3 = (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        float4 wedge_g4 = float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.zxy, other.e23));
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other.e3), (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other.e1), (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other.e2), -(right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) + (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((wedge_g3.zxy * other.group1.yzx), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) - (float3(wedge_g4[3]) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0_y) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Origin anti_reject_via_horizon_from(Plane other) {
        return Origin(
            /* e4 */
            -(other.e321 * other.e321 * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Point anti_reject_via_horizon_from(Point other) {
        float3 wedge_g0 = (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz);
        float3 wedge_g1 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((wedge_g1.yzx * other.group0.zxy) - (wedge_g1.zxy * other.group0.yzx)), -(wedge_g0[1] * other.e2) - (wedge_g0[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Point anti_reject_via_horizon_from(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.e1 * other.scalar, this.e2 * other.scalar, this.e3 * other.scalar, this.e4 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiRejectViaHorizonFromInfixPartial<Scalar> operator /(Scalar lhs, AntiRejectViaHorizonFromInfix rhs) {
    return AntiRejectViaHorizonFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public DualNum anti_reject_via_horizon_from(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar) * float2(other.scalar * this.scalar, other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(other.group0.xyz, 0.0).wwwx * float4(wedge_g1.xyz, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]) - (right_dual_g0[3] * wedge_g1[3])) - (wedge_g1.wwwx * float4(other.group0.xyz, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Line other) {
        float3 wedge_g1 = float3(this.scalar) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Motor anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual_g0[3]) * wedge_g0.xyz) + (float3(wedge_g0[3]) * right_dual_g0.xyz)), right_dual_g0[3] * wedge_g0[3]), 
            /* e23, e31, e12, scalar */
            (wedge_g1 * float4(right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * wedge_g1[0]) - (right_dual_g0[1] * wedge_g1[1]) - (right_dual_g0[2] * wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float2 wedge_g0 = float2(this.scalar) * other.group0.xy;
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        float3 wedge_g3 = float3(this.scalar) * other.group3.xyz;
        float4 wedge_g4 = float4(this.scalar) * other.group4;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0[0] * other.scalar) + (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0[1] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2[0] * wedge_g4[3]) + (wedge_g3[1] * other.e3), (right_dual_g2[1] * wedge_g4[3]) + (wedge_g3[2] * other.e1), (right_dual_g2[2] * wedge_g4[3]) + (wedge_g3[0] * other.e2), -(right_dual_g2[2] * wedge_g4[2]) - (wedge_g2[0] * other.e1) - (wedge_g2[1] * other.e2) - (wedge_g2[2] * other.e3)) + (right_dual_g1 * float4(wedge_g0[1])) + (wedge_g1 * float4(other.scalar)) - (wedge_g4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((wedge_g3.zxy * other.group1.yzx), right_dual_g2[1] * wedge_g4[1]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0[1])) + (wedge_g2 * float3(other.scalar)) + (wedge_g4.zxy * other.group1.yzx) - (wedge_g4.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) - (float3(wedge_g4[3]) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.scalar)) + (float4(wedge_g0[1]) * float4(other.group1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Point other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0;
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Scalar anti_reject_via_horizon_from(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
