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
            other.scalar * other.scalar * this.e1234
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
            float4(other.scalar * this.e1234) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
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
        float wedge_g0 = other.scalar * this.e1234;
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
public AntiScalar operator /(AntiRejectViaHorizonFromInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_reject_via_horizon_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
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
            float2(other.scalar * other.scalar * this.scalar, (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 right_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_dual_g1_xyz.yzx * wedge_g1.zxy) - (right_dual_g1_xyz.zxy * wedge_g1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((right_dual_g1_xyz * float4(wedge_g1[3]).xyz * float3(-1.0)), (wedge_g1[3] * other.e321) + (this.scalar * right_dual_g1_xyz[0] * other.e1))
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
            this.scalar * other.e321 * other.e321
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
        return Scalar(
            /* scalar */
            (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension DualNum {
    public Motor anti_reject_via_horizon_from(Motor other) {
        float wedge_g0_w = (this.scalar * other.e1234) + (this.e1234 * other.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.scalar * other.scalar) * other.group0.xyz) - (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * float4(other.scalar).xyz), (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12) + (wedge_g1[3] * other.scalar))
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
        float4 right_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((this.scalar * other.scalar * other.scalar) + (right_dual_g4[0] * wedge_g1[0]) + (right_dual_g4[1] * wedge_g1[1]) + (right_dual_g4[2] * wedge_g1[2]) + (right_dual_g4[3] * wedge_g1[3]) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(((right_dual_g2 * float3(wedge_g4[3])) + (wedge_g2 * float3(right_dual_g4[3])) + (wedge_g3.yzx * right_dual_g4.zxy) - (wedge_g3.zxy * right_dual_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
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
            this.scalar * other.e321 * other.e321
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
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_dual_g0_xyz[0] * wedge_g0[0]) + (right_dual_g0_xyz[1] * wedge_g0[1]) + (right_dual_g0_xyz[2] * wedge_g0[2])
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
            float2(other.scalar * other.scalar) * this.group0.xy
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
            float4(other.scalar * other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g0 = (this.group0.wwwx * float4(other.group0.xyz, other.e423)) + float4(float3(0.0), -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.wwwx * float4(this.group0.xyz, this.e423));
        float4 wedge_g1 = float4(0.0, 0.0, (other.e2 * this.e1) - (other.e1 * this.e2), 0.0) + float4(((other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx)), 0.0, 0.0);
        float3 right_dual_g1_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((right_dual_g1_xyz.zxy * wedge_g1.yzx) - (right_dual_g1_xyz.yzx * wedge_g1.zxy)), wedge_g0[3] * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((right_dual_g1_xyz * float4(wedge_g0[3]).xyz), 0.0)
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
            this.e4 * other.e321 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Flector {
    public Flector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4((float3((this.e3 * other.e42) - (this.e2 * other.e43), (this.e1 * other.e43) - (this.e3 * other.e41), (this.e2 * other.e41) - (this.e1 * other.e42)) + (float3(this.e4) * other.group1.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e321 * other.scalar);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(other.scalar) * wedge_g0.xyz) - (float3(wedge_g1[3]) * other.group1.xyz)), wedge_g0[3] * other.scalar), 
            /* e423, e431, e412, e321 */
            wedge_g1 * float4(other.scalar)
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
        float3 wedge_g3 = float3((this.e2 * other.e3) - (this.e3 * other.e2), (this.e3 * other.e1) - (this.e1 * other.e3), (this.e1 * other.e2) - (this.e2 * other.e1));
        float4 wedge_g4 = float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e4) * other.group3.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx)), other.scalar * this.e321);
        float4 right_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_dual_g4[0] * wedge_g1[0]) + (right_dual_g4[1] * wedge_g1[1]) + (right_dual_g4[2] * wedge_g1[2]) + (right_dual_g4[3] * wedge_g1[3]) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(((right_dual_g2 * float3(wedge_g4[3])) + (wedge_g2 * float3(right_dual_g4[3])) + (wedge_g3.yzx * right_dual_g4.zxy) - (wedge_g3.zxy * right_dual_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
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
        float4 wedge_g1 = float4(0.0, 0.0, (this.e1 * other.e2) - (this.e2 * other.e1), 0.0) + float4(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), 0.0, 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((right_dual_g0_xyz.zxy * wedge_g1.yzx) - (right_dual_g0_xyz.yzx * wedge_g1.zxy)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((right_dual_g0_xyz * float4(this.e321).xyz * float4(other.e4).xyz * float3(-1.0)), 0.0)
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
            float4(other.scalar * other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar * other.scalar) * this.group1
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
        float wedge_g0 = other.scalar * this.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0 * -1.0) * other.group1.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge_g0 * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Horizon {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = other.e4 * this.e321 * -1.0;
        float4 wedge_g4 = float4(float3(0.0), other.scalar * this.e321);
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(right_dual_g1_w * wedge_g4[3] * -1.0, wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2 * float4(wedge_g4[3]).xyz), right_dual_g1_w * wedge_g0_y), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)), 
            /* e23, e31, e12 */
            (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
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
            float4((float3(this.e321 * -1.0) * float4(other.e4).xyz * other.group0.xyz), 0.0)
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
            this.e321 * other.scalar * other.scalar
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
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float3 wedge_g0_xyz = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float3 right_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_dual_g1_xyz.yzx * wedge_g0_xyz.zxy) - (right_dual_g1_xyz.zxy * wedge_g0_xyz.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
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
            (float3(other.e23) * float3(other.e31 * this.e42, other.e31 * this.e41, other.e12 * this.e41)) + (float3(other.e12) * float3(other.e23 * this.e43, other.e31 * this.e43, other.e31 * this.e42)) + (float3(other.e41 * this.e23) * other.group1.xyz) + (float3(other.e42 * this.e31) * other.group1.xyz) + (float3(other.e43 * this.e12) * other.group1.xyz) + (other.group1.xyz * other.group1.xyz * this.group0.xyz), 
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
        float3 wedge_g1_xyz = float3(other.scalar) * this.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.scalar * other.scalar) * this.group0.xyz) - (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1_xyz * float4(other.scalar).xyz), (wedge_g1_xyz[0] * other.e23) + (wedge_g1_xyz[1] * other.e31) + (wedge_g1_xyz[2] * other.e12))
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
        float4 wedge_g4 = float4(((float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0);
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_dual_g1_w * wedge_g4[3]) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(((right_dual_g2 * float3(wedge_g4[3])) + (wedge_g2 * float3(right_dual_g4[3])) + (wedge_g3.yzx * right_dual_g4.zxy) - (wedge_g3.zxy * right_dual_g4.yzx)), right_dual_g1_w * wedge_g0_y), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiRejectViaHorizonFromInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Line {
    public Line anti_reject_via_horizon_from(Point other) {
        float3 wedge_g0_xyz = (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e41, e42, e43 */
            (right_dual_g0_xyz.yzx * wedge_g0_xyz.zxy) - (right_dual_g0_xyz.zxy * wedge_g0_xyz.yzx), 
            /* e23, e31, e12 */
            float3(0.0)
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
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group1.xyz
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
            float4((this.group0.xyz * float3(float2(other.scalar * other.scalar), other.scalar * other.scalar)), (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float3 wedge_g1_xyz = float3((other.e3 * this.e42) - (other.e2 * this.e43), (other.e1 * this.e43) - (other.e3 * this.e41), (other.e2 * this.e41) - (other.e1 * this.e42)) + (float3(other.e4) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz);
        float wedge_g1_w = other.e321 * this.scalar;
        float3 right_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_dual_g1_xyz.yzx * wedge_g1_xyz.zxy) - (right_dual_g1_xyz.zxy * wedge_g1_xyz.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((right_dual_g1_xyz * float3(wedge_g1_w * -1.0)), (wedge_g1_w * other.e321) + (right_dual_g1_xyz[0] * other.e1 * this.scalar))
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
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Line other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((-(right_dual_g0 * float3(other.e41 * this.e23)) - (right_dual_g0 * float3(other.e42 * this.e31)) - (right_dual_g0 * float3(other.e43 * this.e12)) - (right_dual_g0 * float3(other.e23 * this.e41)) - (right_dual_g0 * float3(other.e31 * this.e42)) - (right_dual_g0 * float3(other.e12 * this.e43))), 0.0), 
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
        float3 wedge_g1_xyz = (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.scalar) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1_xyz * float4(other.scalar).xyz), (wedge_g1_xyz[0] * other.e23) + (wedge_g1_xyz[1] * other.e31) + (wedge_g1_xyz[2] * other.e12) + (other.scalar * other.scalar * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e1234) + (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 wedge_g2 = (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group2.xyz);
        float3 wedge_g3 = (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group3.xyz);
        float4 wedge_g4 = float4((float3((this.e42 * other.e3) - (this.e43 * other.e2), (this.e43 * other.e1) - (this.e41 * other.e3), (this.e41 * other.e2) - (this.e42 * other.e1)) + (float3(this.scalar) * other.group4.xyz) + (float3(other.e4) * this.group1.xyz)), this.scalar * other.e321);
        float4 right_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_dual_g4[0] * wedge_g1[0]) + (right_dual_g4[1] * wedge_g1[1]) + (right_dual_g4[2] * wedge_g1[2]) + (right_dual_g4[3] * wedge_g1[3]) + (other.scalar * other.scalar * this.scalar) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(((right_dual_g2 * float3(wedge_g4[3])) + (wedge_g2 * float3(right_dual_g4[3])) + (wedge_g3.yzx * right_dual_g4.zxy) - (wedge_g3.zxy * right_dual_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
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
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Motor {
    public Motor anti_reject_via_horizon_from(Point other) {
        float4 wedge_g1 = float4(0.0, 0.0, (this.e41 * other.e2) - (this.e42 * other.e1), 0.0) + float4(((float3(other.e4) * this.group1.xyz) + float3(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), 0.0)), 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_dual_g0_xyz.yzx * wedge_g1.zxy) - (right_dual_g0_xyz.zxy * wedge_g1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((right_dual_g0_xyz * float4(wedge_g1[3]).xyz * float3(-1.0)), right_dual_g0_xyz[0] * this.scalar * other.e1)
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
            float4(other.scalar * other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar * other.scalar) * this.group1
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
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(Flector other) {
        float wedge_g0_y = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 wedge_g1 = float4(this.scalar) * other.group0;
        float3 wedge_g3 = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2));
        float3 wedge_g4_xyz = (float3(this.scalar) * other.group1.xyz) + (float3(other.e4) * this.group3.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float wedge_g4_w = this.scalar * other.e321;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group0.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2((right_dual_g1_xyz[0] * wedge_g1[0]) + (right_dual_g1_xyz[1] * wedge_g1[1]) + (right_dual_g1_xyz[2] * wedge_g1[2]) - (right_dual_g0_w * wedge_g4_w), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((right_dual_g1_xyz.zxy * wedge_g3.yzx) - (right_dual_g1_xyz.yzx * wedge_g3.zxy)), right_dual_g0_w * wedge_g0_y), 
            /* e41, e42, e43 */
            (right_dual_g1_xyz.yzx * wedge_g4_xyz.zxy) - (right_dual_g1_xyz.zxy * wedge_g4_xyz.yzx), 
            /* e23, e31, e12 */
            right_dual_g1_xyz * float3(wedge_g4_w * -1.0), 
            /* e423, e431, e412, e321 */
            float4((right_dual_g1_xyz * float3(wedge_g0_y)), 0.0)
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
            float2(right_dual_g0 * this.scalar * other.e321 * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual_g0 * this.e4 * other.e321), 
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
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(right_dual_g0[0] * wedge_g3[0]) - (right_dual_g0[1] * wedge_g3[1]) - (right_dual_g0[2] * wedge_g3[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            -(right_dual_g0 * float3(other.e41 * this.e23)) - (right_dual_g0 * float3(other.e42 * this.e31)) - (right_dual_g0 * float3(other.e43 * this.e12)) - (right_dual_g0 * float3(other.e23 * this.e41)) - (right_dual_g0 * float3(other.e31 * this.e42)) - (right_dual_g0 * float3(other.e12 * this.e43)), 
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
        float wedge_g0_y = (this.scalar * other.e1234) + (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 wedge_g1 = float4(other.scalar) * this.group1;
        float3 wedge_g3 = (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group3.xyz);
        float4 wedge_g4 = float4((float3((other.e42 * this.e3) - (other.e43 * this.e2), (other.e43 * this.e1) - (other.e41 * this.e3), (other.e41 * this.e2) - (other.e42 * this.e1)) + (float3(other.scalar) * this.group4.xyz) + (float3(this.e4) * other.group1.xyz)), other.scalar * this.e321);
        return MultiVector(
            /* scalar, e1234 */
            float2((this.scalar * other.scalar * other.scalar) + (wedge_g3[0] * other.e23) + (wedge_g3[1] * other.e31) + (wedge_g3[2] * other.e12), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(((float3(other.scalar) * wedge_g1.xyz) - (float3(wedge_g4[3]) * other.group1.xyz)), wedge_g1[3] * other.scalar), 
            /* e41, e42, e43 */
            (float3(other.scalar * other.scalar) * this.group2.xyz) + (float3(this.scalar * other.scalar) * other.group0.xyz) - (float3(wedge_g0_y) * other.group1.xyz), 
            /* e23, e31, e12 */
            wedge_g3 * float3(other.scalar), 
            /* e423, e431, e412, e321 */
            wedge_g4 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension MultiVector {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 wedge_g1 = (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1);
        float3 wedge_g2 = (float3(other.scalar) * this.group2.xyz) + (float3(this.scalar) * other.group2.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz);
        float3 wedge_g3 = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2)) + (float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group3.xyz);
        float4 wedge_g4 = (float4(other.scalar) * this.group4) + (float4(this.scalar) * other.group4) + float4(((float3(other.e4) * this.group3.xyz) + (float3(this.e4) * other.group3.xyz) + (other.group2.yzx * this.group1.zxy) + (this.group2.yzx * other.group1.zxy) - (other.group2.zxy * this.group1.yzx) - (this.group2.zxy * other.group1.yzx)), 0.0);
        float4 right_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_dual_g4[0] * wedge_g1[0]) + (right_dual_g4[1] * wedge_g1[1]) + (right_dual_g4[2] * wedge_g1[2]) + (right_dual_g4[3] * wedge_g1[3]) + (other.scalar * other.scalar * this.scalar) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(((right_dual_g2 * float3(wedge_g4[3])) + (wedge_g2 * float3(right_dual_g4[3])) + (wedge_g3.yzx * right_dual_g4.zxy) - (wedge_g3.zxy * right_dual_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
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
            float2(right_dual_g0 * this.scalar * other.e321 * -1.0, 0.0), 
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
        float3 wedge_g3 = float3((this.e2 * other.e3) - (this.e3 * other.e2), (this.e3 * other.e1) - (this.e1 * other.e3), (this.e1 * other.e2) - (this.e2 * other.e1));
        float3 wedge_g4_xyz = (float3(other.e4) * this.group3.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2((right_dual_g0_xyz[0] * wedge_g1[0]) + (right_dual_g0_xyz[1] * wedge_g1[1]) + (right_dual_g0_xyz[2] * wedge_g1[2]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((right_dual_g0_xyz.zxy * wedge_g3.yzx) - (right_dual_g0_xyz.yzx * wedge_g3.zxy)), 0.0), 
            /* e41, e42, e43 */
            (right_dual_g0_xyz.yzx * wedge_g4_xyz.zxy) - (right_dual_g0_xyz.zxy * wedge_g4_xyz.yzx), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((-(right_dual_g0_xyz * float3(this.e423 * other.e1)) - (right_dual_g0_xyz * float3(this.e431 * other.e2)) - (right_dual_g0_xyz * float3(this.e412 * other.e3)) - (right_dual_g0_xyz * float3(this.e321 * other.e4))), 0.0)
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
        float3 right_dual_g1_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_dual_g1_xyz[0] * wedge_g0[0]) - (right_dual_g1_xyz[1] * wedge_g0[1]) - (right_dual_g1_xyz[2] * wedge_g0[2]) - (wedge_g0[3] * other.e321)), 
            /* e423, e431, e412, e321 */
            float4((right_dual_g1_xyz * float4(wedge_g0[3]).xyz), 0.0)
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
        return Origin(
            /* e4 */
            (wedge_g0_xyz[0] * other.e23) + (wedge_g0_xyz[1] * other.e31) + (wedge_g0_xyz[2] * other.e12)
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
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (wedge_g1_xyz[0] * other.e23) + (wedge_g1_xyz[1] * other.e31) + (wedge_g1_xyz[2] * other.e12) + (other.scalar * other.scalar * this.e4)), 
            /* e423, e431, e412, e321 */
            float4((wedge_g1_xyz * float4(other.scalar).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiRejectViaHorizonFromInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Origin {
    public MultiVector anti_reject_via_horizon_from(MultiVector other) {
        float wedge_g0_y = other.e321 * this.e4;
        float3 wedge_g2 = float3(this.e4) * other.group1.xyz;
        float3 wedge_g4_xyz = float3(this.e4) * other.group3.xyz;
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (other.scalar * other.scalar * this.e4) - (wedge_g0_y * other.e321) - (right_dual_g2[0] * wedge_g4_xyz[0]) - (right_dual_g2[1] * wedge_g4_xyz[1]) - (right_dual_g2[2] * wedge_g4_xyz[2]) - (right_dual_g4_xyz[0] * wedge_g2[0]) - (right_dual_g4_xyz[1] * wedge_g2[1]) - (right_dual_g4_xyz[2] * wedge_g2[2])), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)) + (right_dual_g4_xyz.yzx * wedge_g4_xyz.zxy) - (right_dual_g4_xyz.zxy * wedge_g4_xyz.yzx), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((right_dual_g4_xyz * float3(wedge_g0_y)) + (wedge_g4_xyz * float3(other.scalar))), 0.0)
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
            other.e321 * other.e321 * this.e4 * -1.0
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
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Origin(
            /* e4 */
            -(right_dual_g0_xyz[0] * wedge_g0[0]) - (right_dual_g0_xyz[1] * wedge_g0[1]) - (right_dual_g0_xyz[2] * wedge_g0[2])
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
            this.e4 * other.scalar * other.scalar
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
            float4(other.scalar * other.scalar) * this.group0
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
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float4(wedge_g0[3]).xyz * other.group1.xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412, e321 */
            wedge_g0 * float4(other.scalar)
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
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(right_dual_g1_w * wedge_g4[3] * -1.0, wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g2 * float4(wedge_g4[3]).xyz), right_dual_g1_w * wedge_g0_y), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)), 
            /* e23, e31, e12 */
            (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
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
            float4(0.0, 0.0, other.e3 * other.e3 * this.e412 * -1.0, 0.0) + float4((float3(-(other.group0.xy * other.group0.xy * this.group0.xy), 0.0) - (float3(other.e1) * float3(this.e431 * other.e2, this.e423 * other.e2, this.e423 * other.e3)) - (float3(other.e3) * float3(this.e412 * other.e1, this.e412 * other.e2, this.e431 * other.e2)) - (float3(this.e321 * other.e4) * other.group0.xyz)), 0.0)
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
            float4(other.scalar * other.scalar) * this.group0
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
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Flector anti_reject_via_horizon_from(Flector other) {
        float wedge_g0_w = other.e423 * this.e1;
        float4 wedge_g1 = float4(0.0, 0.0, (other.e2 * this.e1) - (other.e1 * this.e2), 0.0) + float4(((other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx)), 0.0, 0.0);
        float3 right_dual_g1_xyz = other.group0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((right_dual_g1_xyz.zxy * wedge_g1.yzx) - (right_dual_g1_xyz.yzx * wedge_g1.zxy)), wedge_g0_w * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((right_dual_g1_xyz * float3(wedge_g0_w)), 0.0)
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
            this.e4 * other.e321 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiRejectViaHorizonFromInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Point {
    public Flector anti_reject_via_horizon_from(Motor other) {
        float4 wedge_g0 = float4(other.scalar) * this.group0;
        float4 wedge_g1 = float4(0.0, 0.0, (other.e41 * this.e2) - (other.e42 * this.e1), 0.0) + float4(((float3(this.e4) * other.group1.xyz) + float3(((other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz)), 0.0)), 0.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(other.scalar) * wedge_g0.xyz) - (float3(wedge_g1[3]) * other.group1.xyz)), wedge_g0[3] * other.scalar), 
            /* e423, e431, e412, e321 */
            wedge_g1 * float4(other.scalar)
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
        float3 wedge_g3 = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2));
        float4 wedge_g4 = float4(((float3(this.e4) * other.group3.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx)), 0.0);
        float4 right_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((right_dual_g4[0] * wedge_g1[0]) + (right_dual_g4[1] * wedge_g1[1]) + (right_dual_g4[2] * wedge_g1[2]) + (right_dual_g4[3] * wedge_g1[3]) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0_y * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0_y)) + (wedge_g1 * float4(other.scalar)) + float4(((right_dual_g2 * float3(wedge_g4[3])) + (wedge_g2 * float3(right_dual_g4[3])) + (wedge_g3.yzx * right_dual_g4.zxy) - (wedge_g3.zxy * right_dual_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0_y)) + (wedge_g2 * float3(other.scalar)), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0_y)) + (wedge_g4 * float4(other.scalar))
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
        float3 wedge_g1 = float3((other.e3 * this.e2) - (other.e2 * this.e3), (other.e1 * this.e3) - (other.e3 * this.e1), (other.e2 * this.e1) - (other.e1 * this.e2));
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Point(
            /* e1, e2, e3, e4 */
            float4(((right_dual_g0_xyz.zxy * wedge_g1.yzx) - (right_dual_g0_xyz.yzx * wedge_g1.zxy)), 0.0)
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
            float4(other.scalar * other.scalar) * this.group0
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
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiRejectViaHorizonFromInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.anti_reject_via_horizon_from(rhs);
}
public extension Scalar {
    public Motor anti_reject_via_horizon_from(Flector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1;
        float3 right_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((right_dual_g1_xyz.yzx * wedge_g1.zxy) - (right_dual_g1_xyz.zxy * wedge_g1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((right_dual_g1_xyz * float4(wedge_g1[3]).xyz * float3(-1.0)), (wedge_g1[3] * other.e321) + (right_dual_g1_xyz[0] * other.e1 * this.scalar))
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
        return Scalar(
            /* scalar */
            (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12)
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
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.scalar) * wedge_g0.xyz) - (float3(wedge_g0[3]) * other.group1.xyz)), wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * float4(other.scalar).xyz), (wedge_g1[0] * other.e23) + (wedge_g1[1] * other.e31) + (wedge_g1[2] * other.e12) + (wedge_g1[3] * other.scalar))
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
        float4 right_dual_g1 = float4(float3(0.0), other.e321 * -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g4 = float4(other.group1.xyz, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0[0] * other.scalar) + (right_dual_g4[0] * wedge_g1[0]) + (right_dual_g4[1] * wedge_g1[1]) + (right_dual_g4[2] * wedge_g1[2]) + (right_dual_g4[3] * wedge_g1[3]) - (right_dual_g2[0] * wedge_g3[0]) - (right_dual_g2[1] * wedge_g3[1]) - (right_dual_g2[2] * wedge_g3[2]) - (right_dual_g1[0] * wedge_g4[0]) - (right_dual_g1[1] * wedge_g4[1]) - (right_dual_g1[2] * wedge_g4[2]) - (right_dual_g1[3] * wedge_g4[3]), wedge_g0[1] * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(wedge_g0[1])) + (wedge_g1 * float4(other.scalar)) + float4(((right_dual_g2 * float3(wedge_g4[3])) + (wedge_g2 * float3(right_dual_g4[3])) + (wedge_g3.yzx * right_dual_g4.zxy) - (wedge_g3.zxy * right_dual_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((right_dual_g4[1] * wedge_g4[2]) - (right_dual_g4[2] * wedge_g4[1]), (right_dual_g4[2] * wedge_g4[0]) - (right_dual_g4[0] * wedge_g4[2]), (right_dual_g4[0] * wedge_g4[1]) - (right_dual_g4[1] * wedge_g4[0])) + (right_dual_g2 * float3(wedge_g0[1])) + (wedge_g2 * float3(other.scalar)), 
            /* e23, e31, e12 */
            (wedge_g3 * float3(other.scalar)) + (float3(right_dual_g4[3]) * wedge_g4.xyz) - (float3(wedge_g4[3]) * right_dual_g4.xyz), 
            /* e423, e431, e412, e321 */
            (right_dual_g4 * float4(wedge_g0[1])) + (wedge_g4 * float4(other.scalar))
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
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_dual_g0_xyz[0] * wedge_g0[0]) + (right_dual_g0_xyz[1] * wedge_g0[1]) + (right_dual_g0_xyz[2] * wedge_g0[2])
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
