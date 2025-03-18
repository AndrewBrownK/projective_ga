implementing rga3d;
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float2(right_anti_dual_g0_w * -1.0) * anti_wedge_g0.xy), right_anti_dual_g0_w * anti_wedge_g0[2] * -1.0, (right_anti_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_anti_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_anti_dual_g1_xyz[2] * anti_wedge_g0[2]) - (right_anti_dual_g0_w * this.e1234 * other.e321)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Line other) {
        float3 anti_wedge_g1 = float3(this.e1234) * other.group1.xyz;
        return AntiScalar(
            /* e1234 */
            -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge_g1[3]) * other.group1.xyz), (anti_wedge_g1[3] * other.scalar) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float2 anti_wedge_g0 = float2(this.e1234) * other.group0.xy;
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float3 anti_wedge_g3 = float3(this.e1234) * other.group3.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0[0] * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1_w * this.e1234 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g1_w * anti_wedge_g0[0]), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0[0])) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0[0])), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Point other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e1234 */
            (right_anti_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<DualNum> operator /(DualNum lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float2(right_anti_dual_g0_w * -1.0) * anti_wedge_g0.xy), right_anti_dual_g0_w * anti_wedge_g0[2] * -1.0, (right_anti_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_anti_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_anti_dual_g1_xyz[2] * anti_wedge_g0[2]) - (right_anti_dual_g0_w * this.e1234 * other.e321)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Line other) {
        float3 anti_wedge_g1 = float3(this.e1234) * other.group1.xyz;
        return AntiScalar(
            /* e1234 */
            -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group1.xyz;
        float anti_wedge_g1_w = (this.scalar * other.e1234) + (this.e1234 * other.scalar);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float2(anti_wedge_g1_w) * other.group1.xy), anti_wedge_g1_w * other.e12, (anti_wedge_g1_w * other.scalar) - (anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.scalar * other.e1234) + (this.e1234 * other.scalar);
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float3 anti_wedge_g3 = float3(this.e1234) * other.group3.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1_w * this.e1234 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0_x)) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Point other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e1234 */
            (right_anti_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Flector> operator /(Flector lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g1 = (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1));
        float right_anti_dual_g0_w = other.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0_w * anti_wedge_g1[3]), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_anti_dual_g0_w) * anti_wedge_g1.xyz) + (float3(anti_wedge_g1[3]) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(Horizon other) {
        float4 anti_wedge_g1 = float4(other.e321) * float4(this.group1.xyz, this.e4);
        float right_anti_dual_g0 = other.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0 * anti_wedge_g1[3]), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual_g0) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Plane reject_orthogonally_from(Line other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41));
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((right_anti_dual_g0.yzx * anti_wedge_g0.zxy) - (right_anti_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Plane reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41));
        float4 right_anti_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 anti_wedge_g1 = float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.zxy, other.e41));
        float3 anti_wedge_g3 = (float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz);
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1_w * this.e321 * other.e1234)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0_x)) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g1 = float4(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1));
        float right_anti_dual_g0 = other.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0 * anti_wedge_g1[3]), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual_g0) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Plane reject_orthogonally_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            -float4((float3(this.e321 * other.e4) * other.group0.xyz), 0.0) - float4((other.group0.xyz * other.group0.xyz * this.group1.xyz), 0.0) - float4((float3(other.e1) * this.group1.yxx * other.group0.yyz), 0.0) - float4((float3(other.e3) * this.group1.zzy * other.group0.xyy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Horizon> operator /(Horizon lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Flector reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g1 = float4(this.e321 * -1.0) * float4(other.group1.xyz, other.e4);
        float right_anti_dual_g0_w = other.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0_w * anti_wedge_g1[3]), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_anti_dual_g0_w) * anti_wedge_g1.xyz) + (float3(anti_wedge_g1[3]) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Plane reject_orthogonally_from(Line other) {
        float4 anti_wedge_g0 = float4((float3(this.e321) * other.group0.xyz), 0.0);
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((right_anti_dual_g0.yzx * anti_wedge_g0.zxy) - (right_anti_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Plane reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4((float3(this.e321) * other.group0.xyz), 0.0);
        float4 right_anti_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Horizon {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = this.e321 * other.e4 * -1.0;
        float3 anti_wedge_g1_xyz = float3(this.e321) * other.group2.xyz;
        float3 anti_wedge_g3 = float3(this.e321 * -1.0) * other.group4.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g1_xyz[0] * right_anti_dual_g4_xyz[0]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g4_xyz[1]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g4_xyz[2]) - (anti_wedge_g0_x * other.scalar) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1_w * this.e321 * other.e1234)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w * -1.0), 
            /* e41, e42, e43 */
            -(anti_wedge_g1_xyz * float3(right_anti_dual_g1_w)) - (right_anti_dual_g2 * float3(anti_wedge_g0_x)), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((anti_wedge_g1_xyz.zxy * right_anti_dual_g2.yzx), 0.0) - float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Plane reject_orthogonally_from(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e321) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Horizon {
    public Plane reject_orthogonally_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Line> operator /(Line lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41));
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual_g0_w * anti_wedge_g0[0], right_anti_dual_g0_w * anti_wedge_g0[1], right_anti_dual_g0_w * anti_wedge_g0[2], (right_anti_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_anti_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_anti_dual_g1_xyz[2] * anti_wedge_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e321 * other.e321) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(Line other) {
        return Line(
            /* e41, e42, e43 */
            (float3(other.e41 * this.e23) * other.group1.xyz) + (float3(other.e42 * this.e31) * other.group1.xyz) + (float3(other.e43 * this.e12) * other.group1.xyz) + (other.group1.xyz * other.group1.xyz * this.group0.xyz) + (other.group1.xyz * other.group1.yxx * this.group0.yxx) + (other.group1.xyz * other.group1.zzy * this.group0.zzy), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float anti_wedge_g1_w = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float2(anti_wedge_g1_w) * other.group1.xy), anti_wedge_g1_w * other.e12, (anti_wedge_g1_w * other.scalar) - (anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 anti_wedge_g1 = float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41));
        float3 anti_wedge_g3 = float3(other.e1234) * this.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0_x)) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(Plane other) {
        return Line(
            /* e41, e42, e43 */
            (float3(other.e321 * other.e321) * this.group0.xyz) + (float3(other.e321) * this.group1.yzx * other.group0.zxy) - (float3(other.e321) * this.group1.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Motor> operator /(Motor lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiScalar reject_orthogonally_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41));
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float2(right_anti_dual_g0_w * -1.0) * anti_wedge_g0.xy), right_anti_dual_g0_w * anti_wedge_g0[2] * -1.0, (right_anti_dual_g1_xyz[0] * anti_wedge_g0[0]) + (right_anti_dual_g1_xyz[1] * anti_wedge_g0[1]) + (right_anti_dual_g1_xyz[2] * anti_wedge_g0[2]) - (right_anti_dual_g0_w * other.e321 * this.e1234)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4((float3(other.e321) * this.group0.xyz), other.e321 * this.e1234), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Line other) {
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group1.xyz;
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((right_anti_dual_g0 * float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))), -(anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g1 = (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(anti_wedge_g1[3]) * other.group1.xyz), (anti_wedge_g1[3] * other.scalar) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 anti_wedge_g1 = float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(float3(other.e321), other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41));
        float3 anti_wedge_g3 = (float3(this.e1234) * other.group3.xyz) + (float3(other.e1234) * this.group1.xyz);
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1_w * this.e1234 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0_x)) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4((float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41))).xyz, this.e1234 * other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiScalar reject_orthogonally_from(Point other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e1234 */
            (right_anti_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_anti_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_anti_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiScalar reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<MultiVector> operator /(MultiVector lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public AntiScalar reject_orthogonally_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Flector other) {
        float anti_wedge_g0_x = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 anti_wedge_g1 = float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.zxy, this.e41));
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual_g1_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g1_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g1_xyz[2] * anti_wedge_g1[2]) - (right_anti_dual_g0_w * other.e321 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g0_w), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0_w * anti_wedge_g1[0], right_anti_dual_g0_w * anti_wedge_g1[1], right_anti_dual_g0_w * anti_wedge_g1[2]) * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual_g1_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((float3(right_anti_dual_g0_w * other.e321) * this.group4.xyz), 0.0) - float4((float3(right_anti_dual_g0_w * this.e321) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Horizon other) {
        float right_anti_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, right_anti_dual_g0 * other.e321 * this.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0 * other.e321 * this.e4), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0 * other.e321 * -1.0) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual_g0 * other.e321) * this.group4.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Line other) {
        float4 anti_wedge_g1 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41));
        float3 anti_wedge_g3 = float3(this.e1234) * other.group1.xyz;
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            -(right_anti_dual_g0 * float3(other.e41 * this.e23)) - (right_anti_dual_g0 * float3(other.e42 * this.e31)) - (right_anti_dual_g0 * float3(other.e43 * this.e12)) - (right_anti_dual_g0 * float3(other.e23 * this.e41)) - (right_anti_dual_g0 * float3(other.e31 * this.e42)) - (right_anti_dual_g0 * float3(other.e12 * this.e43)), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((right_anti_dual_g0.yzx * anti_wedge_g1.zxy) - (right_anti_dual_g0.zxy * anti_wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Motor other) {
        float anti_wedge_g0_x = (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 anti_wedge_g1 = float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(float3(this.e321), this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41));
        float3 anti_wedge_g3 = (float3(other.e1234) * this.group3.xyz) + (float3(this.e1234) * other.group1.xyz);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 anti_wedge_g1 = float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((this.group3.zxy * other.group4.yzx), other.e42 * this.e431);
        float3 anti_wedge_g3 = (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3.xyz) - (float3(this.e321) * other.group4.xyz);
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1_w * other.e1234 * this.e321) - (right_anti_dual_g1_w * other.e321 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0_x)) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Plane other) {
        float right_anti_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, right_anti_dual_g0 * this.e1234 * other.e321) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (right_anti_dual_g0 * this.e1 * other.e423) + (right_anti_dual_g0 * this.e2 * other.e431) + (right_anti_dual_g0 * this.e3 * other.e412) + (right_anti_dual_g0 * this.e4 * other.e321)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g0) * this.group3.zxy * other.group0.yzx) - (float3(right_anti_dual_g0 * other.e321) * this.group2.xyz) - (float3(right_anti_dual_g0) * this.group3.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_anti_dual_g0 * other.e321) * this.group4.xyz) - (float3(right_anti_dual_g0 * this.e321) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Point other) {
        float4 anti_wedge_g1 = float4(this.e1234) * other.group0;
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual_g0_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g0_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g0_xyz[2] * anti_wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            -float4((right_anti_dual_g0_xyz * float3(this.e423 * other.e1)), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e431 * other.e2)), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e412 * other.e3)), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e321 * other.e4)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public AntiScalar reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Origin> operator /(Origin lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Origin {
    public Flector reject_orthogonally_from(Flector other) {
        float anti_wedge_g0 = other.e321 * this.e4;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(RejectOrthogonallyFromInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Origin {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = other.e321 * this.e4;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, anti_wedge_g0_x * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x * -1.0) * other.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(RejectOrthogonallyFromInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Origin {
    public Origin reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            this.e4 * other.e321 * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Plane> operator /(Plane lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g1 = float4(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412, -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1));
        float right_anti_dual_g0_w = other.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual_g0_w * anti_wedge_g1[3]), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_anti_dual_g0_w) * anti_wedge_g1.xyz) + (float3(anti_wedge_g1[3]) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(Horizon other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * other.e321 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(Line other) {
        float4 anti_wedge_g0 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41));
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((right_anti_dual_g0.yzx * anti_wedge_g0.zxy) - (right_anti_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41));
        float4 right_anti_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 anti_wedge_g1 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41));
        float3 anti_wedge_g3 = (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz);
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g2[0]) - (anti_wedge_g3[1] * right_anti_dual_g2[1]) - (anti_wedge_g3[2] * right_anti_dual_g2[2]) - (right_anti_dual_g1_w * other.e1234 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0_x)) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_anti_dual_g1_w)), 0.0) + float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((float3(other.e321 * this.e321) * other.group0.xyz) - (float3(other.e321 * other.e321) * this.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            -float4((float3(this.e321 * other.e4) * other.group0.xyz), 0.0) - float4((other.group0.xyz * other.group0.xyz * this.group0.xyz), 0.0) - float4((float3(other.e1) * this.group0.yxx * other.group0.yyz), 0.0) - float4((float3(other.e3) * this.group0.zzy * other.group0.xyy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Point> operator /(Point lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Point {
    public Flector reject_orthogonally_from(Flector other) {
        float anti_wedge_g0 = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(RejectOrthogonallyFromInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Point {
    public Origin reject_orthogonally_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Point {
    public Plane reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e1234) * this.group0;
        float4 right_anti_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Point {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 anti_wedge_g1 = float4(other.e1234) * this.group0;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (right_anti_dual_g4_xyz[0] * anti_wedge_g1[0]) + (right_anti_dual_g4_xyz[1] * anti_wedge_g1[1]) + (right_anti_dual_g4_xyz[2] * anti_wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            (right_anti_dual_g2 * float3(anti_wedge_g0_x)) - (float3(right_anti_dual_g1_w) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual_g4_xyz * float3(anti_wedge_g0_x)), 0.0) + float4((right_anti_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(RejectOrthogonallyFromInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Point {
    public Origin reject_orthogonally_from(Plane other) {
        return Origin(
            /* e4 */
            -(other.e321 * other.e321 * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Scalar> operator /(Scalar lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiScalar reject_orthogonally_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Motor reject_orthogonally_from(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234 * this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0 = other.e1234 * this.scalar;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, anti_wedge_g0 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0 * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0 * -1.0) * other.group3.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0) * other.group1.xyz), 0.0)
        );
    }
}
