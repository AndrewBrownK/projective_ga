implementing rga3d;
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float anti_wedge_g0_w = this.e1234 * other.e321 * -1.0;
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(float3(0.0), anti_wedge_g1_xyz[0])) + float4(float3(0.0), (anti_wedge_g1_xyz[1] * other.e2) + (anti_wedge_g1_xyz[2] * other.e3) - (anti_wedge_g0_w * other.e321)) - (float4(float3(0.0), anti_wedge_g0_w).wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Horizon other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * other.e321 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz * float3(-1.0);
        return AntiScalar(
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 anti_wedge_g0 = float2(1.0, this.e1234 * other.scalar) * float2(0.0, 1.0);
        float4 anti_wedge_g1 = float4(float3(0.0), this.e1234 * other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 anti_wedge_g2 = float3(this.e1234) * other.group3.xyz * float3(-1.0);
        float3 anti_wedge_g4_xyz = float3(this.e1234) * other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0[0] * other.e1234) + (anti_wedge_g0[1] * other.scalar) + (anti_wedge_g4_xyz[0] * other.e1) + (anti_wedge_g4_xyz[1] * other.e2) + (anti_wedge_g4_xyz[2] * other.e3) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (anti_wedge_g0[0] * other.e4) + (anti_wedge_g1[3] * other.scalar)), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g4_xyz * float3(other.scalar)), 0.0) + float4((float3(anti_wedge_g0[0]) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((anti_wedge_g2.yzx * other.group1.zxy), 0.0) + float4((other.group2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * other.group1.yzx), 0.0) - float4((other.group2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * other.e321 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Point other) {
        float3 anti_wedge_g0_xyz = float3(this.e1234) * other.group0.xyz;
        return AntiScalar(
            /* e1234 */
            (anti_wedge_g0_xyz[0] * other.e1) + (anti_wedge_g0_xyz[1] * other.e2) + (anti_wedge_g0_xyz[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Scalar other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<DualNum> operator /(DualNum lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        float2 anti_wedge_g0 = float2(other.scalar) * this.group0.xy;
        return DualNum(
            /* scalar, e1234 */
            float2(anti_wedge_g0[0] * other.scalar, (anti_wedge_g0[0] * other.e1234) + (anti_wedge_g0[1] * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float anti_wedge_g0_w = this.e1234 * other.e321 * -1.0;
        float3 anti_wedge_g1_xyz = other.group0.xyz * float3(this.group0.yy, this.e1234);
        return Motor(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(float3(0.0), anti_wedge_g1_xyz[0])) + float4(float3(0.0), (anti_wedge_g1_xyz[1] * other.e2) + (anti_wedge_g1_xyz[2] * other.e3) - (anti_wedge_g0_w * other.e321)) - (float4(float3(0.0), anti_wedge_g0_w).wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Horizon other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * other.e321 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz * float3(-1.0);
        return AntiScalar(
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = right_dual_g0 * float4(this.e1234);
        float anti_wedge_g1_w = right_dual_g0[3] * this.scalar;
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g1_w) * other.group0) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g1_w) * other.group1.xyz), anti_wedge_g1_w * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 anti_wedge_g0 = float2(other.scalar) * this.group0.xy;
        float4 anti_wedge_g1 = float4(float3(0.0), this.e1234 * other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 anti_wedge_g2 = float3(this.e1234) * other.group3.xyz * float3(-1.0);
        float4 anti_wedge_g4 = float4((other.group1.xyz * float3(this.group0.yy, this.e1234)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0[0] * other.scalar, (anti_wedge_g0[0] * other.e1234) + (anti_wedge_g0[1] * other.scalar) + (anti_wedge_g4[0] * other.e1) + (anti_wedge_g4[1] * other.e2) + (anti_wedge_g4[2] * other.e3) + (anti_wedge_g4[3] * other.e4) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group1), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0[0]) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g4 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group4) - (other.group1.yzxx * float4(anti_wedge_g2.zxy, 0.0)) - float4((other.group2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * other.e321 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Point other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * float3(this.group0.yy, this.e1234);
        return AntiScalar(
            /* e1234 */
            (anti_wedge_g0_xyz[0] * other.e1) + (anti_wedge_g0_xyz[1] * other.e2) + (anti_wedge_g0_xyz[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_via_horizon_onto(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2_pow(float2(other.scalar), 2) * this.group0.xy
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Flector> operator /(Flector lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(DualNum other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group0, 
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 anti_wedge_g0_xyz = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 anti_wedge_g1 = (float4(other.group0.xyz, 0.0).wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (this.group1.wwwx * float4(other.group0.xyz, right_dual_g0[0]));
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g1[3]) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g1[0] * other.e4) + (anti_wedge_g1[3] * other.e423), (anti_wedge_g1[1] * other.e4) + (anti_wedge_g1[3] * other.e431), (anti_wedge_g1[2] * other.e4) + (anti_wedge_g1[3] * other.e412), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0_xyz.yzx * other.group0.zxy), anti_wedge_g1[3] * other.e321) - (other.group0.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Horizon anti_project_via_horizon_onto(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Plane anti_project_via_horizon_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g0, -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412))) - (this.group1.yzxx * float4(float3(0.0), right_dual_g0[0]));
        return Plane(
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g0[2] * other.e42) + (anti_wedge_g0[3] * other.e23), (anti_wedge_g0[0] * other.e43) + (anti_wedge_g0[3] * other.e31), (anti_wedge_g0[1] * other.e41) + (anti_wedge_g0[3] * other.e12), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (float4(right_dual_g0[3]) * this.group0) + (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412))) - (this.group1.yzxx * float4(float3(0.0), right_dual_g0[0]));
        float4 anti_wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        return Flector(
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g0[3] * other.e23) + (anti_wedge_g1[0] * other.scalar), (anti_wedge_g0[3] * other.e31) + (anti_wedge_g1[1] * other.scalar), (anti_wedge_g0[3] * other.e12) + (anti_wedge_g1[2] * other.scalar), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((anti_wedge_g0.zxy * other.group0.yzx), anti_wedge_g1[3] * other.scalar) - (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float anti_wedge_g0_x = (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        float4 anti_wedge_g1 = (float4(other.scalar) * this.group0) + (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g2, -(right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412))) - (this.group1.yzxx * float4(float3(0.0), right_dual_g2[0]));
        float3 anti_wedge_g2 = (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy);
        float3 anti_wedge_g3 = float3(this.e321) * other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g4 = float4(other.scalar) * this.group1;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e1234) + (anti_wedge_g4[0] * other.e1) + (anti_wedge_g4[1] * other.e2) + (anti_wedge_g4[2] * other.e3) + (anti_wedge_g4[3] * other.e4) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g3[0] * other.e41) - (anti_wedge_g3[1] * other.e42) - (anti_wedge_g3[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g3 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g3[0] * other.e4) + (anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g3[1] * other.e4) + (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g3[2] * other.e4) + (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g3[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g4 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group4) - (other.group1.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0])) - float4((other.group2.zxy * anti_wedge_g1.yzx), anti_wedge_g3[1] * other.e2)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Point other) {
        float3 anti_wedge_g0_xyz = (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = (float4((float3(this.e321) * other.group0.xyz), (this.e2 * other.e2) + (this.e3 * other.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(other.group0.xyz, 0.0).wwwx * float4(this.group1.xyz, this.e1));
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g1[3]) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g0_xyz[1] * other.e3) + (anti_wedge_g1[0] * other.e4), (anti_wedge_g0_xyz[2] * other.e1) + (anti_wedge_g1[1] * other.e4), (anti_wedge_g0_xyz[0] * other.e2) + (anti_wedge_g1[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (other.group0.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Scalar other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group0, 
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Horizon> operator /(Horizon lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Horizon anti_project_via_horizon_onto(DualNum other) {
        return Horizon(
            /* e321 */
            other.scalar * other.scalar * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g1 = float4(this.e321) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g1[3]) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g1[0] * other.e4) + (anti_wedge_g1[3] * other.e423), (anti_wedge_g1[1] * other.e4) + (anti_wedge_g1[3] * other.e431), (anti_wedge_g1[2] * other.e4) + (anti_wedge_g1[3] * other.e412), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4(float3(0.0), anti_wedge_g1[3] * other.e321) - (other.group0.yzxx * float4(float3(0.0), anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Horizon anti_project_via_horizon_onto(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Plane anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e321) * other.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy)), -(anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = float3(this.e321) * right_dual_g0.xyz;
        return Flector(
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g0_xyz * other.group1.www), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)) + float4((anti_wedge_g0_xyz.zxy * other.group0.yzx), right_dual_g0[3] * this.e321 * other.scalar) - float4((anti_wedge_g0_xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 anti_wedge_g0 = float2(this.e321 * other.e321, 1.0) * float2(1.0, 0.0);
        float4 anti_wedge_g1 = float4((float3(this.e321) * other.group3.xyz * float3(-1.0)), 0.0);
        float3 anti_wedge_g3 = float3(this.e321) * other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g4 = float4(float3(0.0), this.e321 * other.scalar);
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0[0] * other.scalar, (anti_wedge_g0[0] * other.e1234) + (anti_wedge_g0[1] * other.scalar) + (anti_wedge_g4[0] * other.e1) + (anti_wedge_g4[1] * other.e2) + (anti_wedge_g4[2] * other.e3) + (anti_wedge_g4[3] * other.e4) - (anti_wedge_g3[0] * other.e41) - (anti_wedge_g3[1] * other.e42) - (anti_wedge_g3[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[0]) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g3 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3[0] * other.e4) + (anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g3[1] * other.e4) + (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g3[2] * other.e4) + (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g3[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g4 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group4) - (other.group1.yzxx * float4(float3(0.0), anti_wedge_g3[0])) - float4((other.group2.zxy * anti_wedge_g1.yzx), anti_wedge_g3[1] * other.e2)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Plane anti_project_via_horizon_onto(Point other) {
        float3 anti_wedge_g1 = float3(this.e321) * other.group0.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g1, -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3))) - (other.group0.yzxx * float4(float3(0.0), anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Horizon> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Horizon {
    public Horizon anti_project_via_horizon_onto(Scalar other) {
        return Horizon(
            /* e321 */
            other.scalar * other.scalar * this.e321
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Line> operator /(Line lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(DualNum other) {
        return Line(
            /* e41, e42, e43 */
            float3_pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3_pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = float4(other.e3 * this.e31, other.e1 * this.e12, other.e2 * this.e23, -(other.e2 * this.e42) - (other.e3 * this.e43)) - float4((this.group1.zxy * other.group0.yzx), 0.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g0.xyz, -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321))) - (anti_wedge_g0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return Line(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((this.group0.xyz * right_dual_g0.www), 0.0);
        float3 anti_wedge_g1_xyz = float3(right_dual_g0[3]) * this.group1.xyz;
        float anti_wedge_g1_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g1_w) * other.group0) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * other.e41) - (anti_wedge_g1_xyz[1] * other.e42) - (anti_wedge_g1_xyz[2] * other.e43) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g1_xyz * float3(other.scalar)) + (float3(anti_wedge_g1_w) * other.group1.xyz)), anti_wedge_g1_w * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float anti_wedge_g0_x = -(right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12);
        float4 anti_wedge_g1 = float4(this.e31 * other.e3, this.e12 * other.e1, this.e23 * other.e2, -(this.e42 * other.e2) - (this.e43 * other.e3)) - float4((this.group1.zxy * other.group1.yzx), 0.0);
        float3 anti_wedge_g2 = float3(other.scalar) * this.group0.xyz;
        float3 anti_wedge_g3 = float3(other.scalar) * this.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e1234) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g3[0] * other.e41) - (anti_wedge_g3[1] * other.e42) - (anti_wedge_g3[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g3 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g3[0] * other.e4) + (anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g3[1] * other.e4) + (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g3[2] * other.e4) + (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g3[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group4) - (other.group1.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0])) - float4((other.group2.zxy * anti_wedge_g1.yzx), anti_wedge_g3[1] * other.e2)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(Point other) {
        float4 anti_wedge_g0 = float4(this.e31 * other.e3, this.e12 * other.e1, this.e23 * other.e2, -(this.e42 * other.e2) - (this.e43 * other.e3)) - float4((this.group1.zxy * other.group0.yzx), 0.0);
        return Line(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(Scalar other) {
        return Line(
            /* e41, e42, e43 */
            float3_pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3_pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Motor> operator /(Motor lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(DualNum other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xx, other.scalar, (anti_wedge_g0[3] * other.scalar) + (anti_wedge_g1[3] * other.e1234)) * float4(anti_wedge_g0.xyz, 1.0), 
            /* e23, e31, e12, scalar */
            anti_wedge_g1 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = float4(other.e3 * this.e31, other.e1 * this.e12, other.e2 * this.e23, -(other.e2 * this.e42) - (other.e3 * this.e43)) + (float4(this.e1234) * float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0)) - float4((other.group0.yzx * this.group1.zxy), 0.0);
        float3 anti_wedge_g1_xyz = other.group0.xyz * this.group0.www;
        return Motor(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge_g0.xyz, anti_wedge_g1_xyz[0])) + float4(float3(0.0), (anti_wedge_g1_xyz[1] * other.e2) + (anti_wedge_g1_xyz[2] * other.e3) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)) - (anti_wedge_g0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(Horizon other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * other.e321 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_dual_g0 * this.group0.www;
        float anti_wedge_g1_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge_g1_w) * other.group0.xyz), -(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g1_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.e1234) * right_dual_g0.xyz)), right_dual_g0[3] * this.e1234);
        float4 anti_wedge_g1 = (float4(right_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12));
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g1[3]) * other.group0) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float anti_wedge_g0_x = (this.scalar * other.scalar) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12);
        float4 anti_wedge_g1 = float4((right_dual_g1[0] * this.e1234) + (this.e31 * other.e3), (right_dual_g1[1] * this.e1234) + (this.e12 * other.e1), (right_dual_g1[2] * this.e1234) + (this.e23 * other.e2), -(this.e42 * other.e2) - (this.e43 * other.e3)) + (this.group0 * float4(float3(0.0), right_dual_g1[3])) - float4((this.group1.zxy * other.group1.yzx), 0.0);
        float3 anti_wedge_g2 = (right_dual_g2 * float3(this.e1234)) + (float3(other.scalar) * this.group0.xyz);
        float3 anti_wedge_g3 = float3(other.scalar) * this.group1.xyz;
        float4 anti_wedge_g4 = float4((other.group1.xyz * this.group0.www), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e1234) + (anti_wedge_g4[0] * other.e1) + (anti_wedge_g4[1] * other.e2) + (anti_wedge_g4[2] * other.e3) + (anti_wedge_g4[3] * other.e4) + (this.e1234 * pow(other.scalar, 2)) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g3[0] * other.e41) - (anti_wedge_g3[1] * other.e42) - (anti_wedge_g3[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g3 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g3[0] * other.e4) + (anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g3[1] * other.e4) + (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g3[2] * other.e4) + (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g3[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g4 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group4) - (other.group1.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0])) - float4((other.group2.zxy * anti_wedge_g1.yzx), anti_wedge_g3[1] * other.e2)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        return AntiScalar(
            /* e1234 */
            other.e321 * other.e321 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Point other) {
        float4 anti_wedge_g0 = float4(this.e31 * other.e3, this.e12 * other.e1, this.e23 * other.e2, -(this.e42 * other.e2) - (this.e43 * other.e3)) - float4((this.group1.zxy * other.group0.yzx), 0.0);
        float3 anti_wedge_g1_xyz = other.group0.xyz * this.group0.www;
        return Motor(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge_g0.xyz, anti_wedge_g1_xyz[0])) + float4((other.group0.xyz * anti_wedge_g0.www * float3(-1.0)), (anti_wedge_g1_xyz[1] * other.e2) + (anti_wedge_g1_xyz[2] * other.e3)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4_pow(float4(other.scalar), 2) * this.group0, 
            /* e23, e31, e12, scalar */
            float4_pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<MultiVector> operator /(MultiVector lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(DualNum other) {
        float2 anti_wedge_g0 = float2(other.scalar) * this.group0.xy;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0[0] * other.scalar, (anti_wedge_g0[0] * other.e1234) + (anti_wedge_g0[1] * other.scalar)), 
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
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float anti_wedge_g0_x = (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        float4 anti_wedge_g1 = float4(other.e3 * this.e31, other.e1 * this.e12, other.e2 * this.e23, -(other.e2 * this.e42) - (other.e3 * this.e43)) + (right_dual_g0 * float4(this.e1234)) - float4((this.group3.zxy * other.group0.yzx), 0.0);
        float3 anti_wedge_g2 = (other.group0.yzx * this.group4.zxy) - (other.group0.zxy * this.group4.yzx);
        float3 anti_wedge_g3 = float3(this.e321) * other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g4_xyz = other.group0.xyz * float3(this.group0.yy, this.e1234);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g4_xyz[0] * other.e1) + (anti_wedge_g4_xyz[1] * other.e2) + (anti_wedge_g4_xyz[2] * other.e3) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * other.group0, 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g3[1] * other.e2) - (anti_wedge_g3[2] * other.e3)) + (float4(anti_wedge_g0_x) * other.group1) - (other.group0.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Horizon other) {
        float right_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, right_dual_g0 * other.e321 * this.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), right_dual_g0 * other.e321 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float anti_wedge_g0_x = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        float4 anti_wedge_g1 = (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g0, -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412))) - (this.group4.yzxx * float4(float3(0.0), right_dual_g0[0]));
        float3 anti_wedge_g2 = right_dual_g0 * float3(this.e1234);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0_x) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_dual_g0[3] * this.scalar) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        float4 anti_wedge_g1 = (right_dual_g0 * float4(this.group4.www, this.e4)) + float4((this.group1.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) - (this.group4.yzxx * float4(float3(0.0), right_dual_g0[0]));
        float3 anti_wedge_g2 = (float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e1234) * right_dual_g0.xyz);
        float3 anti_wedge_g3 = float3(right_dual_g0[3]) * this.group3.xyz;
        float4 anti_wedge_g4 = float4(right_dual_g0[3]) * this.group4;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e1234) + (right_dual_g0[3] * other.scalar * this.e1234) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g3[0] * other.e41) - (anti_wedge_g3[1] * other.e42) - (anti_wedge_g3[2] * other.e43)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g3 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g1[3] * other.e23) + (anti_wedge_g4[0] * other.scalar), (anti_wedge_g1[3] * other.e31) + (anti_wedge_g4[1] * other.scalar), (anti_wedge_g1[3] * other.e12) + (anti_wedge_g4[2] * other.scalar), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g1.zxy * other.group0.yzx), anti_wedge_g4[3] * other.scalar) - (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float anti_wedge_g0_x = (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        float4 anti_wedge_g1 = float4((right_dual_g2[0] * this.e321) + (other.e3 * this.e31), (right_dual_g2[1] * this.e321) + (other.e1 * this.e12), (right_dual_g2[2] * this.e321) + (other.e2 * this.e23), -(right_dual_g2[2] * this.e412) - (other.e1 * this.e41) - (other.e2 * this.e42) - (other.e3 * this.e43)) + (right_dual_g1 * float4(this.e1234)) + (float4(other.scalar) * this.group1) - (this.group4.yzxx * float4(float3(0.0), right_dual_g2[0])) - float4((this.group3.zxy * other.group1.yzx), right_dual_g2[1] * this.e431);
        float3 anti_wedge_g2 = (right_dual_g2 * float3(this.e1234)) + (float3(other.scalar) * this.group2.xyz) + (other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx);
        float3 anti_wedge_g3 = (float3(other.scalar) * this.group3.xyz) - (float3(this.e321) * other.group1.xyz);
        float4 anti_wedge_g4 = (float4(other.scalar) * this.group4) + (float4(this.e1234) * float4(other.group1.xyz, 0.0));
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e1234) + (anti_wedge_g4[0] * other.e1) + (anti_wedge_g4[1] * other.e2) + (anti_wedge_g4[2] * other.e3) + (anti_wedge_g4[3] * other.e4) + (other.scalar * other.scalar * this.e1234) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g3[0] * other.e41) - (anti_wedge_g3[1] * other.e42) - (anti_wedge_g3[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g3 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g3[0] * other.e4) + (anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g3[1] * other.e4) + (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g3[2] * other.e4) + (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g3[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g4 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group4) - (other.group1.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0])) - float4((other.group2.zxy * anti_wedge_g1.yzx), anti_wedge_g3[1] * other.e2)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Plane other) {
        float right_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, right_dual_g0 * this.e1234 * other.e321) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual_g0 * this.e321 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Point other) {
        float4 anti_wedge_g1 = float4(this.e31 * other.e3, this.e12 * other.e1, this.e23 * other.e2, -(this.e42 * other.e2) - (this.e43 * other.e3)) - float4((this.group3.zxy * other.group0.yzx), 0.0);
        float3 anti_wedge_g2 = (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy);
        float3 anti_wedge_g3 = float3(this.e321) * other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g4_xyz = other.group0.xyz * float3(this.group0.yy, this.e1234);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g4_xyz[0] * other.e1) + (anti_wedge_g4_xyz[1] * other.e2) + (anti_wedge_g4_xyz[2] * other.e3)), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)) * other.group0, 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g3[1] * other.e2) - (anti_wedge_g3[2] * other.e3)) - (other.group0.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Scalar other) {
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
public AntiProjectViaHorizonOntoInfixPartial<Origin> operator /(Origin lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Origin operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public Origin anti_project_via_horizon_onto(DualNum other) {
        return Origin(
            /* e4 */
            other.scalar * other.scalar * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float anti_wedge_g0 = other.scalar * this.e4;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0 * other.scalar), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float anti_wedge_g0 = other.scalar * this.e4;
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, anti_wedge_g0 * other.e321) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0 * other.scalar), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0) * other.group3.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(AntiProjectViaHorizonOntoInfixPartial<Origin> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Origin {
    public Origin anti_project_via_horizon_onto(Scalar other) {
        return Origin(
            /* e4 */
            other.scalar * other.scalar * this.e4
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Plane> operator /(Plane lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(DualNum other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 anti_wedge_g0_xyz = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (this.group0.wwwx * float4(other.group0.xyz, right_dual_g0[0]));
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g1[3]) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g1[0] * other.e4) + (anti_wedge_g1[3] * other.e423), (anti_wedge_g1[1] * other.e4) + (anti_wedge_g1[3] * other.e431), (anti_wedge_g1[2] * other.e4) + (anti_wedge_g1[3] * other.e412), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0_xyz.yzx * other.group0.zxy), anti_wedge_g1[3] * other.e321) - (other.group0.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Horizon anti_project_via_horizon_onto(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g0, -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412))) - (this.group0.yzxx * float4(float3(0.0), right_dual_g0[0]));
        return Plane(
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g0[2] * other.e42) + (anti_wedge_g0[3] * other.e23), (anti_wedge_g0[0] * other.e43) + (anti_wedge_g0[3] * other.e31), (anti_wedge_g0[1] * other.e41) + (anti_wedge_g0[3] * other.e12), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412))) - (this.group0.yzxx * float4(float3(0.0), right_dual_g0[0]));
        float4 anti_wedge_g1 = float4(right_dual_g0[3]) * this.group0;
        return Flector(
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g0[3] * other.e23) + (anti_wedge_g1[0] * other.scalar), (anti_wedge_g0[3] * other.e31) + (anti_wedge_g1[1] * other.scalar), (anti_wedge_g0[3] * other.e12) + (anti_wedge_g1[2] * other.scalar), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((anti_wedge_g0.zxy * other.group0.yzx), anti_wedge_g1[3] * other.scalar) - (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float anti_wedge_g0_x = -(right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        float4 anti_wedge_g1 = (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g2, -(right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412))) - (this.group0.yzxx * float4(float3(0.0), right_dual_g2[0]));
        float3 anti_wedge_g2 = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float3 anti_wedge_g3 = float3(this.e321) * other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g4 = float4(other.scalar) * this.group0;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e1234) + (anti_wedge_g4[0] * other.e1) + (anti_wedge_g4[1] * other.e2) + (anti_wedge_g4[2] * other.e3) + (anti_wedge_g4[3] * other.e4) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g3[0] * other.e41) - (anti_wedge_g3[1] * other.e42) - (anti_wedge_g3[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e41, e42, e43 */
            (anti_wedge_g2 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g3 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g2[1] * other.e3) + (anti_wedge_g3[0] * other.e4) + (anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g2[2] * other.e1) + (anti_wedge_g3[1] * other.e4) + (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g2[0] * other.e2) + (anti_wedge_g3[2] * other.e4) + (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g3[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g4 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group4) - (other.group1.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0])) - float4((other.group2.zxy * anti_wedge_g1.yzx), anti_wedge_g3[1] * other.e2)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Point other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 anti_wedge_g1 = float3(this.e321) * other.group0.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g0[1] * other.e3) + (anti_wedge_g1[0] * other.e4), (anti_wedge_g0[2] * other.e1) + (anti_wedge_g1[1] * other.e4), (anti_wedge_g0[0] * other.e2) + (anti_wedge_g1[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (other.group0.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Scalar other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4_pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Point> operator /(Point lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Point operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Point anti_project_via_horizon_onto(DualNum other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float anti_wedge_g0 = (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return Flector(
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g0[2] * other.e42) + (anti_wedge_g0[3] * other.e23), (anti_wedge_g0[0] * other.e43) + (anti_wedge_g0[3] * other.e31), (anti_wedge_g0[1] * other.e41) + (anti_wedge_g0[3] * other.e12), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float anti_wedge_g0_x = (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3);
        float4 anti_wedge_g1 = float4(other.scalar) * this.group0;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e1234) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group2.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0_x) * other.group3.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g1[2] * other.e42) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g1[0] * other.e43) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g1[1] * other.e41) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group4) - float4((other.group2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Point anti_project_via_horizon_onto(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(AntiProjectViaHorizonOntoInfixPartial<Point> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Point {
    public Point anti_project_via_horizon_onto(Scalar other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4_pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Scalar> operator /(Scalar lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float anti_wedge_g0 = other.scalar * this.scalar;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge_g0) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float anti_wedge_g0 = other.scalar * this.scalar;
        return MultiVector(
            /* scalar, e1234 */
            float2(anti_wedge_g0) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group1, 
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge_g0) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public Scalar anti_project_via_horizon_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
