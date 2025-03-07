implementing rga3d;
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public Motor reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0.wwwx * float4(right_dual_g0.xyz, other.e1)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2]) - (anti_wedge_g1[3] * right_dual_g0[3])) - (right_dual_g0.wwwx * float4(anti_wedge_g0.xyz, anti_wedge_g1[0])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Line other) {
        float3 anti_wedge_g1 = float3(this.e1234) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return AntiScalar(
            /* e1234 */
            -(anti_wedge_g1[0] * right_dual_g0[0]) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public Motor reject_via_origin_from(Motor other) {
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (right_dual_g0 * float4(anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g1[0] * right_dual_g0[0]) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float2 anti_wedge_g0 = float2(this.e1234) * other.group0.xy;
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float3 anti_wedge_g2 = float3(this.e1234) * other.group2.xyz;
        float3 anti_wedge_g3 = float3(this.e1234) * other.group3.xyz;
        float4 anti_wedge_g4 = float4(this.e1234) * other.group4;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0[0] * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2]) - (anti_wedge_g4[0] * right_dual_g1[0]) - (anti_wedge_g4[1] * right_dual_g1[1]) - (anti_wedge_g4[2] * right_dual_g1[2]) - (anti_wedge_g4[3] * right_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0[0] * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0[0])) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0[0]) * other.group1.xyz), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g1.zxy), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g1.yzx), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Point rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Point other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        return AntiScalar(
            /* e1234 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<DualNum> operator /(DualNum lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public Motor reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0.wwwx * float4(right_dual_g0.xyz, other.e1)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2]) - (anti_wedge_g1[3] * right_dual_g0[3])) - (right_dual_g0.wwwx * float4(anti_wedge_g0.xyz, anti_wedge_g1[0])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Horizon other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Line other) {
        float3 anti_wedge_g1 = float3(this.e1234) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return AntiScalar(
            /* e1234 */
            -(anti_wedge_g1[0] * right_dual_g0[0]) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public Motor reject_via_origin_from(Motor other) {
        float4 anti_wedge_g1 = float4(this.group0.yy, this.e1234, (this.scalar * other.e1234) + (this.e1234 * other.scalar)) * float4(other.group1.xyz, 1.0);
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (right_dual_g0 * float4(anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g1[0] * right_dual_g0[0]) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.scalar * other.e1234) + (this.e1234 * other.scalar);
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float3 anti_wedge_g2 = float3(this.e1234) * other.group2.xyz;
        float3 anti_wedge_g3 = float3(this.e1234) * other.group3.xyz;
        float4 anti_wedge_g4 = float4(this.e1234) * other.group4;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2]) - (anti_wedge_g4[0] * right_dual_g1[0]) - (anti_wedge_g4[1] * right_dual_g1[1]) - (anti_wedge_g4[2] * right_dual_g1[2]) - (anti_wedge_g4[3] * right_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g1.zxy), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g1.yzx), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Plane other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.e321, 2)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Point other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        return AntiScalar(
            /* e1234 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Flector> operator /(Flector lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0_xyz = (other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx);
        float4 anti_wedge_g1 = (other.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group1.wwwx * float4(other.group1.xyz, other.e1));
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g1[3] * right_dual_g0[3]), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0) + float4((float3(right_dual_g0[3]) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0_xyz.yzx * right_dual_g0.zxy), 0.0) - float4((anti_wedge_g0_xyz.zxy * right_dual_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(Horizon other) {
        float4 anti_wedge_g1 = float4(other.e321) * float4(this.group1.xyz, this.e4);
        float right_dual_g0 = other.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual_g0 * anti_wedge_g1[3]), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual_g0) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Plane reject_via_origin_from(Line other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41));
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((right_dual_g0.yzx * anti_wedge_g0.zxy) - (right_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Plane reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e41));
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0.zxy * right_dual_g0.yzx) - (anti_wedge_g0.yzx * right_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4);
        float4 anti_wedge_g1 = float4((this.e412 * other.e31) + (this.e321 * other.e41), (this.e423 * other.e12) + (this.e321 * other.e42), (this.e431 * other.e23) + (this.e321 * other.e43), -(this.e431 * other.e42) - (this.e412 * other.e43)) + (float4(other.e1234) * this.group0) - (this.group1.yzxx * float4(other.group3.zxy, other.e41));
        float3 anti_wedge_g2 = (this.group1.zxy * other.group4.yzx) - (this.group1.yzx * other.group4.zxy);
        float3 anti_wedge_g3 = (float3(other.e321) * this.group1.xyz) - (float3(this.e321) * other.group4.xyz);
        float4 anti_wedge_g4 = float4(other.e1234) * this.group1;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2]) - (anti_wedge_g4[0] * right_dual_g1[0]) - (anti_wedge_g4[1] * right_dual_g1[1]) - (anti_wedge_g4[2] * right_dual_g1[2]) - (anti_wedge_g4[3] * right_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g1.zxy), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g1.yzx), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(Plane other) {
        float4 anti_wedge_g1 = (float4(this.e321 * other.e423, this.e321 * other.e431, this.e321 * other.e412, (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.wwwx * float4(this.group1.xyz, this.e1));
        float right_dual_g0 = other.e321 * -1.0;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual_g0 * anti_wedge_g1[3]), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual_g0) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Plane reject_via_origin_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Horizon> operator /(Horizon lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Horizon {
    public Flector reject_via_origin_from(Flector other) {
        float4 anti_wedge_g1 = float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0);
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g1[3] * right_dual_g0[3]), 
            /* e423, e431, e412, e321 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + (float3(right_dual_g0[3]) * anti_wedge_g1.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Horizon> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Horizon {
    public Plane reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e321) * other.group0.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g0_xyz.yzx * right_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Horizon> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Horizon {
    public Plane reject_via_origin_from(Motor other) {
        float3 anti_wedge_g0_xyz = float3(this.e321) * other.group0.xyz;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g0_xyz.yzx * right_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Horizon {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float2 anti_wedge_g0 = float2(this.e321 * other.e4, 1.0) * float2(-1.0, 0.0);
        float3 anti_wedge_g1_xyz = float3(this.e321) * other.group2.xyz;
        float3 anti_wedge_g3 = float3(this.e321) * other.group4.xyz * float3(-1.0);
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0[0] * other.scalar) + (anti_wedge_g1_xyz[0] * other.e1) + (anti_wedge_g1_xyz[1] * other.e2) + (anti_wedge_g1_xyz[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2]) - (right_dual_g1[3] * this.e321 * other.e1234)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0[0] * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0[0])) - (anti_wedge_g1_xyz * float3(right_dual_g1[3])), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0[0]) * other.group1.xyz), 0.0) + float4((anti_wedge_g1_xyz.zxy * right_dual_g2.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * right_dual_g2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Horizon {
    public Plane reject_via_origin_from(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(this.e321 * other.e423 * -1.0, this.e321 * other.e431 * -1.0, this.e321 * other.e412 * -1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Horizon> lhs, Point rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Horizon {
    public Plane reject_via_origin_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(this.e321 * other.e4 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Line> operator /(Line lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Motor reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41));
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0.wwwx * float4(right_dual_g0.xyz, other.e1)) + float4((float3(right_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0)), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float3(other.e321 * this.e41, other.e321 * this.e42, other.e321 * this.e43) * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(Line other) {
        return Line(
            /* e41, e42, e43 */
            float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) * float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Motor reject_via_origin_from(Motor other) {
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (right_dual_g0 * float4(-(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43))) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * right_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_dual_g0[2])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 anti_wedge_g1 = float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41));
        float3 anti_wedge_g2 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g3 = float3(other.e1234) * this.group1.xyz;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g1.zxy), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g1.yzx), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(Plane other) {
        return Line(
            /* e41, e42, e43 */
            float3(other.e321 * -1.0) * float3((this.e41 * other.e321) + (this.e31 * other.e412) - (this.e12 * other.e431), (this.e42 * other.e321) + (this.e12 * other.e423) - (this.e23 * other.e412), (this.e43 * other.e321) + (this.e23 * other.e431) - (this.e31 * other.e423)) * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Motor> operator /(Motor lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41));
        float4 anti_wedge_g1 = float4(this.e1234) * other.group1;
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (anti_wedge_g0.wwwx * float4(right_dual_g0.xyz, other.e1)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2]) - (anti_wedge_g1[3] * right_dual_g0[3])) - (right_dual_g0.wwwx * float4(anti_wedge_g0.xyz, anti_wedge_g1[0])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4((float3(other.e321) * this.group0.xyz), other.e321 * this.e1234) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Line other) {
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((right_dual_g0 * float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43))), -(anti_wedge_g1_xyz[0] * right_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_dual_g0[2])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Motor other) {
        float4 anti_wedge_g1 = (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43));
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            (right_dual_g0 * float4(anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g1[0] * right_dual_g0[0]) - (anti_wedge_g1[1] * right_dual_g0[1]) - (anti_wedge_g1[2] * right_dual_g0[2])), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1234 * other.scalar) + (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43);
        float4 anti_wedge_g1 = float4((this.e1234 * other.e1) + (this.e31 * other.e412), (this.e1234 * other.e2) + (this.e12 * other.e423), (this.e1234 * other.e3) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) + (this.group0 * float4(other.group4.www, other.e4)) - (other.group4.yzxx * float4(this.group1.zxy, this.e41));
        float3 anti_wedge_g2 = (float3(this.e1234) * other.group2.xyz) + (float3(other.e1234) * this.group0.xyz);
        float3 anti_wedge_g3 = (float3(this.e1234) * other.group3.xyz) + (float3(other.e1234) * this.group1.xyz);
        float4 anti_wedge_g4 = float4(this.e1234) * other.group4;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2]) - (anti_wedge_g4[0] * right_dual_g1[0]) - (anti_wedge_g4[1] * right_dual_g1[1]) - (anti_wedge_g4[2] * right_dual_g1[2]) - (anti_wedge_g4[3] * right_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g1.zxy), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g1.yzx), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Plane other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e321 * -1.0) * float4((float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41))).xyz, this.e1234 * other.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public AntiScalar reject_via_origin_from(Point other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        return AntiScalar(
            /* e1234 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<MultiVector> operator /(MultiVector lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            (other.scalar * other.scalar * this.e1234) + (other.scalar * other.e1234 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Flector other) {
        float anti_wedge_g0_x = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 anti_wedge_g1 = float4((other.e412 * this.e31) + (other.e321 * this.e41), (other.e423 * this.e12) + (other.e321 * this.e42), (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(this.e1234) * other.group0) - (other.group1.yzxx * float4(this.group3.zxy, this.e41));
        float3 anti_wedge_g2 = (other.group1.yzx * this.group4.zxy) - (other.group1.zxy * this.group4.yzx);
        float4 anti_wedge_g4 = float4(this.e1234) * other.group1;
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g4[0] * right_dual_g0[0]) - (anti_wedge_g4[1] * right_dual_g0[1]) - (anti_wedge_g4[2] * right_dual_g0[2]) - (anti_wedge_g4[3] * right_dual_g0[3])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g0[3]), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g1[3]) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0_x) * other.group0.xyz), 0.0) + float4((float3(right_dual_g0[3]) * ((float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group1.xyz))), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g0.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Horizon other) {
        float right_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, right_dual_g0 * other.e321 * this.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_dual_g0 * other.e321 * this.e4), 
            /* e41, e42, e43 */
            float3(right_dual_g0) * float3(other.e321) * this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual_g0) * float3(other.e321) * this.group4.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Line other) {
        float4 anti_wedge_g1 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41));
        float3 anti_wedge_g3 = float3(this.e1234) * other.group1.xyz;
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, -(anti_wedge_g3[0] * right_dual_g0[0]) - (anti_wedge_g3[1] * right_dual_g0[1]) - (anti_wedge_g3[2] * right_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            right_dual_g0 * float3(-(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((right_dual_g0.yzx * anti_wedge_g1.zxy) - (right_dual_g0.zxy * anti_wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Motor other) {
        float anti_wedge_g0_x = (other.e1234 * this.scalar) + (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 anti_wedge_g1 = float4((other.e1234 * this.e1) + (other.e31 * this.e412), (other.e1234 * this.e2) + (other.e12 * this.e423), (other.e1234 * this.e3) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) + (other.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(other.group1.zxy, other.e41));
        float3 anti_wedge_g3 = (float3(other.e1234) * this.group3.xyz) + (float3(this.e1234) * other.group1.xyz);
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * right_dual_g0[3]) - (anti_wedge_g3[0] * right_dual_g0[0]) - (anti_wedge_g3[1] * right_dual_g0[1]) - (anti_wedge_g3[2] * right_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * right_dual_g0.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g1.zxy * right_dual_g0.yzx) - (anti_wedge_g1.yzx * right_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e1234) + (other.e1234 * this.scalar) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43);
        float4 anti_wedge_g1 = float4((other.e41 * this.e321) + (other.e31 * this.e412) + (other.e412 * this.e31) + (other.e321 * this.e41), (other.e42 * this.e321) + (other.e12 * this.e423) + (other.e423 * this.e12) + (other.e321 * this.e42), (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e431 * this.e23) + (other.e321 * this.e43), -(other.e43 * this.e412) - (other.e423 * this.e41) - (other.e431 * this.e42) - (other.e412 * this.e43)) + (float4(other.e1234) * this.group1) + (float4(this.e1234) * other.group1) - (this.group4.yzxx * float4(other.group3.zxy, other.e41)) - float4((this.group3.zxy * other.group4.yzx), other.e42 * this.e431);
        float3 anti_wedge_g2 = (float3(other.e1234) * this.group2.xyz) + (float3(this.e1234) * other.group2.xyz) + (other.group4.yzx * this.group4.zxy) - (other.group4.zxy * this.group4.yzx);
        float3 anti_wedge_g3 = (float3(other.e1234) * this.group3.xyz) + (float3(other.e321) * this.group4.xyz) + (float3(this.e1234) * other.group3.xyz) - (float3(this.e321) * other.group4.xyz);
        float4 anti_wedge_g4 = (float4(other.e1234) * this.group4) + (float4(this.e1234) * other.group4);
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2]) - (anti_wedge_g4[0] * right_dual_g1[0]) - (anti_wedge_g4[1] * right_dual_g1[1]) - (anti_wedge_g4[2] * right_dual_g1[2]) - (anti_wedge_g4[3] * right_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g1.zxy), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g1.yzx), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Plane other) {
        float right_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, right_dual_g0 * this.e1234 * other.e321) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (right_dual_g0 * this.e1 * other.e423) + (right_dual_g0 * this.e2 * other.e431) + (right_dual_g0 * this.e3 * other.e412) + (right_dual_g0 * this.e4 * other.e321)), 
            /* e41, e42, e43 */
            float3(right_dual_g0) * (float4((this.e41 * other.e321) + (this.e31 * other.e412), (this.e42 * other.e321) + (this.e12 * other.e423), (this.e43 * other.e321) + (this.e23 * other.e431), -(this.e42 * other.e431) - (this.e43 * other.e412)) - (other.group0.yzxx * float4(this.group3.zxy, this.e41))).xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_dual_g0) * ((float3(other.e321) * this.group4.xyz) - (float3(this.e321) * other.group0.xyz))), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Point other) {
        float4 anti_wedge_g1 = float4(this.e1234) * other.group0;
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Origin> operator /(Origin lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Origin>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Origin> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Origin {
    public Flector reject_via_origin_from(Flector other) {
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
public Origin operator /(RejectViaOriginFromInfixPartial<Origin> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Origin {
    public Origin reject_via_origin_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Origin> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Origin {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float2 anti_wedge_g0 = float2(other.e321 * this.e4, 1.0) * float2(1.0, 0.0);
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, anti_wedge_g0[0] * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0[0] * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (float3(other.e1234 * this.e4) * right_dual_g1.xyz) - (float3(anti_wedge_g0[0]) * other.group3.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0[0]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(RejectViaOriginFromInfixPartial<Origin> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Origin {
    public Origin reject_via_origin_from(Plane other) {
        return Origin(
            /* e4 */
            this.e4 * pow(other.e321, 2) * -1.0
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Plane> operator /(Plane lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Flector reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4((float3(other.e321) * this.group0.xyz), -(other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) - (this.group0.wwwx * float4(other.group1.xyz, other.e1));
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g1[3] * right_dual_g0[3]), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0) + float4((float3(right_dual_g0[3]) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0_xyz.yzx * right_dual_g0.zxy), 0.0) - float4((anti_wedge_g0_xyz.zxy * right_dual_g0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(Horizon other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3(other.e321 * this.e423, other.e321 * this.e431, other.e321 * this.e412)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(Line other) {
        float4 anti_wedge_g0 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41));
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((right_dual_g0.yzx * anti_wedge_g0.zxy) - (right_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41));
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0.zxy * right_dual_g0.yzx) - (anti_wedge_g0.yzx * right_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321);
        float4 anti_wedge_g1 = float4((other.e41 * this.e321) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e12 * this.e423), (other.e43 * this.e321) + (other.e23 * this.e431), -(other.e42 * this.e431) - (other.e43 * this.e412)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41));
        float3 anti_wedge_g2 = (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx);
        float3 anti_wedge_g3 = (float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group4.xyz);
        float4 anti_wedge_g4 = float4(other.e1234) * this.group0;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g3[0] * right_dual_g2[0]) - (anti_wedge_g3[1] * right_dual_g2[1]) - (anti_wedge_g3[2] * right_dual_g2[2]) - (anti_wedge_g4[0] * right_dual_g1[0]) - (anti_wedge_g4[1] * right_dual_g1[1]) - (anti_wedge_g4[2] * right_dual_g1[2]) - (anti_wedge_g4[3] * right_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge_g3 * float3(right_dual_g1[3])), 0.0) + float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0) + float4((anti_wedge_g2.yzx * right_dual_g1.zxy), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((anti_wedge_g2.zxy * right_dual_g1.yzx), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(other.e321 * -1.0) * float3((other.e321 * this.e423) - (other.e423 * this.e321), (other.e321 * this.e431) - (other.e431 * this.e321), (other.e321 * this.e412) - (other.e412 * this.e321))), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Point rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(Point other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((float3(-(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Point> operator /(Point lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Point {
    public Flector reject_via_origin_from(Flector other) {
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
public Origin operator /(RejectViaOriginFromInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Point {
    public Origin reject_via_origin_from(Horizon other) {
        return Origin(
            /* e4 */
            other.e321 * other.e321 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Point {
    public Plane reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e1234) * this.group0;
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e423, e431, e412, e321 */
            float4(((anti_wedge_g0.zxy * right_dual_g0.yzx) - (anti_wedge_g0.yzx * right_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Point {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4);
        float4 anti_wedge_g1 = float4(other.e1234) * this.group0;
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0_x * right_dual_g1[3]), 
            /* e41, e42, e43 */
            (right_dual_g2 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0) + float4((right_dual_g2.yzx * anti_wedge_g1.zxy), 0.0) - float4((right_dual_g2.zxy * anti_wedge_g1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Origin operator /(RejectViaOriginFromInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Point {
    public Origin reject_via_origin_from(Plane other) {
        return Origin(
            /* e4 */
            -(other.e321 * other.e321 * this.e4) - (other.e423 * other.e321 * this.e1) - (other.e431 * other.e321 * this.e2) - (other.e412 * other.e321 * this.e3)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Scalar> operator /(Scalar lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public AntiScalar reject_via_origin_from(DualNum other) {
        return AntiScalar(
            /* e1234 */
            other.scalar * other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public Motor reject_via_origin_from(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e1234 * this.scalar) * float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, other.scalar), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0 = other.e1234 * this.scalar;
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, anti_wedge_g0 * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), anti_wedge_g0 * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(anti_wedge_g0) * other.group1.xyz), 0.0)
        );
    }
}
