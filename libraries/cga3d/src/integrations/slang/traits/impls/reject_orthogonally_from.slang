implementing cga3d;
using traits;
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint reject_orthogonally_from(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group0.xyz * float3(other.group0.xx, other.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint reject_orthogonally_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Flector reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e45);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g0.xyz * right_anti_dual_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g0.zxy, -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]))) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Dipole reject_orthogonally_from(Circle other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g2[3])) + (right_anti_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g2_xyz[0] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g2_xyz[1] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g2_xyz[2] * right_anti_dual_g2[3]), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * right_anti_dual_g2[2]) + (anti_wedge_g2_xyz[2] * other.e431) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]), (anti_wedge_g0[2] * right_anti_dual_g2[0]) + (anti_wedge_g2_xyz[0] * other.e412) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]), (anti_wedge_g0[0] * right_anti_dual_g2[1]) + (anti_wedge_g2_xyz[1] * other.e423) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]), -(anti_wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((this.group2.xyz * other.group2.www), this.e42 * other.e4315);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * right_anti_dual_g3.zxyw) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g1 * right_anti_dual_g2[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4((float2(anti_wedge_g1) * right_anti_dual_g3.xy * float2(-1.0)), anti_wedge_g1 * right_anti_dual_g3[2] * -1.0, (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(anti_wedge_g1) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g2 = float4(other.e12345) * this.group2;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0[1]) * float4((float3(other.e12345) * this.group0.xyz), anti_wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2.xyz * float3(right_anti_dual_g0.yy, right_anti_dual_g0[1])), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g0[0] * anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41));
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g1.yzx)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g1[3]) * anti_wedge_g0.xyz) - (float3(anti_wedge_g1) * right_anti_dual_g1.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz), -(anti_wedge_g1 * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public AntiLine reject_orthogonally_from(Line other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g0, anti_wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g2_w) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * right_anti_dual_g1[0]) + (anti_wedge_g2_xyz[0] * right_anti_dual_g0[3]), (anti_wedge_g2_w * right_anti_dual_g1[1]) + (anti_wedge_g2_xyz[1] * right_anti_dual_g0[3]), (anti_wedge_g2_w * right_anti_dual_g1[2]) + (anti_wedge_g2_xyz[2] * right_anti_dual_g0[3]), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g0[2], 1.0) * float4(anti_wedge_g1.www, -(anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]))) + (right_anti_dual_g1.zxyw * float4(anti_wedge_g0.yzx, anti_wedge_g2_w)) - (right_anti_dual_g1.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 anti_wedge_g1 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(other.e1234) * this.group2.xyz), this.e42 * other.e4315);
        float anti_wedge_g2 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 anti_wedge_g3 = float4(other.e12345) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(other.e12345) * this.group1.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), 0.0) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), 0.0) + float4((other.group7.yzx * anti_wedge_g3.zxy), 0.0) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4(float3(0.0), anti_wedge_g3[2] * other.e435) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Dipole reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35 */
            (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz) - (float3((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public Dipole reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(other.e1234) * this.group2.xyz), this.e42 * other.e4315);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Dipole(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_anti_dual_g0.zxy, other.e3215)) - float4((anti_wedge_g0.zxy * right_anti_dual_g0.yzx), anti_wedge_g1 * right_anti_dual_g0[3]), 
            /* e15, e25, e35 */
            (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g1) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float4 anti_wedge_g2 = float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g2[3])) + (float4(right_anti_dual_g2[0], right_anti_dual_g2[1], right_anti_dual_g2[2], 1.0) * float4(anti_wedge_g2.www, -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)) - float4((this.group2.xyz * other.group2.www), this.e42 * other.e4315);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g1 * right_anti_dual_g3[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g0.xyzx * float4(right_anti_dual_g2.www, right_anti_dual_g0[0])) + float4((float2(anti_wedge_g1) * right_anti_dual_g3.xy * float2(-1.0)), anti_wedge_g1 * right_anti_dual_g3[2] * -1.0, (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])) - float4((float3(anti_wedge_g1) * right_anti_dual_g0.xyz), anti_wedge_g0[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.yzx * other.group2.zxy), other.e431 * this.e425);
        float anti_wedge_g1 = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e423) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g1 * other.e431) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g1 * other.e412) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual_g2[1], right_anti_dual_g2[2], right_anti_dual_g2[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g3[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(anti_wedge_g1) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (anti_wedge_g0.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Motor reject_orthogonally_from(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e4);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.scalar), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Line reject_orthogonally_from(AntiFlatPoint other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e315) + (this.e415 * other.e321), (this.e423 * other.e125) + (this.e425 * other.e321), (this.e431 * other.e235) + (this.e435 * other.e321), -(this.e425 * other.e315) - (this.e435 * other.e125)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e315) + (this.e415 * other.e321), (this.e423 * other.e125) + (this.e425 * other.e321), (this.e431 * other.e235) + (this.e435 * other.e321), -(this.e425 * other.e315) - (this.e435 * other.e125)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0)), (anti_wedge_g0[0] * right_anti_dual_g1[0]) + (anti_wedge_g0[1] * right_anti_dual_g1[1]) + (anti_wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Line reject_orthogonally_from(AntiLine other) {
        float anti_wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.e3215) * this.group0.xyz;
        float anti_wedge_g0_w = (this.e4 * other.e3215) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz), (anti_wedge_g0_w * right_anti_dual_g0[3]) - (anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g1.xyz)), anti_wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Circle reject_orthogonally_from(Circle other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group0.yzx * other.group2.zxy), this.e423 * other.e415) - float4((other.group0.yzx * this.group2.zxy), this.e431 * other.e425);
        float anti_wedge_g1 = -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e423) + (anti_wedge_g0[3] * other.e235), (anti_wedge_g1 * other.e431) + (anti_wedge_g0[3] * other.e315), (anti_wedge_g1 * other.e412) + (anti_wedge_g0[3] * other.e125), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * right_anti_dual_g1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float4 anti_wedge_g3 = float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e1 * other.e12345), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e2 * other.e12345), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e3 * other.e12345), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group2.yzxw * float4(this.group0.zxy, this.e5)) - (other.group2.zxyx * float4(this.group0.yzx, this.e415)) - float4((other.group0.yzx * this.group2.zxy), this.e425 * other.e315);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g2_w * right_anti_dual_g1[0]) + (anti_wedge_g0[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[2] * other.e431), (anti_wedge_g2_w * right_anti_dual_g1[1]) + (anti_wedge_g0[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[0] * other.e412), (anti_wedge_g2_w * right_anti_dual_g1[2]) + (anti_wedge_g0[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[1] * other.e423), -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g1[0] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g1[1] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g1[2] * right_anti_dual_g2[3]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g1[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g3[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g3[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g2[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Circle reject_orthogonally_from(Dipole other) {
        float anti_wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 anti_wedge_g1 = float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx);
        float4 anti_wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[1] * right_anti_dual_g3[2]) + (right_anti_dual_g0[0] * anti_wedge_g2[3]) + (anti_wedge_g1[0] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g3[0]) + (right_anti_dual_g0[1] * anti_wedge_g2[3]) + (anti_wedge_g1[1] * right_anti_dual_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g3[1]) + (right_anti_dual_g0[2] * anti_wedge_g2[3]) + (anti_wedge_g1[2] * right_anti_dual_g2[3]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[0] * anti_wedge_g2[0]) - (right_anti_dual_g0[1] * anti_wedge_g2[1]) - (right_anti_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[0]), (anti_wedge_g2[1] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[1]), (anti_wedge_g2[2] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[2]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g0 * right_anti_dual_g3.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g2[3] * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g2 = float4(other.e12345) * this.group2;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], right_anti_dual_g0[0]) * float4((float3(other.e12345) * this.group0.xyz), anti_wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0[1]) * float4(anti_wedge_g2.xyz, anti_wedge_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[1]) * float4(anti_wedge_g3.xyz, anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlatPoint reject_orthogonally_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx);
        float4 anti_wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group2.zxy * other.group1.yzx), this.e423 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], 1.0) * float4(anti_wedge_g0.yzx, -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3]))) - float4((anti_wedge_g0.zxy * right_anti_dual_g1.yzx), anti_wedge_g0[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0 * right_anti_dual_g1.www), anti_wedge_g2[3] * right_anti_dual_g0[3]) - (anti_wedge_g1.wwwx * right_anti_dual_g1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g1.zxy)), anti_wedge_g2[3] * right_anti_dual_g1[3]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1.xyz * anti_wedge_g2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public Circle reject_orthogonally_from(Line other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e415);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125 */
            (float3(-(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g0.yzx) - (other.group1.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float4 anti_wedge_g3 = float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((this.group0.zxy * other.group1.yzx), this.e5 * other.e12345) - (other.group1.zxyx * float4(this.group0.yzx, this.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g0[2], 1.0) * float4(float3(anti_wedge_g2_w), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))) + float4((anti_wedge_g0 * right_anti_dual_g0.www), anti_wedge_g2_w * right_anti_dual_g1[3]), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(float3(anti_wedge_g2_w), -(anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]))), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g3[3]) * right_anti_dual_g0.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g1.zxy) + (float3(right_anti_dual_g0[3]) * float3(other.e12345) * this.group2.xyz) - (anti_wedge_g3.zxy * right_anti_dual_g1.yzx)), anti_wedge_g3[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 anti_wedge_g1 = float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + (float4(other.e12345) * float4(this.group3.xyz, this.e4)) - float4((this.group0.yzx * other.group8.zxy), this.e423 * other.e415) - float4((other.group7.yzx * this.group2.zxy), this.e431 * other.e425);
        float anti_wedge_g2 = (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g7 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(other.e12345) * this.group2.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx);
        float4 anti_wedge_g2 = (float4(this.e125 * other.e4315, this.e235 * other.e4125, this.e315 * other.e4235, (this.e2 * other.e4315) + (this.e3 * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float3(right_anti_dual_g0[3])), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g2.zxy * right_anti_dual_g0.yzx), 0.0) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            right_anti_dual_g0 * float4(anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx);
        float4 anti_wedge_g2 = (float4(this.e125 * other.e4315, this.e235 * other.e4125, this.e315 * other.e4235, (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e3215) + (anti_wedge_g2[0] * right_anti_dual_g0[3]), (anti_wedge_g0[1] * other.e3215) + (anti_wedge_g2[1] * right_anti_dual_g0[3]), (anti_wedge_g0[2] * other.e3215) + (anti_wedge_g2[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g0.zxy)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g2[3]) * float4(right_anti_dual_g0.xyz, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float4 anti_wedge_g3 = float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e1 * other.e12345), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e2 * other.e12345), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e3 * other.e12345), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((this.group0.zxy * other.group2.yzx), this.e5 * other.e12345) - (other.group2.zxyx * float4(this.group0.yzx, this.e415)) - float4((this.group2.zxy * other.group0.yzx), this.e425 * other.e315);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g3.zxyy * float4(right_anti_dual_g0.yzx, right_anti_dual_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g0 * right_anti_dual_g0.www), anti_wedge_g2_w * right_anti_dual_g3[3]) + float4((float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz), anti_wedge_g3[0] * right_anti_dual_g3[0]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * right_anti_dual_g0[0]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * right_anti_dual_g0[1]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * right_anti_dual_g0[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g3[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g3[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 anti_wedge_g1 = float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx);
        float4 anti_wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g1[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[0]), (anti_wedge_g1[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[1]), (anti_wedge_g1[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g0[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[0]), (anti_wedge_g2[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[1]), (anti_wedge_g2[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[2]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g0 * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g2[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_orthogonally_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(anti_wedge_g0.zxy, -(anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g0[3] * right_anti_dual_g1[3]))) - (anti_wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * anti_wedge_g0.www), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e4);
        float3 anti_wedge_g1_xyz = other.group1.xyz * float3(this.group0.xx, this.e3215);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g0[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[0] * right_anti_dual_g1[3]) + (anti_wedge_g0[3] * right_anti_dual_g3[0]), (anti_wedge_g0[1] * right_anti_dual_g1[3]) + (anti_wedge_g0[3] * right_anti_dual_g3[1]), (anti_wedge_g0[2] * right_anti_dual_g1[3]) + (anti_wedge_g0[3] * right_anti_dual_g3[2]), -(anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), anti_wedge_g0[3] * right_anti_dual_g3[3]) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_orthogonally_from(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * -1.0) * float2(this.e3215 * other.e12345, this.scalar * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e3215) * other.group0.xyz;
        float3 anti_wedge_g1 = float3(this.e3215) * other.group1.xyz;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[2] * other.e431), (anti_wedge_g0[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[0] * other.e412), (anti_wedge_g0[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[1] * other.e423), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g1.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.group0.xx, this.e3215, this.scalar) * float4(other.group0.xyz, other.e12345);
        float4 anti_wedge_g1 = float4(this.e3215) * float4(other.group1.xyz, other.e12345);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * float4(other.group0.xyz, right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0[3] * right_anti_dual_g2[0]) + (anti_wedge_g1[0] * right_anti_dual_g2[3]), (anti_wedge_g0[3] * right_anti_dual_g2[1]) + (anti_wedge_g1[1] * right_anti_dual_g2[3]), (anti_wedge_g0[3] * right_anti_dual_g2[2]) + (anti_wedge_g1[2] * right_anti_dual_g2[3]), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.zxyw * float4(other.group0.yzx, right_anti_dual_g2[3])) + float4((float3(right_anti_dual_g1[3]) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - (anti_wedge_g1.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_orthogonally_from(Dipole other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(anti_wedge_g0.zxy, -(right_anti_dual_g2[2] * anti_wedge_g0[2]) - (anti_wedge_g0[3] * right_anti_dual_g1[3]))) - (anti_wedge_g0.wwwx * float4(right_anti_dual_g0, right_anti_dual_g2[0])) - float4((anti_wedge_g0.yzx * right_anti_dual_g1.zxy), right_anti_dual_g2[1] * anti_wedge_g0[1]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0);
        float4 anti_wedge_g1 = float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx)), anti_wedge_g1[3] * right_anti_dual_g2[3] * -1.0), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g1.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4(float3(0.0), (right_anti_dual_g0[1] * anti_wedge_g1[1]) + (right_anti_dual_g0[2] * anti_wedge_g1[2])) - float4((right_anti_dual_g3.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g3[3]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) + float4((anti_wedge_g1.zxy * right_anti_dual_g1.yzx), anti_wedge_g0[0] * right_anti_dual_g3[0]) - (float4(anti_wedge_g1[3]) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])) - (right_anti_dual_g2.wwwy * float4(anti_wedge_g0.xyz, anti_wedge_g1[1]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_orthogonally_from(DualNum other) {
        float2 anti_wedge_g0 = float2(other.e12345) * this.group0.xy;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((anti_wedge_g0[0] * right_anti_dual_g0[1]) + (anti_wedge_g0[1] * right_anti_dual_g0[0]), anti_wedge_g0[1] * right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_orthogonally_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215 * pow(other.e45, 2) * -1.0, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Flector reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0_xyz = other.group1.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0);
        float4 anti_wedge_g1 = float4(float3(0.0), this.e3215 * other.e45) * float4(0.0, 0.0, 0.0, -1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g0_xyz[0] * right_anti_dual_g1[0]) + (anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) + (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e3215);
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor reject_orthogonally_from(Motor other) {
        float anti_wedge_g0_w = this.scalar * other.e12345;
        float4 anti_wedge_g1 = float4(this.e3215) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz), anti_wedge_g0_w * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e3215 */
            (float4(anti_wedge_g0_w) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e3215 * other.e4) + (this.scalar * other.e12345);
        float4 anti_wedge_g1 = float4((other.group4.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0)), 0.0);
        float anti_wedge_g2 = this.e3215 * other.e45 * -1.0;
        float4 anti_wedge_g3 = float4((other.group6.xyz * float3(this.group0.xx, this.e3215)), 0.0);
        float3 anti_wedge_g5 = float3(this.e3215) * other.group7.xyz;
        float4 anti_wedge_g6 = float4(float3(0.0), this.e3215 * other.e1234) * float4(0.0, 0.0, 0.0, -1.0);
        float3 anti_wedge_g8 = float3(this.e3215) * other.group9.xyz * float3(-1.0);
        float4 anti_wedge_g9 = float4(float3(0.0), this.e3215 * other.e12345);
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4(float3(0.0), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4(float3(0.0), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2]), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0 = this.e3215 * other.e4;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e1234) * float4(-1.0);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0)), (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * other.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * right_anti_dual_g0[3] * -1.0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.group0.xx, this.e3215, (this.e3215 * other.e4) + (this.scalar * other.e12345)) * float4(other.group0.xyz, 1.0);
        float4 anti_wedge_g1 = float4(this.e3215) * float4(other.group1.xyz, other.e12345);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g3[3])) + (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g1.zxy, -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]))) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), anti_wedge_g1[3] * right_anti_dual_g0[3]) - float4((anti_wedge_g1.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiDualNum {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0);
        float4 anti_wedge_g1 = float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx)), anti_wedge_g1[3] * right_anti_dual_g3[3] * -1.0), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g1.xyzx * float4(right_anti_dual_g2.www, right_anti_dual_g0[0])) + float4(float3(0.0), (anti_wedge_g1[1] * right_anti_dual_g0[1]) + (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (right_anti_dual_g3 * float4(anti_wedge_g1.www, anti_wedge_g0[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[3]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g1.zxy * right_anti_dual_g1.yzx), anti_wedge_g0[0] * right_anti_dual_g3[0]) - (anti_wedge_g1.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - (anti_wedge_g1.wwwz * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[2])) - float4((anti_wedge_g0.xyz * right_anti_dual_g3.www), anti_wedge_g1[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e315) + (other.e415 * this.e321), (other.e423 * this.e125) + (other.e425 * this.e321), (other.e431 * this.e235) + (other.e435 * this.e321), -(other.e425 * this.e315) - (other.e435 * this.e125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual_g2[1], right_anti_dual_g2[2], right_anti_dual_g2[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (anti_wedge_g0.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[0])) + (anti_wedge_g0.wwwy * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[1]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_orthogonally_from(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * float4(this.e235 * other.e12345, this.e315 * other.e12345, this.e125 * other.e12345, this.e321 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public Circle reject_orthogonally_from(Circle other) {
        float4 anti_wedge_g0 = float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.yzx * this.group0.zxy), this.e235 * other.e415);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.yzx * this.group0.zxy), this.e235 * other.e415);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((other.group0.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * right_anti_dual_g2[3]) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g2.yzx)), anti_wedge_g1[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public Circle reject_orthogonally_from(Dipole other) {
        float anti_wedge_g0 = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(other.e1234) * this.group0.xyz) - (other.group3.xyz * this.group0.www);
        float anti_wedge_g0_w = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g2[3])) + (right_anti_dual_g0 * float3(anti_wedge_g0_w))), -(anti_wedge_g0_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2])), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(anti_wedge_g0_w)) + float4((anti_wedge_g1_xyz * float3(right_anti_dual_g2[3])), -(anti_wedge_g0_xyz[0] * right_anti_dual_g3[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g3[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g3[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g3[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g2.xyz) + (anti_wedge_g1_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0_w * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_orthogonally_from(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * float4(this.e235 * other.e12345, this.e315 * other.e12345, this.e125 * other.e12345, this.e321 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_orthogonally_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e321 * other.e45 * -1.0) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e321) * float4(other.group1.xyz, other.e45) * float4(-1.0);
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g1_xyz[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[0] * right_anti_dual_g1[3]), (anti_wedge_g1_xyz[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[1] * right_anti_dual_g1[3]), (anti_wedge_g1_xyz[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[2] * right_anti_dual_g1[3]), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) + (float4(anti_wedge_g0[3]) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)) - (right_anti_dual_g1.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            right_anti_dual_g1 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e321) * other.group0.xyz;
        float anti_wedge_g0_w = -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_w * other.e415) + (anti_wedge_g0_xyz[1] * other.e125), (anti_wedge_g0_w * other.e425) + (anti_wedge_g0_xyz[2] * other.e235), (anti_wedge_g0_w * other.e435) + (anti_wedge_g0_xyz[0] * other.e315), -(anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)) - float4((anti_wedge_g0_xyz.zxy * other.group1.yzx), anti_wedge_g0_w * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g1 = float4((other.group0.xyz * this.group0.www), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g1[1] * right_anti_dual_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g0[0]), (anti_wedge_g1[2] * right_anti_dual_g1[0]) + (anti_wedge_g1[3] * right_anti_dual_g0[1]), (anti_wedge_g1[0] * right_anti_dual_g1[1]) + (anti_wedge_g1[3] * right_anti_dual_g0[2]), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) + (float4(right_anti_dual_g0[3]) * float4(other.e12345) * this.group0) - (anti_wedge_g1.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g0[0])), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((other.group7.zxy * this.group0.yzx), 0.0) - float4((other.group7.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0);
        float3 anti_wedge_g5 = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), this.e321 * other.e12345);
        float3 anti_wedge_g8 = float3(other.e12345) * this.group0.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4(float3(0.0), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4(float3(0.0), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = float3(this.e321) * other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0[0] * right_anti_dual_g0[3]) + (anti_wedge_g1[2] * right_anti_dual_g0[1]), (anti_wedge_g0[1] * right_anti_dual_g0[3]) + (anti_wedge_g1[0] * right_anti_dual_g0[2]), (anti_wedge_g0[2] * right_anti_dual_g0[3]) + (anti_wedge_g1[1] * right_anti_dual_g0[0]), -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.zxyx * float4(anti_wedge_g1.yzx, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public Circle reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 anti_wedge_g1 = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Circle(
            /* e423, e431, e412 */
            anti_wedge_g0 * float3(right_anti_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * float3(right_anti_dual_g0[3])), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125 */
            (anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g1.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g1.yzx * right_anti_dual_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g1[1] * right_anti_dual_g3[1]) + (anti_wedge_g1[2] * right_anti_dual_g3[2]) + (anti_wedge_g1[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[3] * right_anti_dual_g1[3])) - (right_anti_dual_g0.zxyx * float4(anti_wedge_g1.yzx, anti_wedge_g0[0])), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0 * float4(anti_wedge_g1.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g2.yzx)), anti_wedge_g1[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4((float3(other.e1234) * this.group0.xyz), -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e41));
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_anti_dual_g3.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) + float4((right_anti_dual_g0.xyz * anti_wedge_g0.www), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(anti_wedge_g0[3])) + float4((anti_wedge_g1_xyz * float3(right_anti_dual_g3[3])), -(anti_wedge_g0[0] * right_anti_dual_g3[0]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiFlector> operator /(AntiFlector lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e315) + (other.e415 * this.e321), (other.e423 * this.e125) + (other.e425 * this.e321), (other.e431 * this.e235) + (other.e435 * this.e321), -(other.e425 * this.e315) - (other.e435 * this.e125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual_g2[1], right_anti_dual_g2[2], right_anti_dual_g2[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (anti_wedge_g0.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[0])) + (anti_wedge_g0.wwwy * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[1]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public Circle reject_orthogonally_from(Circle other) {
        float4 anti_wedge_g0 = float4((this.e315 * other.e412) + (this.e321 * other.e415), (this.e125 * other.e423) + (this.e321 * other.e425), (this.e235 * other.e431) + (this.e321 * other.e435), -(this.e315 * other.e425) - (this.e125 * other.e435)) - float4((other.group0.yzx * this.group0.zxy), this.e235 * other.e415);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((other.group0.zxy * this.group0.yzx), this.e5 * other.e12345) - float4((other.group0.yzx * this.group0.zxy), this.e235 * other.e415);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((other.group0.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * right_anti_dual_g2[3]) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g2.yzx)), anti_wedge_g1[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public Circle reject_orthogonally_from(Dipole other) {
        float anti_wedge_g0 = -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) + float4((this.group0.xyz * other.group2.www), this.e1 * other.e4235) - float4((other.group3.xyz * this.group0.www), this.e235 * other.e41);
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * anti_wedge_g0.www) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz)), -(anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(anti_wedge_g0[3])) + float4((anti_wedge_g1_xyz * float3(right_anti_dual_g2[3])), -(anti_wedge_g0[0] * right_anti_dual_g3[0]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint reject_orthogonally_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e321 * other.e45 * -1.0) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0_xyz = other.group1.xyz * this.group0.www * float3(-1.0);
        float anti_wedge_g0_w = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45);
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_xyz[0] * right_anti_dual_g1[3]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g1[1]), (anti_wedge_g0_xyz[1] * right_anti_dual_g1[3]) + (anti_wedge_g1_xyz[0] * right_anti_dual_g1[2]), (anti_wedge_g0_xyz[2] * right_anti_dual_g1[3]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g1[0]), -(anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2])) + (float4(anti_wedge_g0_w) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)) - (right_anti_dual_g1.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0_xyz[0])), 
            /* e1, e2, e3, e5 */
            right_anti_dual_g1 * float4(anti_wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e321) * other.group0.xyz;
        float anti_wedge_g0_w = -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_w * other.e415) + (anti_wedge_g0_xyz[1] * other.e125), (anti_wedge_g0_w * other.e425) + (anti_wedge_g0_xyz[2] * other.e235), (anti_wedge_g0_w * other.e435) + (anti_wedge_g0_xyz[0] * other.e315), -(anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)) - float4((anti_wedge_g0_xyz.zxy * other.group1.yzx), anti_wedge_g0_w * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g1 = (other.group0 * float4(this.group0.www, this.e5)) + float4((float3(other.e12345) * this.group1.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g1[1] * right_anti_dual_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g0[0]), (anti_wedge_g1[2] * right_anti_dual_g1[0]) + (anti_wedge_g1[3] * right_anti_dual_g0[1]), (anti_wedge_g1[0] * right_anti_dual_g1[1]) + (anti_wedge_g1[3] * right_anti_dual_g0[2]), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) + (float4(right_anti_dual_g0[3]) * float4(other.e12345) * this.group0) - (anti_wedge_g1.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g0[0])), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((other.group7.zxy * this.group0.yzx), 0.0) - float4((other.group7.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = (this.e5 * other.e12345) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0);
        float3 anti_wedge_g5 = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), this.e321 * other.e12345);
        float3 anti_wedge_g8 = float3(other.e12345) * this.group0.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4(float3(0.0), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4(float3(0.0), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * this.group0.www * float3(-1.0);
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_xyz[0] * right_anti_dual_g0[3]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g0[1]), (anti_wedge_g0_xyz[1] * right_anti_dual_g0[3]) + (anti_wedge_g1_xyz[0] * right_anti_dual_g0[2]), (anti_wedge_g0_xyz[2] * right_anti_dual_g0[3]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g0[0]), -(anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0_xyz[0])), 
            /* e1, e2, e3, e5 */
            right_anti_dual_g0 * float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = (float4((float3(this.e321) * other.group0.xyz), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + float4((float3(other.e1234) * this.group0.xyz), this.e1 * other.e4235);
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1_xyz * float3(right_anti_dual_g0[3])), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g1_xyz.yzx * right_anti_dual_g0.zxy)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0[3]) * float4(right_anti_dual_g0.xyz, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4((this.e321 * other.e415) + (this.e1 * other.e12345), (this.e321 * other.e425) + (this.e2 * other.e12345), (this.e321 * other.e435) + (this.e3 * other.e12345), -(this.e315 * other.e425) - (this.e125 * other.e435)) + (other.group0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(other.group0.yzx, other.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g1[1] * right_anti_dual_g3[1]) + (anti_wedge_g1[2] * right_anti_dual_g3[2]) + (anti_wedge_g1[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[3] * right_anti_dual_g1[3])) - (right_anti_dual_g0.zxyx * float4(anti_wedge_g1.yzx, anti_wedge_g0[0])), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0 * float4(anti_wedge_g1.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g2.yzx)), anti_wedge_g1[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) + float4((this.group0.xyz * other.group2.www), this.e1 * other.e4235) - (this.group0.wwwx * float4(other.group3.xyz, other.e41));
        float3 anti_wedge_g1_xyz = (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_anti_dual_g3.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) + float4((right_anti_dual_g0.xyz * anti_wedge_g0.www), 0.0), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(anti_wedge_g0[3])) + float4((anti_wedge_g1_xyz * float3(right_anti_dual_g3[3])), -(anti_wedge_g0[0] * right_anti_dual_g3[0]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiLine> operator /(AntiLine lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public Dipole reject_orthogonally_from(Circle other) {
        float anti_wedge_g0 = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float3 anti_wedge_g1_xyz = this.group1.xyz * other.group2.www;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g1.xyz)), anti_wedge_g0_w * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0_w * right_anti_dual_g2[0]) + (anti_wedge_g1_xyz[0] * right_anti_dual_g2[3]), (anti_wedge_g0_w * right_anti_dual_g2[1]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g2[3]), (anti_wedge_g0_w * right_anti_dual_g2[2]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g2[3]), -(anti_wedge_g0_xyz[0] * other.e423) - (anti_wedge_g0_xyz[1] * other.e431) - (anti_wedge_g0_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz * float3(right_anti_dual_g1[3])), -(anti_wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g3.yzx)), anti_wedge_g0[3] * right_anti_dual_g2[3] * -1.0), 
            /* e15, e25, e35, e1234 */
            (float4(right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g3[2], 1.0) * float4(anti_wedge_g0.www, (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (anti_wedge_g0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(anti_wedge_g0.zxy, -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]))) - (float4(anti_wedge_g0[3]) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiMotor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = (float4(this.e12 * other.e4315, this.e23 * other.e4125, this.e31 * other.e4235, (this.e25 * other.e4315) + (this.e35 * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group1.zxyx * float4(this.group0.yzx, this.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g1.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_anti_dual_g1[3]) * anti_wedge_g0.xyz), -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[3] * right_anti_dual_g0[3])) - (anti_wedge_g0.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_orthogonally_from(Line other) {
        float anti_wedge_g0 = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiMotor reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4((this.group1.xyz * other.group0.www), 0.0);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz)), anti_wedge_g0_w * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0_w)) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 anti_wedge_g1 = float4((this.group0.yzx * other.group9.zxy), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group9.yzx), 0.0);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 anti_wedge_g3 = float4((this.group1.xyz * float3(other.group0.yy, other.e12345)), 0.0);
        float3 anti_wedge_g5 = float3(other.e12345) * this.group0.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), 0.0) + float4((other.group7.yzx * anti_wedge_g3.zxy), 0.0) - (right_anti_dual_g3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4(float3(0.0), anti_wedge_g3[2] * other.e435) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = (float4(this.e12 * other.e4315, this.e23 * other.e4125, this.e31 * other.e4235, (this.e25 * other.e4315) + (this.e35 * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiLine(
            /* e23, e31, e12 */
            (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e15, e25, e35 */
            (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public Dipole reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx)), anti_wedge_g0[3] * right_anti_dual_g0[3] * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412));
        float3 anti_wedge_g1_xyz = this.group1.xyz * other.group0.www;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4((anti_wedge_g1_xyz * float3(right_anti_dual_g0[3])), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g3[3])) + (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g1_xyz.zxy, -(anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]))) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), 0.0) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiLine {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g3[3]) * anti_wedge_g0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g3.yzx)), anti_wedge_g0[3] * right_anti_dual_g3[3] * -1.0), 
            /* e15, e25, e35, e1234 */
            (float4(right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g3[2], 1.0) * float4(anti_wedge_g0.www, (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (anti_wedge_g0.xyzx * float4(right_anti_dual_g2.www, right_anti_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(anti_wedge_g0.zxy, -(anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g0[3] * right_anti_dual_g1[3]))) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])) - (anti_wedge_g0.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[1]))
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiMotor> operator /(AntiMotor lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public Sphere reject_orthogonally_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e45) * float4(-1.0);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(anti_wedge_g0.zxy, -(anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g0[3] * right_anti_dual_g1[3]))) - (anti_wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * anti_wedge_g0.www), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215) * other.group0.xyz;
        float anti_wedge_g0_w = (other.e4 * this.e3215) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12);
        float3 anti_wedge_g1_xyz = other.group1.xyz * this.group1.www;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g0_w), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g2.xyz), (anti_wedge_g0_w * right_anti_dual_g2[3]) - (anti_wedge_g0_xyz[0] * other.e423) - (anti_wedge_g0_xyz[1] * other.e431) - (anti_wedge_g0_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_w * right_anti_dual_g3[0]) + (anti_wedge_g0_xyz[0] * right_anti_dual_g1[3]), (anti_wedge_g0_w * right_anti_dual_g3[1]) + (anti_wedge_g0_xyz[1] * right_anti_dual_g1[3]), (anti_wedge_g0_w * right_anti_dual_g3[2]) + (anti_wedge_g0_xyz[2] * right_anti_dual_g1[3]), -(anti_wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), anti_wedge_g0_w * right_anti_dual_g3[3]) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0_xyz[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215) * other.group0.xyz;
        float anti_wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float3 anti_wedge_g1_xyz = other.group1.xyz * this.group1.www;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g0_w), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0_w) * other.group2.xyz), -(anti_wedge_g0_xyz[0] * other.e423) - (anti_wedge_g0_xyz[1] * other.e431) - (anti_wedge_g0_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz[0] * right_anti_dual_g1[3]) + (anti_wedge_g1_xyz[2] * other.e431), (anti_wedge_g0_xyz[1] * right_anti_dual_g1[3]) + (anti_wedge_g1_xyz[0] * other.e412), (anti_wedge_g0_xyz[2] * right_anti_dual_g1[3]) + (anti_wedge_g1_xyz[1] * other.e423), -(anti_wedge_g0_xyz[1] * other.e315) - (anti_wedge_g0_xyz[2] * other.e125) - (anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0_xyz[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e12345) * this.group0.xyz) + (other.group0.xyz * this.group1.www);
        float anti_wedge_g0_w = (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz);
        float anti_wedge_g1_w = this.e3215 * other.e12345;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g1.xyz)), anti_wedge_g0_w * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0_w * right_anti_dual_g2[0]) + (anti_wedge_g1_xyz[0] * right_anti_dual_g2[3]), (anti_wedge_g0_w * right_anti_dual_g2[1]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g2[3]), (anti_wedge_g0_w * right_anti_dual_g2[2]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g2[3]), -(anti_wedge_g0_xyz[0] * other.e423) - (anti_wedge_g0_xyz[1] * other.e431) - (anti_wedge_g0_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz * float3(right_anti_dual_g1[3])), -(anti_wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), anti_wedge_g1_w * right_anti_dual_g2[3]) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g1_w * right_anti_dual_g1[0])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public Sphere reject_orthogonally_from(Dipole other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * this.group1.www * float3(-1.0);
        float anti_wedge_g0_w = this.e3215 * other.e45 * -1.0;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[1], right_anti_dual_g1[2], right_anti_dual_g1[0], 1.0) * float4(anti_wedge_g0_xyz.zxy, -(anti_wedge_g0_w * right_anti_dual_g1[3]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g2[2]))) - (float4(anti_wedge_g0_xyz, anti_wedge_g0_w).wwwx * float4(right_anti_dual_g0, right_anti_dual_g2[0])) - float4((anti_wedge_g0_xyz.yzx * right_anti_dual_g1.zxy), anti_wedge_g0_xyz[1] * right_anti_dual_g2[1]), 
            /* e1234 */
            (anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0);
        float4 anti_wedge_g1 = float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((other.group0.xyz * this.group1.www), this.e3215 * other.e45);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx)), anti_wedge_g1[3] * right_anti_dual_g2[3] * -1.0), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g1.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4(float3(0.0), (right_anti_dual_g0[1] * anti_wedge_g1[1]) + (right_anti_dual_g0[2] * anti_wedge_g1[2])) - float4((right_anti_dual_g3.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g3[3]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) + float4((anti_wedge_g1.zxy * right_anti_dual_g1.yzx), anti_wedge_g0[0] * right_anti_dual_g3[0]) - (float4(anti_wedge_g1[3]) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])) - (right_anti_dual_g2.wwwy * float4(anti_wedge_g0.xyz, anti_wedge_g1[1]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            anti_wedge_g0 * float4(right_anti_dual_g0[1]), 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], (right_anti_dual_g0[0] * anti_wedge_g0[3]) + (right_anti_dual_g0[1] * anti_wedge_g1[3])) * float4(anti_wedge_g1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiDualNum reject_orthogonally_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e3215 * pow(other.e45, 2) * -1.0, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0_xyz = other.group1.xyz * this.group1.www * float3(-1.0);
        float4 anti_wedge_g1 = (other.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((this.group0.zxy * other.group1.yzx), this.e3215 * other.e45);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            (right_anti_dual_g1.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g0_xyz[0])) + float4(float3(0.0), (anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) + (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3])) - (anti_wedge_g1.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_orthogonally_from(Line other) {
        float anti_wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float3 anti_wedge_g1_xyz = other.group0.xyz * this.group1.www;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), -(anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g0_w = (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz)), anti_wedge_g0_w * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0_w)) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 anti_wedge_g1 = float4((this.group0.yzx * other.group9.zxy), 0.0) - float4((float3(this.e3215) * other.group4.xyz), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group9.yzx), 0.0);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e3215 * other.e45);
        float4 anti_wedge_g3 = float4(((float3(this.e3215) * other.group6.xyz) + (float3(other.e12345) * this.group1.xyz)), 0.0);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), this.e3215 * other.e1234) * float4(0.0, 0.0, 0.0, -1.0);
        float3 anti_wedge_g8 = float3(this.e3215) * other.group9.xyz * float3(-1.0);
        float4 anti_wedge_g9 = float4(float3(0.0), this.e3215 * other.e12345);
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4(float3(0.0), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4(float3(0.0), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * this.group1.www * float3(-1.0);
        float4 anti_wedge_g1 = (float4(this.e12 * other.e4315, this.e23 * other.e4125, this.e31 * other.e4235, (this.e25 * other.e4315) + (this.e35 * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            (float4(right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g0[2], 1.0) * float4(anti_wedge_g1.www, (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public Sphere reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0 = this.e3215 * other.e4;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e1234) * float4(-1.0);
        float4 anti_wedge_g1 = float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group0.zxyx * float4(this.group0.yzx, this.e15));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), anti_wedge_g1[3] * right_anti_dual_g0[3] * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3] * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0)), (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = (float4(this.e3215) * float4(other.group0.xyz, other.e4)) + (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412));
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4((anti_wedge_g1_xyz * float3(right_anti_dual_g0[3])), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g3[3])) + (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g1_xyz.zxy, -(anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]))) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), right_anti_dual_g0[3] * this.e3215 * other.e12345) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group3.xyz, other.e1234) * float4(-1.0);
        float4 anti_wedge_g1 = float4(-(this.e15 * other.e1234) - (this.e3215 * other.e41), -(this.e25 * other.e1234) - (this.e3215 * other.e42), -(this.e35 * other.e1234) - (this.e3215 * other.e43), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (other.group3.zxyx * float4(this.group0.yzx, this.e15)) - float4((this.group0.zxy * other.group3.yzx), this.e3215 * other.e45);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx)), anti_wedge_g1[3] * right_anti_dual_g3[3] * -1.0), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g1.xyzx * float4(right_anti_dual_g2.www, right_anti_dual_g0[0])) + float4(float3(0.0), (anti_wedge_g1[1] * right_anti_dual_g0[1]) + (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (right_anti_dual_g3 * float4(anti_wedge_g1.www, anti_wedge_g0[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[3]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g1.zxy * right_anti_dual_g1.yzx), anti_wedge_g0[0] * right_anti_dual_g3[0]) - (anti_wedge_g1.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - (anti_wedge_g1.wwwz * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[2])) - float4((anti_wedge_g0.xyz * right_anti_dual_g3.www), anti_wedge_g1[0] * right_anti_dual_g2[0])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiPlane> operator /(AntiPlane lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiPlane operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane reject_orthogonally_from(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * float4(this.e1 * other.e12345, this.e2 * other.e12345, this.e3 * other.e12345, this.e5 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e4 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), 0.0) - float4((anti_wedge_g0.zxy * right_anti_dual_g2.yzx), 0.0), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane reject_orthogonally_from(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * float4(this.e1 * other.e12345, this.e2 * other.e12345, this.e3 * other.e12345, this.e5 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiFlector reject_orthogonally_from(Flector other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(anti_wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiFlector reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0[1] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g0[0]), (anti_wedge_g0[2] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g0[1]), (anti_wedge_g0[0] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g0[2]), -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])) - (anti_wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g0[0])), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        float4 anti_wedge_g1 = float4((this.group0.xyz * float3(other.group0.yy, other.e12345)), 0.0);
        float anti_wedge_g2 = this.e5 * other.e12345;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), 0.0), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), 0.0) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public AntiPlane reject_orthogonally_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public RoundPoint reject_orthogonally_from(Sphere other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            anti_wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g0.yzx, (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (anti_wedge_g0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g3[0])), 
            /* e415, e425, e435, e321 */
            (float4(right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g0[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]))) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g2.yzx)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g0.xyz * right_anti_dual_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiPlane {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[0] * anti_wedge_g2[0]) - (right_anti_dual_g0[1] * anti_wedge_g2[1]) - (right_anti_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2[3] * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2[3] * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g3.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public DualNum reject_orthogonally_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * other.e321 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[3] * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g1[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            anti_wedge_g0 * float3(right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            anti_wedge_g1 * float4(right_anti_dual_g2[3]), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g2.xyz * right_anti_dual_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[0] * right_anti_dual_g2[3]), (anti_wedge_g0[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[1] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e12345 */
            (right_anti_dual_g3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g2[3] * right_anti_dual_g3[3]) + (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[1] * anti_wedge_g2[1]) - (right_anti_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g2.zxy * right_anti_dual_g3.yzx), anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g2.yzxx * float4(right_anti_dual_g3.zxy, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public DualNum reject_orthogonally_from(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * float2(this.e12345 * other.e5, this.e12345 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * pow(other.e45, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[3] * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public Motor reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0.xyz * right_anti_dual_g0.www), (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float2 anti_wedge_g0 = float2(this.e12345) * other.group0.xy;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float anti_wedge_g2 = this.e12345 * other.e5;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 anti_wedge_g4 = float3(this.e12345) * other.group4.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group5.xyz;
        float4 anti_wedge_g6 = float4(this.e12345) * other.group6;
        float3 anti_wedge_g7 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g8 = float3(this.e12345) * other.group8.xyz;
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = this.e12345 * other.e1234;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0[0] * right_anti_dual_g0[0], (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g0[0] * right_anti_dual_g0[1]) + (anti_wedge_g0[1] * right_anti_dual_g0[0]) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0[0])), 
            /* e5 */
            (anti_wedge_g2 * right_anti_dual_g0[0]) + (anti_wedge_g0[0] * other.e3215), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0[0])) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0[0]) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0[0]) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0[0])) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0[0])) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0[0])) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0[0])) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g10 * anti_wedge_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (this.e12345 * other.e4 * other.e5)
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
            /* e12345 */
            this.e12345 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g0[3]) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_anti_dual_g0[3])) + (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g3.zxy * right_anti_dual_g0.yzx), anti_wedge_g2[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * right_anti_dual_g0[0]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g2[3] * right_anti_dual_g0[1]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g2[3] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension AntiScalar {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(anti_wedge_g0[3])) + (right_anti_dual_g3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g2[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * right_anti_dual_g2[3]) + (anti_wedge_g2[0] * right_anti_dual_g3[3]), (anti_wedge_g0[1] * right_anti_dual_g2[3]) + (anti_wedge_g2[1] * right_anti_dual_g3[3]), (anti_wedge_g0[2] * right_anti_dual_g2[3]) + (anti_wedge_g2[2] * right_anti_dual_g3[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Circle> operator /(Circle lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.yzx * other.group2.zxy), other.e431 * this.e425);
        float anti_wedge_g1 = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e423) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g1 * other.e431) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g1 * other.e412) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual_g2[1], right_anti_dual_g2[2], right_anti_dual_g2[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g3[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(anti_wedge_g1) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (anti_wedge_g0.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiFlatPoint reject_orthogonally_from(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group0.xyz * float3(other.group0.xx, other.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Line reject_orthogonally_from(AntiFlatPoint other) {
        float4 anti_wedge_g0 = float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.yzx * other.group0.zxy), other.e235 * this.e415);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.yzx * other.group0.zxy), other.e235 * this.e415);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz * float3(-1.0)), (anti_wedge_g0[0] * right_anti_dual_g1[0]) + (anti_wedge_g0[1] * right_anti_dual_g1[1]) + (anti_wedge_g0[2] * right_anti_dual_g1[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Line reject_orthogonally_from(AntiLine other) {
        float anti_wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.e3215) * this.group0.xyz;
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz), (anti_wedge_g0_w * right_anti_dual_g0[3]) - (anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g1.xyz)), anti_wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle reject_orthogonally_from(Circle other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((other.group2.zxy * this.group0.yzx), other.e431 * this.e425);
        float anti_wedge_g1 = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e423) + (anti_wedge_g0[3] * other.e235), (anti_wedge_g1 * other.e431) + (anti_wedge_g0[3] * other.e315), (anti_wedge_g1 * other.e412) + (anti_wedge_g0[3] * other.e125), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * right_anti_dual_g1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float4 anti_wedge_g3 = float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e125 * other.e435)) - float4((this.group0.yzx * other.group2.zxy), this.e235 * other.e415) - float4((this.group2.zxy * other.group0.yzx), this.e315 * other.e425);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g2_w * right_anti_dual_g1[0]) + (anti_wedge_g0[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[2] * other.e431), (anti_wedge_g2_w * right_anti_dual_g1[1]) + (anti_wedge_g0[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[0] * other.e412), (anti_wedge_g2_w * right_anti_dual_g1[2]) + (anti_wedge_g0[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[1] * other.e423), -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g1[0] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g1[1] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g1[2] * right_anti_dual_g2[3]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g1[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g3[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g3[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g2[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle reject_orthogonally_from(Dipole other) {
        float anti_wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 anti_wedge_g1 = float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx);
        float4 anti_wedge_g2 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[1] * right_anti_dual_g3[2]) + (right_anti_dual_g0[0] * anti_wedge_g2[3]) + (anti_wedge_g1[0] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g3[0]) + (right_anti_dual_g0[1] * anti_wedge_g2[3]) + (anti_wedge_g1[1] * right_anti_dual_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g3[1]) + (right_anti_dual_g0[2] * anti_wedge_g2[3]) + (anti_wedge_g1[2] * right_anti_dual_g2[3]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[0] * anti_wedge_g2[0]) - (right_anti_dual_g0[1] * anti_wedge_g2[1]) - (right_anti_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[0]), (anti_wedge_g2[1] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[1]), (anti_wedge_g2[2] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[2]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g0 * right_anti_dual_g3.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g2[3] * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public AntiFlatPoint reject_orthogonally_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx);
        float4 anti_wedge_g2 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group2.zxy * other.group1.yzx), this.e423 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], 1.0) * float4(anti_wedge_g0.yzx, -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3]))) - float4((anti_wedge_g0.zxy * right_anti_dual_g1.yzx), anti_wedge_g0[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0 * right_anti_dual_g1.www), anti_wedge_g2[3] * right_anti_dual_g0[3]) - (anti_wedge_g1.wwwx * right_anti_dual_g1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g1.zxy)), anti_wedge_g2[3] * right_anti_dual_g1[3]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1.xyz * anti_wedge_g2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle reject_orthogonally_from(Line other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e415);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125 */
            (float3(-(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g0.yzx) - (other.group1.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float4 anti_wedge_g3 = float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group0.yzx * other.group1.zxy), this.e235 * other.e415);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g0[2], 1.0) * float4(float3(anti_wedge_g2_w), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))) + float4((anti_wedge_g0 * right_anti_dual_g0.www), anti_wedge_g2_w * right_anti_dual_g1[3]), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(float3(anti_wedge_g2_w), -(anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]))), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g3[3]) * right_anti_dual_g0.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g1.zxy) + (float3(right_anti_dual_g0[3]) * float3(other.e12345) * this.group2.xyz) - (anti_wedge_g3.zxy * right_anti_dual_g1.yzx)), anti_wedge_g3[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 anti_wedge_g1 = float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group0.yzx * other.group8.zxy), this.e423 * other.e415) - float4((this.group2.zxy * other.group7.yzx), this.e431 * other.e425);
        float anti_wedge_g2 = -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g7 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(other.e12345) * this.group2.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float3(right_anti_dual_g0[3])), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public Circle reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Circle(
            /* e423, e431, e412 */
            (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e3215) + (anti_wedge_g2[0] * right_anti_dual_g0[3]), (anti_wedge_g0[1] * other.e3215) + (anti_wedge_g2[1] * right_anti_dual_g0[3]), (anti_wedge_g0[2] * other.e3215) + (anti_wedge_g2[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e3215) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float4 anti_wedge_g3 = float4((this.e412 * other.e315) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e423 * other.e125) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e431 * other.e235) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e125 * other.e435)) - float4((this.group0.yzx * other.group2.zxy), this.e235 * other.e415) - float4((this.group2.zxy * other.group0.yzx), this.e315 * other.e425);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g3.zxyy * float4(right_anti_dual_g0.yzx, right_anti_dual_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g0 * right_anti_dual_g0.www), anti_wedge_g2_w * right_anti_dual_g3[3]) + float4((float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz), anti_wedge_g3[0] * right_anti_dual_g3[0]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * right_anti_dual_g0[0]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * right_anti_dual_g0[1]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * right_anti_dual_g0[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g3[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g3[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Circle {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float4 anti_wedge_g1 = float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx);
        float4 anti_wedge_g2 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g1[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[0]), (anti_wedge_g1[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[1]), (anti_wedge_g1[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g0[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[0]), (anti_wedge_g2[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[1]), (anti_wedge_g2[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[2]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g0 * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g2[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<CircleRotor> operator /(CircleRotor lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * right_anti_dual_g2.xyz), (anti_wedge_g2_w * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float4 anti_wedge_g3 = float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group2.yzxw * float4(other.group0.zxy, other.e5)) - (this.group2.zxyx * float4(other.group0.yzx, other.e415)) - float4((this.group0.yzx * other.group2.zxy), other.e425 * this.e315);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g3.yzx * right_anti_dual_g2.zxy), anti_wedge_g2_w * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Motor reject_orthogonally_from(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.group0.xx, other.e3215, other.scalar) * float4(this.group0.xyz, this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.scalar), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Motor reject_orthogonally_from(AntiFlatPoint other) {
        float4 anti_wedge_g1 = float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - float4((this.group0.yzx * other.group0.zxy), other.e235 * this.e415);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g1.xyz, other.e321 * this.e12345) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g1 = float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((this.group0.zxy * other.group0.yzx), other.e5 * this.e12345) - float4((this.group0.yzx * other.group0.zxy), other.e235 * this.e415);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[3] * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Motor reject_orthogonally_from(AntiLine other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float3(anti_wedge_g0_w)), -(anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = (float3(this.e12345) * other.group0.xyz) + (this.group0.xyz * other.group1.www);
        float anti_wedge_g0_w = (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz), (anti_wedge_g0_w * right_anti_dual_g0[3]) - (anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g1.xyz)), anti_wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiScalar reject_orthogonally_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float4 anti_wedge_g3 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e125 * this.e435)) - float4((other.group0.yzx * this.group2.zxy), other.e235 * this.e415) - float4((other.group2.zxy * this.group0.yzx), other.e315 * this.e425);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e235) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2_w * other.e315) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2_w * other.e125) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e125) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g3[2] * other.e235) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g3[0] * other.e315) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) - float4((other.group2.yzx * anti_wedge_g3.zxy), anti_wedge_g0[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1);
        float anti_wedge_g2_x = (other.e235 * this.e12345) + (other.e12345 * this.e235);
        float anti_wedge_g2_y = (other.e315 * this.e12345) + (other.e12345 * this.e315);
        float anti_wedge_g2_z = (other.e125 * this.e12345) + (other.e12345 * this.e125);
        float anti_wedge_g2_w = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.yzx * other.group2.zxy), other.e431 * this.e425);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0_xyz[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g0_xyz[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g0_xyz[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g2_y * other.e431) - (anti_wedge_g2_z * other.e412) - (anti_wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((other.group0.yzx * anti_wedge_g3.zxy), right_anti_dual_g2[3] * other.e12345 * this.e12345) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_x * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g1[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g1[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + float4((float3(anti_wedge_g2_w) * other.group0.xyz), anti_wedge_g1[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g2[3]) * float3(anti_wedge_g2_x, anti_wedge_g2_y, anti_wedge_g2_z)) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2_w * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual_g2 * float3(anti_wedge_g2_w)), -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4((this.e412 * other.e4315) + (this.e415 * other.e1234) + (this.e12345 * other.e41), (this.e423 * other.e4125) + (this.e425 * other.e1234) + (this.e12345 * other.e42), (this.e431 * other.e4235) + (this.e435 * other.e1234) + (this.e12345 * other.e43), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group0.yzx * other.group3.zxy), this.e423 * other.e15);
        float4 anti_wedge_g1 = float4((this.e235 * other.e1234) + (this.e12345 * other.e23), (this.e315 * other.e1234) + (this.e12345 * other.e31), (this.e125 * other.e1234) + (this.e12345 * other.e12), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((this.group0.xyz * other.group3.www), this.e12345 * other.e45) - (this.group1.wwwx * other.group3.xyzx);
        float3 anti_wedge_g2_xyz = (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g3.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (right_anti_dual_g3[3] * this.e12345 * other.e1234) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((right_anti_dual_g0 * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g2[3]) + float4((anti_wedge_g1.xyz * right_anti_dual_g2.www), anti_wedge_g3[0] * right_anti_dual_g3[0]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * right_anti_dual_g2[3]) + (anti_wedge_g0[0] * right_anti_dual_g3[3]), (anti_wedge_g2_xyz[1] * right_anti_dual_g2[3]) + (anti_wedge_g0[1] * right_anti_dual_g3[3]), (anti_wedge_g2_xyz[2] * right_anti_dual_g2[3]) + (anti_wedge_g0[2] * right_anti_dual_g3[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e12345) * float4(this.group0.xyz, this.e12345);
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], right_anti_dual_g0[1] * anti_wedge_g0[3]) * float4(anti_wedge_g0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0[1]) * this.group2 * float4(other.group0.yy, other.e12345, other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(FlatPoint other) {
        float anti_wedge_g2_w = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * right_anti_dual_g0.xyz), right_anti_dual_g0[3] * this.e12345 * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e412 * other.e4315, this.e423 * other.e4125, this.e431 * other.e4235, -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e15);
        float4 anti_wedge_g1 = float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((this.group0.xyz * other.group1.www), this.e12345 * other.e45) - (this.group1.wwwx * other.group1.xyzx);
        float4 anti_wedge_g2 = float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group1.zxy), 0.0) - float4((this.group2.zxy * other.group1.yzx), 0.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g1.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g2[3] * right_anti_dual_g1[3]) + (anti_wedge_g3[1] * right_anti_dual_g1[1]) + (anti_wedge_g3[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3])) - (anti_wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g0[0])), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g1.zxy)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1.xyz * anti_wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(Line other) {
        float3 anti_wedge_g1_xyz = other.group0.xyz * this.group2.www;
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float4 anti_wedge_g3 = float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group0.yzx * other.group1.zxy), this.e235 * other.e415);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e415) - (anti_wedge_g3[1] * other.e425) - (anti_wedge_g3[2] * other.e435)), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e125) + (anti_wedge_g3[3] * other.e415), (anti_wedge_g3[2] * other.e235) + (anti_wedge_g3[3] * other.e425), (anti_wedge_g3[0] * other.e315) + (anti_wedge_g3[3] * other.e435), -(anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435)) - float4((other.group1.yzx * anti_wedge_g3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * float4(this.group0.xyz, this.e12345);
        float4 anti_wedge_g1 = float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e321 * other.e12345);
        float4 anti_wedge_g2 = (this.group2 * float4(other.group0.www, other.e5)) + float4((float3(this.e12345) * other.group1.xyz), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435));
        float4 anti_wedge_g3 = float4((this.e412 * other.e315) + (this.e321 * other.e415), (this.e423 * other.e125) + (this.e321 * other.e425), (this.e431 * other.e235) + (this.e321 * other.e435), -(this.e431 * other.e425) - (this.e412 * other.e435)) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e415);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(anti_wedge_g3.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g0.xyz * right_anti_dual_g0.www), anti_wedge_g3[3] * right_anti_dual_g1[3]), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g3.www, -(anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]))), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g1.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 anti_wedge_g1 = float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e12345 * other.e3), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((this.group0.zxy * other.group8.yzx), this.e12345 * other.e4) - float4((this.group0.yzx * other.group8.zxy), this.e423 * other.e415) - float4((other.group7.yzx * this.group2.zxy), this.e431 * other.e425);
        float anti_wedge_g2 = (this.e12345 * other.e5) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((this.e315 * other.e4125) + (this.e12345 * other.e15), (this.e125 * other.e4235) + (this.e12345 * other.e25), (this.e235 * other.e4315) + (this.e12345 * other.e35), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((this.group1.xyz * other.group9.www), this.e12345 * other.e45) - (other.group9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group5.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = (float4(this.e12345) * other.group6) + (float4(other.e12345) * this.group1);
        float3 anti_wedge_g7 = (float3(this.e12345) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz);
        float3 anti_wedge_g8 = (float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group2.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = this.e12345 * other.e1234;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) + (right_anti_dual_g0[0] * this.e12345 * other.e12345) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx);
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - float4((this.group2.zxy * other.group0.yzx), 0.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float3(right_anti_dual_g0[3])), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g2.yzx, (anti_wedge_g2[3] * right_anti_dual_g0[3]) + (anti_wedge_g3[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0])) + (right_anti_dual_g0.wwwy * float4(anti_wedge_g1.xyz, anti_wedge_g3[1]))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiScalar reject_orthogonally_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (this.e12345 * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public AntiScalar reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = float4((this.e423 * other.e3215) + (this.e235 * other.e1234), (this.e431 * other.e3215) + (this.e315 * other.e1234), (this.e412 * other.e3215) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e3215) + (anti_wedge_g2_xyz[0] * right_anti_dual_g0[3]), (anti_wedge_g0[1] * other.e3215) + (anti_wedge_g2_xyz[1] * right_anti_dual_g0[3]), (anti_wedge_g0[2] * other.e3215) + (anti_wedge_g2_xyz[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g2_xyz.yzx, (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g0[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(other.e3215) * float4(anti_wedge_g1.xyz, this.e12345 * other.e1234)) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g2_xyz.zxy, anti_wedge_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e12345);
        float4 anti_wedge_g1 = (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group1);
        float4 anti_wedge_g2 = (this.group2 * float4(other.group0.www, other.e5)) + float4((float3(this.e12345) * other.group2.xyz), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435));
        float4 anti_wedge_g3 = float4((this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431) + (this.e12345 * other.e3), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((this.group0.zxy * other.group2.yzx), this.e12345 * other.e4) - float4((this.group0.yzx * other.group2.zxy), this.e423 * other.e415) - float4((this.group2.zxy * other.group0.yzx), this.e431 * other.e425);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_anti_dual_g0[3])) + (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g3.zxy * right_anti_dual_g0.yzx), anti_wedge_g2[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * right_anti_dual_g0[0]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g2[3] * right_anti_dual_g0[1]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g2[3] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4((this.e415 * other.e1234) + (this.e12345 * other.e41), (this.e425 * other.e1234) + (this.e12345 * other.e42), (this.e435 * other.e1234) + (this.e12345 * other.e43), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group0.zxy * other.group3.yzx), this.e12345 * other.scalar) - float4((this.group0.yzx * other.group3.zxy), this.e423 * other.e15);
        float4 anti_wedge_g1 = float4((this.e235 * other.e1234) + (this.e12345 * other.e23), (this.e315 * other.e1234) + (this.e12345 * other.e31), (this.e125 * other.e1234) + (this.e12345 * other.e12), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((this.group0.xyz * other.group3.www), this.e12345 * other.e45) - (this.group1.wwwx * other.group3.xyzx);
        float3 anti_wedge_g2_xyz = (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(anti_wedge_g0[3])) + (right_anti_dual_g3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), right_anti_dual_g2[3] * this.e12345 * other.e1234) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * right_anti_dual_g3[3]) + (anti_wedge_g0[0] * right_anti_dual_g2[3]), (anti_wedge_g2_xyz[1] * right_anti_dual_g3[3]) + (anti_wedge_g0[1] * right_anti_dual_g2[3]), (anti_wedge_g2_xyz[2] * right_anti_dual_g3[3]) + (anti_wedge_g0[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Dipole> operator /(Dipole lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public FlatPoint reject_orthogonally_from(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group0.xyz * float3(other.group0.xx, other.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public FlatPoint reject_orthogonally_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float anti_wedge_g0 = -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Flector reject_orthogonally_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = this.group0.xyz * other.group1.www;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g0_xyz * right_anti_dual_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g0_xyz.zxy, -(anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2]))) - float4((anti_wedge_g0_xyz.yzx * right_anti_dual_g0.zxy), right_anti_dual_g1[0] * other.e3215 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole reject_orthogonally_from(Circle other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g2[3])) + (right_anti_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g2_xyz[0] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g2_xyz[1] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g2_xyz[2] * right_anti_dual_g2[3]), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * right_anti_dual_g2[2]) + (anti_wedge_g2_xyz[2] * other.e431) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]), (anti_wedge_g0[2] * right_anti_dual_g2[0]) + (anti_wedge_g2_xyz[0] * other.e412) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]), (anti_wedge_g0[0] * right_anti_dual_g2[1]) + (anti_wedge_g2_xyz[1] * other.e423) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]), -(anti_wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * other.group2.www), this.e41 * other.e4235);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * right_anti_dual_g3.zxyw) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g1 * right_anti_dual_g2[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4((float2(anti_wedge_g1) * right_anti_dual_g3.xy * float2(-1.0)), anti_wedge_g1 * right_anti_dual_g3[2] * -1.0, (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(anti_wedge_g1) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group1.yzxx * float4(this.group1.zxy, this.e41));
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g1.yzx)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g1[3]) * anti_wedge_g0.xyz) - (float3(anti_wedge_g1) * right_anti_dual_g1.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz), -(anti_wedge_g1 * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public AntiLine reject_orthogonally_from(Line other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g0, anti_wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g2_w) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * right_anti_dual_g1[0]) + (anti_wedge_g2_xyz[0] * right_anti_dual_g0[3]), (anti_wedge_g2_w * right_anti_dual_g1[1]) + (anti_wedge_g2_xyz[1] * right_anti_dual_g0[3]), (anti_wedge_g2_w * right_anti_dual_g1[2]) + (anti_wedge_g2_xyz[2] * right_anti_dual_g0[3]), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g0[2], 1.0) * float4(anti_wedge_g1.www, -(anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]))) + (right_anti_dual_g1.zxyw * float4(anti_wedge_g0.yzx, anti_wedge_g2_w)) - (right_anti_dual_g1.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 anti_wedge_g1 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(other.e1234) * this.group2.xyz), this.e41 * other.e4235);
        float anti_wedge_g2 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 anti_wedge_g3 = float4(other.e12345) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(other.e12345) * this.group1.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), 0.0) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), 0.0) + float4((other.group7.yzx * anti_wedge_g3.zxy), 0.0) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4(float3(0.0), anti_wedge_g3[2] * other.e435) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35 */
            (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz) - (float3((this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public Dipole reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(other.e1234) * this.group2.xyz), this.e41 * other.e4235);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Dipole(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_anti_dual_g0.zxy, other.e3215)) - float4((anti_wedge_g0.zxy * right_anti_dual_g0.yzx), anti_wedge_g1 * right_anti_dual_g0[3]), 
            /* e15, e25, e35 */
            (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g1) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float4 anti_wedge_g2 = float4((float3(other.e12345) * this.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g2[3])) + (float4(right_anti_dual_g2[0], right_anti_dual_g2[1], right_anti_dual_g2[2], 1.0) * float4(anti_wedge_g2.www, -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * other.group2.www), this.e41 * other.e4235);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g1 * right_anti_dual_g3[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g0.xyzx * float4(right_anti_dual_g2.www, right_anti_dual_g0[0])) + float4((float2(anti_wedge_g1) * right_anti_dual_g3.xy * float2(-1.0)), anti_wedge_g1 * right_anti_dual_g3[2] * -1.0, (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])) - float4((float3(anti_wedge_g1) * right_anti_dual_g0.xyz), anti_wedge_g0[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_orthogonally_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4((other.group2.xyz * this.group2.www), other.e42 * this.e4315);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_orthogonally_from(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(((float4(other.e3215) * float4(this.group0.xyz, this.e45)).xyz * float3(other.group0.xx, other.e3215)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_orthogonally_from(AntiFlatPoint other) {
        float3 anti_wedge_g0_xyz = (float3(this.e1234) * other.group0.xyz) - (this.group3.xyz * other.group0.www);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz * float3(right_anti_dual_g0[3])), -(anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) + float4((other.group0.xyz * this.group2.www), other.e1 * this.e4235) - float4((this.group3.xyz * other.group0.www), other.e235 * this.e41);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Plane reject_orthogonally_from(AntiLine other) {
        float4 anti_wedge_g0 = float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group3.zxyx * float4(other.group0.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], 1.0) * float4(right_anti_dual_g0.yzx, -(right_anti_dual_g1[1] * anti_wedge_g0[1]) - (right_anti_dual_g1[2] * anti_wedge_g0[2]))) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((this.group0.xyz * other.group1.www), other.e3215 * this.e45) - (this.group3.zxyx * float4(other.group0.yzx, other.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1.zxy * right_anti_dual_g0.yzx), right_anti_dual_g1[3] * other.e3215 * this.e1234) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Plane reject_orthogonally_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e125) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431), (anti_wedge_g0[2] * other.e235) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412), (anti_wedge_g0[0] * other.e315) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423), -(anti_wedge_g1[2] * other.e125) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e235) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e4315) + (other.e415 * this.e1234) + (other.e12345 * this.e41), (other.e423 * this.e4125) + (other.e425 * this.e1234) + (other.e12345 * this.e42), (other.e431 * this.e4235) + (other.e435 * this.e1234) + (other.e12345 * this.e43), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group0.yzx * this.group3.zxy), other.e423 * this.e15);
        float4 anti_wedge_g1 = float4((other.e235 * this.e1234) + (other.e12345 * this.e23), (other.e315 * this.e1234) + (other.e12345 * this.e31), (other.e125 * this.e1234) + (other.e12345 * this.e12), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((other.group0.xyz * this.group3.www), other.e12345 * this.e45) - (other.group1.wwwx * this.group3.xyzx);
        float3 anti_wedge_g2_xyz = (float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx);
        float anti_wedge_g2_w = other.e12345 * this.e1234;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g2[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(anti_wedge_g0.www, anti_wedge_g2_w)) + float4((anti_wedge_g2_xyz * float3(right_anti_dual_g2[3])), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * right_anti_dual_g2[2]) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g3[0] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g2[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g3[1] * right_anti_dual_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g2[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g3[2] * right_anti_dual_g2[3]), -(anti_wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g2[0]) - (anti_wedge_g1[1] * right_anti_dual_g2[1]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), anti_wedge_g3[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_w * right_anti_dual_g1[0]) - float4((anti_wedge_g0.zxy * right_anti_dual_g2.yzx), anti_wedge_g2_xyz[1] * right_anti_dual_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_orthogonally_from(Dipole other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxy * float4(other.group1.zxy, other.e42)) + float4((other.group2.xyz * this.group2.www), other.e41 * this.e4235);
        float anti_wedge_g1 = -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g2[0] * anti_wedge_g0[3]) + (anti_wedge_g0[2] * right_anti_dual_g1[1]), (right_anti_dual_g2[1] * anti_wedge_g0[3]) + (anti_wedge_g0[0] * right_anti_dual_g1[2]), (right_anti_dual_g2[2] * anti_wedge_g0[3]) + (anti_wedge_g0[1] * right_anti_dual_g1[0]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (right_anti_dual_g2[2] * anti_wedge_g0[2])) - (anti_wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * float3(anti_wedge_g1)), right_anti_dual_g2[0] * anti_wedge_g0[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e1234 * this.e45) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((other.group3.xyz * this.group3.www), other.e4235 * this.e41);
        float4 anti_wedge_g3 = (other.group3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(other.group2.xyz, other.e4125)) + float4((this.group0.xyz * other.group3.www), other.e4235 * this.e15) + float4((other.group1.zxy * this.group3.yzx), other.e4315 * this.e25) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])) - (anti_wedge_g3.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g2[3])), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (right_anti_dual_g0[1] * anti_wedge_g3[1]) + (right_anti_dual_g0[2] * anti_wedge_g3[2]) + (anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2]) - (anti_wedge_g1[3] * right_anti_dual_g2[3])) + float4((anti_wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g3[3]) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (anti_wedge_g3.yzxw * right_anti_dual_g1.zxyw) - (right_anti_dual_g2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) - float4((right_anti_dual_g0 * anti_wedge_g3.www), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g1.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.group0.yy, other.e12345, other.e5) * float4(this.group0.xyz, this.e1234);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_anti_dual_g0[1]), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], (right_anti_dual_g0[0] * anti_wedge_g0[3]) + (right_anti_dual_g0[1] * anti_wedge_g3[3])) * float4(anti_wedge_g3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_orthogonally_from(FlatPoint other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz), (right_anti_dual_g0[3] * this.e4235 * other.e15) + (right_anti_dual_g0[3] * this.e4315 * other.e25) + (right_anti_dual_g0[3] * this.e4125 * other.e35) + (right_anti_dual_g0[3] * this.e3215 * other.e45) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e1234 */
            anti_wedge_g0[3] * right_anti_dual_g0[3]
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz;
        float3 anti_wedge_g1_xyz = (this.group3.zxy * other.group1.yzx) - (this.group3.yzx * other.group1.zxy);
        float4 anti_wedge_g2 = float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((this.group3.xyz * other.group1.www), this.e1234 * other.e45) - (other.group1.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group0.xyz, other.e4315)) + (other.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * other.e4125) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45)) - float4((this.group1.zxy * other.group1.yzx), this.e4235 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g1.yzx)), anti_wedge_g2[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g3.xyz * right_anti_dual_g1.www), anti_wedge_g2[3] * right_anti_dual_g0[3]) - (right_anti_dual_g1.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[3]) * float4(anti_wedge_g0, this.e1234 * other.e3215)) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[1])) + (right_anti_dual_g1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) + float4(float3(0.0), (anti_wedge_g2[2] * right_anti_dual_g1[2]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g3[3] * right_anti_dual_g0[3])) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group1.xyz;
        float anti_wedge_g1_w = -(this.e4235 * other.e415) - (this.e4315 * other.e425) - (this.e4125 * other.e435);
        float4 anti_wedge_g2 = float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) - float4((other.group1.zxy * this.group3.yzx), this.e41 * other.e235);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * anti_wedge_g2.www), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_w * other.e415) + (anti_wedge_g0[1] * other.e125), (anti_wedge_g1_w * other.e425) + (anti_wedge_g0[2] * other.e235), (anti_wedge_g1_w * other.e435) + (anti_wedge_g0[0] * other.e315), -(anti_wedge_g1_xyz[0] * other.e235) - (anti_wedge_g1_xyz[1] * other.e315) - (anti_wedge_g1_xyz[2] * other.e125) - (anti_wedge_g2[1] * other.e425) - (anti_wedge_g2[2] * other.e435)) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(((float3(this.e1234) * other.group0.xyz) + (this.group0.xyz * other.group0.www)), (this.e1234 * other.e5) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435));
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + float4((float3(this.e1234) * other.group1.xyz), -(this.e4235 * other.e415) - (this.e4315 * other.e425) - (this.e4125 * other.e435));
        float4 anti_wedge_g2 = float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group2.xyz) + (this.group3.zxy * other.group1.yzx) - (this.group3.yzx * other.group1.zxy)), this.e1234 * other.e12345);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0 * float4(anti_wedge_g1.www, anti_wedge_g3[3])) + (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], 1.0) * float4(anti_wedge_g0.yzx, -(anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2]))) + float4((anti_wedge_g3.xyz * right_anti_dual_g0.www), anti_wedge_g0[3] * right_anti_dual_g1[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 anti_wedge_g1 = (float4(this.e1234) * other.group3) + (this.group3.yzxy * float4(other.group5.zxy, other.e42)) + float4((this.group0.xyz * other.group9.www), this.e4235 * other.e41) + float4((this.group1.yzx * other.group9.zxy), this.e4125 * other.e43) - (float4(other.e1234) * float4(this.group2.xyz, this.e45)) - (other.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group4.xyz * this.group3.www), this.e41 * other.e4235) - float4((other.group5.yzx * this.group3.zxy), this.e42 * other.e4315);
        float anti_wedge_g2 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45);
        float4 anti_wedge_g3 = float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * float4(this.group2.xyz, this.e45)) - float4((other.group8.zxy * this.group3.yzx), this.e4235 * other.e415);
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz) + (other.group7.zxy * this.group3.yzx) - (other.group7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group1.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 anti_wedge_g6 = (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group9.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group3;
        float anti_wedge_g10 = this.e1234 * other.e12345;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g1_xyz = (this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy);
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (float4(this.e12 * other.e4315, this.e23 * other.e4125, this.e31 * other.e4235, (this.e25 * other.e4315) + (this.e35 * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g0.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g3.xyz), -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g1_xyz.yzx, (anti_wedge_g2[1] * right_anti_dual_g0[1]) + (anti_wedge_g2[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(right_anti_dual_g0[3]) * float4(anti_wedge_g0, this.e1234 * other.e3215)) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0 = (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group0.zxy, other.e1234));
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), -(this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = float4(-(this.e12 * other.e4315) - (this.e15 * other.e1234), -(this.e23 * other.e4125) - (this.e25 * other.e1234), -(this.e31 * other.e4235) - (this.e35 * other.e1234), (this.e25 * other.e4315) + (this.e35 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g2[3] * other.e3215) - (anti_wedge_g3.zxyw * right_anti_dual_g0.yzxw), 
            /* e15, e25, e35, e1234 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(anti_wedge_g3.xyz, -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3]))) - (right_anti_dual_g0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g1[1] * right_anti_dual_g0[2]) - (anti_wedge_g2[0] * right_anti_dual_g0[3]), -(anti_wedge_g1[2] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[3]), -(anti_wedge_g1[0] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[3]), (anti_wedge_g2[1] * right_anti_dual_g0[1]) + (anti_wedge_g2[2] * right_anti_dual_g0[2])) + (float4(other.e3215) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = (this.group3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((this.group0.xyz * other.group0.www), this.e1234 * other.e5) + float4((other.group1.xyz * this.group2.www), this.e4235 * other.e1) - float4((this.group3.zxy * other.group0.yzx), this.e41 * other.e235);
        float4 anti_wedge_g1 = float4((this.e1234 * other.e235) + (this.e3215 * other.e423), (this.e1234 * other.e315) + (this.e3215 * other.e431), (this.e1234 * other.e125) + (this.e3215 * other.e412), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group1) - (this.group3.xyzx * other.group1.wwwx);
        float4 anti_wedge_g2 = float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group2.xyz) + (this.group3.zxy * other.group2.yzx) - (this.group3.yzx * other.group2.zxy)), this.e1234 * other.e12345);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g3[0] * right_anti_dual_g0[3]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g3[1] * right_anti_dual_g0[3]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g3[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + (anti_wedge_g0.yzxw * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[3])) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g0[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = float4(float3(0.0), (this.e4235 * other.e41) + (this.e4315 * other.e42) + (this.e4125 * other.e43) - (this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) + float4((this.group3.xyz * other.group3.www), this.e1234 * other.e45) - (other.group3.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + (other.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4((this.group3.yzx * other.group1.zxy), this.e35 * other.e4125) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group3.zxyz * float4(other.group1.yzx, other.e35)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - float4((this.group1.zxy * other.group3.yzx), this.e4235 * other.e15);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.zxyw * right_anti_dual_g3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (anti_wedge_g3[0] * right_anti_dual_g0[0]) + (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2]) - (anti_wedge_g1[3] * right_anti_dual_g3[3])) + float4((anti_wedge_g3.xyz * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g2[3]) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (float4(anti_wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (anti_wedge_g3.yzxz * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[2])) - float4((anti_wedge_g1.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g2.xyz * right_anti_dual_g3.www), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<DualNum> operator /(DualNum lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (right_anti_dual_g0[0] * anti_wedge_g2[0]) - (right_anti_dual_g0[1] * anti_wedge_g2[1]) - (right_anti_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2[3] * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2[3] * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g3.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum reject_orthogonally_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * other.e321 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[3] * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g1[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum reject_orthogonally_from(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * float2(other.e12345 * this.e5, other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e12345);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = other.group2 * float4(this.group0.yy, this.e12345, this.e5);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * right_anti_dual_g2[3]) + float4((float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) - float4(float3(0.0), anti_wedge_g2[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4(((other.group0.xyz * anti_wedge_g2.www) + (anti_wedge_g1.xyz * right_anti_dual_g2.www)), anti_wedge_g1[3] * right_anti_dual_g2[3]), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g2.xyz)), anti_wedge_g2[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.group0.yy, this.e12345, this.e5) * float4(other.group0.xyz, other.e1234);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g3.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g2[3] * right_anti_dual_g3[3]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) - (right_anti_dual_g0[1] * anti_wedge_g2[1]) - (right_anti_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((right_anti_dual_g0 * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g2[3]) + float4((anti_wedge_g1.xyz * right_anti_dual_g2.www), anti_wedge_g3[0] * right_anti_dual_g3[0]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), right_anti_dual_g0[0] * anti_wedge_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[0] * right_anti_dual_g2[3]), (anti_wedge_g0[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[1] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public DualNum reject_orthogonally_from(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * float2((other.e5 * this.e12345) + (other.e12345 * this.e5), other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * pow(other.e45, 2) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[3] * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public Motor reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.group0.yy, this.e12345, (this.e5 * other.e12345) + (this.e12345 * other.e5)) * float4(other.group1.xyz, 1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0.xyz * right_anti_dual_g0.www), (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e5 * other.e1234) + (this.e12345 * other.scalar);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float anti_wedge_g2 = (this.e5 * other.e12345) + (this.e12345 * other.e5);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 anti_wedge_g4 = float3(this.e12345) * other.group4.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group5.xyz;
        float4 anti_wedge_g6 = float4(this.e12345) * other.group6;
        float3 anti_wedge_g7 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g8 = float3(this.e12345) * other.group8.xyz;
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = this.e12345 * other.e1234;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) + (right_anti_dual_g0[0] * this.e12345 * other.e12345) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_orthogonally_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (this.e12345 * other.e4 * other.e5)
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
            /* e12345 */
            this.e12345 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public VersorEven reject_orthogonally_from(Sphere other) {
        float anti_wedge_g0_w = this.e5 * other.e1234;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_wedge_g3[0] * right_anti_dual_g0[0]) + (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g0[3]) + (this.e12345 * other.e3215 * other.e1234)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(anti_wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.group0.yy, this.e12345, (this.e5 * other.e12345) + (this.e12345 * other.e5)) * float4(other.group2.xyz, 1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_anti_dual_g0[3])) + (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g3.zxy * right_anti_dual_g0.yzx), anti_wedge_g2[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * right_anti_dual_g0[0]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g2[3] * right_anti_dual_g0[1]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g2[3] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension DualNum {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.group0.yy, this.e12345, (this.e5 * other.e1234) + (this.e12345 * other.scalar)) * float4(other.group0.xyz, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(anti_wedge_g0[3])) + (right_anti_dual_g3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g2[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * right_anti_dual_g2[3]) + (anti_wedge_g2[0] * right_anti_dual_g3[3]), (anti_wedge_g0[1] * right_anti_dual_g2[3]) + (anti_wedge_g2[1] * right_anti_dual_g3[3]), (anti_wedge_g0[2] * right_anti_dual_g2[3]) + (anti_wedge_g2[2] * right_anti_dual_g3[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<FlatPoint> operator /(FlatPoint lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_orthogonally_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e321 * this.e45 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float anti_wedge_g0 = other.e321 * this.e45 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_orthogonally_from(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * float4(other.e12345 * this.e15, other.e12345 * this.e25, other.e12345 * this.e35, other.e12345 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Dipole reject_orthogonally_from(Circle other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g1 = float4(float3(0.0), other.e12345 * this.e45);
        float3 anti_wedge_g2_xyz = this.group0.xyz * other.group2.www;
        float anti_wedge_g2_w = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(anti_wedge_g2_w) * other.group0.xyz), anti_wedge_g2_w * right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g2[3])) + (right_anti_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g2_w) * right_anti_dual_g2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2_xyz[2] * other.e431) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]), (anti_wedge_g2_xyz[0] * other.e412) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]), (anti_wedge_g2_xyz[1] * other.e423) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]), -(anti_wedge_g2_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) - (right_anti_dual_g2.yzxx * float4(float3(0.0), anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(other.e1234) * this.group0 * float4(-1.0);
        float anti_wedge_g1 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * right_anti_dual_g3.zxyw) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g1 * right_anti_dual_g2[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4((float2(anti_wedge_g1) * right_anti_dual_g3.xy * float2(-1.0)), anti_wedge_g1 * right_anti_dual_g3[2] * -1.0, (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(anti_wedge_g1) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_orthogonally_from(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * float4(other.e12345 * this.e15, other.e12345 * this.e25, other.e12345 * this.e35, other.e12345 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Flector reject_orthogonally_from(Flector other) {
        float anti_wedge_g0_x = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g0_x) * (other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_wedge_g0_x * other.e45) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Flector reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = float4(other.e1234) * this.group0 * float4(-1.0);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, 0.0)) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), 0.0), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((other.group7.yzx * anti_wedge_g3.zxy), 0.0) - float4(float3(0.0), anti_wedge_g3[2] * other.e435) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), 0.0) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3])
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_orthogonally_from(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public Dipole reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4(other.e1234) * this.group0 * float4(-1.0);
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Dipole(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_anti_dual_g0.zxy, other.e3215)) - float4((anti_wedge_g0.zxy * right_anti_dual_g0.yzx), anti_wedge_g1 * right_anti_dual_g0[3]), 
            /* e15, e25, e35 */
            (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g1) * right_anti_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g1 = float4(float3(0.0), this.e45 * other.e12345);
        float4 anti_wedge_g2 = float4((float3(other.e12345) * this.group0.xyz), -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0.www, right_anti_dual_g2[3])) + (float4(right_anti_dual_g2[0], right_anti_dual_g2[1], right_anti_dual_g2[2], 1.0) * float4(anti_wedge_g2.www, -(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4(float3(0.0), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(float3(0.0), anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension FlatPoint {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(other.e1234) * this.group0 * float4(-1.0);
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3])) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g1 * right_anti_dual_g3[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g0.xyzx * float4(right_anti_dual_g2.www, right_anti_dual_g0[0])) + float4((float2(anti_wedge_g1) * right_anti_dual_g3.xy * float2(-1.0)), anti_wedge_g1 * right_anti_dual_g3[2] * -1.0, (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])) - float4((float3(anti_wedge_g1) * right_anti_dual_g0.xyz), anti_wedge_g0[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Flector> operator /(Flector lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Sphere reject_orthogonally_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group1.yzxx * float4(other.group1.zxy, other.e41));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float4 anti_wedge_g1 = float4((float3(this.e3215) * other.group0.xyz), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group1.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group2.zxy * this.group1.yzx), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(AntiFlatPoint other) {
        float4 anti_wedge_g0 = float4(other.e321) * float4(this.group1.xyz, this.e45) * float4(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4((this.group1.xyz * other.group0.www * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Plane reject_orthogonally_from(AntiLine other) {
        float4 anti_wedge_g0 = float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group1.zxyx * float4(other.group0.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], 1.0) * float4(right_anti_dual_g0.yzx, -(right_anti_dual_g1[1] * anti_wedge_g0[1]) - (right_anti_dual_g1[2] * anti_wedge_g0[2]))) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group1.yzx), other.e3215 * this.e45) - (this.group1.zxyx * float4(other.group0.yzx, other.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1.zxy * right_anti_dual_g0.yzx), 0.0) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Plane reject_orthogonally_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float4 anti_wedge_g1 = float4((float3(this.e3215) * other.group0.xyz), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group2.zxy * this.group1.yzx), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e125) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431), (anti_wedge_g0[2] * other.e235) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412), (anti_wedge_g0[0] * other.e315) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423), -(anti_wedge_g1[2] * other.e125) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e235) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e412 * this.e4315, other.e423 * this.e4125, other.e431 * this.e4235, -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e15);
        float4 anti_wedge_g1 = float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((other.group0.xyz * this.group1.www), other.e12345 * this.e45) - (other.group1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = float4((float3(other.e12345) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group1.zxy), 0.0) - float4((other.group2.zxy * this.group1.yzx), 0.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group1;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g2[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(anti_wedge_g0.www, anti_wedge_g2[3])) + float4((float3(right_anti_dual_g2[3]) * anti_wedge_g2.xyz), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * right_anti_dual_g2[2]) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g3[0] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g2[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g3[1] * right_anti_dual_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g2[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g3[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[0] * right_anti_dual_g2[0]) - (anti_wedge_g1[1] * right_anti_dual_g2[1]) - (anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((other.group0.yzx * anti_wedge_g2.zxy), anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0])) - float4((anti_wedge_g0.zxy * right_anti_dual_g2.yzx), anti_wedge_g2[1] * right_anti_dual_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Sphere reject_orthogonally_from(Dipole other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group1.yzxx * float4(other.group1.zxy, other.e41));
        float anti_wedge_g1 = -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g2[0] * anti_wedge_g0[3]) + (anti_wedge_g0[2] * right_anti_dual_g1[1]), (right_anti_dual_g2[1] * anti_wedge_g0[3]) + (anti_wedge_g0[0] * right_anti_dual_g1[2]), (right_anti_dual_g2[2] * anti_wedge_g0[3]) + (anti_wedge_g0[1] * right_anti_dual_g1[0]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (right_anti_dual_g2[2] * anti_wedge_g0[2])) - (anti_wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * float3(anti_wedge_g1)), right_anti_dual_g2[0] * anti_wedge_g0[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (other.group3.yzx * this.group1.zxy) - (other.group3.zxy * this.group1.yzx);
        float anti_wedge_g1_w = other.e1234 * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((other.group3.xyz * this.group1.www), other.e1234 * this.e45);
        float4 anti_wedge_g3 = float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group1.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(other.group1.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])) - (anti_wedge_g3.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g2[3])), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (right_anti_dual_g0[1] * anti_wedge_g3[1]) + (right_anti_dual_g0[2] * anti_wedge_g3[2]) + (anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g1_w * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g3[3]) * float4(anti_wedge_g0, anti_wedge_g1_w)) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (anti_wedge_g3.yzxw * right_anti_dual_g1.zxyw) - (right_anti_dual_g2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) - float4((right_anti_dual_g0 * anti_wedge_g3.www), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public AntiDualNum reject_orthogonally_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e45 * other.e45 * this.e3215) - (other.e15 * other.e45 * this.e4235) - (other.e25 * other.e45 * this.e4315) - (other.e35 * other.e45 * this.e4125), 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0_xyz = (other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx);
        float4 anti_wedge_g1 = (other.group1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - (this.group1.wwwx * float4(other.group1.xyz, other.e15));
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g1.xyz * anti_wedge_g1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g1.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0])) + float4(float3(0.0), (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g0_xyz.yzx * right_anti_dual_g1.zxy), anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Plane reject_orthogonally_from(Line other) {
        float4 anti_wedge_g0 = float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group1.yzxx * float4(other.group1.zxy, other.e415));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group0) - (this.group1.yzxx * float4(other.group1.zxy, other.e415));
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0 * float4(anti_wedge_g0.www, anti_wedge_g1[3])) + float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = float4(-(this.e15 * other.e1234) - (this.e4125 * other.e31), -(this.e25 * other.e1234) - (this.e4235 * other.e12), -(this.e35 * other.e1234) - (this.e4315 * other.e23), (this.e4315 * other.e42) + (this.e4125 * other.e43)) + (this.group1.yzxx * float4(other.group5.zxy, other.e41)) - float4((other.group4.xyz * this.group1.www), this.e45 * other.e1234);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45);
        float4 anti_wedge_g3 = float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group0) - float4((other.group8.zxy * this.group1.yzx), this.e4235 * other.e415);
        float3 anti_wedge_g4 = (other.group7.zxy * this.group1.yzx) - (other.group7.yzx * this.group1.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group1.xyz);
        float4 anti_wedge_g6 = float4(((this.group1.zxy * other.group9.yzx) - (this.group1.yzx * other.group9.zxy)), this.e3215 * other.e1234 * -1.0);
        float3 anti_wedge_g7 = float3(other.e1234) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group1;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Flector reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0_xyz = (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float4 anti_wedge_g1 = (float4((float3(this.e3215) * other.group0.xyz), (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.wwwx * float4(this.group1.xyz, this.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g0.xyz * anti_wedge_g1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g0_xyz.yzx, (anti_wedge_g1[1] * right_anti_dual_g0[1]) + (anti_wedge_g1[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public Sphere reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0 = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy);
        float anti_wedge_g1_w = this.e3215 * other.e1234 * -1.0;
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g2_w = this.e45 * other.e1234 * -1.0;
        float3 anti_wedge_g3_xyz = float3(other.e1234) * this.group0.xyz * float3(-1.0);
        float anti_wedge_g3_w = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2_w) * right_anti_dual_g0.xyz) - (anti_wedge_g3_xyz * float3(right_anti_dual_g0[3])), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3_xyz.yzx * right_anti_dual_g0.zxy), anti_wedge_g2_w * other.e3215) - (right_anti_dual_g0.yzxw * float4(anti_wedge_g3_xyz.zxy, anti_wedge_g3_w)), 
            /* e15, e25, e35, e1234 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(anti_wedge_g3_xyz, -(anti_wedge_g1_w * right_anti_dual_g0[3]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))) - (right_anti_dual_g0.xyzx * float4(float3(anti_wedge_g3_w), anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g1_xyz[1] * right_anti_dual_g0[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[3]), -(anti_wedge_g1_xyz[2] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[3]), -(anti_wedge_g1_xyz[0] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[3]), (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) + (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2])) + (float4(other.e3215) * float4(anti_wedge_g0, anti_wedge_g1_w)) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = (this.group1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (other.group0.yzxx * float4(this.group1.zxy, this.e15));
        float4 anti_wedge_g1 = (other.group0 * float4(this.group1.www, this.e45)) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group1.xyzx * other.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group2.yzx), 0.0) - float4((this.group1.yzx * other.group2.zxy), 0.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g3[0] * right_anti_dual_g0[3]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g3[1] * right_anti_dual_g0[3]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g3[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + (anti_wedge_g0.yzxw * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[3])) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g0[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy);
        float anti_wedge_g1_w = this.e3215 * other.e1234 * -1.0;
        float4 anti_wedge_g2 = (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (this.e4315 * other.e42) + (this.e4125 * other.e43)) - float4((other.group3.xyz * this.group1.www), this.e45 * other.e1234);
        float4 anti_wedge_g3 = float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e3215 * other.e45)) + float4((this.group1.yzx * other.group1.zxy), this.e15 * other.e4235) - (this.group1.zxyy * float4(other.group1.yzx, other.e25)) - (this.group1.wwwz * float4(other.group0.xyz, other.e35)) - (other.group2.wwwx * float4(this.group0.xyz, this.e4235));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.zxyw * right_anti_dual_g3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (anti_wedge_g3[0] * right_anti_dual_g0[0]) + (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g1_w * right_anti_dual_g3[3]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g3.xyz * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g2[3]) * float4(anti_wedge_g0, anti_wedge_g1_w)) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (float4(anti_wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (anti_wedge_g3.yzxz * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[2])) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g2.xyz * right_anti_dual_g3.www), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Line> operator /(Line lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e415);
        float anti_wedge_g1 = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e423) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g1 * other.e431) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g1 * other.e412) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(right_anti_dual_g2[1], right_anti_dual_g2[2], right_anti_dual_g2[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g0[1] * right_anti_dual_g3[1]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g3[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(anti_wedge_g1) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (anti_wedge_g0.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[0]))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(AntiFlatPoint other) {
        float3 anti_wedge_g0_xyz = float3(other.e321) * this.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Line(
            /* e415, e425, e435 */
            anti_wedge_g0_xyz * float3(right_anti_dual_g0[3]) * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0_xyz.zxy * right_anti_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float3 anti_wedge_g0_xyz = float3(other.e321) * this.group0.xyz;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float3(right_anti_dual_g0[3]) * float3(-1.0)), (anti_wedge_g0_xyz[0] * right_anti_dual_g1[0]) + (anti_wedge_g0_xyz[1] * right_anti_dual_g1[1]) + (anti_wedge_g0_xyz[2] * right_anti_dual_g1[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0_xyz.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(AntiLine other) {
        float anti_wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz), anti_wedge_g0_w * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g1.xyz), anti_wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Circle reject_orthogonally_from(Circle other) {
        float4 anti_wedge_g0 = float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e415);
        float anti_wedge_g1 = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e423) + (anti_wedge_g0[3] * other.e235), (anti_wedge_g1 * other.e431) + (anti_wedge_g0[3] * other.e315), (anti_wedge_g1 * other.e412) + (anti_wedge_g0[3] * other.e125), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * right_anti_dual_g1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g1_xyz = this.group0.xyz * other.group2.www;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group1.xyz;
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float4 anti_wedge_g3 = float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group0.yzx * this.group1.zxy), other.e235 * this.e415);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g2_w * right_anti_dual_g1[0]) + (anti_wedge_g3[2] * other.e431), (anti_wedge_g2_w * right_anti_dual_g1[1]) + (anti_wedge_g3[0] * other.e412), (anti_wedge_g2_w * right_anti_dual_g1[2]) + (anti_wedge_g3[1] * other.e423), -(anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412)) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g1_xyz[0] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g2[3]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g2[3]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + float4((other.group0.xyz * anti_wedge_g3.www), 0.0) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g3[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g3[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g2[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Circle reject_orthogonally_from(Dipole other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float anti_wedge_g1_w = -(other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435);
        float4 anti_wedge_g2 = float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) - float4((this.group1.zxy * other.group3.yzx), other.e41 * this.e235);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[1] * right_anti_dual_g3[2]) + (anti_wedge_g1_xyz[0] * right_anti_dual_g2[3]) + (right_anti_dual_g0[0] * anti_wedge_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g3[0]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g2[3]) + (right_anti_dual_g0[1] * anti_wedge_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g3[1]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g2[3]) + (right_anti_dual_g0[2] * anti_wedge_g2[3]), -(anti_wedge_g1_w * right_anti_dual_g1[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * anti_wedge_g2[0]) - (right_anti_dual_g0[1] * anti_wedge_g2[1]) - (right_anti_dual_g0[2] * anti_wedge_g2[2])) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[0]), (anti_wedge_g2[1] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[1]), (anti_wedge_g2[2] * right_anti_dual_g2[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[2]), -(anti_wedge_g1_xyz[1] * right_anti_dual_g3[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g0 * right_anti_dual_g3.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g1_xyz, anti_wedge_g1_w).wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(right_anti_dual_g3[3])) + (float3(anti_wedge_g2[3]) * right_anti_dual_g2.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g2[3] * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group1.yzxx * float4(this.group1.zxy, this.e415));
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_wedge_g0[3]) * float4(right_anti_dual_g1.xyz, other.e45) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public AntiFlatPoint reject_orthogonally_from(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Motor reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0_xyz = this.group0.xyz * other.group0.www;
        float anti_wedge_g1_w = -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * right_anti_dual_g0.www), -(anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1_w) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * float3(other.e12345) * this.group1.xyz)), anti_wedge_g1_w * right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 anti_wedge_g1 = float4((this.e415 * other.e321) + (this.e315 * other.e412), (this.e425 * other.e321) + (this.e125 * other.e423), (this.e435 * other.e321) + (this.e235 * other.e431), -(this.e425 * other.e431) - (this.e435 * other.e412)) - float4((this.group1.zxy * other.group7.yzx), this.e415 * other.e423);
        float anti_wedge_g2 = -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group9.yzxx * float4(this.group1.zxy, this.e415));
        float3 anti_wedge_g4 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(other.e1234) * this.group1.xyz;
        float4 anti_wedge_g6 = float4((this.group0.xyz * float3(other.group0.yy, other.e12345)), 0.0);
        float3 anti_wedge_g8 = float3(other.e12345) * this.group1.xyz;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4(float3(0.0), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Line reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public Circle reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz * float3(right_anti_dual_g0[3])) + (anti_wedge_g0.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e3215) + (anti_wedge_g2[0] * right_anti_dual_g0[3]), (anti_wedge_g0[1] * other.e3215) + (anti_wedge_g2[1] * right_anti_dual_g0[3]), (anti_wedge_g0[2] * other.e3215) + (anti_wedge_g2[2] * right_anti_dual_g0[3]), -(anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(anti_wedge_g1_xyz, -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)).wwwx), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g1 = float4((this.group0.xyz * other.group0.www), 0.0);
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group1.xyz;
        float anti_wedge_g2_w = -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float4 anti_wedge_g3 = float4((this.e415 * other.e321) + (this.e315 * other.e412), (this.e425 * other.e321) + (this.e125 * other.e423), (this.e435 * other.e321) + (this.e235 * other.e431), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) - float4((this.group1.zxy * other.group0.yzx), this.e415 * other.e235);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g3.zxyy * float4(right_anti_dual_g0.yzx, right_anti_dual_g3[1])) + float4(float3(0.0), anti_wedge_g2_w * right_anti_dual_g3[3]) + float4(float3(0.0), (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz), anti_wedge_g3[0] * right_anti_dual_g3[0]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * right_anti_dual_g0[0]), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * right_anti_dual_g0[1]), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * right_anti_dual_g0[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g3[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g3[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Line {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float anti_wedge_g1_w = -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125);
        float4 anti_wedge_g2 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - float4((this.group1.zxy * other.group3.yzx), this.e415 * other.e23);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g1_xyz[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[0]), (anti_wedge_g1_xyz[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[1]), (anti_wedge_g1_xyz[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g0[2]), -(anti_wedge_g1_w * right_anti_dual_g1[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g0[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[0]), (anti_wedge_g2[1] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[1]), (anti_wedge_g2[2] * right_anti_dual_g3[3]) + (anti_wedge_g2[3] * right_anti_dual_g1[2]), -(anti_wedge_g1_xyz[1] * right_anti_dual_g3[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g0 * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g1_xyz, anti_wedge_g1_w).wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g2[3]) * right_anti_dual_g2.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g3.zxy)), anti_wedge_g2[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Motor> operator /(Motor lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * right_anti_dual_g2.xyz), (anti_wedge_g2_w * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float4 anti_wedge_g3 = float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((other.group0.zxy * this.group1.yzx), other.e5 * this.e12345) - (this.group1.zxyx * float4(other.group0.yzx, other.e415));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g3.yzx * right_anti_dual_g2.zxy), anti_wedge_g2_w * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public DualNum reject_orthogonally_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(AntiFlatPoint other) {
        float3 anti_wedge_g1_xyz = this.group0.xyz * other.group0.www;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g1_xyz, other.e321 * this.e12345) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1_xyz.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g1 = (this.group0 * float4(other.group0.www, other.e5)) + float4((float3(this.e12345) * other.group1.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[3] * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(AntiLine other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float3(anti_wedge_g0_w)), -(anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * right_anti_dual_g0.xyz), (anti_wedge_g0_w * right_anti_dual_g0[3]) - (anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(right_anti_dual_g1[3])) + (float3(anti_wedge_g0_w) * right_anti_dual_g1.xyz)), anti_wedge_g0_w * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiScalar reject_orthogonally_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public CircleRotor reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float4 anti_wedge_g3 = float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group0.yzx * this.group1.zxy), other.e235 * this.e415);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e235) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2_w * other.e315) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2_w * other.e125) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e125) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g3[2] * other.e235) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g3[0] * other.e315) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) - float4((other.group2.yzx * anti_wedge_g3.zxy), anti_wedge_g0[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e12345);
        float3 anti_wedge_g1_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz);
        float anti_wedge_g1_w = other.e321 * this.e12345;
        float4 anti_wedge_g2 = (other.group2 * float4(this.group0.www, this.e5)) + float4((float3(other.e12345) * this.group1.xyz), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 anti_wedge_g3 = float4((other.e412 * this.e315) + (other.e321 * this.e415), (other.e423 * this.e125) + (other.e321 * this.e425), (other.e431 * this.e235) + (other.e321 * this.e435), -(other.e431 * this.e425) - (other.e412 * this.e435)) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e415);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g0[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g0[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g1_w * right_anti_dual_g1[3]) - (anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((other.group0.yzx * anti_wedge_g3.zxy), anti_wedge_g0[3] * right_anti_dual_g2[3]) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1_xyz[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g1_xyz[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g1_xyz[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + float4((other.group0.xyz * anti_wedge_g2.www), anti_wedge_g1_w * right_anti_dual_g2[3]) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public CircleRotor reject_orthogonally_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual_g2 * float3(anti_wedge_g2_w)), -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(other.e1234) * this.group0.xyz) + (other.group0.xyz * this.group0.www);
        float anti_wedge_g0_w = (other.e1234 * this.e5) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 anti_wedge_g1 = (float4(this.e12345) * other.group1) + float4((float3(other.e1234) * this.group1.xyz), -(other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435));
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz) + (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g3.zxyy * float4(anti_wedge_g0_xyz.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (right_anti_dual_g3[3] * other.e1234 * this.e12345) - (anti_wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((right_anti_dual_g0 * float3(anti_wedge_g0_w)), anti_wedge_g3[3] * right_anti_dual_g2[3]) + float4((anti_wedge_g1.xyz * right_anti_dual_g2.www), anti_wedge_g3[0] * right_anti_dual_g3[0]) - float4((anti_wedge_g0_xyz.zxy * right_anti_dual_g3.yzx), anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0_xyz[0] * right_anti_dual_g3[3]) + (anti_wedge_g2_xyz[0] * right_anti_dual_g2[3]), (anti_wedge_g0_xyz[1] * right_anti_dual_g3[3]) + (anti_wedge_g2_xyz[1] * right_anti_dual_g2[3]), (anti_wedge_g0_xyz[2] * right_anti_dual_g3[3]) + (anti_wedge_g2_xyz[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0_w)) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0_w) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0_w * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0[1]) * float4(other.group0.yy, other.e12345, (other.e5 * this.e12345) + (other.e12345 * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiScalar reject_orthogonally_from(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * other.e45 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group0) - (other.group1.yzxx * float4(this.group1.zxy, this.e415));
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[3] * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * this.group0.www;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Motor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * right_anti_dual_g0.www), (right_anti_dual_g0[3] * other.e12345 * this.e12345) - (anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 anti_wedge_g1 = float4((this.e415 * other.e321) + (this.e12345 * other.e1), (this.e425 * other.e321) + (this.e12345 * other.e2), (this.e435 * other.e321) + (this.e12345 * other.e3), -(this.e425 * other.e431) - (this.e435 * other.e412)) + float4((other.group7.zxy * this.group1.yzx), this.e12345 * other.e4) - float4((other.group7.yzx * this.group1.zxy), this.e415 * other.e423);
        float anti_wedge_g2 = (this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((this.e12345 * other.e15) + (this.e315 * other.e4125), (this.e12345 * other.e25) + (this.e125 * other.e4235), (this.e12345 * other.e35) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (this.group0 * float4(other.group9.www, other.e45)) - (other.group9.yzxx * float4(this.group1.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group0.xyz);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group5.xyz) + (float3(other.e1234) * this.group1.xyz);
        float4 anti_wedge_g6 = float4(((float3(this.e12345) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e321);
        float3 anti_wedge_g7 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g8 = (float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group1.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = this.e12345 * other.e1234;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) + (right_anti_dual_g0[0] * this.e12345 * other.e12345) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public Motor reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group1.zxy, this.e415));
        float4 anti_wedge_g1 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g0[0]) + (anti_wedge_g1[1] * right_anti_dual_g0[1]) + (anti_wedge_g1[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g0.yzx * right_anti_dual_g0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public AntiScalar reject_orthogonally_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (this.e12345 * other.e4 * other.e5)
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
            /* e12345 */
            this.e12345 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public VersorEven reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = float4(other.e1234) * float4(this.group0.xyz, this.e5);
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g3[2] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g0[3]) + (this.e12345 * other.e3215 * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + (right_anti_dual_g0.wwwy * float4(anti_wedge_g1_xyz, anti_wedge_g3[1])), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * right_anti_dual_g0[3]) + (anti_wedge_g0[0] * other.e3215), (anti_wedge_g2_xyz[1] * right_anti_dual_g0[3]) + (anti_wedge_g0[1] * other.e3215), (anti_wedge_g2_xyz[2] * right_anti_dual_g0[3]) + (anti_wedge_g0[2] * other.e3215), -(anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(anti_wedge_g1_xyz, -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)).wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g0.zxy)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(((float3(this.e12345) * other.group1.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e321);
        float4 anti_wedge_g2 = (float4(this.e12345) * other.group2) + (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435));
        float4 anti_wedge_g3 = float4((this.e12345 * other.e1) + (this.e315 * other.e412), (this.e12345 * other.e2) + (this.e125 * other.e423), (this.e12345 * other.e3) + (this.e235 * other.e431), -(this.e425 * other.e431) - (this.e435 * other.e412)) + (this.group0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(this.group1.zxy, this.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_anti_dual_g0[3])) + (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g3.zxy * right_anti_dual_g0.yzx), anti_wedge_g2[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * right_anti_dual_g0[0]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g2[3] * right_anti_dual_g0[1]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g2[3] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Motor {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = (this.group0 * float4(other.group2.www, other.scalar)) + float4(float3(0.0), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((other.group0.xyz * this.group0.www), this.e5 * other.e1234);
        float4 anti_wedge_g1 = (float4(this.e12345) * other.group1) + float4((float3(other.e1234) * this.group1.xyz), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125));
        float3 anti_wedge_g2_xyz = (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(anti_wedge_g0[3])) + (right_anti_dual_g3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), right_anti_dual_g2[3] * this.e12345 * other.e1234) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * right_anti_dual_g3[3]) + (anti_wedge_g0[0] * right_anti_dual_g2[3]), (anti_wedge_g2_xyz[1] * right_anti_dual_g3[3]) + (anti_wedge_g0[1] * right_anti_dual_g2[3]), (anti_wedge_g2_xyz[2] * right_anti_dual_g3[3]) + (anti_wedge_g0[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<MultiVector> operator /(MultiVector lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiCircleRotor other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 anti_wedge_g1 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group9.yzxx * float4(other.group1.zxy, other.e41)) + float4((float3(this.e1234) * other.group2.xyz), other.e42 * this.e4315);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group2.xyz, other.e45);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group1.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * right_anti_dual_g2[3]) - (anti_wedge_g4[0] * right_anti_dual_g2[0]) - (anti_wedge_g4[1] * right_anti_dual_g2[1]) - (anti_wedge_g4[2] * right_anti_dual_g2[2]) - (anti_wedge_g5[0] * right_anti_dual_g1[0]) - (anti_wedge_g5[1] * right_anti_dual_g1[1]) - (anti_wedge_g5[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g0[1] * anti_wedge_g3[1]) - (right_anti_dual_g0[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g0_x), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g0_x), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x) * right_anti_dual_g2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[2] * right_anti_dual_g1[1]) + (anti_wedge_g1[3] * right_anti_dual_g2[0]), (anti_wedge_g1[0] * right_anti_dual_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g2[1]), (anti_wedge_g1[1] * right_anti_dual_g1[0]) + (anti_wedge_g1[3] * right_anti_dual_g2[2]), -(anti_wedge_g1[1] * right_anti_dual_g2[1]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) - (float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g1[0]) + (right_anti_dual_g0[1] * anti_wedge_g1[1]) + (right_anti_dual_g0[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiDipoleInversion other) {
        float anti_wedge_g0_x = (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 anti_wedge_g1 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + (float4(this.e12345) * float4(other.group3.xyz, other.e4)) - float4((other.group0.yzx * this.group8.zxy), other.e423 * this.e415) - float4((this.group7.yzx * other.group2.zxy), other.e431 * this.e425);
        float anti_wedge_g2 = (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group2.zxy, other.e415));
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g7 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g8 = float3(this.e12345) * other.group2.xyz;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g2 * right_anti_dual_g2[3]) + (anti_wedge_g1[0] * right_anti_dual_g3[0]) + (anti_wedge_g1[1] * right_anti_dual_g3[1]) + (anti_wedge_g1[2] * right_anti_dual_g3[2]) + (anti_wedge_g1[3] * right_anti_dual_g3[3]) - (anti_wedge_g7[0] * right_anti_dual_g2[0]) - (anti_wedge_g7[1] * right_anti_dual_g2[1]) - (anti_wedge_g7[2] * right_anti_dual_g2[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (anti_wedge_g6[0] * right_anti_dual_g1[0]) - (anti_wedge_g6[1] * right_anti_dual_g1[1]) - (anti_wedge_g6[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0_x) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3]), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0_x) * right_anti_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g2[0]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g2[1]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g2[2]), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * right_anti_dual_g1.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * right_anti_dual_g2[2]) + (anti_wedge_g5[0] * right_anti_dual_g1[3]) + (anti_wedge_g3[2] * other.e431) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g4[2] * right_anti_dual_g2[0]) + (anti_wedge_g5[1] * right_anti_dual_g1[3]) + (anti_wedge_g3[0] * other.e412) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g4[0] * right_anti_dual_g2[1]) + (anti_wedge_g5[2] * right_anti_dual_g1[3]) + (anti_wedge_g3[1] * other.e423) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g5[2] * right_anti_dual_g2[2]) - (anti_wedge_g3[0] * right_anti_dual_g1[0]) - (anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g2.yzxy * float4(anti_wedge_g4.zxy, anti_wedge_g5[1])) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g5[0] * right_anti_dual_g2[0]), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g2[3]) - (anti_wedge_g4[0] * right_anti_dual_g1[0]) - (anti_wedge_g4[1] * right_anti_dual_g1[1]) - (anti_wedge_g4[2] * right_anti_dual_g1[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiDualNum other) {
        float anti_wedge_g0_x = (other.e3215 * this.e4) + (other.scalar * this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0, anti_wedge_g0_x * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            float4((this.group4.xyz * float3(other.group0.xx, other.e3215) * float3(other.group0.xx, other.e3215)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            pow(float3(other.e3215), 2) * this.group7.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e1234), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiFlatPoint other) {
        float4 anti_wedge_g1 = float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((this.group7.zxy * other.group0.yzx), 0.0) - float4((this.group7.yzx * other.group0.zxy), 0.0);
        float3 anti_wedge_g5 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0, right_anti_dual_g0[3] * other.e321 * this.e12345) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g5 * float3(right_anti_dual_g0[3])), -(anti_wedge_g5[1] * right_anti_dual_g0[1]) - (anti_wedge_g5[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(float3(0.0), anti_wedge_g5[0])), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiFlector other) {
        float anti_wedge_g0_x = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45);
        float4 anti_wedge_g1 = float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((this.group7.zxy * other.group0.yzx), 0.0) - float4((this.group7.yzx * other.group0.zxy), 0.0);
        float3 anti_wedge_g5 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (right_anti_dual_g0[3] * other.e321 * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0_x), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g1 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * float3(right_anti_dual_g0[3])), -(anti_wedge_g5[1] * right_anti_dual_g0[1]) - (anti_wedge_g5[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.yzxx * float4(float3(0.0), anti_wedge_g5[0])), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiLine other) {
        float anti_wedge_g0_x = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 anti_wedge_g1 = float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0);
        float3 anti_wedge_g5 = float3(this.e12345) * other.group0.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g5[0] * right_anti_dual_g0[0]) - (anti_wedge_g5[1] * right_anti_dual_g0[1]) - (anti_wedge_g5[2] * right_anti_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0 * float3(anti_wedge_g0_x)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_anti_dual_g1 * float3(anti_wedge_g0_x), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g0[1] * anti_wedge_g1[2]) + (right_anti_dual_g1[0] * anti_wedge_g1[3]), (right_anti_dual_g0[2] * anti_wedge_g1[0]) + (right_anti_dual_g1[1] * anti_wedge_g1[3]), (right_anti_dual_g0[0] * anti_wedge_g1[1]) + (right_anti_dual_g1[2] * anti_wedge_g1[3]), -(right_anti_dual_g1[1] * anti_wedge_g1[1]) - (right_anti_dual_g1[2] * anti_wedge_g1[2])) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0])), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiMotor other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * right_anti_dual_g0[3]) - (anti_wedge_g5[0] * right_anti_dual_g0[0]) - (anti_wedge_g5[1] * right_anti_dual_g0[1]) - (anti_wedge_g5[2] * right_anti_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * right_anti_dual_g1[3], 
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0_x) * right_anti_dual_g0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[2] * right_anti_dual_g0[1]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]), (anti_wedge_g1[0] * right_anti_dual_g0[2]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]), (anti_wedge_g1[1] * right_anti_dual_g0[0]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4(float3(0.0), right_anti_dual_g1[3] * other.e3215 * this.e1234) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0])), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiPlane other) {
        float3 anti_wedge_g1_xyz = other.group0.xyz * float3(this.group0.yy, this.e12345);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1_xyz[0] * right_anti_dual_g0[0]) + (anti_wedge_g1_xyz[1] * right_anti_dual_g0[1]) + (anti_wedge_g1_xyz[2] * right_anti_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0) * float2(other.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e5 */
            right_anti_dual_g0 * other.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0) * float3(other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group9, 
            /* e1234 */
            right_anti_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Circle other) {
        float anti_wedge_g0_x = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 anti_wedge_g1 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group0.yzx * this.group8.zxy), other.e423 * this.e415) - float4((other.group2.zxy * this.group7.yzx), other.e431 * this.e425);
        float anti_wedge_g2 = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group2.zxy, other.e415));
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g7 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g8 = float3(this.e12345) * other.group2.xyz;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g7[0] * other.e235) - (anti_wedge_g7[1] * other.e315) - (anti_wedge_g7[2] * other.e125) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (anti_wedge_g6[0] * right_anti_dual_g1[0]) - (anti_wedge_g6[1] * right_anti_dual_g1[1]) - (anti_wedge_g6[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0_x) * float4(other.group2.xyz, right_anti_dual_g1[3]), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0_x) * right_anti_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * other.e235), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * other.e315), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * other.e125), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * right_anti_dual_g1.xyz) + (other.group2.zxy * anti_wedge_g1.yzx) - (other.group2.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * other.e125) + (anti_wedge_g5[0] * right_anti_dual_g1[3]) + (anti_wedge_g3[2] * other.e431) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g4[2] * other.e235) + (anti_wedge_g5[1] * right_anti_dual_g1[3]) + (anti_wedge_g3[0] * other.e412) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g4[0] * other.e315) + (anti_wedge_g5[2] * right_anti_dual_g1[3]) + (anti_wedge_g3[1] * other.e423) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g5[2] * other.e125) - (anti_wedge_g3[0] * right_anti_dual_g1[0]) - (anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g4.zxy * other.group2.yzx), anti_wedge_g5[1] * other.e315) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g5[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g4[0] * right_anti_dual_g1[0]) - (anti_wedge_g4[1] * right_anti_dual_g1[1]) - (anti_wedge_g4[2] * right_anti_dual_g1[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(CircleRotor other) {
        float anti_wedge_g0_x = (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 anti_wedge_g1 = float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e12345 * this.e3), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((other.group0.zxy * this.group8.yzx), other.e12345 * this.e4) - float4((other.group0.yzx * this.group8.zxy), other.e423 * this.e415) - float4((this.group7.yzx * other.group2.zxy), other.e431 * this.e425);
        float anti_wedge_g2 = (other.e12345 * this.e5) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((other.e315 * this.e4125) + (other.e12345 * this.e15), (other.e125 * this.e4235) + (other.e12345 * this.e25), (other.e235 * this.e4315) + (other.e12345 * this.e35), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((other.group1.xyz * this.group9.www), other.e12345 * this.e45) - (this.group9.yzxx * float4(other.group2.zxy, other.e415));
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group5.xyz) + (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group1);
        float3 anti_wedge_g7 = (float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz);
        float3 anti_wedge_g8 = (float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group2.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group9;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g2[3], (right_anti_dual_g2[3] * other.e12345 * this.e12345) - (anti_wedge_g7[0] * right_anti_dual_g2[0]) - (anti_wedge_g7[1] * right_anti_dual_g2[1]) - (anti_wedge_g7[2] * right_anti_dual_g2[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (anti_wedge_g6[0] * right_anti_dual_g1[0]) - (anti_wedge_g6[1] * right_anti_dual_g1[1]) - (anti_wedge_g6[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(right_anti_dual_g2[3]), 
            /* e5 */
            anti_wedge_g2 * right_anti_dual_g2[3], 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(right_anti_dual_g2[3])) + (float4(anti_wedge_g0_x) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1[3] * right_anti_dual_g2[0]) + (anti_wedge_g6[0] * right_anti_dual_g2[3]), (anti_wedge_g1[3] * right_anti_dual_g2[1]) + (anti_wedge_g6[1] * right_anti_dual_g2[3]), (anti_wedge_g1[3] * right_anti_dual_g2[2]) + (anti_wedge_g6[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((float3(anti_wedge_g2) * other.group0.xyz), anti_wedge_g6[3] * right_anti_dual_g2[3]) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g2) * right_anti_dual_g1.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * right_anti_dual_g2[2]) + (anti_wedge_g5[0] * right_anti_dual_g1[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[0]) + (anti_wedge_g9[0] * right_anti_dual_g2[3]), (anti_wedge_g4[2] * right_anti_dual_g2[0]) + (anti_wedge_g5[1] * right_anti_dual_g1[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[1]) + (anti_wedge_g9[1] * right_anti_dual_g2[3]), (anti_wedge_g4[0] * right_anti_dual_g2[1]) + (anti_wedge_g5[2] * right_anti_dual_g1[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[2]) + (anti_wedge_g9[2] * right_anti_dual_g2[3]), -(anti_wedge_g5[2] * right_anti_dual_g2[2]) - (anti_wedge_g3[0] * right_anti_dual_g1[0]) - (anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + float4((other.group0.yzx * anti_wedge_g3.zxy), anti_wedge_g9[3] * right_anti_dual_g2[3]) - (right_anti_dual_g2.yzxy * float4(anti_wedge_g4.zxy, anti_wedge_g5[1])) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g5[0] * right_anti_dual_g2[0]), 
            /* e1234 */
            (right_anti_dual_g2[3] * other.e12345 * this.e1234) - (anti_wedge_g4[0] * right_anti_dual_g1[0]) - (anti_wedge_g4[1] * right_anti_dual_g1[1]) - (anti_wedge_g4[2] * right_anti_dual_g1[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Dipole other) {
        float anti_wedge_g0_x = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 anti_wedge_g1 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group9.yzxy * float4(other.group1.zxy, other.e42)) + float4((float3(this.e1234) * other.group2.xyz), other.e41 * this.e4235);
        float anti_wedge_g2 = -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group2.xyz, other.e45);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group1.xyz;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g4[0] * right_anti_dual_g2[0]) - (anti_wedge_g4[1] * right_anti_dual_g2[1]) - (anti_wedge_g4[2] * right_anti_dual_g2[2]) - (anti_wedge_g5[0] * right_anti_dual_g1[0]) - (anti_wedge_g5[1] * right_anti_dual_g1[1]) - (anti_wedge_g5[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g0[1] * anti_wedge_g3[1]) - (right_anti_dual_g0[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g0_x), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g0_x), 
            /* e235, e315, e125 */
            right_anti_dual_g2 * float3(anti_wedge_g0_x), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g2[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g1[1]), (right_anti_dual_g2[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g1[2]), (right_anti_dual_g2[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g1[3]) - (right_anti_dual_g2[2] * anti_wedge_g1[2])) - (anti_wedge_g1.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * float3(anti_wedge_g2)), right_anti_dual_g2[0] * anti_wedge_g1[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g1[0]) + (right_anti_dual_g0[1] * anti_wedge_g1[1]) + (right_anti_dual_g0[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(DipoleInversion other) {
        float anti_wedge_g0_x = (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + (this.group9.yzxz * float4(other.group1.zxy, other.e43)) + float4((this.group4.xyz * other.group3.www), other.e41 * this.e4235) + float4((this.group5.yzx * other.group3.zxy), other.e42 * this.e4315) - (float4(other.e1234) * this.group3) - (other.group3.yzxy * float4(this.group5.zxy, this.e42)) - float4((other.group0.xyz * this.group9.www), other.e4235 * this.e41) - float4((other.group1.yzx * this.group9.zxy), other.e4125 * this.e43);
        float anti_wedge_g2 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float4 anti_wedge_g3 = float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * float4(other.group2.xyz, other.e45)) - float4((this.group8.zxy * other.group3.yzx), other.e4235 * this.e415);
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.zxy * other.group3.yzx) - (this.group7.yzx * other.group3.zxy);
        float3 anti_wedge_g5 = (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group1.xyz) - (float3(this.e321) * other.group3.xyz);
        float4 anti_wedge_g6 = (other.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * right_anti_dual_g3[0]) + (anti_wedge_g9[1] * right_anti_dual_g3[1]) + (anti_wedge_g9[2] * right_anti_dual_g3[2]) + (anti_wedge_g9[3] * right_anti_dual_g2[3]) + (right_anti_dual_g3[3] * other.e1234 * this.e12345) - (anti_wedge_g4[0] * right_anti_dual_g2[0]) - (anti_wedge_g4[1] * right_anti_dual_g2[1]) - (anti_wedge_g4[2] * right_anti_dual_g2[2]) - (anti_wedge_g5[0] * right_anti_dual_g1[0]) - (anti_wedge_g5[1] * right_anti_dual_g1[1]) - (anti_wedge_g5[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g0[1] * anti_wedge_g3[1]) - (right_anti_dual_g0[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3]), 
            /* e5 */
            anti_wedge_g0_x * right_anti_dual_g3[3], 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g3[3])) - (float4(anti_wedge_g2) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g1[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g4[0] * right_anti_dual_g3[3]) + (anti_wedge_g3[0] * right_anti_dual_g2[3]), (anti_wedge_g4[1] * right_anti_dual_g3[3]) + (anti_wedge_g3[1] * right_anti_dual_g2[3]), (anti_wedge_g4[2] * right_anti_dual_g3[3]) + (anti_wedge_g3[2] * right_anti_dual_g2[3]), -(anti_wedge_g5[1] * right_anti_dual_g3[1]) - (anti_wedge_g5[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)) - float4((right_anti_dual_g3.xyz * anti_wedge_g3.www), anti_wedge_g5[0] * right_anti_dual_g3[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g2[3])) + (right_anti_dual_g0 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g3.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(right_anti_dual_g3[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g2.xyz) + (anti_wedge_g3.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g6.zxyw * right_anti_dual_g3.yzxw) + (right_anti_dual_g3.wwwx * float4(anti_wedge_g7, anti_wedge_g8[0])) + float4((anti_wedge_g1.zxy * right_anti_dual_g1.yzx), anti_wedge_g8[1] * right_anti_dual_g3[1]) + float4((right_anti_dual_g2.xyz * anti_wedge_g1.www), anti_wedge_g8[2] * right_anti_dual_g3[2]) - (float4(anti_wedge_g2) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g1.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((anti_wedge_g8 * right_anti_dual_g2.www), anti_wedge_g1[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g6.yzx * right_anti_dual_g3.zxy), anti_wedge_g1[2] * right_anti_dual_g2[2]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g1[0]) + (right_anti_dual_g0[1] * anti_wedge_g1[1]) + (right_anti_dual_g0[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g6[3] * right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(DualNum other) {
        float anti_wedge_g0_x = (other.e5 * this.e1234) + (other.e12345 * this.scalar);
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float4 anti_wedge_g9 = float4(other.e12345) * this.group9;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[1], (right_anti_dual_g0[0] * anti_wedge_g1[3]) + (right_anti_dual_g0[1] * other.e12345 * this.e12345)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(right_anti_dual_g0[1]), 
            /* e5 */
            (right_anti_dual_g0[1] * other.e5 * this.e12345) + (right_anti_dual_g0[1] * other.e12345 * this.e5), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * float3(other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], (anti_wedge_g0_x * right_anti_dual_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g9[3])) * float4(anti_wedge_g9.xyz, 1.0), 
            /* e1234 */
            right_anti_dual_g0[1] * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(FlatPoint other) {
        float anti_wedge_g0_x = -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float4 anti_wedge_g1 = float4(this.e1234) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0, right_anti_dual_g0[3] * other.e45 * this.e12345) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g0_x * right_anti_dual_g0[3]), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x) * right_anti_dual_g0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz), (right_anti_dual_g0[3] * other.e15 * this.e4235) + (right_anti_dual_g0[3] * other.e25 * this.e4315) + (right_anti_dual_g0[3] * other.e35 * this.e4125) + (right_anti_dual_g0[3] * other.e45 * this.e3215) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])), 
            /* e1234 */
            anti_wedge_g1[3] * right_anti_dual_g0[3]
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Flector other) {
        float anti_wedge_g0_x = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float4 anti_wedge_g1 = float4((other.e15 * this.e1234) + (other.e4125 * this.e31), (other.e25 * this.e1234) + (other.e4235 * this.e12), (other.e35 * this.e1234) + (other.e4315 * this.e23), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((this.group4.xyz * other.group1.www), other.e45 * this.e1234) - (other.group1.yzxx * float4(this.group5.zxy, this.e41));
        float anti_wedge_g2 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group0) - float4((this.group8.zxy * other.group1.yzx), other.e4235 * this.e415);
        float3 anti_wedge_g4 = (this.group7.zxy * other.group1.yzx) - (this.group7.yzx * other.group1.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group1.xyz);
        float3 anti_wedge_g6_xyz = (other.group1.yzx * this.group9.zxy) - (other.group1.zxy * this.group9.yzx);
        float3 anti_wedge_g7 = float3(this.e1234) * other.group1.xyz;
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group1.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) - (anti_wedge_g4[0] * right_anti_dual_g0[0]) - (anti_wedge_g4[1] * right_anti_dual_g0[1]) - (anti_wedge_g4[2] * right_anti_dual_g0[2]) - (anti_wedge_g3[3] * right_anti_dual_g0[3])), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_x) * right_anti_dual_g1.xyz), 0.0), 
            /* e5 */
            anti_wedge_g0_x * right_anti_dual_g1[3], 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz) - (float3(anti_wedge_g2) * right_anti_dual_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g1[3]), 
            /* e41, e42, e43 */
            float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz, 
            /* e23, e31, e12 */
            (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g5[1] * right_anti_dual_g1[1]) - (anti_wedge_g5[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g4 * right_anti_dual_g1.www), anti_wedge_g0_x * right_anti_dual_g0[3]) - float4((right_anti_dual_g1.xyz * anti_wedge_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g0.xyz) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g1.wwwx * float4(anti_wedge_g7, anti_wedge_g8[0])) + float4(float3(0.0), (right_anti_dual_g1[3] * other.e3215 * this.e1234) - (anti_wedge_g2 * right_anti_dual_g0[3]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g6_xyz.zxy * right_anti_dual_g1.yzx), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((right_anti_dual_g0.xyz * anti_wedge_g1.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) - float4((anti_wedge_g6_xyz.yzx * right_anti_dual_g1.zxy), anti_wedge_g1[0] * right_anti_dual_g0[0]), 
            /* e1234 */
            (anti_wedge_g1[3] * right_anti_dual_g0[3]) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Line other) {
        float anti_wedge_g0_x = -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 anti_wedge_g1 = float4((other.e415 * this.e321) + (other.e315 * this.e412), (other.e425 * this.e321) + (other.e125 * this.e423), (other.e435 * this.e321) + (other.e235 * this.e431), -(other.e425 * this.e431) - (other.e435 * this.e412)) - float4((other.group1.zxy * this.group7.yzx), other.e415 * this.e423);
        float4 anti_wedge_g3 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group9.yzxx * float4(other.group1.zxy, other.e415));
        float3 anti_wedge_g4 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g5 = float3(this.e1234) * other.group1.xyz;
        float3 anti_wedge_g6_xyz = other.group0.xyz * float3(this.group0.yy, this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g6_xyz[0] * other.e415) - (anti_wedge_g6_xyz[1] * other.e425) - (anti_wedge_g6_xyz[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g1[3]) * other.group1.xyz), -(anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)), 
            /* e423, e431, e412 */
            float3(anti_wedge_g1[3]) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g1.yzx) - (other.group1.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * other.e125) + (anti_wedge_g3[3] * other.e415), (anti_wedge_g4[2] * other.e235) + (anti_wedge_g3[3] * other.e425), (anti_wedge_g4[0] * other.e315) + (anti_wedge_g3[3] * other.e435), -(anti_wedge_g5[0] * other.e235) - (anti_wedge_g5[1] * other.e315) - (anti_wedge_g5[2] * other.e125) - (anti_wedge_g3[1] * other.e425) - (anti_wedge_g3[2] * other.e435)) - float4((anti_wedge_g4.zxy * other.group1.yzx), anti_wedge_g3[0] * other.e415), 
            /* e1234 */
            -(anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Motor other) {
        float anti_wedge_g0_x = (other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 anti_wedge_g1 = float4((other.e415 * this.e321) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e12345 * this.e3), -(other.e425 * this.e431) - (other.e435 * this.e412)) + float4((this.group7.zxy * other.group1.yzx), other.e12345 * this.e4) - float4((this.group7.yzx * other.group1.zxy), other.e415 * this.e423);
        float anti_wedge_g2 = (other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((other.e12345 * this.e15) + (other.e315 * this.e4125), (other.e12345 * this.e25) + (other.e125 * this.e4235), (other.e12345 * this.e35) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (other.group0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(other.group1.zxy, other.e415));
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group0.xyz);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group5.xyz) + (float3(this.e1234) * other.group1.xyz);
        float4 anti_wedge_g6 = float4(((float3(other.e12345) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e321);
        float3 anti_wedge_g7 = float3(other.e12345) * this.group7.xyz;
        float4 anti_wedge_g9 = float4(other.e12345) * this.group9;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[3], (anti_wedge_g1[3] * right_anti_dual_g1[3]) + (right_anti_dual_g0[3] * other.e12345 * this.e12345) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[0] * right_anti_dual_g0[0]) - (anti_wedge_g6[1] * right_anti_dual_g0[1]) - (anti_wedge_g6[2] * right_anti_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(right_anti_dual_g0[3]), 
            /* e5 */
            anti_wedge_g2 * right_anti_dual_g0[3], 
            /* e15, e25, e35, e45 */
            float4(((float3(anti_wedge_g0_x) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g3.xyz)), anti_wedge_g3[3] * right_anti_dual_g0[3]), 
            /* e41, e42, e43 */
            anti_wedge_g4 * float3(right_anti_dual_g0[3]), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g1.www, -(anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2]))), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * ((float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group1.xyz))) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0 * float4(anti_wedge_g3.www, anti_wedge_g9[3])) + (right_anti_dual_g1.zxyw * float4(anti_wedge_g4.yzx, anti_wedge_g0_x)) + float4((float3(right_anti_dual_g0[3]) * anti_wedge_g9.xyz), -(anti_wedge_g5[1] * right_anti_dual_g1[1]) - (anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2])) - (right_anti_dual_g1.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])), 
            /* e1234 */
            (right_anti_dual_g0[3] * other.e12345 * this.e1234) - (anti_wedge_g4[0] * right_anti_dual_g0[0]) - (anti_wedge_g4[1] * right_anti_dual_g0[1]) - (anti_wedge_g4[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 anti_wedge_g1 = float4((other.e15 * this.e1234) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e4125 * this.e31), (other.e25 * this.e1234) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e4235 * this.e12), (other.e35 * this.e1234) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e4315 * this.e23), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412) - (other.e1234 * this.e45)) + (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + (this.group9.yzxx * float4(other.group5.zxy, other.e41)) + float4((this.group4.xyz * other.group9.www), other.e45 * this.e1234) + float4((other.group7.zxy * this.group8.yzx), other.e42 * this.e4315) + float4((other.group8.yzx * this.group7.zxy), other.e43 * this.e4125) - (other.group9.yzxy * float4(this.group5.zxy, this.e42)) - float4((float3(other.e1234) * this.group3.xyz), other.e4125 * this.e43) - float4((other.group4.xyz * this.group9.www), other.e423 * this.e415) - float4((other.group5.yzx * this.group9.zxy), other.e431 * this.e425) - float4((other.group7.yzx * this.group8.zxy), other.e412 * this.e435) - float4((other.group8.zxy * this.group7.yzx), other.e4235 * this.e41);
        float anti_wedge_g2 = (other.e12345 * this.e5) + (other.e5 * this.e12345) + (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125) + (other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e425 * this.e3215) + (other.e125 * this.e4235) + (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e435 * this.e3215) + (other.e235 * this.e4315) + (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e435 * this.e4125) - (other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(other.e12345) * this.group3) + (float4(this.e12345) * other.group3) - (this.group9.yzxx * float4(other.group8.zxy, other.e415)) - float4((this.group8.zxy * other.group9.yzx), other.e425 * this.e4315);
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group4.xyz) + (float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group4.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group9.yzx) + (this.group7.zxy * other.group9.yzx) - (other.group7.yzx * this.group9.zxy) - (this.group7.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group5.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(this.e12345) * other.group5.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group9.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group6) + (other.group9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((other.group9.zxy * this.group9.yzx), other.e1234 * this.e3215);
        float3 anti_wedge_g7 = (float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e12345) * this.group8.xyz) + (float3(other.e3215) * this.group9.xyz) + (float3(this.e12345) * other.group8.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = (float4(other.e12345) * this.group9) + (float4(this.e12345) * other.group9);
        float anti_wedge_g10 = (other.e12345 * this.e1234) + (other.e1234 * this.e12345);
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) + (right_anti_dual_g0[0] * other.e12345 * this.e12345) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Plane other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215);
        float4 anti_wedge_g1 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group5.zxy, this.e41));
        float4 anti_wedge_g3 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 anti_wedge_g6_xyz = (this.group9.zxy * other.group0.yzx) - (this.group9.yzx * other.group0.zxy);
        float3 anti_wedge_g7 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * right_anti_dual_g0[0]) + (anti_wedge_g9[1] * right_anti_dual_g0[1]) + (anti_wedge_g9[2] * right_anti_dual_g0[2])), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_x) * right_anti_dual_g0.xyz), 0.0), 
            /* e5 */
            anti_wedge_g0_x * right_anti_dual_g0[3], 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz) - (float3((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)) * right_anti_dual_g0.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3]), 
            /* e41, e42, e43 */
            float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz, 
            /* e23, e31, e12 */
            (anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g4 * float3(right_anti_dual_g0[3])) - (right_anti_dual_g0.xyz * anti_wedge_g3.www)), -(anti_wedge_g5[0] * right_anti_dual_g0[0]) - (anti_wedge_g5[1] * right_anti_dual_g0[1]) - (anti_wedge_g5[2] * right_anti_dual_g0[2])), 
            /* e423, e431, e412 */
            (anti_wedge_g4.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g0.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[3])) + (anti_wedge_g3.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g6_xyz.yzx, (anti_wedge_g8[2] * right_anti_dual_g0[2]) + (right_anti_dual_g0[3] * this.e1234 * other.e3215)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.wwwx * float4(anti_wedge_g7, anti_wedge_g8[0])) + float4((anti_wedge_g6_xyz.zxy * right_anti_dual_g0.yzx), anti_wedge_g8[1] * right_anti_dual_g0[1]), 
            /* e1234 */
            -(anti_wedge_g7[0] * right_anti_dual_g0[0]) - (anti_wedge_g7[1] * right_anti_dual_g0[1]) - (anti_wedge_g7[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0_x = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1[0] * right_anti_dual_g0[0]) + (anti_wedge_g1[1] * right_anti_dual_g0[1]) + (anti_wedge_g1[2] * right_anti_dual_g0[2]) + (anti_wedge_g1[3] * right_anti_dual_g0[3]) + (right_anti_dual_g1 * this.e12345 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(anti_wedge_g0_x), 
            /* e1234 */
            anti_wedge_g0_x * right_anti_dual_g1
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
            /* e12345 */
            this.e12345 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(Sphere other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        float4 anti_wedge_g1 = float4((this.e41 * other.e3215) + (this.e31 * other.e4125), (this.e42 * other.e3215) + (this.e12 * other.e4235), (this.e43 * other.e3215) + (this.e23 * other.e4315), -(this.e45 * other.e1234) - (this.e43 * other.e4125)) - (other.group0.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(other.e1234) * this.group3.xyz), this.e42 * other.e4315);
        float4 anti_wedge_g3 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) - (other.group0.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group6.xyz) + (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) - (float3(this.e321) * other.group0.xyz);
        float4 anti_wedge_g6 = float4((this.group9.zxy * other.group0.yzx), this.e1234 * other.e3215) - (this.group9.yzxw * float4(other.group0.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * right_anti_dual_g0[0]) + (anti_wedge_g9[1] * right_anti_dual_g0[1]) + (anti_wedge_g9[2] * right_anti_dual_g0[2]) + (anti_wedge_g9[3] * right_anti_dual_g0[3]) + (this.e12345 * other.e3215 * other.e1234)), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(anti_wedge_g0_x), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) - (right_anti_dual_g0 * float4((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215))), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g1[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g4[0] * other.e3215) + (anti_wedge_g3[0] * right_anti_dual_g0[3]), (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g3[1] * right_anti_dual_g0[3]), (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g3[2] * right_anti_dual_g0[3]), -(anti_wedge_g5[1] * right_anti_dual_g0[1]) - (anti_wedge_g5[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[3])) + (anti_wedge_g4.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g0.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g3.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g8[0] * right_anti_dual_g0[3]) - (anti_wedge_g6[1] * right_anti_dual_g0[2]), -(anti_wedge_g8[1] * right_anti_dual_g0[3]) - (anti_wedge_g6[2] * right_anti_dual_g0[0]), -(anti_wedge_g8[2] * right_anti_dual_g0[3]) - (anti_wedge_g6[0] * right_anti_dual_g0[1]), (anti_wedge_g8[2] * right_anti_dual_g0[2]) + (anti_wedge_g6[3] * other.e3215)) + (right_anti_dual_g0.yzxy * float4(anti_wedge_g6.zxy, anti_wedge_g8[1])) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[0] * right_anti_dual_g0[0]), 
            /* e1234 */
            -(anti_wedge_g7[0] * right_anti_dual_g0[0]) - (anti_wedge_g7[1] * right_anti_dual_g0[1]) - (anti_wedge_g7[2] * right_anti_dual_g0[2]) - (anti_wedge_g6[3] * right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(VersorEven other) {
        float anti_wedge_g0_x = (this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4((this.e1 * other.e12345) + (this.e415 * other.e321) + (this.e321 * other.e415) + (this.e315 * other.e412), (this.e2 * other.e12345) + (this.e425 * other.e321) + (this.e321 * other.e425) + (this.e125 * other.e423), (this.e3 * other.e12345) + (this.e435 * other.e321) + (this.e321 * other.e435) + (this.e235 * other.e431), -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412) - (this.e412 * other.e435)) + (float4(this.e12345) * other.group3) + float4((this.group7.zxy * other.group2.yzx), this.e4 * other.e12345) - float4((this.group7.yzx * other.group2.zxy), this.e423 * other.e415) - float4((this.group8.zxy * other.group0.yzx), this.e431 * other.e425);
        float anti_wedge_g2 = (this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((this.e4125 * other.e315) + (this.e3215 * other.e415), (this.e4235 * other.e125) + (this.e3215 * other.e425), (this.e4315 * other.e235) + (this.e3215 * other.e435), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + (float4(other.e12345) * this.group3) - (this.group9.yzxx * float4(other.group2.zxy, other.e415));
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group1.xyz) + (float3(other.e12345) * this.group4.xyz) + (this.group9.yzx * other.group0.zxy) - (this.group9.zxy * other.group0.yzx);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) + (float3(other.e12345) * this.group5.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group6);
        float3 anti_wedge_g7 = (float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group7.xyz);
        float3 anti_wedge_g8 = (float3(this.e12345) * other.group2.xyz) + (float3(other.e12345) * this.group8.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group9;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[3], (anti_wedge_g2 * right_anti_dual_g2[3]) + (anti_wedge_g1[0] * right_anti_dual_g3[0]) + (anti_wedge_g1[1] * right_anti_dual_g3[1]) + (anti_wedge_g1[2] * right_anti_dual_g3[2]) + (anti_wedge_g1[3] * right_anti_dual_g3[3]) + (right_anti_dual_g0[3] * this.e12345 * other.e12345) - (anti_wedge_g7[0] * right_anti_dual_g2[0]) - (anti_wedge_g7[1] * right_anti_dual_g2[1]) - (anti_wedge_g7[2] * right_anti_dual_g2[2]) - (anti_wedge_g8[0] * right_anti_dual_g0[0]) - (anti_wedge_g8[1] * right_anti_dual_g0[1]) - (anti_wedge_g8[2] * right_anti_dual_g0[2]) - (anti_wedge_g6[0] * right_anti_dual_g1[0]) - (anti_wedge_g6[1] * right_anti_dual_g1[1]) - (anti_wedge_g6[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(right_anti_dual_g0[3]), 
            /* e5 */
            anti_wedge_g2 * right_anti_dual_g0[3], 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(right_anti_dual_g0[3])) + (float4(anti_wedge_g0_x) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * right_anti_dual_g0[0]) + (anti_wedge_g6[0] * right_anti_dual_g0[3]), (anti_wedge_g2 * right_anti_dual_g0[1]) + (anti_wedge_g6[1] * right_anti_dual_g0[3]), (anti_wedge_g2 * right_anti_dual_g0[2]) + (anti_wedge_g6[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((right_anti_dual_g2.xyz * anti_wedge_g1.www), anti_wedge_g6[3] * right_anti_dual_g0[3]) - (anti_wedge_g1.xyzx * right_anti_dual_g1.wwwx), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) + (anti_wedge_g1.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g1.yzx * right_anti_dual_g0.zxy), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g2) * right_anti_dual_g1.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g5[0] * right_anti_dual_g1[3]) + (anti_wedge_g3[2] * right_anti_dual_g0[1]) + (anti_wedge_g3[3] * right_anti_dual_g1[0]) + (anti_wedge_g9[0] * right_anti_dual_g0[3]), (anti_wedge_g5[1] * right_anti_dual_g1[3]) + (anti_wedge_g3[0] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g1[1]) + (anti_wedge_g9[1] * right_anti_dual_g0[3]), (anti_wedge_g5[2] * right_anti_dual_g1[3]) + (anti_wedge_g3[1] * right_anti_dual_g0[0]) + (anti_wedge_g3[3] * right_anti_dual_g1[2]) + (anti_wedge_g9[2] * right_anti_dual_g0[3]), -(anti_wedge_g5[2] * right_anti_dual_g2[2]) - (anti_wedge_g3[0] * right_anti_dual_g1[0]) - (anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g4.yzx * right_anti_dual_g2.zxy), anti_wedge_g9[3] * right_anti_dual_g0[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g5[1] * right_anti_dual_g2[1]), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g2[3]) + (right_anti_dual_g0[3] * this.e1234 * other.e12345) - (anti_wedge_g4[0] * right_anti_dual_g1[0]) - (anti_wedge_g4[1] * right_anti_dual_g1[1]) - (anti_wedge_g4[2] * right_anti_dual_g1[2]) - (anti_wedge_g5[0] * right_anti_dual_g0[0]) - (anti_wedge_g5[1] * right_anti_dual_g0[1]) - (anti_wedge_g5[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_orthogonally_from(VersorOdd other) {
        float anti_wedge_g0_x = (this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + (this.group9.yzxz * float4(other.group1.zxy, other.e43)) + float4((this.group4.xyz * other.group3.www), this.e4235 * other.e41) + float4((this.group5.yzx * other.group3.zxy), this.e4315 * other.e42) - (other.group3.yzxx * float4(this.group5.zxy, this.e41)) - float4((this.group9.zxy * other.group1.yzx), this.e43 * other.e4125) - float4((this.group3.xyz * other.group2.www), this.e42 * other.e4315) - float4((other.group0.xyz * this.group9.www), this.e45 * other.e1234);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45);
        float4 anti_wedge_g3 = float4((this.e415 * other.e3215) + (this.e315 * other.e4125), (this.e425 * other.e3215) + (this.e125 * other.e4235), (this.e435 * other.e3215) + (this.e235 * other.e4315), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (float4(this.e12345) * float4(other.group2.xyz, other.e45)) - (other.group3.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group6.xyz) + (this.group7.zxy * other.group3.yzx) - (this.group7.yzx * other.group3.zxy);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group3.xyz);
        float4 anti_wedge_g6 = float4((this.group9.zxy * other.group3.yzx), this.e1234 * other.e3215) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * right_anti_dual_g0[3]) + (anti_wedge_g9[0] * right_anti_dual_g3[0]) + (anti_wedge_g9[1] * right_anti_dual_g3[1]) + (anti_wedge_g9[2] * right_anti_dual_g3[2]) + (anti_wedge_g9[3] * right_anti_dual_g3[3]) + (right_anti_dual_g2[3] * this.e12345 * other.e1234) - (anti_wedge_g4[0] * right_anti_dual_g2[0]) - (anti_wedge_g4[1] * right_anti_dual_g2[1]) - (anti_wedge_g4[2] * right_anti_dual_g2[2]) - (anti_wedge_g5[0] * right_anti_dual_g1[0]) - (anti_wedge_g5[1] * right_anti_dual_g1[1]) - (anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g3[3] * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0_x), 
            /* e5 */
            anti_wedge_g0_x * right_anti_dual_g2[3], 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g2[3])) - (right_anti_dual_g3 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g1[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g4[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[0] * right_anti_dual_g3[3]), (anti_wedge_g4[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[1] * right_anti_dual_g3[3]), (anti_wedge_g4[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[2] * right_anti_dual_g3[3]), -(anti_wedge_g5[1] * right_anti_dual_g3[1]) - (anti_wedge_g5[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g3[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g0.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g3.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(right_anti_dual_g2[3])) + (float3(anti_wedge_g0_x) * right_anti_dual_g2.xyz) + (anti_wedge_g3.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g6.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g2[3])) + float4((anti_wedge_g7 * right_anti_dual_g2.www), anti_wedge_g8[0] * right_anti_dual_g3[0]) + float4((anti_wedge_g1.zxy * right_anti_dual_g1.yzx), anti_wedge_g8[1] * right_anti_dual_g3[1]) + float4((right_anti_dual_g2.xyz * anti_wedge_g1.www), anti_wedge_g8[2] * right_anti_dual_g3[2]) - (float4(anti_wedge_g2) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (anti_wedge_g1.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((anti_wedge_g8 * right_anti_dual_g3.www), anti_wedge_g1[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g6.yzx * right_anti_dual_g3.zxy), anti_wedge_g1[2] * right_anti_dual_g2[2]), 
            /* e1234 */
            (anti_wedge_g1[0] * right_anti_dual_g0[0]) + (anti_wedge_g1[1] * right_anti_dual_g0[1]) + (anti_wedge_g1[2] * right_anti_dual_g0[2]) + (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g6[3] * right_anti_dual_g3[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Plane> operator /(Plane lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Sphere reject_orthogonally_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((float3(this.e3215) * other.group0.xyz), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = (float4(other.e125 * this.e4315, other.e235 * this.e4125, other.e315 * this.e4235, (other.e2 * this.e4315) + (other.e3 * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(AntiFlatPoint other) {
        float3 anti_wedge_g0 = float3(other.e321) * this.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float3(right_anti_dual_g0[3])), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4((this.group0.xyz * other.group0.www * float3(-1.0)), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(AntiLine other) {
        float4 anti_wedge_g0 = float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], 1.0) * float4(right_anti_dual_g0.yzx, -(right_anti_dual_g1[1] * anti_wedge_g0[1]) - (right_anti_dual_g1[2] * anti_wedge_g0[2]))) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1.zxy * right_anti_dual_g0.yzx), 0.0) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * float4(other.e12345 * this.e4235, other.e12345 * this.e4315, other.e12345 * this.e4125, other.e12345 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Sphere reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((float3(this.e3215) * other.group0.xyz), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx);
        float3 anti_wedge_g2 = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e125) + (anti_wedge_g2[2] * other.e431) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]), (anti_wedge_g0[2] * other.e235) + (anti_wedge_g2[0] * other.e412) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]), (anti_wedge_g0[0] * other.e315) + (anti_wedge_g2[1] * other.e423) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]), -(anti_wedge_g2[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e235) - (anti_wedge_g1[1] * other.e315) - (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2[0] * right_anti_dual_g1[0]) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g2[1] * right_anti_dual_g1[1]), 
            /* e1234 */
            -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((float3(this.e3215) * other.group0.xyz), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - float4((other.group2.zxy * this.group0.yzx), 0.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(right_anti_dual_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2.xyz * right_anti_dual_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * right_anti_dual_g2[2]) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g3[0] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g2[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g3[1] * right_anti_dual_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g2[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g3[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[0] * right_anti_dual_g2[0]) - (anti_wedge_g1[1] * right_anti_dual_g2[1]) - (anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((other.group0.yzx * anti_wedge_g2.zxy), anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, right_anti_dual_g1[0])) - float4((anti_wedge_g0.zxy * right_anti_dual_g2.yzx), anti_wedge_g2[1] * right_anti_dual_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Sphere reject_orthogonally_from(Dipole other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41));
        float anti_wedge_g1 = -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g2[0] * anti_wedge_g0[3]) + (anti_wedge_g0[2] * right_anti_dual_g1[1]), (right_anti_dual_g2[1] * anti_wedge_g0[3]) + (anti_wedge_g0[0] * right_anti_dual_g1[2]), (right_anti_dual_g2[2] * anti_wedge_g0[3]) + (anti_wedge_g0[1] * right_anti_dual_g1[0]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (right_anti_dual_g2[2] * anti_wedge_g0[2])) - (anti_wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * float3(anti_wedge_g1)), right_anti_dual_g2[0] * anti_wedge_g0[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx);
        float anti_wedge_g1_w = other.e1234 * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (float4((float3(this.e3215) * other.group3.xyz), (other.e42 * this.e4315) + (other.e43 * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group3.www, other.e41));
        float4 anti_wedge_g3 = float4(other.e12 * this.e4315, other.e23 * this.e4125, other.e31 * this.e4235, -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])) - (anti_wedge_g3.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g2[3])), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (right_anti_dual_g0[1] * anti_wedge_g3[1]) + (right_anti_dual_g0[2] * anti_wedge_g3[2]) + (anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g1_w * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g3[3]) * float4(anti_wedge_g0, anti_wedge_g1_w)) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (anti_wedge_g3.yzxw * right_anti_dual_g1.zxyw) - (right_anti_dual_g2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) - float4((right_anti_dual_g0 * anti_wedge_g3.www), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * float4(other.e12345 * this.e4235, other.e12345 * this.e4315, other.e12345 * this.e4125, other.e12345 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public AntiDualNum reject_orthogonally_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e45 * other.e45 * this.e3215) - (other.e15 * other.e45 * this.e4235) - (other.e25 * other.e45 * this.e4315) - (other.e35 * other.e45 * this.e4125), 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), -(other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - (this.group0.wwwx * float4(other.group1.xyz, other.e15));
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_anti_dual_g1.xyz * anti_wedge_g1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g1.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0])) + float4(float3(0.0), (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g0_xyz.yzx * right_anti_dual_g1.zxy), anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(Line other) {
        float4 anti_wedge_g0 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group1.zxy, other.e415));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Flector reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group1.zxy, other.e415));
        float4 anti_wedge_g1 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0 * float4(anti_wedge_g0.www, anti_wedge_g1[3])) + float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215);
        float4 anti_wedge_g1 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group5.zxy, other.e41));
        float anti_wedge_g2 = -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group8.zxy, other.e415));
        float3 anti_wedge_g4 = (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group0.xyz);
        float4 anti_wedge_g6 = float4(((other.group9.yzx * this.group0.zxy) - (other.group9.zxy * this.group0.yzx)), other.e1234 * this.e3215 * -1.0);
        float3 anti_wedge_g7 = float3(other.e1234) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group0;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Plane reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float3 anti_wedge_g1 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g0.yzx, (anti_wedge_g1[1] * right_anti_dual_g0[1]) + (anti_wedge_g1[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Sphere reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0 = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public Sphere reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float anti_wedge_g1_w = this.e3215 * other.e1234 * -1.0;
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g1_xyz[1] * right_anti_dual_g0[2]) - (anti_wedge_g2[0] * right_anti_dual_g0[3]), -(anti_wedge_g1_xyz[2] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[3]), -(anti_wedge_g1_xyz[0] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[3]), (anti_wedge_g1_w * other.e3215) + (anti_wedge_g2[2] * right_anti_dual_g0[2])) + (right_anti_dual_g0.yzxy * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[1])) + float4((anti_wedge_g0 * float3(other.e3215)), anti_wedge_g2[0] * right_anti_dual_g0[0]), 
            /* e1234 */
            -(anti_wedge_g1_w * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = (float4(this.e4125 * other.e431, this.e4235 * other.e412, this.e4315 * other.e423, (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.yzxx * float4(other.group0.zxy, other.e1));
        float4 anti_wedge_g1 = float4((float3(this.e3215) * other.group0.xyz), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group0.xyzx * other.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((this.group0.yzx * other.group2.zxy), 0.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g3[0] * right_anti_dual_g0[3]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g3[1] * right_anti_dual_g0[3]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g3[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + (anti_wedge_g0.yzxw * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[3])) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g0[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float anti_wedge_g1_w = this.e3215 * other.e1234 * -1.0;
        float4 anti_wedge_g2 = (float4((float3(this.e3215) * other.group3.xyz), (this.e4315 * other.e42) + (this.e4125 * other.e43)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group3.www, other.e41));
        float4 anti_wedge_g3 = float4(this.e4315 * other.e12, this.e4125 * other.e23, this.e4235 * other.e31, -(this.e4125 * other.e35) - (this.e3215 * other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.zxyw * right_anti_dual_g3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (anti_wedge_g3[0] * right_anti_dual_g0[0]) + (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g1_w * right_anti_dual_g3[3]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2])) + float4((anti_wedge_g3.xyz * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g2[3]) * float4(anti_wedge_g0, anti_wedge_g1_w)) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (float4(anti_wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (anti_wedge_g3.yzxz * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[2])) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g2.xyz * right_anti_dual_g3.www), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<RoundPoint> operator /(RoundPoint lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public DualNum reject_orthogonally_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * this.e4) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float anti_wedge_g0 = other.e3215 * this.e4;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e5 */
            right_anti_dual_g0 * other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float anti_wedge_g1 = other.e12345 * this.e5;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e423) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g1 * other.e431) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g1 * other.e412) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e4 */
            float4(((float3(anti_wedge_g1) * right_anti_dual_g1.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g2.yzx)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g2[3]) * float4(anti_wedge_g0.xyz, anti_wedge_g1)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float anti_wedge_g0 = (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), right_anti_dual_g0[0] * anti_wedge_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), right_anti_dual_g0[1] * other.e12345 * this.e5), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiFlector reject_orthogonally_from(Flector other) {
        float anti_wedge_g0 = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(anti_wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float anti_wedge_g1 = other.e12345 * this.e5;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3]), 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1) * right_anti_dual_g0.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g1.yzx)), anti_wedge_g1 * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5);
        float4 anti_wedge_g1 = float4(other.e12345) * this.group0;
        float anti_wedge_g2 = other.e12345 * this.e5;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[3] * right_anti_dual_g3[0]), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[3] * right_anti_dual_g3[1]), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), 0.0), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), 0.0) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public AntiPlane reject_orthogonally_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public RoundPoint reject_orthogonally_from(Sphere other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            anti_wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float anti_wedge_g1 = this.e5 * other.e12345;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g0.yzx, (anti_wedge_g1 * right_anti_dual_g2[3]) + (anti_wedge_g0[2] * right_anti_dual_g3[2]) + (anti_wedge_g0[3] * right_anti_dual_g3[3])) * float4(-1.0, -1.0, -1.0, 1.0)) + (anti_wedge_g0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g3[0])) + (anti_wedge_g0.wwwy * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[1])), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * right_anti_dual_g0[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g1 * right_anti_dual_g0[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g1 * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) - (anti_wedge_g0.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1) * right_anti_dual_g1.xyz) + (anti_wedge_g0.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g0.zxy * right_anti_dual_g2.yzx)), anti_wedge_g1 * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Scalar> operator /(Scalar lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public Scalar reject_orthogonally_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor reject_orthogonally_from(CircleRotor other) {
        float anti_wedge_g0 = other.e12345 * this.scalar;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(anti_wedge_g0) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiDualNum reject_orthogonally_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * this.scalar) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public AntiMotor reject_orthogonally_from(Motor other) {
        float anti_wedge_g0 = other.e12345 * this.scalar;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(anti_wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0 = other.e12345 * this.scalar;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            anti_wedge_g0 * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Scalar {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float anti_wedge_g0 = this.scalar * other.e12345;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<Sphere> operator /(Sphere lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)) + float4((float3(this.e1234) * other.group2.xyz), other.e42 * this.e4315);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = (float4(other.e125 * this.e4315, other.e235 * this.e4125, other.e315 * this.e4235, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1));
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public AntiDualNum reject_orthogonally_from(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * other.e3215 * this.e1234, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Plane reject_orthogonally_from(AntiFlatPoint other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float3(right_anti_dual_g0[3])), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g0 = (float4(other.e321 * this.e4235, other.e321 * this.e4315, other.e321 * this.e4125, (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + float4((float3(this.e1234) * other.group0.xyz), other.e1 * this.e4235);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Plane reject_orthogonally_from(AntiLine other) {
        float4 anti_wedge_g0 = float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], 1.0) * float4(right_anti_dual_g0.yzx, -(right_anti_dual_g1[1] * anti_wedge_g0[1]) - (right_anti_dual_g1[2] * anti_wedge_g0[2]))) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Flector reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1.zxy * right_anti_dual_g0.yzx), right_anti_dual_g1[3] * other.e3215 * this.e1234) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Plane reject_orthogonally_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e1234 */
            right_anti_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx);
        float3 anti_wedge_g2 = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e125) + (anti_wedge_g2[2] * other.e431) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]), (anti_wedge_g0[2] * other.e235) + (anti_wedge_g2[0] * other.e412) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]), (anti_wedge_g0[0] * other.e315) + (anti_wedge_g2[1] * other.e423) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]), -(anti_wedge_g2[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e235) - (anti_wedge_g1[1] * other.e315) - (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2[0] * right_anti_dual_g1[0]) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g2[1] * right_anti_dual_g1[1]), 
            /* e1234 */
            -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group0.xyzx);
        float3 anti_wedge_g2_xyz = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx);
        float anti_wedge_g2_w = other.e12345 * this.e1234;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(right_anti_dual_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_xyz * right_anti_dual_g2.www), (anti_wedge_g2_w * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * right_anti_dual_g2[2]) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g3[0] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g2[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g3[1] * right_anti_dual_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g2[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g3[2] * right_anti_dual_g2[3]), -(anti_wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g2[0]) - (anti_wedge_g1[1] * right_anti_dual_g2[1]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), anti_wedge_g3[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g2.yzx), anti_wedge_g2_xyz[1] * right_anti_dual_g1[1]) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_w * right_anti_dual_g1[0])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(Dipole other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group0.yzxy * float4(other.group1.zxy, other.e42)) + float4((float3(this.e1234) * other.group2.xyz), other.e41 * this.e4235);
        float anti_wedge_g1 = -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g2[0] * anti_wedge_g0[3]) + (anti_wedge_g0[2] * right_anti_dual_g1[1]), (right_anti_dual_g2[1] * anti_wedge_g0[3]) + (anti_wedge_g0[0] * right_anti_dual_g1[2]), (right_anti_dual_g2[2] * anti_wedge_g0[3]) + (anti_wedge_g0[1] * right_anti_dual_g1[0]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (right_anti_dual_g2[2] * anti_wedge_g0[2])) - (anti_wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * float3(anti_wedge_g1)), right_anti_dual_g2[0] * anti_wedge_g0[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = (float4(other.e4235 * this.e3215, other.e4315 * this.e3215, other.e4125 * this.e3215, (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group3.www, other.e41));
        float4 anti_wedge_g3 = float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])) - (anti_wedge_g3.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g2[3])), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (right_anti_dual_g0[1] * anti_wedge_g3[1]) + (right_anti_dual_g0[2] * anti_wedge_g3[2]) + (anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2]) - (anti_wedge_g1[3] * right_anti_dual_g2[3])) + float4((anti_wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g3[3]) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (anti_wedge_g3.yzxw * right_anti_dual_g1.zxyw) - (right_anti_dual_g2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) - float4((right_anti_dual_g0 * anti_wedge_g3.www), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g1.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public VersorOdd reject_orthogonally_from(DualNum other) {
        float anti_wedge_g0_w = other.e5 * this.e1234;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), anti_wedge_g0_w * right_anti_dual_g0[1]), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_anti_dual_g0[1] * other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], (anti_wedge_g0_w * right_anti_dual_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g3[3])) * float4(anti_wedge_g3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(FlatPoint other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz), (right_anti_dual_g0[3] * other.e15 * this.e4235) + (right_anti_dual_g0[3] * other.e25 * this.e4315) + (right_anti_dual_g0[3] * other.e35 * this.e4125) + (right_anti_dual_g0[3] * other.e45 * this.e3215) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e1234 */
            anti_wedge_g0[3] * right_anti_dual_g0[3]
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz;
        float3 anti_wedge_g1_xyz = (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group1.xyz);
        float anti_wedge_g2_w = other.e45 * this.e1234;
        float3 anti_wedge_g3_xyz = float3(this.e1234) * other.group0.xyz;
        float anti_wedge_g3_w = -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3_xyz.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g3_xyz.zxy * right_anti_dual_g1.yzx)), anti_wedge_g2_w * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g3_xyz * right_anti_dual_g1.www), anti_wedge_g2_w * right_anti_dual_g0[3]) - (right_anti_dual_g1.xyzx * float4(float3(anti_wedge_g3_w), anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[3]) * float4(anti_wedge_g0, other.e3215 * this.e1234)) + (right_anti_dual_g1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_xyz[0])) + (float4(anti_wedge_g2_xyz, anti_wedge_g2_w).wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[1])) + float4(float3(0.0), (anti_wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g3_w * right_anti_dual_g0[3]) - (anti_wedge_g3_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g3_xyz[2] * right_anti_dual_g0[2])) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g1.zxy), anti_wedge_g3_xyz[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group1.xyz;
        float anti_wedge_g1_w = -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125);
        float3 anti_wedge_g2 = (float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_w * other.e415) + (anti_wedge_g0[1] * other.e125), (anti_wedge_g1_w * other.e425) + (anti_wedge_g0[2] * other.e235), (anti_wedge_g1_w * other.e435) + (anti_wedge_g0[0] * other.e315), -(anti_wedge_g1_xyz[0] * other.e235) - (anti_wedge_g1_xyz[1] * other.e315) - (anti_wedge_g1_xyz[2] * other.e125) - (anti_wedge_g2[1] * other.e425) - (anti_wedge_g2[2] * other.e435)) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e415), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public VersorOdd reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(this.e1234) * float4(other.group0.xyz, other.e5);
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group1.xyz;
        float anti_wedge_g1_w = -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125);
        float4 anti_wedge_g2 = float4(((float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.e12345 * this.e1234);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1_xyz * float3(right_anti_dual_g0[3])) + (float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz)), anti_wedge_g1_w * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0 * float4(float3(anti_wedge_g1_w), anti_wedge_g3[3])) + (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], 1.0) * float4(anti_wedge_g0.yzx, -(anti_wedge_g1_xyz[0] * right_anti_dual_g1[0]) - (anti_wedge_g1_xyz[1] * right_anti_dual_g1[1]) - (anti_wedge_g1_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2]))) + float4((anti_wedge_g3.xyz * right_anti_dual_g0.www), anti_wedge_g0[3] * right_anti_dual_g1[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234);
        float4 anti_wedge_g1 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e45 * this.e1234) + (other.e43 * this.e4125)) + (this.group0.yzxx * float4(other.group5.zxy, other.e41)) + float4((float3(this.e1234) * other.group3.xyz), other.e42 * this.e4315);
        float anti_wedge_g2 = -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (this.group0.yzxx * float4(other.group8.zxy, other.e415));
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group0.xyz);
        float4 anti_wedge_g6 = (other.group9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((other.group9.zxy * this.group0.yzx), other.e1234 * this.e3215);
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group0.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group0;
        float anti_wedge_g10 = other.e12345 * this.e1234;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g1_xyz = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g1_xyz.yzx, (anti_wedge_g2[2] * right_anti_dual_g0[2]) + (right_anti_dual_g0[3] * other.e3215 * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.wwwx * float4(anti_wedge_g0, anti_wedge_g2[0])) + float4((anti_wedge_g1_xyz.zxy * right_anti_dual_g0.yzx), anti_wedge_g2[1] * right_anti_dual_g0[1]), 
            /* e1234 */
            -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0 = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public Sphere reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group0.xyz);
        float4 anti_wedge_g1 = (other.group0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((other.group0.zxy * this.group0.yzx), other.e1234 * this.e3215);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g2[0] * right_anti_dual_g0[3]) - (anti_wedge_g1[1] * right_anti_dual_g0[2]), -(anti_wedge_g2[1] * right_anti_dual_g0[3]) - (anti_wedge_g1[2] * right_anti_dual_g0[0]), -(anti_wedge_g2[2] * right_anti_dual_g0[3]) - (anti_wedge_g1[0] * right_anti_dual_g0[1]), (anti_wedge_g2[2] * right_anti_dual_g0[2]) + (anti_wedge_g1[3] * other.e3215)) + (right_anti_dual_g0.yzxy * float4(anti_wedge_g1.zxy, anti_wedge_g2[1])) + float4((anti_wedge_g0 * float3(other.e3215)), anti_wedge_g2[0] * right_anti_dual_g0[0]), 
            /* e1234 */
            -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = (float4(this.e4125 * other.e431, this.e4235 * other.e412, this.e4315 * other.e423, (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.yzxx * float4(other.group0.zxy, other.e1)) + float4((float3(this.e1234) * other.group1.xyz), this.e4315 * other.e2);
        float4 anti_wedge_g1 = float4((this.e3215 * other.e423) + (this.e1234 * other.e235), (this.e3215 * other.e431) + (this.e1234 * other.e315), (this.e3215 * other.e412) + (this.e1234 * other.e125), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group0.xyzx * other.group1.wwwx);
        float4 anti_wedge_g2 = float4(((float3(this.e3215) * other.group1.xyz) + (this.group0.zxy * other.group2.yzx) - (this.group0.yzx * other.group2.zxy)), this.e1234 * other.e12345);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g3[0] * right_anti_dual_g0[3]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g3[1] * right_anti_dual_g0[3]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g3[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + (anti_wedge_g0.yzxw * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[3])) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g0[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz);
        float4 anti_wedge_g1 = float4((this.group0.zxy * other.group3.yzx), this.e1234 * other.e3215) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = (float4(this.e3215 * other.e4235, this.e3215 * other.e4315, this.e3215 * other.e4125, (this.e4315 * other.e42) + (this.e4125 * other.e43) + (this.e1234 * other.e45)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(other.group3.www, other.e41));
        float4 anti_wedge_g3 = float4((this.e4315 * other.e12) + (this.e1234 * other.e15), (this.e4125 * other.e23) + (this.e1234 * other.e25), (this.e4235 * other.e31) + (this.e1234 * other.e35), -(this.e4125 * other.e35) - (this.e3215 * other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.zxyw * right_anti_dual_g3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (anti_wedge_g3[0] * right_anti_dual_g0[0]) + (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2]) - (anti_wedge_g1[3] * right_anti_dual_g3[3])) + float4((anti_wedge_g3.xyz * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g2[3]) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (float4(anti_wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (anti_wedge_g3.yzxz * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[2])) - float4((anti_wedge_g1.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g2.xyz * right_anti_dual_g3.www), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<VersorEven> operator /(VersorEven lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_orthogonally_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * right_anti_dual_g2.xyz), (anti_wedge_g2_w * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float4 anti_wedge_g3 = float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((other.group0.zxy * this.group2.yzx), other.e5 * this.e12345) - (this.group2.zxyx * float4(other.group0.yzx, other.e415)) - float4((other.group2.zxy * this.group0.yzx), other.e425 * this.e315);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * right_anti_dual_g2[0]) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2_w * right_anti_dual_g2[1]) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2_w * right_anti_dual_g2[2]) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g3.yzx * right_anti_dual_g2.zxy), anti_wedge_g2_w * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public Motor reject_orthogonally_from(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.group0.xx, other.e3215, (other.e3215 * this.e4) + (other.scalar * this.e12345)) * float4(this.group0.xyz, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.scalar), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public Motor reject_orthogonally_from(AntiFlatPoint other) {
        float4 anti_wedge_g1 = float4((other.e315 * this.e412) + (other.e321 * this.e415), (other.e125 * this.e423) + (other.e321 * this.e425), (other.e235 * this.e431) + (other.e321 * this.e435), -(other.e315 * this.e425) - (other.e125 * this.e435)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual_g0[3]) * float4(anti_wedge_g1.xyz, other.e321 * this.e12345) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public Motor reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g1 = float4((other.e321 * this.e415) + (other.e1 * this.e12345), (other.e321 * this.e425) + (other.e2 * this.e12345), (other.e321 * this.e435) + (other.e3 * this.e12345), -(other.e315 * this.e425) - (other.e125 * this.e435)) + (this.group0.zxyw * float4(other.group0.yzx, other.e5)) - (other.group0.zxyx * float4(this.group0.yzx, this.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * right_anti_dual_g1[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (right_anti_dual_g0[3] * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public Motor reject_orthogonally_from(AntiLine other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float3(anti_wedge_g0_w)), -(anti_wedge_g0_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public Motor reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g1[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiScalar reject_orthogonally_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float4 anti_wedge_g3 = float4((other.e412 * this.e315) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e423 * this.e125) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e431 * this.e235) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e125 * this.e435)) - float4((other.group0.yzx * this.group2.zxy), other.e235 * this.e415) - float4((other.group2.zxy * this.group0.yzx), other.e315 * this.e425);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * right_anti_dual_g1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e235) + (anti_wedge_g3[3] * other.e423), (anti_wedge_g2_w * other.e315) + (anti_wedge_g3[3] * other.e431), (anti_wedge_g2_w * other.e125) + (anti_wedge_g3[3] * other.e412), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e125) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g3[2] * other.e235) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g3[0] * other.e315) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) - float4((other.group2.yzx * anti_wedge_g3.zxy), anti_wedge_g0[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1);
        float4 anti_wedge_g2 = (other.group2 * float4(this.group0.www, this.e5)) + float4((float3(other.e12345) * this.group2.xyz), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 anti_wedge_g3 = float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e12345 * this.e1), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e12345 * this.e2), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e12345 * this.e3), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((other.group0.zxy * this.group2.yzx), other.e12345 * this.e4) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((other.group2.zxy * this.group0.yzx), other.e431 * this.e425);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0_xyz[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[0]), (anti_wedge_g0_xyz[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[1]), (anti_wedge_g0_xyz[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g1[2]), -(anti_wedge_g0_xyz[0] * right_anti_dual_g2[0]) - (anti_wedge_g0_xyz[1] * right_anti_dual_g2[1]) - (anti_wedge_g0_xyz[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((other.group0.yzx * anti_wedge_g3.zxy), right_anti_dual_g2[3] * other.e12345 * this.e12345) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1[0] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g1[1] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g1[2] * right_anti_dual_g2[3]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + float4((other.group0.xyz * anti_wedge_g2.www), anti_wedge_g1[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_orthogonally_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual_g2 * float3(anti_wedge_g2_w)), -(anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(DipoleInversion other) {
        float4 anti_wedge_g0 = (other.group3.yzxy * float4(this.group0.zxy, this.e2)) + float4(float3(0.0), (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((other.group0.xyz * this.group0.www), other.e1234 * this.e5) + float4((this.group1.xyz * other.group2.www), other.e4235 * this.e1) - float4((other.group3.zxy * this.group0.yzx), other.e41 * this.e235);
        float4 anti_wedge_g1 = float4((other.e1234 * this.e235) + (other.e3215 * this.e423), (other.e1234 * this.e315) + (other.e3215 * this.e431), (other.e1234 * this.e125) + (other.e3215 * this.e412), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group1) - (other.group3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group2.xyz) + (other.group3.zxy * this.group2.yzx) - (other.group3.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g3.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (right_anti_dual_g3[3] * other.e1234 * this.e12345) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((right_anti_dual_g0 * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g2[3]) + float4((anti_wedge_g1.xyz * right_anti_dual_g2.www), anti_wedge_g3[0] * right_anti_dual_g3[0]) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * right_anti_dual_g2[3]) + (anti_wedge_g0[0] * right_anti_dual_g3[3]), (anti_wedge_g2_xyz[1] * right_anti_dual_g2[3]) + (anti_wedge_g0[1] * right_anti_dual_g3[3]), (anti_wedge_g2_xyz[2] * right_anti_dual_g2[3]) + (anti_wedge_g0[2] * right_anti_dual_g3[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g3[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], (right_anti_dual_g0[0] * anti_wedge_g3[3]) + (right_anti_dual_g0[1] * anti_wedge_g0[3])) * float4(anti_wedge_g0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0[1]) * float4(other.group0.yy, other.e12345, (other.e5 * this.e12345) + (other.e12345 * this.e5)) * float4(this.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[1])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_orthogonally_from(FlatPoint other) {
        float anti_wedge_g2_w = -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * right_anti_dual_g0.xyz), right_anti_dual_g0[3] * other.e45 * this.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(Flector other) {
        float4 anti_wedge_g0 = (other.group1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15));
        float4 anti_wedge_g1 = (this.group0 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group2.yzx), 0.0) - float4((other.group1.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group1;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g1.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g2[3] * right_anti_dual_g1[3]) + (anti_wedge_g3[1] * right_anti_dual_g1[1]) + (anti_wedge_g3[2] * right_anti_dual_g1[2]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3])) - (anti_wedge_g0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g0[0])), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(right_anti_dual_g1.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g1.zxy)), anti_wedge_g0[3] * right_anti_dual_g1[3]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1.xyz * anti_wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_orthogonally_from(Line other) {
        float3 anti_wedge_g1_xyz = other.group0.xyz * this.group0.www;
        float anti_wedge_g2_w = -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float4 anti_wedge_g3 = float4((other.e415 * this.e321) + (other.e315 * this.e412), (other.e425 * this.e321) + (other.e125 * this.e423), (other.e435 * this.e321) + (other.e235 * this.e431), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) - float4((other.group1.zxy * this.group0.yzx), other.e415 * this.e235);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e415) - (anti_wedge_g3[1] * other.e425) - (anti_wedge_g3[2] * other.e435)), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e125) + (anti_wedge_g3[3] * other.e415), (anti_wedge_g3[2] * other.e235) + (anti_wedge_g3[3] * other.e425), (anti_wedge_g3[0] * other.e315) + (anti_wedge_g3[3] * other.e435), -(anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435)) - float4((other.group1.yzx * anti_wedge_g3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e321);
        float4 anti_wedge_g2 = (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 anti_wedge_g3 = float4((other.e12345 * this.e1) + (other.e315 * this.e412), (other.e12345 * this.e2) + (other.e125 * this.e423), (other.e12345 * this.e3) + (other.e235 * this.e431), -(other.e425 * this.e431) - (other.e435 * this.e412)) + (other.group0 * float4(this.group1.www, this.e4)) - (this.group0.yzxx * float4(other.group1.zxy, other.e415));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(anti_wedge_g3.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g0.xyz * right_anti_dual_g0.www), anti_wedge_g3[3] * right_anti_dual_g1[3]), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g3.www, -(anti_wedge_g3[0] * right_anti_dual_g0[0]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]))), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g1.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 anti_wedge_g1 = float4((other.e1 * this.e12345) + (other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412), (other.e2 * this.e12345) + (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423), (other.e3 * this.e12345) + (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412) - (other.e412 * this.e435)) + (float4(other.e12345) * this.group3) + float4((other.group7.zxy * this.group2.yzx), other.e4 * this.e12345) - float4((other.group7.yzx * this.group2.zxy), other.e423 * this.e415) - float4((other.group8.zxy * this.group0.yzx), other.e431 * this.e425);
        float anti_wedge_g2 = (other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((other.e4125 * this.e315) + (other.e3215 * this.e415), (other.e4235 * this.e125) + (other.e3215 * this.e425), (other.e4315 * this.e235) + (other.e3215 * this.e435), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + (float4(this.e12345) * other.group3) - (other.group9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group1.xyz) + (float3(this.e12345) * other.group4.xyz) + (other.group9.yzx * this.group0.zxy) - (other.group9.zxy * this.group0.yzx);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(this.e12345) * other.group5.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group6);
        float3 anti_wedge_g7 = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz);
        float3 anti_wedge_g8 = (float3(other.e12345) * this.group2.xyz) + (float3(this.e12345) * other.group8.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = other.e1234 * this.e12345;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) + (right_anti_dual_g0[0] * other.e12345 * this.e12345) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(Plane other) {
        float4 anti_wedge_g0 = (float4(other.e4125 * this.e431, other.e4235 * this.e412, other.e4315 * this.e423, (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.yzxx * float4(this.group0.zxy, this.e1));
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group0.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((other.group0.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g2[3] * right_anti_dual_g0[3]) + (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])), 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g2.yzx * right_anti_dual_g0.zxy)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g0.xyz * anti_wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiScalar reject_orthogonally_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * right_anti_dual_g0[0]) + (anti_wedge_g0[1] * right_anti_dual_g0[1]) + (anti_wedge_g0[2] * right_anti_dual_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g0[3]) - (other.e4 * other.e5 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public AntiScalar reject_orthogonally_from(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * other.scalar * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(Sphere other) {
        float4 anti_wedge_g0 = (float4(other.e4125 * this.e431, other.e4235 * this.e412, other.e4315 * this.e423, (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.yzxx * float4(this.group0.zxy, this.e1)) + float4((float3(other.e1234) * this.group1.xyz), other.e4315 * this.e2);
        float4 anti_wedge_g1 = float4((other.e3215 * this.e423) + (other.e1234 * this.e235), (other.e3215 * this.e431) + (other.e1234 * this.e315), (other.e3215 * this.e412) + (other.e1234 * this.e125), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group0.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (other.group0.zxy * this.group2.yzx) - (other.group0.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0[1], right_anti_dual_g0[2], right_anti_dual_g0[0], 1.0) * float4(anti_wedge_g0.zxy, (anti_wedge_g3[2] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g0[3]) + (other.e3215 * other.e1234 * this.e12345)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + (right_anti_dual_g0.wwwy * float4(anti_wedge_g1.xyz, anti_wedge_g3[1])), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * right_anti_dual_g0[3]) + (anti_wedge_g0[0] * other.e3215), (anti_wedge_g2_xyz[1] * right_anti_dual_g0[3]) + (anti_wedge_g0[1] * other.e3215), (anti_wedge_g2_xyz[2] * right_anti_dual_g0[3]) + (anti_wedge_g0[2] * other.e3215), -(anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])) - (anti_wedge_g1.wwwx * right_anti_dual_g0.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g0.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g0.zxy)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1);
        float4 anti_wedge_g2 = (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 anti_wedge_g3 = float4((other.e415 * this.e321) + (other.e321 * this.e415) + (other.e315 * this.e412) + (other.e1 * this.e12345), (other.e425 * this.e321) + (other.e321 * this.e425) + (other.e125 * this.e423) + (other.e2 * this.e12345), (other.e435 * this.e321) + (other.e321 * this.e435) + (other.e235 * this.e431) + (other.e3 * this.e12345), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + (other.group0.zxyw * float4(this.group2.yzx, this.e4)) + float4((this.group3.xyz * other.group0.www), other.e4 * this.e12345) - (other.group0.yzxx * float4(this.group2.zxy, this.e415)) - float4((other.group2.zxy * this.group0.yzx), other.e431 * this.e425);
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_anti_dual_g0[3])) + (anti_wedge_g3.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3]) - (anti_wedge_g2[0] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2])) + float4((anti_wedge_g3.zxy * right_anti_dual_g0.yzx), anti_wedge_g2[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g0[0] * right_anti_dual_g2[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * right_anti_dual_g0[0]) + (anti_wedge_g3[3] * right_anti_dual_g2[0]), (anti_wedge_g2[3] * right_anti_dual_g0[1]) + (anti_wedge_g3[3] * right_anti_dual_g2[1]), (anti_wedge_g2[3] * right_anti_dual_g0[2]) + (anti_wedge_g3[3] * right_anti_dual_g2[2]), -(anti_wedge_g3[1] * right_anti_dual_g1[1]) - (anti_wedge_g3[2] * right_anti_dual_g1[2])) + (anti_wedge_g1 * float4(right_anti_dual_g0[3])) - (anti_wedge_g3.xyzx * right_anti_dual_g1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * right_anti_dual_g2.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g2.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_anti_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectOrthogonallyFromInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_orthogonally_from(VersorOdd other) {
        float4 anti_wedge_g0 = (this.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((this.group1.xyz * other.group2.www), this.e1 * other.e4235) + float4((other.group0.xyz * this.group0.www), this.e5 * other.e1234) - (this.group0.yzxx * float4(other.group3.zxy, other.e15));
        float4 anti_wedge_g1 = float4((this.e12345 * other.e23) + (this.e235 * other.e1234), (this.e12345 * other.e31) + (this.e315 * other.e1234), (this.e12345 * other.e12) + (this.e125 * other.e1234), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + (this.group0 * float4(other.group3.www, other.e45)) - (this.group1.wwwx * other.group3.xyzx);
        float3 anti_wedge_g2_xyz = (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(anti_wedge_g0[3])) + (right_anti_dual_g3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * right_anti_dual_g3[1]) + (anti_wedge_g3[2] * right_anti_dual_g3[2]) + (anti_wedge_g3[3] * right_anti_dual_g3[3]) - (anti_wedge_g2_xyz[1] * right_anti_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[0] * right_anti_dual_g2[0]) - (anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2]) - (anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[3] * right_anti_dual_g1[3])) + float4((anti_wedge_g0.yzx * right_anti_dual_g3.zxy), right_anti_dual_g2[3] * this.e12345 * other.e1234) - float4((anti_wedge_g0.zxy * right_anti_dual_g3.yzx), anti_wedge_g2_xyz[0] * right_anti_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * right_anti_dual_g3[3]) + (anti_wedge_g0[0] * right_anti_dual_g2[3]), (anti_wedge_g2_xyz[1] * right_anti_dual_g3[3]) + (anti_wedge_g0[1] * right_anti_dual_g2[3]), (anti_wedge_g2_xyz[2] * right_anti_dual_g3[3]) + (anti_wedge_g0[2] * right_anti_dual_g2[3]), -(anti_wedge_g1[1] * right_anti_dual_g3[1]) - (anti_wedge_g1[2] * right_anti_dual_g3[2])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])) - (anti_wedge_g1.wwwx * right_anti_dual_g3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g2.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_anti_dual_g3.zxy)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectOrthogonallyFromInfixPartial<VersorOdd> operator /(VersorOdd lhs, RejectOrthogonallyFromInfix rhs) {
    return RejectOrthogonallyFromInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_orthogonally_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4((other.group2.xyz * this.group2.www), other.e42 * this.e4315);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[2] * right_anti_dual_g1[1]) + (anti_wedge_g0[3] * right_anti_dual_g2[0]), (anti_wedge_g0[0] * right_anti_dual_g1[2]) + (anti_wedge_g0[3] * right_anti_dual_g2[1]), (anti_wedge_g0[1] * right_anti_dual_g1[0]) + (anti_wedge_g0[3] * right_anti_dual_g2[2]), -(anti_wedge_g0[1] * right_anti_dual_g2[1]) - (anti_wedge_g0[2] * right_anti_dual_g2[2])) - (float4(-(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) * float4(right_anti_dual_g0, right_anti_dual_g1[3])) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[0])), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431) + (anti_wedge_g2[3] * right_anti_dual_g3[0]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412) + (anti_wedge_g2[3] * right_anti_dual_g3[1]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423) + (anti_wedge_g2[3] * right_anti_dual_g3[2]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g0.yzx * right_anti_dual_g2.zxy), anti_wedge_g2[3] * right_anti_dual_g3[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Flector reject_orthogonally_from(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(((float4(other.e3215) * float4(this.group0.xyz, this.e45)).xyz * float3(other.group0.xx, other.e3215)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Flector reject_orthogonally_from(AntiFlatPoint other) {
        float4 anti_wedge_g0 = float4((float3(this.e1234) * other.group0.xyz), -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e41));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Flector reject_orthogonally_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) + float4((other.group0.xyz * this.group2.www), other.e1 * this.e4235) - (other.group0.wwwx * float4(this.group3.xyz, this.e41));
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Plane reject_orthogonally_from(AntiLine other) {
        float4 anti_wedge_g0 = float4((other.e12 * this.e4315) + (other.e15 * this.e1234), (other.e23 * this.e4125) + (other.e25 * this.e1234), (other.e31 * this.e4235) + (other.e35 * this.e1234), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group3.zxyx * float4(other.group0.yzx, other.e15));
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], 1.0) * float4(right_anti_dual_g0.yzx, -(right_anti_dual_g1[1] * anti_wedge_g0[1]) - (right_anti_dual_g1[2] * anti_wedge_g0[2]))) - (anti_wedge_g0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Flector reject_orthogonally_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4((other.e15 * this.e1234) + (other.e3215 * this.e41), (other.e25 * this.e1234) + (other.e3215 * this.e42), (other.e35 * this.e1234) + (other.e3215 * this.e43), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group3.yzx), other.e3215 * this.e45) - (this.group3.zxyx * float4(other.group0.yzx, other.e15));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * right_anti_dual_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g1.zxy * right_anti_dual_g0.yzx), right_anti_dual_g1[3] * other.e3215 * this.e1234) - (anti_wedge_g1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Plane reject_orthogonally_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_orthogonally_from(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(Circle other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((other.e423 * this.e3215) + (other.e235 * this.e1234), (other.e431 * this.e3215) + (other.e315 * this.e1234), (other.e412 * this.e3215) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e125) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * other.e431), (anti_wedge_g0[2] * other.e235) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * other.e412), (anti_wedge_g0[0] * other.e315) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * other.e423), -(anti_wedge_g1[2] * other.e125) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e235) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_orthogonally_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4((other.e415 * this.e1234) + (other.e12345 * this.e41), (other.e425 * this.e1234) + (other.e12345 * this.e42), (other.e435 * this.e1234) + (other.e12345 * this.e43), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group0.zxy * this.group3.yzx), other.e12345 * this.scalar) - float4((other.group0.yzx * this.group3.zxy), other.e423 * this.e15);
        float4 anti_wedge_g1 = float4((other.e235 * this.e1234) + (other.e12345 * this.e23), (other.e315 * this.e1234) + (other.e12345 * this.e31), (other.e125 * this.e1234) + (other.e12345 * this.e12), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((other.group0.xyz * this.group3.www), other.e12345 * this.e45) - (other.group1.wwwx * this.group3.xyzx);
        float3 anti_wedge_g2_xyz = (float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx);
        float anti_wedge_g2_w = other.e12345 * this.e1234;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(right_anti_dual_g2[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g2[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(anti_wedge_g0.www, anti_wedge_g2_w)) + float4((anti_wedge_g2_xyz * float3(right_anti_dual_g2[3])), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * right_anti_dual_g2[2]) + (anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g3[0] * right_anti_dual_g2[3]), (anti_wedge_g0[2] * right_anti_dual_g2[0]) + (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g3[1] * right_anti_dual_g2[3]), (anti_wedge_g0[0] * right_anti_dual_g2[1]) + (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g3[2] * right_anti_dual_g2[3]), -(anti_wedge_g2_xyz[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g2[0]) - (anti_wedge_g1[1] * right_anti_dual_g2[1]) - (anti_wedge_g1[2] * right_anti_dual_g2[2])) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), anti_wedge_g3[3] * right_anti_dual_g2[3]) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_w * right_anti_dual_g1[0]) - float4((anti_wedge_g0.zxy * right_anti_dual_g2.yzx), anti_wedge_g2_xyz[1] * right_anti_dual_g1[1])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_orthogonally_from(Dipole other) {
        float4 anti_wedge_g0 = float4(-(other.e41 * this.e3215) - (other.e31 * this.e4125), -(other.e42 * this.e3215) - (other.e12 * this.e4235), -(other.e43 * this.e3215) - (other.e23 * this.e4315), (other.e43 * this.e4125) + (other.e45 * this.e1234)) + (this.group3.yzxy * float4(other.group1.zxy, other.e42)) + float4((other.group2.xyz * this.group2.www), other.e41 * this.e4235);
        float anti_wedge_g1 = -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g2[0] * anti_wedge_g0[3]) + (anti_wedge_g0[2] * right_anti_dual_g1[1]), (right_anti_dual_g2[1] * anti_wedge_g0[3]) + (anti_wedge_g0[0] * right_anti_dual_g1[2]), (right_anti_dual_g2[2] * anti_wedge_g0[3]) + (anti_wedge_g0[1] * right_anti_dual_g1[0]), -(anti_wedge_g1 * right_anti_dual_g1[3]) - (right_anti_dual_g2[2] * anti_wedge_g0[2])) - (anti_wedge_g0.yzxy * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[1])) - float4((right_anti_dual_g0 * float3(anti_wedge_g1)), right_anti_dual_g2[0] * anti_wedge_g0[0]), 
            /* e1234 */
            (right_anti_dual_g0[0] * anti_wedge_g0[0]) + (right_anti_dual_g0[1] * anti_wedge_g0[1]) + (right_anti_dual_g0[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((other.group3.xyz * this.group3.www), other.e1234 * this.e45);
        float4 anti_wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4((other.group1.zxy * this.group3.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125);
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])) - (anti_wedge_g3.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g2[3])), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (right_anti_dual_g0[1] * anti_wedge_g3[1]) + (right_anti_dual_g0[2] * anti_wedge_g3[2]) + (anti_wedge_g2[3] * right_anti_dual_g1[3]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2]) - (anti_wedge_g1[3] * right_anti_dual_g2[3])) + float4((anti_wedge_g3.xyz * right_anti_dual_g3.www), right_anti_dual_g0[0] * anti_wedge_g3[0]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g3[3]) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (anti_wedge_g3.yzxw * right_anti_dual_g1.zxyw) - (right_anti_dual_g2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) - float4((right_anti_dual_g0 * anti_wedge_g3.www), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g1.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_orthogonally_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.group0.yy, other.e12345, (other.e5 * this.e1234) + (other.e12345 * this.scalar)) * float4(this.group0.xyz, 1.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_anti_dual_g0[1]), 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0[1]) * float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0.yy, right_anti_dual_g0[1], (right_anti_dual_g0[0] * anti_wedge_g0[3]) + (right_anti_dual_g0[1] * anti_wedge_g3[3])) * float4(anti_wedge_g3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_orthogonally_from(FlatPoint other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz), (right_anti_dual_g0[3] * other.e15 * this.e4235) + (right_anti_dual_g0[3] * other.e25 * this.e4315) + (right_anti_dual_g0[3] * other.e35 * this.e4125) + (right_anti_dual_g0[3] * other.e45 * this.e3215) - (anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])), 
            /* e1234 */
            anti_wedge_g0[3] * right_anti_dual_g0[3]
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(Flector other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz;
        float3 anti_wedge_g1_xyz = (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4(float3(0.0), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((this.group3.xyz * other.group1.www), other.e45 * this.e1234) - (other.group1.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (other.group1.zxyy * float4(this.group1.yzx, this.e25)) + (other.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e3215 * this.e45) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - float4((other.group1.yzx * this.group1.zxy), other.e15 * this.e4235);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * right_anti_dual_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g1.yzx)), anti_wedge_g2[3] * right_anti_dual_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2])) + float4((anti_wedge_g3.xyz * right_anti_dual_g1.www), anti_wedge_g2[3] * right_anti_dual_g0[3]) - (right_anti_dual_g1.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g1[3]) * float4(anti_wedge_g0, other.e3215 * this.e1234)) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[1])) + (right_anti_dual_g1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) + float4(float3(0.0), (anti_wedge_g2[2] * right_anti_dual_g1[2]) - (anti_wedge_g3[1] * right_anti_dual_g0[1]) - (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g3[3] * right_anti_dual_g0[3])) - float4((anti_wedge_g1_xyz.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(this.e1234) * other.group1.xyz;
        float anti_wedge_g1_w = -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125);
        float4 anti_wedge_g2 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - float4((other.group1.zxy * this.group3.yzx), other.e415 * this.e23);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * anti_wedge_g2.www), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_w * other.e415) + (anti_wedge_g0[1] * other.e125), (anti_wedge_g1_w * other.e425) + (anti_wedge_g0[2] * other.e235), (anti_wedge_g1_w * other.e435) + (anti_wedge_g0[0] * other.e315), -(anti_wedge_g1_xyz[0] * other.e235) - (anti_wedge_g1_xyz[1] * other.e315) - (anti_wedge_g1_xyz[2] * other.e125) - (anti_wedge_g2[1] * other.e425) - (anti_wedge_g2[2] * other.e435)) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_orthogonally_from(Motor other) {
        float4 anti_wedge_g0 = (other.group0 * float4(this.group2.www, this.scalar)) + float4(float3(0.0), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((this.group0.xyz * other.group0.www), other.e5 * this.e1234);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + float4((float3(this.e1234) * other.group1.xyz), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125));
        float4 anti_wedge_g2 = float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx)), other.e12345 * this.e1234);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], 1.0) * float4(anti_wedge_g0.www, -(anti_wedge_g0[0] * right_anti_dual_g0[0]) - (anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]))), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0 * float4(anti_wedge_g1.www, anti_wedge_g3[3])) + (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], 1.0) * float4(anti_wedge_g0.yzx, -(anti_wedge_g1[0] * right_anti_dual_g1[0]) - (anti_wedge_g1[1] * right_anti_dual_g1[1]) - (anti_wedge_g1[2] * right_anti_dual_g1[2]) - (anti_wedge_g2[1] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[2]))) + float4((anti_wedge_g3.xyz * right_anti_dual_g0.www), anti_wedge_g0[3] * right_anti_dual_g1[3]) - float4((anti_wedge_g0.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[0] * right_anti_dual_g0[0])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public MultiVector reject_orthogonally_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float4 anti_wedge_g1 = (this.group3.yzxx * float4(other.group5.zxy, other.e41)) + float4((other.group9.zxy * this.group1.yzx), other.e43 * this.e4125) + float4((other.group3.xyz * this.group2.www), other.e42 * this.e4315) + float4((this.group0.xyz * other.group9.www), other.e45 * this.e1234) - (float4(other.e1234) * float4(this.group2.xyz, this.e45)) - (other.group9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group4.xyz * this.group3.www), other.e4235 * this.e41) - float4((other.group5.yzx * this.group3.zxy), other.e4315 * this.e42);
        float anti_wedge_g2 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = float4((other.e415 * this.e3215) + (other.e315 * this.e4125), (other.e425 * this.e3215) + (other.e125 * this.e4235), (other.e435 * this.e3215) + (other.e235 * this.e4315), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (float4(other.e12345) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(other.group8.zxy, other.e415));
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group3.yzx) - (other.group7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 anti_wedge_g6 = (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((other.group9.zxy * this.group3.yzx), other.e1234 * this.e3215);
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group3;
        float anti_wedge_g10 = other.e12345 * this.e1234;
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_anti_dual_g0[0], (anti_wedge_g0_x * right_anti_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_anti_dual_g10) + (anti_wedge_g1[0] * right_anti_dual_g9[0]) + (anti_wedge_g1[1] * right_anti_dual_g9[1]) + (anti_wedge_g1[2] * right_anti_dual_g9[2]) + (anti_wedge_g1[3] * right_anti_dual_g9[3]) + (anti_wedge_g9[0] * right_anti_dual_g1[0]) + (anti_wedge_g9[1] * right_anti_dual_g1[1]) + (anti_wedge_g9[2] * right_anti_dual_g1[2]) + (anti_wedge_g9[3] * right_anti_dual_g1[3]) - (anti_wedge_g4[0] * right_anti_dual_g8[0]) - (anti_wedge_g4[1] * right_anti_dual_g8[1]) - (anti_wedge_g4[2] * right_anti_dual_g8[2]) - (anti_wedge_g5[0] * right_anti_dual_g6[0]) - (anti_wedge_g5[1] * right_anti_dual_g6[1]) - (anti_wedge_g5[2] * right_anti_dual_g6[2]) - (anti_wedge_g7[0] * right_anti_dual_g3[0]) - (anti_wedge_g7[1] * right_anti_dual_g3[1]) - (anti_wedge_g7[2] * right_anti_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_anti_dual_g7[0] * anti_wedge_g3[0]) - (right_anti_dual_g7[1] * anti_wedge_g3[1]) - (right_anti_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_anti_dual_g6[3]) - (anti_wedge_g6[0] * other.e415) - (anti_wedge_g6[1] * other.e425) - (anti_wedge_g6[2] * other.e435) - (anti_wedge_g6[3] * right_anti_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_anti_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g3 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_anti_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g1.zxy) - (anti_wedge_g1.zxy * right_anti_dual_g1.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g4[0] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[0]) + (anti_wedge_g3[0] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e431) + (anti_wedge_g4[1] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[1]) + (anti_wedge_g3[1] * right_anti_dual_g1[3]), (anti_wedge_g2 * other.e412) + (anti_wedge_g4[2] * other.e3215) + (anti_wedge_g1[3] * right_anti_dual_g3[2]) + (anti_wedge_g3[2] * right_anti_dual_g1[3]), -(anti_wedge_g5[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435)) + (anti_wedge_g6 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g6 * float4(anti_wedge_g0_x)) - (right_anti_dual_g1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * right_anti_dual_g3.www), anti_wedge_g5[0] * right_anti_dual_g1[0]), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_anti_dual_g1[3])) + (anti_wedge_g7 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g7 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g1[3]) * other.group6.xyz) + (anti_wedge_g4.yzx * right_anti_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_anti_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_anti_dual_g0[0])) + (right_anti_dual_g8 * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group6.xyz) + (anti_wedge_g1.yzx * right_anti_dual_g3.zxy) + (anti_wedge_g3.zxy * right_anti_dual_g1.yzx) - (anti_wedge_g1.zxy * right_anti_dual_g3.yzx) - (anti_wedge_g3.yzx * right_anti_dual_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g8[0] * anti_wedge_g1[3]) + (anti_wedge_g1[2] * right_anti_dual_g6[1]) + (anti_wedge_g3[3] * other.e415) + (anti_wedge_g6[2] * right_anti_dual_g1[1]), (right_anti_dual_g8[1] * anti_wedge_g1[3]) + (anti_wedge_g1[0] * right_anti_dual_g6[2]) + (anti_wedge_g3[3] * other.e425) + (anti_wedge_g6[0] * right_anti_dual_g1[2]), (right_anti_dual_g8[2] * anti_wedge_g1[3]) + (anti_wedge_g1[1] * right_anti_dual_g6[0]) + (anti_wedge_g3[3] * other.e435) + (anti_wedge_g6[1] * right_anti_dual_g1[0]), -(anti_wedge_g2 * right_anti_dual_g6[3]) - (right_anti_dual_g8[0] * anti_wedge_g1[0]) - (right_anti_dual_g8[1] * anti_wedge_g1[1]) - (right_anti_dual_g8[2] * anti_wedge_g1[2])) + (anti_wedge_g9 * float4(right_anti_dual_g0[0])) + (right_anti_dual_g9 * float4(anti_wedge_g0_x)) + float4((anti_wedge_g5 * right_anti_dual_g3.www), anti_wedge_g8[1] * right_anti_dual_g1[1]) + float4((anti_wedge_g7 * float3(other.e3215)), anti_wedge_g8[2] * right_anti_dual_g1[2]) + float4((anti_wedge_g4.yzx * right_anti_dual_g3.zxy), anti_wedge_g8[0] * right_anti_dual_g1[0]) + float4((other.group7.yzx * anti_wedge_g3.zxy), anti_wedge_g6[3] * other.e3215) - (right_anti_dual_g3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g8 * right_anti_dual_g1.www), anti_wedge_g5[1] * right_anti_dual_g3[1]) - float4((right_anti_dual_g7 * float3(anti_wedge_g2)), anti_wedge_g3[0] * other.e415) - float4((other.group7.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * right_anti_dual_g3[2]) - float4((anti_wedge_g1.yzx * right_anti_dual_g6.zxy), anti_wedge_g3[1] * other.e425) - float4((anti_wedge_g6.yzx * right_anti_dual_g1.zxy), anti_wedge_g3[2] * other.e435), 
            /* e1234 */
            (anti_wedge_g0_x * right_anti_dual_g10) + (anti_wedge_g10 * right_anti_dual_g0[0]) + (right_anti_dual_g7[0] * anti_wedge_g1[0]) + (right_anti_dual_g7[1] * anti_wedge_g1[1]) + (right_anti_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * right_anti_dual_g6[3]) - (anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_anti_dual_g1[0]) - (anti_wedge_g7[1] * right_anti_dual_g1[1]) - (anti_wedge_g7[2] * right_anti_dual_g1[2]) - (anti_wedge_g6[3] * right_anti_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(Plane other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        float3 anti_wedge_g1_xyz = (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) - (other.group0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (float4(other.e4315 * this.e12, other.e4125 * this.e23, other.e4235 * this.e31, (other.e4125 * this.e35) + (other.e3215 * this.e45)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25));
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.yzx * right_anti_dual_g0.zxy) - (anti_wedge_g3.zxy * right_anti_dual_g0.yzx)), anti_wedge_g2[3] * right_anti_dual_g0[3]), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * anti_wedge_g3.xyz), -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2])) - (right_anti_dual_g0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], 1.0) * float4(anti_wedge_g1_xyz.yzx, (anti_wedge_g2[1] * right_anti_dual_g0[1]) + (anti_wedge_g2[2] * right_anti_dual_g0[2])) * float4(-1.0, -1.0, -1.0, 1.0)) + (float4(right_anti_dual_g0[3]) * float4(anti_wedge_g0, other.e3215 * this.e1234)) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_orthogonally_from(RoundPoint other) {
        float anti_wedge_g0 = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((other.group0.zxy * this.group3.yzx), other.e1234 * this.e3215);
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), -(other.e4315 * this.e42) - (other.e4125 * this.e43) - (other.e1234 * this.e45)) - (other.group0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = float4(-(other.e4315 * this.e12) - (other.e1234 * this.e15), -(other.e4125 * this.e23) - (other.e1234 * this.e25), -(other.e4235 * this.e31) - (other.e1234 * this.e35), (other.e4125 * this.e35) + (other.e3215 * this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25));
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.yzx * right_anti_dual_g0.zxy), anti_wedge_g2[3] * other.e3215) - (anti_wedge_g3.zxyw * right_anti_dual_g0.yzxw), 
            /* e15, e25, e35, e1234 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(anti_wedge_g3.xyz, -(anti_wedge_g0[1] * right_anti_dual_g0[1]) - (anti_wedge_g0[2] * right_anti_dual_g0[2]) - (anti_wedge_g1[3] * right_anti_dual_g0[3]))) - (right_anti_dual_g0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g1[1] * right_anti_dual_g0[2]) - (anti_wedge_g2[0] * right_anti_dual_g0[3]), -(anti_wedge_g1[2] * right_anti_dual_g0[0]) - (anti_wedge_g2[1] * right_anti_dual_g0[3]), -(anti_wedge_g1[0] * right_anti_dual_g0[1]) - (anti_wedge_g2[2] * right_anti_dual_g0[3]), (anti_wedge_g2[1] * right_anti_dual_g0[1]) + (anti_wedge_g2[2] * right_anti_dual_g0[2])) + (float4(other.e3215) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (right_anti_dual_g0.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_orthogonally_from(VersorEven other) {
        float4 anti_wedge_g0 = (other.group0.zxyw * float4(this.group3.yzx, this.scalar)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((other.group1.xyz * this.group2.www), other.e1 * this.e4235) + float4((this.group0.xyz * other.group0.www), other.e5 * this.e1234) - (other.group0.yzxx * float4(this.group3.zxy, this.e15));
        float4 anti_wedge_g1 = float4((other.e12345 * this.e23) + (other.e235 * this.e1234), (other.e12345 * this.e31) + (other.e315 * this.e1234), (other.e12345 * this.e12) + (other.e125 * this.e1234), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + (other.group0 * float4(this.group3.www, this.e45)) - (other.group1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx)), other.e12345 * this.e1234);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * right_anti_dual_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_anti_dual_g0[3])) + (right_anti_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * right_anti_dual_g1[0]) - (anti_wedge_g0[1] * right_anti_dual_g1[1]) - (anti_wedge_g0[2] * right_anti_dual_g1[2]) - (anti_wedge_g1[0] * right_anti_dual_g0[0]) - (anti_wedge_g1[1] * right_anti_dual_g0[1]) - (anti_wedge_g1[2] * right_anti_dual_g0[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[0]) + (anti_wedge_g2[2] * right_anti_dual_g0[1]) + (anti_wedge_g3[0] * right_anti_dual_g0[3]), (anti_wedge_g1[1] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[1]) + (anti_wedge_g2[0] * right_anti_dual_g0[2]) + (anti_wedge_g3[1] * right_anti_dual_g0[3]), (anti_wedge_g1[2] * right_anti_dual_g1[3]) + (anti_wedge_g1[3] * right_anti_dual_g1[2]) + (anti_wedge_g2[1] * right_anti_dual_g0[0]) + (anti_wedge_g3[2] * right_anti_dual_g0[3]), -(anti_wedge_g1[2] * right_anti_dual_g2[2]) - (anti_wedge_g2[0] * right_anti_dual_g1[0]) - (anti_wedge_g2[1] * right_anti_dual_g1[1]) - (anti_wedge_g2[2] * right_anti_dual_g1[2])) + (anti_wedge_g0.yzxw * float4(right_anti_dual_g2.zxy, right_anti_dual_g3[3])) + float4((right_anti_dual_g3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * right_anti_dual_g0[3]) - (right_anti_dual_g2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * right_anti_dual_g0.zxy), anti_wedge_g1[1] * right_anti_dual_g2[1])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectOrthogonallyFromInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.reject_orthogonally_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_orthogonally_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - float4((other.group3.xyz * this.group3.www), other.e1234 * this.e45);
        float4 anti_wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4((other.group1.zxy * this.group3.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), other.e35 * this.e4125);
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(anti_wedge_g2[3]) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.yzx * right_anti_dual_g3.zxy), anti_wedge_g2[3] * right_anti_dual_g2[3]) - (anti_wedge_g3.zxyw * right_anti_dual_g3.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (anti_wedge_g3[0] * right_anti_dual_g0[0]) + (anti_wedge_g3[1] * right_anti_dual_g0[1]) + (anti_wedge_g3[2] * right_anti_dual_g0[2]) - (anti_wedge_g0[1] * right_anti_dual_g3[1]) - (anti_wedge_g0[2] * right_anti_dual_g3[2]) - (anti_wedge_g1[3] * right_anti_dual_g3[3])) + float4((anti_wedge_g3.xyz * right_anti_dual_g2.www), anti_wedge_g2[3] * right_anti_dual_g1[3]) - (right_anti_dual_g3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g2[3]) * float4(anti_wedge_g0, anti_wedge_g1[3])) + (anti_wedge_g2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + (right_anti_dual_g3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) + float4((anti_wedge_g3.zxy * right_anti_dual_g1.yzx), anti_wedge_g2[2] * right_anti_dual_g3[2]) - (float4(anti_wedge_g3[3]) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (anti_wedge_g3.yzxz * float4(right_anti_dual_g1.zxy, right_anti_dual_g2[2])) - float4((anti_wedge_g1.yzx * right_anti_dual_g3.zxy), anti_wedge_g3[0] * right_anti_dual_g2[0]) - float4((anti_wedge_g2.xyz * right_anti_dual_g3.www), anti_wedge_g3[1] * right_anti_dual_g2[1])
        );
    }
}
