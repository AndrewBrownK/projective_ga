implementing cga3d;
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321);
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
public FlatPoint operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint reject_via_origin_from(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, other.e3215 * other.e3215, 0.0) * float4((this.group0.xyz * float3(float2(other.e3215 * other.e3215), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint reject_via_origin_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((-(float3(this.e45 * other.e321) * other.group0.xyz) - (float3(other.e235 * other.e235, other.e315 * other.e315, other.e125 * other.e125) * this.group0.xyz) - (float3(other.e235) * this.group0.yxx * other.group0.yyz) - (float3(other.e125) * this.group0.zzy * other.group0.xyy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public Flector reject_via_origin_from(AntiFlector other) {
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
public Flector operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public Flector reject_via_origin_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.e3215) * this.group0.xyz;
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g0_xyz * float4(other.e3215).xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz.yzx * other.group0.zxy) - (anti_wedge_g0_xyz.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual_g0 * other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public Dipole reject_via_origin_from(Circle other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321);
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
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_dual_g2_w)) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g2_w)) + (right_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g2_w)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w))), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group3.zx) - (this.group1.zx * other.group3.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(anti_wedge_g0[3]) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.zxyw * other.group3.yzxw) + float4(-(anti_wedge_g0.yz * other.group3.zx), anti_wedge_g0[0] * other.e4315 * -1.0, anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g0.xyz)), right_dual_g0[0] * anti_wedge_g0[0]), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23), 0.0) + float4((float3(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0) - (right_dual_g0 * float3(anti_wedge_g1)) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_via_origin_from(DualNum other) {
        float4 anti_wedge_g2 = float4(other.e12345) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345 * -1.0) * float4((this.group0.xyz * float3(float2(other.e12345), other.e12345)), anti_wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(1.0, 1.0, other.e12345, 0.0) * float4((anti_wedge_g2.xyz * float3(float2(other.e12345 * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e5 * anti_wedge_g2[3] * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group1.zx) - (this.group1.zx * other.group1.yz)), 0.0)), 0.0);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zx * other.group1.yz) - (anti_wedge_g0.yz * other.group1.zx)), (anti_wedge_g0[1] * other.e4235) - (anti_wedge_g0[0] * other.e4315), anti_wedge_g0[3] * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + (float3(other.e3215) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public AntiLine reject_via_origin_from(Line other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_via_origin_from(Motor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((anti_wedge_g0 * float3(right_dual_g0_w)), anti_wedge_g2_w * right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w * other.e12345) * this.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1[3])) + (anti_wedge_g0.yzx * right_dual_g1_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g1_xyz.yzx)), anti_wedge_g2_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e12345 * this.scalar) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group9.zx) - (this.group1.zx * other.group9.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g2 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 anti_wedge_g3 = float4(other.e12345) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(other.e12345) * this.group1.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (right_dual_g9_w * anti_wedge_g1[3]) + (right_dual_g9_xyz[0] * anti_wedge_g1[0]) + (right_dual_g9_xyz[1] * anti_wedge_g1[1]) + (right_dual_g9_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g5[0] * right_dual_g6_xyz[0]) - (anti_wedge_g5[1] * right_dual_g6_xyz[1]) - (anti_wedge_g5[2] * right_dual_g6_xyz[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g4 * float3(other.e3215)) + (right_dual_g6_xyz * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (right_dual_g9_xyz * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (right_dual_g6_xyz.yzx * anti_wedge_g1.zxy) + (other.group7.yzx * anti_wedge_g3.zxy) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (right_dual_g6_xyz.zxy * anti_wedge_g1.yzx) - (other.group7.zxy * anti_wedge_g3.yzx)), anti_wedge_g0_x * right_dual_g9_w), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public Dipole reject_via_origin_from(Plane other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0)), 0.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zx * other.group0.yz) - (anti_wedge_g0.yz * other.group0.zx)), (anti_wedge_g0[1] * other.e4235) - (anti_wedge_g0[0] * other.e4315), anti_wedge_g0[3] * other.e3215), 
            /* e15, e25, e35 */
            float3(other.e4235 * other.e4235 * this.e15, other.e4315 * other.e4315 * this.e25, other.e4125 * other.e4125 * this.e35) + (float3(other.e4235) * float3(this.e25 * other.e4315, this.e15 * other.e4315, this.e15 * other.e4125)) + (float3(other.e4125) * float3(this.e35 * other.e4235, this.e35 * other.e4315, this.e25 * other.e4315)) + (float3(other.e3215) * anti_wedge_g0.xyz) + (float3(this.e45 * other.e3215) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public Dipole reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(anti_wedge_g0[3])) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_dual_g0_xyz.zxy, other.e3215)) + float4(-(right_dual_g0_xyz.yzx * anti_wedge_g0.zxy), anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * anti_wedge_g0.xyz) - (right_dual_g0_xyz * float3(anti_wedge_g1))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_dual_g0_w)) + (right_dual_g0_xyz * float3(anti_wedge_g2_w))), anti_wedge_g2_w * right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0_w)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w))), anti_wedge_g2_w * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g2_w) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (anti_wedge_g2_xyz.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g0_xyz.zxy)), anti_wedge_g2_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group3.zx) - (this.group1.zx * other.group3.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g1 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g0[3])) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_dual_g3_xyz.zxy, other.e3215)) + float4(-(right_dual_g3_xyz.yzx * anti_wedge_g0.zxy), anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g0.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g1))), anti_wedge_g0[0] * other.e41 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23), 0.0) + float4(((float3(anti_wedge_g1) * other.group0.xyz) + float3(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) + (this.group0.zxy * other.group2.yzx) - (other.group0.yzx * this.group2.zxy) - (this.group0.yzx * other.group2.zxy);
        float anti_wedge_g1 = -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e321)) + (float3(anti_wedge_g1) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g0_xyz[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Motor reject_via_origin_from(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e4);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * anti_wedge_g0[3]), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Line reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            anti_wedge_g0_xyz * float3(other.e321), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g0_xyz.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Motor reject_via_origin_from(AntiFlector other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float4(other.e321).xyz), (anti_wedge_g0_xyz[0] * right_dual_g1_xyz[0]) + (anti_wedge_g0_xyz[1] * right_dual_g1_xyz[1]) + (anti_wedge_g0_xyz[2] * right_dual_g1_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Line reject_via_origin_from(AntiLine other) {
        float anti_wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Motor reject_via_origin_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.e3215) * this.group0.xyz;
        float anti_wedge_g0_w = (this.e4 * other.e3215) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group0.xyz), (anti_wedge_g0_w * other.scalar) + (anti_wedge_g0_xyz[0] * other.e23) + (anti_wedge_g0_xyz[1] * other.e31) + (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0 * other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Circle reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0_xyz = (float3(this.e321) * other.group1.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group2.yzx) + (other.group0.zxy * this.group2.yzx) - (this.group0.yzx * other.group2.zxy) - (other.group0.yzx * this.group2.zxy);
        float anti_wedge_g1 = -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e321)) + (float3(anti_wedge_g1) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0_xyz.yzx * other.group2.zxy) - (anti_wedge_g0_xyz.zxy * other.group2.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float3 anti_wedge_g3_xyz = (float3(this.e321) * other.group1.xyz) + (float3(other.e321) * this.group1.xyz) + (float3(other.e12345) * this.group3.xyz) + (this.group0.zxy * other.group2.yzx) + (other.group0.zxy * this.group2.yzx) - (this.group0.yzx * other.group2.zxy) - (other.group0.yzx * this.group2.zxy);
        float anti_wedge_g3_w = this.e5 * other.e12345;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g2_w * other.e12345) * this.group0.xyz) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy)), right_dual_g1_w * anti_wedge_g1[3] * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(anti_wedge_g3_w) * other.group0.xyz) + (float3(right_dual_g2_w) * anti_wedge_g1.xyz) - (anti_wedge_g3_xyz * float3(right_dual_g1_w))), right_dual_g2_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3_w)) + (float3(right_dual_g2_w * other.e12345) * this.group2.xyz) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g3_w * right_dual_g2_w), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g2_w)), anti_wedge_g2_w * right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Circle reject_via_origin_from(Dipole other) {
        float anti_wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float4 anti_wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(-(this.group2.zx * other.group3.yz), this.e315 * other.e4235 * -1.0, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.zxy * other.group3.yzx) - (float3(anti_wedge_g2[3]) * other.group0.xyz) - (anti_wedge_g0.yzx * other.group3.zxy)), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(other.e1234) * anti_wedge_g2.xyz) - (float3(anti_wedge_g2[3]) * other.group1.xyz)), anti_wedge_g2[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[1] * other.e4125) - (anti_wedge_g2[2] * other.e4315), (anti_wedge_g2[2] * other.e4235) - (anti_wedge_g2[0] * other.e4125), (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235)) + (anti_wedge_g1_xyz * float3(other.e3215)) - (float3(anti_wedge_g2[3]) * other.group2.xyz)), anti_wedge_g2[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_via_origin_from(DualNum other) {
        float4 anti_wedge_g2 = float4(other.e12345) * this.group2;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz * float3(float2(other.e12345 * other.e12345 * -1.0), other.e12345 * other.e12345) * float3(1.0, 1.0, -1.0)), other.e5 * anti_wedge_g2[3] * -1.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * -1.0) * float4(anti_wedge_g2.xyz, anti_wedge_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * -1.0) * float4(anti_wedge_g3.xyz, anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlatPoint reject_via_origin_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((float3(this.e321 * other.e45) * other.group0.xyz) + (float3(other.e15 * other.e15, other.e25 * other.e25, other.e35 * other.e35) * this.group0.xyz) + (float3(other.e15) * this.group0.yxx * other.group0.yyz) + (float3(other.e35) * this.group0.zzy * other.group0.xyy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group1.xyz);
        float4 anti_wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(-(this.group2.zx * other.group1.yz), this.e315 * other.e4235 * -1.0, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float4(other.e3215).xyz), (anti_wedge_g1_xyz[0] * other.e4235) + (anti_wedge_g1_xyz[1] * other.e4315) + (anti_wedge_g1_xyz[2] * other.e4125) + (anti_wedge_g2[3] * other.e45)), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[1] * other.e4125) - (anti_wedge_g2[2] * other.e4315), (anti_wedge_g2[2] * other.e4235) - (anti_wedge_g2[0] * other.e4125), (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235)) + (anti_wedge_g1_xyz * float3(other.e3215)) - (float3(anti_wedge_g2[3]) * other.group0.xyz)), anti_wedge_g2[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            float4((float4(anti_wedge_g2[3]).xyz * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public Circle reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = (float3(this.e321) * other.group0.xyz) + (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        return Circle(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(other.e415) * float3(other.e425 * this.e315, other.e425 * this.e235, other.e435 * this.e235)) - (float3(other.e435) * float3(other.e415 * this.e125, other.e425 * this.e125, other.e425 * this.e315)) - (float3(other.e235 * this.e415) * other.group0.xyz) - (float3(other.e315 * this.e425) * other.group0.xyz) - (float3(other.e125 * this.e435) * other.group0.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group2.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_via_origin_from(Motor other) {
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float3 anti_wedge_g3_xyz = (float3(this.e321) * other.group0.xyz) + (float3(other.e12345) * this.group3.xyz) + (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float anti_wedge_g3_w = this.e5 * other.e12345;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w * other.e12345) * this.group0.xyz)), anti_wedge_g2_w * other.e5 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g3_w)) + (float3(right_dual_g0_w * other.e12345) * this.group2.xyz) + (anti_wedge_g3_xyz.yzx * right_dual_g1_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g1_xyz.yzx)), anti_wedge_g3_w * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g0_w)), anti_wedge_g2_w * right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4(((float3(other.e12345) * this.group3.xyz) + (float3(this.e321) * other.group6.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group8.yzx) + (other.group7.zxy * this.group2.yzx) - (this.group0.yzx * other.group8.zxy) - (other.group7.yzx * this.group2.zxy)), other.e12345 * this.e4);
        float anti_wedge_g2 = (other.e12345 * this.e5) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group1.xyz) + float3(((this.group2.yz * other.group9.zx) - (this.group2.zx * other.group9.yz)), 0.0)), 0.0);
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g7 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(other.e12345) * this.group2.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion reject_via_origin_from(Plane other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g2 = float4(0.0, 0.0, this.e315 * other.e4235 * -1.0, 0.0) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1)) + float4(-(this.group2.zx * other.group0.yz), 0.0, 0.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * right_dual_g0.zxy) - (anti_wedge_g0.zxy * right_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float4(right_dual_g0[3]).xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0, 0.0, (anti_wedge_g2[1] * right_dual_g0[0]) - (anti_wedge_g2[0] * right_dual_g0[1]), 0.0) + float4(((float3(right_dual_g0[3] * other.e3215) * this.group0.xyz) + float3(((anti_wedge_g2.zx * right_dual_g0.yz) - (anti_wedge_g2.yz * right_dual_g0.zx)), 0.0) - (float3(right_dual_g0[3] * this.e321) * other.group0.xyz)), 0.0), 
            /* e1, e2, e3, e5 */
            right_dual_g0 * float4(anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group0.xyz);
        float4 anti_wedge_g2 = float4(0.0, 0.0, this.e315 * other.e4235 * -1.0, 0.0) + (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1)) + float4(-(this.group2.zx * other.group0.yz), 0.0, 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g0_xyz.yzx), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(other.e1234) * anti_wedge_g2.xyz)), 0.0), 
            /* e235, e315, e125, e4 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (right_dual_g0_xyz.yzx * anti_wedge_g2.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g2.yzx)), anti_wedge_g2[3] * other.e1234), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g2[3]) * float4(right_dual_g0_xyz, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float3 anti_wedge_g3_xyz = float3((this.e315 * other.e412) - (this.e125 * other.e431), (this.e125 * other.e423) - (this.e235 * other.e412), (this.e235 * other.e431) - (this.e315 * other.e423)) + (float3(this.e321) * other.group1.xyz) + (float3(other.e12345) * this.group3.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group2.yzx) - (this.group0.yzx * other.group2.zxy);
        float anti_wedge_g3_w = this.e5 * other.e12345;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float3 right_dual_g3_xyz = other.group3.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2_w) * float4(right_dual_g1_xyz, other.e5 * -1.0)) + float4(((anti_wedge_g0 * float3(right_dual_g0_w)) + (anti_wedge_g3_xyz.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g3_xyz.yzx * right_dual_g0_xyz.zxy)), (anti_wedge_g3_xyz[0] * right_dual_g3_xyz[0]) + (anti_wedge_g3_xyz[1] * right_dual_g3_xyz[1]) - (right_dual_g1_w * anti_wedge_g1[3]) - (anti_wedge_g0[0] * right_dual_g2_xyz[0]) - (anti_wedge_g0[1] * right_dual_g2_xyz[1]) - (anti_wedge_g0[2] * right_dual_g2_xyz[2]) - (anti_wedge_g2_xyz[0] * right_dual_g0_xyz[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0_xyz[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0_xyz[2]) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2])), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g3_w)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz) - (anti_wedge_g3_xyz * float3(right_dual_g1_w))), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0_w)) + (right_dual_g1_xyz * float3(anti_wedge_g3_w)) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g3_w * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g0_w)), anti_wedge_g2_w * right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float4 anti_wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(-(this.group2.zx * other.group3.yz), this.e315 * other.e4235 * -1.0, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43));
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g3[3])) + (anti_wedge_g0.yzx * right_dual_g3.zxy) - (float3(anti_wedge_g2[3]) * other.group0.xyz) - (anti_wedge_g0.zxy * right_dual_g3.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(right_dual_g3[3]) * anti_wedge_g2.xyz) - (float3(anti_wedge_g2[3]) * other.group1.xyz)), anti_wedge_g2[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[2] * right_dual_g3[1]) - (anti_wedge_g2[1] * right_dual_g3[2]), (anti_wedge_g2[0] * right_dual_g3[2]) - (anti_wedge_g2[2] * right_dual_g3[0]), (anti_wedge_g2[1] * right_dual_g3[0]) - (anti_wedge_g2[0] * right_dual_g3[1])) + (anti_wedge_g1_xyz * float3(other.e3215)) - (float3(anti_wedge_g2[3]) * other.group2.xyz)), anti_wedge_g2[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_via_origin_from(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g0[1] * other.e12) - (anti_wedge_g0[2] * other.e31), (anti_wedge_g0[2] * other.e23) - (anti_wedge_g0[0] * other.e12), (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23)) - (right_dual_g0 * float3(anti_wedge_g0[3]))), anti_wedge_g0[3] * other.e45 * -1.0), 
            /* e1234 */
            (right_dual_g0[0] * anti_wedge_g0[0]) + (right_dual_g0[1] * anti_wedge_g0[1]) + (right_dual_g0[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e4);
        float4 anti_wedge_g1 = float4(1.0, 1.0, this.e3215, 0.0) * float4((other.group1.xyz * float3(float2(this.e3215), 1.0)), 0.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g0[3]), 
            /* e15, e25, e35, e1234 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group2.xyz), -(other.e423 * anti_wedge_g0[0]) - (other.e431 * anti_wedge_g0[1]) - (other.e412 * anti_wedge_g0[2]) - (anti_wedge_g0[3] * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g0[3]) * other.group3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g0.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), anti_wedge_g0[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_via_origin_from(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e3215) * other.group0.xyz;
        float3 anti_wedge_g1 = float3(this.e3215) * other.group1.xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g0 * float3(other.e321)) - (anti_wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215) * other.group0.xyz;
        float anti_wedge_g0_w = this.scalar * other.e12345;
        float4 anti_wedge_g1 = float4(this.e3215) * float4(other.group1.xyz, other.e12345);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * right_dual_g1_w), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g0_w) * other.group2.xyz) + (float3(right_dual_g2_w) * anti_wedge_g1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g1_w)) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), right_dual_g2_w * anti_wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_via_origin_from(Dipole other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g0[1] * other.e12) - (anti_wedge_g0[2] * other.e31), (anti_wedge_g0[2] * other.e23) - (anti_wedge_g0[0] * other.e12), (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23)) - (right_dual_g0 * float3(anti_wedge_g0[3]))), anti_wedge_g0[3] * other.e45 * -1.0), 
            /* e1234 */
            (right_dual_g0[0] * anti_wedge_g0[0]) + (right_dual_g0[1] * anti_wedge_g0[1]) + (right_dual_g0[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float4 anti_wedge_g1 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zx * other.group3.yz) - (anti_wedge_g1.yz * other.group3.zx)), (anti_wedge_g1[1] * other.e4235) - (anti_wedge_g1[0] * other.e4315), anti_wedge_g1[3] * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1[3]) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e15)) + float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + (float3(this.e3215 * other.e1234) * other.group3.xyz)), (anti_wedge_g1[1] * other.e25) + (other.e4235 * other.e4235 * this.e3215)) - (anti_wedge_g1.zxyw * other.group1.yzxw)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_via_origin_from(DualNum other) {
        float2 anti_wedge_g0 = float2(other.e12345) * this.group0.xy;
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(anti_wedge_g0[0] * other.e12345) - (anti_wedge_g0[1] * other.e5), anti_wedge_g0[1] * other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_via_origin_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e45 * other.e45 * this.e3215, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public Flector reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(1.0, 1.0, this.e3215, 0.0) * float4((other.group1.xyz * float3(float2(this.e3215 * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0);
        float anti_wedge_g1_w = this.e3215 * other.e45 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g1_w) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g0[3] * other.e3215) - (anti_wedge_g1_w * other.e45) - (anti_wedge_g0[0] * other.e4235) - (anti_wedge_g0[1] * other.e4315) - (anti_wedge_g0[2] * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_via_origin_from(Line other) {
        float4 anti_wedge_g0 = float4(1.0, 1.0, this.e3215, 0.0) * float4((other.group0.xyz * float3(float2(this.e3215), 1.0)), 0.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e415 * anti_wedge_g0[0]) - (other.e425 * anti_wedge_g0[1]) - (other.e435 * anti_wedge_g0[2]), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public AntiMotor reject_via_origin_from(Motor other) {
        float anti_wedge_g0_w = this.scalar * other.e12345;
        float4 anti_wedge_g1 = float4(this.e3215) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(anti_wedge_g0_w) * float4(right_dual_g0_xyz, right_dual_g0_w), 
            /* e15, e25, e35, e3215 */
            float4(((float3(anti_wedge_g0_w) * other.group1.xyz) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz)), (right_dual_g0_w * anti_wedge_g1[3]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e3215 * other.e4) + (this.scalar * other.e12345);
        float4 anti_wedge_g1 = float4(1.0, 1.0, this.e3215, 0.0) * float4((other.group4.xyz * float3(float2(this.e3215 * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0);
        float anti_wedge_g2 = this.e3215 * other.e45 * -1.0;
        float4 anti_wedge_g3 = float4(1.0, 1.0, this.e3215, 0.0) * float4((other.group6.xyz * float3(float2(this.e3215), 1.0)), 0.0);
        float3 anti_wedge_g5 = float3(this.e3215) * other.group7.xyz;
        float anti_wedge_g6_w = this.e3215 * other.e1234 * -1.0;
        float3 anti_wedge_g8 = float3(this.e3215 * -1.0) * other.group9.xyz;
        float anti_wedge_g9_w = this.e3215 * other.e12345;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g9_w * right_dual_g1[3]) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) - (anti_wedge_g6_w * right_dual_g3[3]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), (anti_wedge_g0_x * other.e45) + (anti_wedge_g6_w * right_dual_g0[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g6_w * other.e3215) + (anti_wedge_g9_w * right_dual_g0[0]) + (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2])), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6_w * right_dual_g1[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public AntiDualNum reject_via_origin_from(Plane other) {
        float4 anti_wedge_g0 = float4(1.0, 1.0, this.e3215, 0.0) * float4((other.group0.xyz * float3(float2(this.e3215 * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2((anti_wedge_g0[3] * other.e3215) - (anti_wedge_g0[0] * other.e4235) - (anti_wedge_g0[1] * other.e4315) - (anti_wedge_g0[2] * other.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_via_origin_from(RoundPoint other) {
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
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public Sphere reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e1234);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e1234 * -1.0) * anti_wedge_g0.xyz), (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * other.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215) * other.group0.xyz;
        float anti_wedge_g0_w = (this.e3215 * other.e4) + (this.scalar * other.e12345);
        float4 anti_wedge_g1 = float4(this.e3215) * float4(other.group1.xyz, other.e12345);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_dual_g0 * float4(anti_wedge_g0_w), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g0[3])) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * right_dual_g1_w), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g0_w) * other.group2.xyz) + (float3(right_dual_g0[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_w * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g1[2] * right_dual_g0[1]) - (anti_wedge_g1[1] * right_dual_g0[2]), (anti_wedge_g1[0] * right_dual_g0[2]) - (anti_wedge_g1[2] * right_dual_g0[0]), (anti_wedge_g1[1] * right_dual_g0[0]) - (anti_wedge_g1[0] * right_dual_g0[1])) + (anti_wedge_g0_xyz * float3(right_dual_g1_w)) + (float3(anti_wedge_g0_w) * other.group3.xyz)), anti_wedge_g1[3] * right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiDualNum {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group3.xyz, other.e1234);
        float4 anti_wedge_g1 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g3_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g3_xyz.yzx * anti_wedge_g1.zxy)), anti_wedge_g1[3] * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g1.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g1[3]))), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + float3(((anti_wedge_g1.yz * other.group1.zx) - (anti_wedge_g1.zx * other.group1.yz)), (anti_wedge_g1[0] * other.e31) - (anti_wedge_g1[1] * other.e23)) - (float3(other.e1234) * anti_wedge_g0.xyz)), right_dual_g3_xyz[0] * anti_wedge_g0[0])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0_xyz * float4(other.e321).xyz), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g0_xyz[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_via_origin_from(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public Circle reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0_xyz = (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0_xyz * float4(other.e321).xyz), 0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * other.group2.zxy) - (anti_wedge_g0_xyz.zxy * other.group2.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float3 anti_wedge_g1_xyz = (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy)), right_dual_g1_w * anti_wedge_g0[3] * -1.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1_xyz * float3(right_dual_g1_w * -1.0)), (right_dual_g2_w * anti_wedge_g0[3]) - (anti_wedge_g1_xyz[0] * right_dual_g1_xyz[0]) - (anti_wedge_g1_xyz[1] * right_dual_g1_xyz[1]) - (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2_w) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g2_xyz.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1_xyz * float3(right_dual_g2_w)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public Circle reject_via_origin_from(Dipole other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float anti_wedge_g0_w = this.e321 * other.e45 * -1.0;
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group3.zx) - (this.group0.zx * other.group3.yz)), 0.0, 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0_xyz * float3(other.e1234)) - (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g1[3] * other.e1234), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * anti_wedge_g1.xyz) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g1[1] * other.e4125) - (anti_wedge_g1[2] * other.e4315), (anti_wedge_g1[2] * other.e4235) - (anti_wedge_g1[0] * other.e4125), (anti_wedge_g1[0] * other.e4315) - (anti_wedge_g1[1] * other.e4235)) + (anti_wedge_g0_xyz * float3(other.e3215)) - (float3(anti_wedge_g0_w) * other.group2.xyz)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_via_origin_from(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_via_origin_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float4(this.e321).xyz * float4(other.e45).xyz * other.group0.xyz), other.e45 * other.e45 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e321 * -1.0) * float4(other.group1.xyz, other.e45);
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group1.zx) - (this.group0.zx * other.group1.yz)), 0.0, 0.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3((anti_wedge_g1[2] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[0]), (anti_wedge_g1[1] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[1])) + (float3(right_dual_g1[3]) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e1, e2, e3, e5 */
            right_dual_g1 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e321) * other.group0.xyz;
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((anti_wedge_g0_xyz.yzx * other.group1.zxy) - (anti_wedge_g0_xyz.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group0.xyz) - (float3(other.e415) * other.group0.yyz * this.group0.yxx) - (float3(other.e435) * other.group0.xyy * this.group0.zzy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4((float4(this.e321).xyz * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435));
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float3(right_dual_g0_w) * anti_wedge_g0.xyz) + (float3(anti_wedge_g1[3]) * other.group0.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g1.zxy)), right_dual_g0_w * anti_wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4(((float3(this.e321) * other.group6.xyz) + (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy)), 0.0);
        float anti_wedge_g2 = -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group9.zx) - (this.group0.zx * other.group9.yz)), 0.0, 0.0);
        float3 anti_wedge_g5 = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz);
        float anti_wedge_g6_w = other.e12345 * this.e321;
        float3 anti_wedge_g8 = float3(other.e12345) * this.group0.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) - (anti_wedge_g6_w * right_dual_g3[3]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), (anti_wedge_g0_x * other.e45) + (anti_wedge_g6_w * right_dual_g0[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g6_w * other.e3215) + (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2])), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6_w * right_dual_g1[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint reject_via_origin_from(Plane other) {
        float3 anti_wedge_g1 = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235));
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (float3(this.e321 * other.e3215) * other.group0.xyz) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public Circle reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 anti_wedge_g1 = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235));
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Circle(
            /* e423, e431, e412 */
            anti_wedge_g0 * float3(other.e1234), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * float3(other.e1234)), -(anti_wedge_g0[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0[2] * right_dual_g0_xyz[2])), 
            /* e235, e315, e125 */
            (anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g1.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g1.yzx * right_dual_g0_xyz.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e431) - (this.e315 * other.e423), 0.0) + float4(((float3(this.e321) * other.group1.xyz) + float3(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), 0.0)), 0.0);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.zxyx * float4(right_dual_g0.yzx, other.e1)) + float4(-(anti_wedge_g1.yz * right_dual_g0.zx), anti_wedge_g1[0] * right_dual_g0[1] * -1.0, -(right_dual_g1_w * anti_wedge_g0[3]) - (anti_wedge_g0[0] * right_dual_g0[0]) - (anti_wedge_g0[1] * right_dual_g0[1]) - (anti_wedge_g0[2] * right_dual_g0[2])), 
            /* e415, e425, e435, e321 */
            (right_dual_g0 * float4(float3(anti_wedge_g1[3]), anti_wedge_g0[3])) + float4(float3(0.0), -(right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2])) - (anti_wedge_g1.xyzx * float4(float3(right_dual_g1_w), right_dual_g1_xyz[0])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g1[3])) + (float3(right_dual_g0[3]) * anti_wedge_g0.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g1.zxy)), anti_wedge_g1[3] * right_dual_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * float4(right_dual_g0[3]).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlatPoint {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0_xyz = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float anti_wedge_g0_w = this.e321 * other.e45 * -1.0;
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group3.zx) - (this.group0.zx * other.group3.yz)), 0.0, 0.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g3[3])) - (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_xyz[0] * other.e23), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g3[3]) * anti_wedge_g1.xyz) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g1[1] * right_dual_g3[2]), (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g1[2] * right_dual_g3[0]), (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g1[0] * right_dual_g3[1])) + (anti_wedge_g0_xyz * float3(other.e3215)) - (float3(anti_wedge_g0_w) * other.group2.xyz)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0_w)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiFlector> operator /(AntiFlector lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0_xyz * float4(other.e321).xyz), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g0_xyz[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public Circle reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0_xyz = (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0_xyz * float4(other.e321).xyz), 0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * other.group2.zxy) - (anti_wedge_g0_xyz.zxy * other.group2.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float3 anti_wedge_g1_xyz = (float3(this.e321) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float anti_wedge_g1_w = this.e5 * other.e12345;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy)), right_dual_g1_w * anti_wedge_g0[3] * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g1_w) * other.group0.xyz) - (anti_wedge_g1_xyz * float3(right_dual_g1_w))), (right_dual_g2_w * anti_wedge_g0[3]) - (anti_wedge_g1_xyz[0] * right_dual_g1_xyz[0]) - (anti_wedge_g1_xyz[1] * right_dual_g1_xyz[1]) - (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g1_w)) + (float3(right_dual_g2_w) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g1_w * right_dual_g2_w), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1_xyz * float3(right_dual_g2_w)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public Circle reject_via_origin_from(Dipole other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4((float3(other.e1234) * this.group0.xyz), (this.e1 * other.e4235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e321 * other.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e41));
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group3.zx) - (this.group0.zx * other.group3.yz)), 0.0, 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e1234) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz)), anti_wedge_g1[3] * other.e1234), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * anti_wedge_g1.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g1[1] * other.e4125) - (anti_wedge_g1[2] * other.e4315), (anti_wedge_g1[2] * other.e4235) - (anti_wedge_g1[0] * other.e4125), (anti_wedge_g1[0] * other.e4315) - (anti_wedge_g1[1] * other.e4235)) + (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_via_origin_from(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint reject_via_origin_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float4(this.e321).xyz * float4(other.e45).xyz * other.group0.xyz), other.e45 * other.e45 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_via_origin_from(Flector other) {
        float anti_wedge_g0_w = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group1.zx) - (this.group0.zx * other.group1.yz)), 0.0, 0.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3((anti_wedge_g1[2] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[0]), (anti_wedge_g1[1] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[1])) - (float3(anti_wedge_g0_w) * other.group0.xyz) - (float3(right_dual_g1[3] * this.e321) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e1, e2, e3, e5 */
            right_dual_g1 * float4(anti_wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e321) * other.group0.xyz;
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((anti_wedge_g0_xyz.yzx * other.group1.zxy) - (anti_wedge_g0_xyz.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group0.xyz) - (float3(other.e415) * other.group0.yyz * this.group0.yxx) - (float3(other.e435) * other.group0.xyy * this.group0.zzy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4(((float3(this.e321) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e5 * other.e12345);
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float3(right_dual_g0_w) * anti_wedge_g0.xyz) + (float3(anti_wedge_g1[3]) * other.group0.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g1.zxy)), right_dual_g0_w * anti_wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e321) * other.group6.xyz) + (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy)), 0.0);
        float anti_wedge_g2 = (other.e12345 * this.e5) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group9.zx) - (this.group0.zx * other.group9.yz)), 0.0, 0.0);
        float3 anti_wedge_g5 = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz);
        float anti_wedge_g6_w = other.e12345 * this.e321;
        float3 anti_wedge_g8 = float3(other.e12345) * this.group0.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) - (anti_wedge_g6_w * right_dual_g3[3]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), (anti_wedge_g0_x * other.e45) + (anti_wedge_g6_w * right_dual_g0[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g6_w * other.e3215) + (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2])), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6_w * right_dual_g1[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiFlector reject_via_origin_from(Plane other) {
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), 0.0, 0.0);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(0.0, 0.0, (anti_wedge_g1[1] * right_dual_g0[0]) - (anti_wedge_g1[0] * right_dual_g0[1]), 0.0) + float4((float3(((anti_wedge_g1.zx * right_dual_g0.yz) - (anti_wedge_g1.yz * right_dual_g0.zx)), 0.0) - (float3(right_dual_g0[3] * this.e321) * other.group0.xyz)), 0.0), 
            /* e1, e2, e3, e5 */
            (right_dual_g0 * float4(this.e1 * other.e4235)) + (right_dual_g0 * float4(this.e2 * other.e4315)) + (right_dual_g0 * float4(this.e3 * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0_xyz = (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz);
        float anti_wedge_g0_w = this.e1 * other.e4235;
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), 0.0, 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            anti_wedge_g0_xyz * float3(other.e1234), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e1234) * anti_wedge_g1.xyz), -(anti_wedge_g0_xyz[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0_xyz[2])), 
            /* e235, e315, e125, e4 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (right_dual_g0_xyz.yzx * anti_wedge_g1.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g1.yzx)), anti_wedge_g0_w * other.e1234), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0_w) * float4(right_dual_g0_xyz, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float3 anti_wedge_g1_xyz = float3((this.e315 * other.e412) - (this.e125 * other.e431), (this.e125 * other.e423) - (this.e235 * other.e412), (this.e235 * other.e431) - (this.e315 * other.e423)) + (float3(this.e321) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz);
        float anti_wedge_g1_w = this.e5 * other.e12345;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g1_xyz.yzx * right_dual_g0_xyz.zxy)), (anti_wedge_g1_xyz[0] * other.e1) - (right_dual_g1_w * anti_wedge_g0[3]) - (right_dual_g0_xyz[0] * anti_wedge_g0[0]) - (right_dual_g0_xyz[1] * anti_wedge_g0[1]) - (right_dual_g0_xyz[2] * anti_wedge_g0[2])), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1_w)) - (anti_wedge_g1_xyz * float3(right_dual_g1_w))), (right_dual_g0_w * anti_wedge_g0[3]) - (anti_wedge_g1_xyz[0] * right_dual_g1_xyz[0]) - (anti_wedge_g1_xyz[1] * right_dual_g1_xyz[1]) - (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g1_w)) + (float3(right_dual_g0_w) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g1_w * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1_xyz * float3(right_dual_g0_w)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiFlector {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4((float3(other.e1234) * this.group0.xyz), (this.e1 * other.e4235) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e41));
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((this.group0.yz * other.group3.zx) - (this.group0.zx * other.group3.yz)), 0.0, 0.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.xyzx * float4(float3(right_dual_g3[3]), other.e23)) + float4(-(float3(anti_wedge_g0[3]) * other.group0.xyz), anti_wedge_g1[3] * right_dual_g3[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g3[3]) * anti_wedge_g1.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g1[1] * right_dual_g3[2]), (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g1[2] * right_dual_g3[0]), (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g1[0] * right_dual_g3[1])) + (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiLine> operator /(AntiLine lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
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
public AntiLine operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual_g0 * other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public Dipole reject_via_origin_from(Circle other) {
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
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float3 anti_wedge_g1_xyz = float3(other.e12345) * this.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * right_dual_g1_w), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g1_w)) + (anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0_xyz = (this.group0.yzx * other.group3.zxy) - (float3(other.e1234) * this.group1.xyz) - (this.group0.zxy * other.group3.yzx);
        float anti_wedge_g0_w = this.e15 * other.e4235;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0_xyz * float3(other.e1234 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz.zxy * other.group3.yzx) - (anti_wedge_g0_xyz.yzx * other.group3.zxy)), anti_wedge_g0_w * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (float3(anti_wedge_g0_w) * other.group3.xyz)), anti_wedge_g0_xyz[0] * right_dual_g0[0]), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz.yzx * other.group1.zxy) - (right_dual_g0 * float3(anti_wedge_g0_w)) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), anti_wedge_g0_xyz[2] * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_via_origin_from(DualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public AntiMotor reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0_xyz = (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx);
        float anti_wedge_g0_w = this.e15 * other.e4235;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz.zxy * other.group1.yzx) - (anti_wedge_g0_xyz.yzx * other.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_via_origin_from(Line other) {
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
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public AntiMotor reject_via_origin_from(Motor other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4((this.group1.xyz * float4(other.e12345).xyz), 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g0_w)) + (right_dual_g0_xyz * float3(anti_wedge_g0_w))), anti_wedge_g0_w * right_dual_g0_w), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g0_w)) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * right_dual_g1[0]) - (anti_wedge_g0_xyz[1] * right_dual_g1[1]) - (anti_wedge_g0_xyz[2] * right_dual_g1[2]) - (right_dual_g0_xyz[0] * anti_wedge_g1[0]) - (right_dual_g0_xyz[1] * anti_wedge_g1[1]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float4 anti_wedge_g1 = float4(((this.group0.yzx * other.group9.zxy) - (float3(other.e1234) * this.group1.xyz) - (this.group0.zxy * other.group9.yzx)), 0.0);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125);
        float4 anti_wedge_g3 = float4(1.0, 1.0, other.e12345, 0.0) * float4((this.group1.xyz * float3(float2(other.e12345), 1.0)), 0.0);
        float3 anti_wedge_g5 = float3(other.e12345) * this.group0.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (right_dual_g9_w * anti_wedge_g1[3]) + (right_dual_g9_xyz[0] * anti_wedge_g1[0]) + (right_dual_g9_xyz[1] * anti_wedge_g1[1]) + (right_dual_g9_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g5[0] * right_dual_g6_xyz[0]) - (anti_wedge_g5[1] * right_dual_g6_xyz[1]) - (anti_wedge_g5[2] * right_dual_g6_xyz[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g6_xyz * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (right_dual_g9_xyz * float3(anti_wedge_g0_x)) + (right_dual_g6_xyz.yzx * anti_wedge_g1.zxy) + (other.group7.yzx * anti_wedge_g3.zxy) - (right_dual_g7 * float3(anti_wedge_g2)) - (right_dual_g6_xyz.zxy * anti_wedge_g1.yzx) - (other.group7.zxy * anti_wedge_g3.yzx)), anti_wedge_g0_x * right_dual_g9_w), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public AntiLine reject_via_origin_from(Plane other) {
        float3 anti_wedge_g0_xyz = (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx);
        return AntiLine(
            /* e23, e31, e12 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e15, e25, e35 */
            (anti_wedge_g0_xyz * float3(other.e3215)) + (float3(other.e4235) * float3(this.e15 * other.e4235, this.e15 * other.e4315, this.e15 * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public Dipole reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0_xyz = (this.group0.yzx * other.group0.zxy) - (float3(other.e1234) * this.group1.xyz) - (this.group0.zxy * other.group0.yzx);
        float anti_wedge_g0_w = this.e15 * other.e4235;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            anti_wedge_g0_xyz * float3(other.e1234 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g0_xyz.yzx)), anti_wedge_g0_w * other.e1234 * -1.0), 
            /* e15, e25, e35 */
            (anti_wedge_g0_xyz * float3(other.e3215)) - (right_dual_g0_xyz * float3(anti_wedge_g0_w))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g0_w = -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412);
        float3 anti_wedge_g1_xyz = float3(other.e12345) * this.group1.xyz;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_dual_g0 * float4(anti_wedge_g0_w), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g0[3])) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * right_dual_g1_w), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g0[3])) + (float3(anti_wedge_g0_w) * other.group2.xyz)), anti_wedge_g0_w * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g1_w)) + (float3(anti_wedge_g0_w) * other.group3.xyz) + (anti_wedge_g1_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g1_xyz.yzx * right_dual_g0.zxy)), anti_wedge_g0_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiLine {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0_xyz = (this.group0.yzx * other.group3.zxy) - (float3(other.e1234) * this.group1.xyz) - (this.group0.zxy * other.group3.yzx);
        float anti_wedge_g0_w = this.e15 * other.e4235;
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0_xyz * float3(other.e1234 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz.yzx * right_dual_g3_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g3_xyz.yzx)), anti_wedge_g0_w * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) - (right_dual_g3_xyz * float3(anti_wedge_g0_w))), anti_wedge_g0_xyz[0] * other.e41 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g0_w) * other.group0.xyz) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), anti_wedge_g0_w * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiMotor> operator /(AntiMotor lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public Sphere reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215 * -1.0) * other.group0.xyz;
        float anti_wedge_g0_w = other.e45 * this.e3215 * -1.0;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz.yzx * other.group1.zxy) - (right_dual_g0 * float3(anti_wedge_g0_w)) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), anti_wedge_g0_w * other.e45 * -1.0), 
            /* e1234 */
            (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215) * other.group0.xyz;
        float anti_wedge_g0_w = (other.e4 * this.e3215) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12);
        float4 anti_wedge_g1 = float4((float4(this.e3215).xyz * other.group1.xyz), 0.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g0_w), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0_w) * other.group2.xyz), -(anti_wedge_g0_w * other.e4) - (anti_wedge_g0_xyz[0] * other.e423) - (anti_wedge_g0_xyz[1] * other.e431) - (anti_wedge_g0_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g1[3])) + (float3(anti_wedge_g0_w) * other.group3.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), anti_wedge_g0_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4((float4(this.e3215).xyz * other.group1.xyz), 0.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g0_w), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0_w) * other.group2.xyz), -(anti_wedge_g0_xyz[0] * other.e423) - (anti_wedge_g0_xyz[1] * other.e431) - (anti_wedge_g0_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g1[3])) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = (float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group0.xyz);
        float anti_wedge_g0_w = this.scalar * other.e12345;
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * right_dual_g1_w), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g1_w)) + (anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy)), right_dual_g2_w * this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public Sphere reject_via_origin_from(Dipole other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g0[1] * other.e12) - (anti_wedge_g0[2] * other.e31), (anti_wedge_g0[2] * other.e23) - (anti_wedge_g0[0] * other.e12), (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23)) - (right_dual_g0 * float3(anti_wedge_g0[3]))), anti_wedge_g0[3] * other.e45 * -1.0), 
            /* e1234 */
            (right_dual_g0[0] * anti_wedge_g0[0]) + (right_dual_g0[1] * anti_wedge_g0[1]) + (right_dual_g0[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float4 anti_wedge_g1 = float4(0.0, 0.0, this.e31 * other.e4235 * -1.0, 0.0) + (other.group3.zxyx * float4(this.group0.yzx, this.e15)) + float4((float3(-(this.group0.zx * other.group3.yz), 0.0) - (float3(other.e1234) * this.group1.xyz)), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zx * other.group3.yz) - (anti_wedge_g1.yz * other.group3.zx)), (anti_wedge_g1[1] * other.e4235) - (anti_wedge_g1[0] * other.e4315), anti_wedge_g1[3] * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1[3]) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e25)) + (anti_wedge_g1.wwwx * float4(other.group0.xyz, other.e15)) + float4((float3(this.e3215 * other.e1234) * other.group3.xyz), other.e4235 * other.e4235 * this.e3215) - (anti_wedge_g1.zxyw * other.group1.yzxw)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_via_origin_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            anti_wedge_g0 * float4(other.e12345 * -1.0), 
            /* e15, e25, e35, e3215 */
            float4((anti_wedge_g1.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * anti_wedge_g0[3]) - (other.e12345 * anti_wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public AntiDualNum reject_via_origin_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e45 * other.e45 * this.e3215, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_via_origin_from(Flector other) {
        float4 anti_wedge_g1 = (other.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(-(this.group0.zx * other.group1.yz), this.e31 * other.e4235 * -1.0, this.e3215 * other.e45 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, (anti_wedge_g1[1] * other.e4235) - (anti_wedge_g1[0] * other.e4315), 0.0) + float4(((anti_wedge_g1.zx * other.group1.yz) - (anti_wedge_g1.yz * other.group1.zx)), 0.0, 0.0), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1.xyzx * float4(float3(other.e3215), other.e15)) + (anti_wedge_g1.wwwy * float4(other.group1.xyz, other.e25)) + float4(float3(0.0), (anti_wedge_g1[2] * other.e35) + (other.e4235 * other.e4235 * this.e3215) - (anti_wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_via_origin_from(Line other) {
        float anti_wedge_g0_w = -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12);
        float3 anti_wedge_g1_xyz = float3(this.e3215) * other.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), -(anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_via_origin_from(Motor other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g0_w = (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g0_w)) + (right_dual_g0_xyz * float3(anti_wedge_g0_w))), anti_wedge_g0_w * right_dual_g0_w), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g0_w)) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * right_dual_g1[0]) - (anti_wedge_g0_xyz[1] * right_dual_g1[1]) - (anti_wedge_g0_xyz[2] * right_dual_g1[2]) - (right_dual_g0_xyz[0] * anti_wedge_g1[0]) - (right_dual_g0_xyz[1] * anti_wedge_g1[1]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e12345 * this.scalar) + (this.e3215 * other.e4) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4((float3(((this.group0.yz * other.group9.zx) - (this.group0.zx * other.group9.yz)), 0.0) - (float3(this.e3215) * other.group4.xyz) - (float3(other.e1234) * this.group1.xyz)), 0.0);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e3215 * other.e45);
        float4 anti_wedge_g3 = float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e3215) * other.group6.xyz)), 0.0);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group0.xyz) + (float3(this.e3215) * other.group7.xyz);
        float anti_wedge_g6_w = this.e3215 * other.e1234 * -1.0;
        float3 anti_wedge_g8 = float3(this.e3215 * -1.0) * other.group9.xyz;
        float anti_wedge_g9_w = other.e12345 * this.e3215;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g9_w * right_dual_g1[3]) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) - (anti_wedge_g6_w * right_dual_g3[3]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), (anti_wedge_g0_x * other.e45) + (anti_wedge_g6_w * right_dual_g0[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g6_w * other.e3215) + (anti_wedge_g9_w * right_dual_g0[0]) + (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2])), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6_w * right_dual_g1[3]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public AntiMotor reject_via_origin_from(Plane other) {
        float2 anti_wedge_g1_xy = (this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz);
        float anti_wedge_g1_z = (this.e23 * other.e4315) - (this.e31 * other.e4235);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, (anti_wedge_g1_xy[1] * other.e4235) - (anti_wedge_g1_xy[0] * other.e4315), 0.0) + float4(((float2(anti_wedge_g1_z, anti_wedge_g1_xy[0]) * other.group0.yz) - (float2(anti_wedge_g1_xy[1], anti_wedge_g1_z) * other.group0.zx)), 0.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e4235 * other.e4235 * this.e15, this.e15 * other.e4235 * other.e4315, this.e15 * other.e4235 * other.e4125) + (float3(other.e3215) * float3(anti_wedge_g1_xy, anti_wedge_g1_z))), other.e4235 * other.e4235 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public Sphere reject_via_origin_from(RoundPoint other) {
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
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e1234);
        float3 anti_wedge_g1_xyz = float3((this.e31 * other.e4125) - (this.e12 * other.e4315), (this.e12 * other.e4235) - (this.e23 * other.e4125), (this.e23 * other.e4315) - (this.e31 * other.e4235)) - (float3(other.e1234) * this.group1.xyz);
        float anti_wedge_g1_w = this.e15 * other.e4235;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g1_xyz * float3(other.e1234 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0_xyz.yzx)), anti_wedge_g1_w * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) - (right_dual_g0_xyz * float3(anti_wedge_g1_w))), anti_wedge_g0[3] * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e1234 * -1.0) * anti_wedge_g0.xyz), (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = (float4(this.e3215) * float4(other.group0.xyz, other.e4)) + (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412));
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(right_dual_g0[3]) * anti_wedge_g0.xyz)), right_dual_g1_w * anti_wedge_g0[3]), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g0[3])) + (float3(anti_wedge_g0[3]) * other.group2.xyz)), anti_wedge_g0[3] * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_dual_g1_w) * anti_wedge_g0.xyz) + (float3(anti_wedge_g0[3]) * other.group3.xyz) + (anti_wedge_g1_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g1_xyz.yzx * right_dual_g0.zxy)), right_dual_g0[3] * this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiMotor {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group3.xyz, other.e1234);
        float4 anti_wedge_g1 = (other.group3.zxyx * float4(this.group0.yzx, this.e15)) + float4((float3(-(this.group0.zx * other.group3.yz), this.e31 * other.e4235 * -1.0) - (float3(this.e3215) * other.group0.xyz) - (float3(other.e1234) * this.group1.xyz)), this.e3215 * other.e45 * -1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g3_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g3_xyz.yzx * anti_wedge_g1.zxy)), anti_wedge_g1[3] * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g1.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g1[3]))), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + float3(((anti_wedge_g1.yz * other.group1.zx) - (anti_wedge_g1.zx * other.group1.yz)), (anti_wedge_g1[0] * other.e31) - (anti_wedge_g1[1] * other.e23)) - (float3(other.e1234) * anti_wedge_g0.xyz)), right_dual_g3_xyz[0] * anti_wedge_g0[0])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiPlane> operator /(AntiPlane lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiPlane operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public AntiPlane reject_via_origin_from(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.e321) * anti_wedge_g0.xyz)), 0.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g0.zxy)), 0.0), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion reject_via_origin_from(DipoleInversion other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
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
public AntiPlane operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public AntiPlane reject_via_origin_from(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public AntiFlector reject_via_origin_from(Flector other) {
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
public AntiFlector operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public AntiFlector reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g0.zxy)), 0.0), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        float4 anti_wedge_g1 = float4(1.0, 1.0, other.e12345, 0.0) * float4((this.group0.xyz * float3(float2(other.e12345), 1.0)), 0.0);
        float anti_wedge_g2 = other.e12345 * this.e5;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (right_dual_g9_w * anti_wedge_g1[3]) + (right_dual_g9_xyz[0] * anti_wedge_g1[0]) + (right_dual_g9_xyz[1] * anti_wedge_g1[1]) + (right_dual_g9_xyz[2] * anti_wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g6_xyz * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) - (anti_wedge_g1[2] * right_dual_g3[1]), (anti_wedge_g1[2] * right_dual_g3[0]) - (anti_wedge_g1[0] * right_dual_g3[2]), (anti_wedge_g1[0] * right_dual_g3[1]) - (anti_wedge_g1[1] * right_dual_g3[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g8 * float3(anti_wedge_g1[3])) + (right_dual_g9_xyz * float3(anti_wedge_g0_x)) + (right_dual_g6_xyz.yzx * anti_wedge_g1.zxy) - (right_dual_g7 * float3(anti_wedge_g2)) - (right_dual_g6_xyz.zxy * anti_wedge_g1.yzx)), anti_wedge_g0_x * right_dual_g9_w), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public AntiPlane reject_via_origin_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(0.0, 0.0, other.e4125 * other.e4125 * this.e3 * -1.0, 0.0) + float4((float3(-(other.group0.xy * other.group0.xy * this.group0.xy), 0.0) - (float3(other.e4235) * float3(this.e2 * other.e4315, this.e1 * other.e4315, this.e1 * other.e4125)) - (float3(other.e4125) * float3(this.e3 * other.e4235, this.e3 * other.e4315, this.e2 * other.e4315))), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public RoundPoint reject_via_origin_from(Sphere other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            anti_wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0_xyz.yzx * anti_wedge_g0.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g0.yzx)), anti_wedge_g0[0] * other.e1), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(other.e321) * anti_wedge_g0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g0.zxy)), right_dual_g0_w * anti_wedge_g0[3]), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0_w) * anti_wedge_g0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiPlane {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<AntiScalar> operator /(AntiScalar lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e235, e315, e125, e12345 */
            float4((float4(anti_wedge_g2[3]).xyz * other.group2.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) + (anti_wedge_g2[3] * other.scalar) - (right_dual_g0[0] * anti_wedge_g2[0]) - (right_dual_g0[1] * anti_wedge_g2[1]) - (right_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual_g1_xyz * float3(anti_wedge_g2[3])) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g2[3])) + (float3(anti_wedge_g3[3]) * other.group0.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(right_dual_g1_xyz, other.e1)) + float4(((right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), -(other.e423 * anti_wedge_g2[0]) - (other.e431 * anti_wedge_g2[1]) - (other.e412 * anti_wedge_g2[2]) - (anti_wedge_g2[3] * other.e5) - (right_dual_g2_xyz[2] * other.e412 * this.e12345))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public DualNum reject_via_origin_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * other.e321 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public Motor reject_via_origin_from(AntiFlector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual_g0_w * -1.0) * anti_wedge_g1.xyz), (right_dual_g1_xyz[0] * anti_wedge_g1[0]) + (right_dual_g1_xyz[1] * anti_wedge_g1[1]) + (right_dual_g1_xyz[2] * anti_wedge_g1[2]) - (right_dual_g0_w * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g0_xyz.yzx * anti_wedge_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e23) + (anti_wedge_g0[1] * other.e31) + (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public Motor reject_via_origin_from(AntiMotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e23) + (anti_wedge_g0[1] * other.e31) + (anti_wedge_g0[2] * other.e12) + (anti_wedge_g0[3] * other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g1[3] * other.e321) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            anti_wedge_g0 * float3(right_dual_g2_w), 
            /* e415, e425, e435, e321 */
            anti_wedge_g1 * float4(right_dual_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_dual_g2_w) * anti_wedge_g2.xyz), (right_dual_g2_w * anti_wedge_g2[3]) + (anti_wedge_g1[3] * other.e321) - (anti_wedge_g0[0] * right_dual_g2_xyz[0]) - (anti_wedge_g0[1] * right_dual_g2_xyz[1]) - (anti_wedge_g0[2] * right_dual_g2_xyz[2]) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2]) - (other.e423 * anti_wedge_g2[0]) - (other.e431 * anti_wedge_g2[1]) - (other.e412 * anti_wedge_g2[2]))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) + (anti_wedge_g2[0] * other.e41) + (anti_wedge_g2[1] * other.e42) + (anti_wedge_g2[2] * other.e43) + (anti_wedge_g1[0] * other.e23) + (anti_wedge_g1[1] * other.e31) + (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public CircleRotor reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * anti_wedge_g1.xyz) + (anti_wedge_g0.zxy * other.group3.yzx) - (anti_wedge_g0.yzx * other.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(anti_wedge_g1[3]) * other.group3.xyz) + (float3(other.e1234) * anti_wedge_g2.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * anti_wedge_g1.xyz) + float3(((anti_wedge_g2.yz * other.group3.zx) - (anti_wedge_g2.zx * other.group3.yz)), (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235))), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (other.e43 * anti_wedge_g2[2]) + (anti_wedge_g1[0] * other.e23) + (anti_wedge_g1[1] * other.e31) + (anti_wedge_g1[2] * other.e12) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g3[0] * other.e4235))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public DualNum reject_via_origin_from(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * this.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * other.e45 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public Motor reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group1.xyz), -(anti_wedge_g0[3] * other.e45) - (anti_wedge_g1[0] * other.e4235) - (anti_wedge_g1[1] * other.e4315) - (anti_wedge_g1[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e4315) - (anti_wedge_g0[1] * other.e4235), 0.0) + float4(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public Motor reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual_g0_w) * anti_wedge_g0.xyz), (right_dual_g0_w * anti_wedge_g0[3]) - (right_dual_g0_xyz[0] * anti_wedge_g0[0]) - (right_dual_g0_xyz[1] * anti_wedge_g0[1]) - (right_dual_g0_xyz[2] * anti_wedge_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1[3])) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz)), right_dual_g0_w * anti_wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public MultiVector reject_via_origin_from(MultiVector other) {
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
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0[0] * right_dual_g0[0], (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g0[0] * right_dual_g0[1]) + (anti_wedge_g0[1] * right_dual_g0[0]) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g5[0] * right_dual_g6_xyz[0]) - (anti_wedge_g5[1] * right_dual_g6_xyz[1]) - (anti_wedge_g5[2] * right_dual_g6_xyz[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0[0])), 
            /* e5 */
            (anti_wedge_g2 * right_dual_g0[0]) + (anti_wedge_g0[0] * other.e3215), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0[0])) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0[0]) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0[0])), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g4 * float3(other.e3215)) + (right_dual_g6_xyz * float3(anti_wedge_g0[0])) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(right_dual_g0[0]) * anti_wedge_g6.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), (anti_wedge_g0[0] * other.e45) + (right_dual_g0[0] * anti_wedge_g6[3])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0[0])) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0[0])) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (right_dual_g6_xyz.yzx * anti_wedge_g1.zxy) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (right_dual_g6_xyz.zxy * anti_wedge_g1.yzx) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g10 * anti_wedge_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e4235) - (anti_wedge_g0[1] * other.e4315) - (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g0[3] * other.e5) - (other.e4 * this.e12345 * other.e5)
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
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public AntiScalar reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) + (anti_wedge_g0[3] * other.e1234) + (other.e3215 * this.e12345 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_dual_g0_w)) + (anti_wedge_g3.zxyx * float4(right_dual_g0_xyz.yzx, other.e1)) + float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) - (right_dual_g0_xyz.zxy * anti_wedge_g3.yzx)), -(right_dual_g1_xyz[2] * anti_wedge_g1[2]) - (right_dual_g2_xyz[0] * anti_wedge_g0[0]) - (right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g2[3] * other.e4)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2[3])) + (right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2[3])) + (float3(right_dual_g0_w) * anti_wedge_g2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), right_dual_g0_w * anti_wedge_g2[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension AntiScalar {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.yzxw * float4(right_dual_g3.zxy, other.scalar)) + (anti_wedge_g1.xyzx * float4(float3(right_dual_g3[3]), other.e23)) + float4((float3(-(anti_wedge_g0.zx * right_dual_g3.yz), anti_wedge_g0[1] * right_dual_g3[0] * -1.0) - (float3(anti_wedge_g0[3]) * other.group0.xyz)), (right_dual_g3[0] * other.e4235 * this.e12345) - (right_dual_g2_xyz[0] * anti_wedge_g0[0]) - (right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g1[3] * other.e45)), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g3[3]) * anti_wedge_g2.xyz) + (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz) - (float3(anti_wedge_g1[3]) * right_dual_g3.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[2] * right_dual_g3[1]) - (anti_wedge_g2[1] * right_dual_g3[2]), (anti_wedge_g2[0] * right_dual_g3[2]) - (anti_wedge_g2[2] * right_dual_g3[0]), (anti_wedge_g2[1] * right_dual_g3[0]) - (anti_wedge_g2[0] * right_dual_g3[1])) + (right_dual_g2_xyz * float3(anti_wedge_g0[3])) + (float3(other.e3215) * anti_wedge_g1.xyz)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Circle> operator /(Circle lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) + (this.group0.zxy * other.group2.yzx) - (other.group0.yzx * this.group2.zxy) - (this.group0.yzx * other.group2.zxy);
        float anti_wedge_g1 = -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e321)) + (float3(anti_wedge_g1) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g0_xyz[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public AntiFlatPoint reject_via_origin_from(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(1.0, 1.0, other.e3215 * other.e3215, 0.0) * float4((this.group0.xyz * float3(float2(other.e3215 * other.e3215), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Line reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            anti_wedge_g0_xyz * float3(other.e321), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g0_xyz.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Motor reject_via_origin_from(AntiFlector other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float4(other.e321).xyz), (anti_wedge_g0_xyz[0] * right_dual_g1_xyz[0]) + (anti_wedge_g0_xyz[1] * right_dual_g1_xyz[1]) + (anti_wedge_g0_xyz[2] * right_dual_g1_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Line reject_via_origin_from(AntiLine other) {
        float anti_wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Motor reject_via_origin_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.e3215) * this.group0.xyz;
        float anti_wedge_g0_w = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group0.xyz), (anti_wedge_g0_w * other.scalar) + (anti_wedge_g0_xyz[0] * other.e23) + (anti_wedge_g0_xyz[1] * other.e31) + (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Circle reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual_g0 * other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Circle reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) + (other.group2.yzx * this.group0.zxy) - (other.group0.yzx * this.group2.zxy) - (other.group2.zxy * this.group0.yzx);
        float anti_wedge_g1 = -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e321)) + (float3(anti_wedge_g1) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0_xyz.yzx * other.group2.zxy) - (anti_wedge_g0_xyz.zxy * other.group2.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float3 anti_wedge_g3_xyz = (float3(this.e321) * other.group1.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group2.yzx) + (this.group2.yzx * other.group0.zxy) - (this.group0.yzx * other.group2.zxy) - (this.group2.zxy * other.group0.yzx);
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(anti_wedge_g2_w) * other.group1.xyz) + (float3(right_dual_g2_w * other.e12345) * this.group0.xyz) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy)), right_dual_g1_w * anti_wedge_g1[3] * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g2_w) * anti_wedge_g1.xyz) - (anti_wedge_g3_xyz * float3(right_dual_g1_w))), right_dual_g2_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2_w * other.e12345) * this.group2.xyz) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g2_w)), anti_wedge_g2_w * right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Circle reject_via_origin_from(Dipole other) {
        float anti_wedge_g0 = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.zxy * other.group3.yzx) - (anti_wedge_g0.yzx * other.group3.zxy)), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(other.e1234))), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.yzx * other.group3.zxy) - (anti_wedge_g2_xyz.zxy * other.group3.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Circle reject_via_origin_from(DualNum other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public AntiFlatPoint reject_via_origin_from(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((float3(this.e321 * other.e45) * other.group0.xyz) + (float3(other.e15 * other.e15, other.e25 * other.e25, other.e35 * other.e35) * this.group0.xyz) + (float3(other.e15) * this.group0.yxx * other.group0.yyz) + (float3(other.e35) * this.group0.zzy * other.group0.xyy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public VersorEven reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group1.xyz);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group1.zxy) - (this.group2.zxy * other.group1.yzx);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float4(other.e3215).xyz), (anti_wedge_g1_xyz[0] * other.e4235) + (anti_wedge_g1_xyz[1] * other.e4315) + (anti_wedge_g1_xyz[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.yzx * other.group1.zxy) - (anti_wedge_g2_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Circle reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = (float3(this.e321) * other.group0.xyz) + (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        return Circle(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(other.e415) * float3(this.e315 * other.e425, this.e235 * other.e425, this.e235 * other.e435)) - (float3(other.e435) * float3(this.e125 * other.e415, this.e125 * other.e425, this.e315 * other.e425)) - (float3(other.e235 * this.e415) * other.group0.xyz) - (float3(other.e315 * this.e425) * other.group0.xyz) - (float3(other.e125 * this.e435) * other.group0.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group2.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public VersorEven reject_via_origin_from(Motor other) {
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float3 anti_wedge_g3_xyz = (float3(this.e321) * other.group0.xyz) + (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(anti_wedge_g2_w) * other.group0.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group0.xyz)), anti_wedge_g2_w * other.e5 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0_w * other.e12345) * this.group2.xyz) + (anti_wedge_g3_xyz.yzx * right_dual_g1_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g1_xyz.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g0_w)), anti_wedge_g2_w * right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4(((float3(this.e321) * other.group6.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group8.yzx) + (this.group2.yzx * other.group7.zxy) - (this.group0.yzx * other.group8.zxy) - (this.group2.zxy * other.group7.yzx)), 0.0);
        float anti_wedge_g2 = -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4(((float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group9.zxy) - (this.group2.zxy * other.group9.yzx)), 0.0);
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g7 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(other.e12345) * this.group2.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Circle reject_via_origin_from(Plane other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float4(other.e3215).xyz), 0.0), 
            /* e235, e315, e125 */
            (float3(other.e3215 * other.e3215) * this.group0.xyz) + (anti_wedge_g2.yzx * other.group0.zxy) - (float3(this.e321 * other.e3215) * other.group0.xyz) - (anti_wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public Circle reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g0_xyz.yzx), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g2 * float3(other.e1234))), 0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g2.yzx * right_dual_g0_xyz.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float3 anti_wedge_g3_xyz = (float3(this.e321) * other.group1.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group2.yzx) + (this.group2.yzx * other.group0.zxy) - (this.group0.yzx * other.group2.zxy) - (this.group2.zxy * other.group0.yzx);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float3 right_dual_g3_xyz = other.group3.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2_w) * float4(right_dual_g1_xyz, other.e5 * -1.0)) + float4(((anti_wedge_g0 * float3(right_dual_g0_w)) + (anti_wedge_g3_xyz.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g3_xyz.yzx * right_dual_g0_xyz.zxy)), (anti_wedge_g3_xyz[0] * right_dual_g3_xyz[0]) + (anti_wedge_g3_xyz[1] * right_dual_g3_xyz[1]) - (right_dual_g1_w * anti_wedge_g1[3]) - (anti_wedge_g0[0] * right_dual_g2_xyz[0]) - (anti_wedge_g0[1] * right_dual_g2_xyz[1]) - (anti_wedge_g0[2] * right_dual_g2_xyz[2]) - (anti_wedge_g2_xyz[0] * right_dual_g0_xyz[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0_xyz[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0_xyz[2]) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2])), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz) - (anti_wedge_g3_xyz * float3(right_dual_g1_w))), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0_w)) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g0_w)), anti_wedge_g2_w * right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Circle {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.yzx * right_dual_g3_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g3_xyz.yzx)), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(other.e1234))), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.zxy * right_dual_g3_xyz.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g3_xyz.zxy)), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<CircleRotor> operator /(CircleRotor lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w * -1.0) * other.group2.xyz), (anti_wedge_g2_w * other.scalar) + (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float3 anti_wedge_g3_xyz = (float3(other.e321) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (float3(this.e12345) * other.group3.xyz) + (other.group0.zxy * this.group2.yzx) + (this.group0.zxy * other.group2.yzx) - (other.group0.yzx * this.group2.zxy) - (this.group0.yzx * other.group2.zxy);
        float anti_wedge_g3_w = other.e5 * this.e12345;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e321)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(anti_wedge_g3_w) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3_w)) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), (anti_wedge_g3_xyz[0] * other.e1) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public Motor reject_via_origin_from(AntiDualNum other) {
        float4 anti_wedge_g0 = float4((this.group0.xyz * float3(float2(other.e3215), other.e3215)), other.scalar * this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * anti_wedge_g0[3]), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public Motor reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g1_xyz = (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e321) * float4(anti_wedge_g1_xyz, other.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public Motor reject_via_origin_from(AntiFlector other) {
        float3 anti_wedge_g1_xyz = (float3(other.e321) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g1_xyz * float3(right_dual_g0_w * -1.0)), (anti_wedge_g1_xyz[0] * right_dual_g1_xyz[0]) + (anti_wedge_g1_xyz[1] * right_dual_g1_xyz[1]) + (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2]) - (right_dual_g0_w * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public Motor reject_via_origin_from(AntiLine other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_dual_g0 * float3(anti_wedge_g0_w)), -(anti_wedge_g0_xyz[0] * right_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public Motor reject_via_origin_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz);
        float anti_wedge_g0_w = other.scalar * this.e12345;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group0.xyz), (anti_wedge_g0_w * other.scalar) + (anti_wedge_g0_xyz[0] * other.e23) + (anti_wedge_g0_xyz[1] * other.e31) + (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public AntiScalar reject_via_origin_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual_g0 * other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(Circle other) {
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float3 anti_wedge_g3_xyz = (float3(other.e321) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) + (other.group2.yzx * this.group0.zxy) - (other.group0.yzx * this.group2.zxy) - (other.group2.zxy * this.group0.yzx);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e321)) + (float3(anti_wedge_g2_w) * other.group2.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g3_xyz.yzx * other.group2.zxy) - (anti_wedge_g3_xyz.zxy * other.group2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1);
        float4 anti_wedge_g3 = float4(((float3(other.e321) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) + (this.group0.zxy * other.group2.yzx) - (other.group0.yzx * this.group2.zxy) - (this.group0.yzx * other.group2.zxy)), 0.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(anti_wedge_g3[3]) * other.group1.xyz) + (float3(right_dual_g2_w * other.e12345) * this.group0.xyz) + (float3(right_dual_g2_w * this.e12345) * other.group0.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx)), right_dual_g2_w * other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g2_w) * anti_wedge_g1.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), right_dual_g2_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2_w * other.e12345) * this.group2.xyz) + (float3(right_dual_g2_w * this.e12345) * other.group2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), 0.0), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((right_dual_g2 * float3(anti_wedge_g2_w)), -(anti_wedge_g0[0] * right_dual_g2[0]) - (anti_wedge_g0[1] * right_dual_g2[1]) - (anti_wedge_g0[2] * right_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float anti_wedge_g0_w = this.e321 * other.e45 * -1.0;
        float3 anti_wedge_g1_xyz = (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float anti_wedge_g1_w = this.e12345 * other.e45;
        float3 anti_wedge_g2_xyz = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group2.wwwz * float4(anti_wedge_g1_xyz, anti_wedge_g0_xyz[2])) + float4(((right_dual_g0 * float3(anti_wedge_g0_w)) + (anti_wedge_g0_xyz.zxy * other.group3.yzx) - (anti_wedge_g0_xyz.yzx * other.group3.zxy)), (anti_wedge_g0_xyz[0] * other.e15) + (anti_wedge_g0_xyz[1] * other.e25) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g1_w * other.e45) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g3[0] * other.e4235) - (anti_wedge_g3[1] * other.e4315)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(other.e1234)) + (float3(anti_wedge_g1_w) * other.group3.xyz) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.yzx * other.group3.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g2_xyz.zxy * other.group3.yzx)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_via_origin_from(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345 * other.e12345 * -1.0) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(float2(other.e12345 * other.e12345 * -1.0), other.e12345 * other.e12345, other.e5 * other.e12345) * float4(1.0, 1.0, -1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(FlatPoint other) {
        float anti_wedge_g2_w = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * right_dual_g0[3]), 
            /* e235, e315, e125, e12345 */
            right_dual_g0 * float4(float3(anti_wedge_g2_w), this.e12345 * other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(((this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy)), this.e321 * other.e45 * -1.0);
        float4 anti_wedge_g1 = float4((float3(other.e3215) * this.group0.xyz), (this.e12345 * other.e45) - (this.e425 * other.e4315) - (this.e435 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx);
        float4 anti_wedge_g2 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e3215) * this.group1.xyz) + float3(((this.group2.yz * other.group1.zx) - (this.group2.zx * other.group1.yz)), 0.0)), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(-(anti_wedge_g0.yz * other.group1.zx), anti_wedge_g0[0] * other.e4315 * -1.0, -(anti_wedge_g1[3] * other.e45) - (other.e4235 * other.e4235 * this.e12345)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e3215), other.e45)) + (anti_wedge_g1.wwwx * other.group1.xyzx) + float4(float3(0.0), (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[1] * other.e4125) - (anti_wedge_g2[2] * other.e4315), (anti_wedge_g2[2] * other.e4235) - (anti_wedge_g2[0] * other.e4125), (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235)) + (float3(other.e3215) * anti_wedge_g1.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(Line other) {
        float anti_wedge_g2_w = -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435);
        float3 anti_wedge_g3_xyz = (float3(this.e321) * other.group0.xyz) + (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3_xyz[0] * other.e415) - (anti_wedge_g3_xyz[1] * other.e425) - (anti_wedge_g3_xyz[2] * other.e435)), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g3_xyz.yzx * other.group1.zxy) - (anti_wedge_g3_xyz.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_via_origin_from(Motor other) {
        float anti_wedge_g2_w = this.e12345 * other.e5;
        float4 anti_wedge_g3 = float4(((float3(this.e321) * other.group0.xyz) + (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy)), 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w * other.e12345) * this.group0.xyz)), right_dual_g0_w * this.e12345 * other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w * this.e12345) * other.group0.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group1.xyz)), right_dual_g0_w * this.e321 * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w * this.e12345) * other.group1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group2.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g3.zxy)), anti_wedge_g2_w * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = float4(((float3(this.e321) * other.group6.xyz) + (float3(this.e12345) * other.group1.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group8.yzx) + (other.group7.zxy * this.group2.yzx) - (this.group0.yzx * other.group8.zxy) - (other.group7.yzx * this.group2.zxy)), this.e12345 * other.e4);
        float anti_wedge_g2 = (this.e12345 * other.e5) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(this.e12345) * other.group3.xyz) + (float3(other.e3215) * this.group1.xyz)), this.e12345 * other.e45);
        float3 anti_wedge_g4 = (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group5.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group6);
        float3 anti_wedge_g7 = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz);
        float3 anti_wedge_g8 = (float3(other.e12345) * this.group2.xyz) + (float3(this.e12345) * other.group8.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = this.e12345 * other.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) + (right_dual_g0[0] * other.e12345 * this.e12345) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(Plane other) {
        float3 anti_wedge_g0 = (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float4 anti_wedge_g2 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group1.xyz) + float3(((this.group2.yz * other.group0.zx) - (this.group2.zx * other.group0.yz)), 0.0)), 0.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0 * float4(other.e3215).xyz), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(0.0, 0.0, (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235), 0.0) + float4(((float3(other.e3215 * other.e3215) * this.group0.xyz) + float3(((anti_wedge_g2.yz * other.group0.zx) - (anti_wedge_g2.zx * other.group0.yz)), 0.0) - (float3(this.e321 * other.e3215) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public AntiScalar reject_via_origin_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g0[3] * other.e5) - (this.e12345 * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public CircleRotor reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 anti_wedge_g2_xyz = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(other.e3215) * this.group1.xyz);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g0_xyz.yzx), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(other.e1234))), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g0_xyz.zxy)), (right_dual_g0_xyz[0] * this.e12345 * other.e4235) + (this.e12345 * other.e3215 * other.e1234))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e12345);
        float4 anti_wedge_g1 = (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group1);
        float anti_wedge_g2_w = this.e12345 * other.e5;
        float4 anti_wedge_g3 = float4((float3((this.e315 * other.e412) - (this.e125 * other.e431), (this.e125 * other.e423) - (this.e235 * other.e412), (this.e235 * other.e431) - (this.e315 * other.e423)) + (float3(this.e321) * other.group1.xyz) + (float3(this.e12345) * other.group3.xyz) + (float3(other.e321) * this.group1.xyz) + (this.group0.zxy * other.group2.yzx) - (this.group0.yzx * other.group2.zxy)), this.e12345 * other.e4);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_dual_g0_w)) + (anti_wedge_g3.zxyx * float4(right_dual_g0_xyz.yzx, other.e1)) + float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) - (right_dual_g0_xyz.zxy * anti_wedge_g3.yzx)), -(right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2])), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2_w)) + (right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w * this.e12345) * other.group2.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), anti_wedge_g2_w * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension CircleRotor {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0_xyz = (float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy);
        float anti_wedge_g0_w = this.e12345 * other.scalar;
        float3 anti_wedge_g1_xyz = (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float anti_wedge_g1_w = this.e12345 * other.e45;
        float3 anti_wedge_g2_xyz = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual_g3.wwwx * float4(anti_wedge_g1_xyz, this.e12345 * other.e4235)) + float4(((anti_wedge_g0_xyz.yzx * right_dual_g3.zxy) - (float3(anti_wedge_g0_w) * other.group0.xyz) - (anti_wedge_g0_xyz.zxy * right_dual_g3.yzx)), (anti_wedge_g0_w * other.scalar) - (anti_wedge_g1_w * other.e45) - (anti_wedge_g0_xyz[0] * right_dual_g2_xyz[0]) - (anti_wedge_g0_xyz[1] * right_dual_g2_xyz[1]) - (anti_wedge_g0_xyz[2] * right_dual_g2_xyz[2])), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(right_dual_g3[3])) - (float3(anti_wedge_g0_w) * other.group1.xyz) - (float3(anti_wedge_g1_w) * right_dual_g3.xyz)), anti_wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (right_dual_g2_xyz * float3(anti_wedge_g0_w)) + (anti_wedge_g2_xyz.zxy * right_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g3.zxy)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0_w)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Dipole> operator /(Dipole lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45);
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
public FlatPoint operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public FlatPoint reject_via_origin_from(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, other.e3215 * other.e3215, 0.0) * float4((this.group0.xyz * float3(float2(other.e3215 * other.e3215), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public FlatPoint reject_via_origin_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((-(float3(other.e321 * this.e45) * other.group0.xyz) - (float3(other.e235 * other.e235, other.e315 * other.e315, other.e125 * other.e125) * this.group0.xyz) - (float3(other.e235) * this.group0.yxx * other.group0.yyz) - (float3(other.e125) * this.group0.zzy * other.group0.xyy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public Flector reject_via_origin_from(AntiFlector other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e321 * this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public Flector reject_via_origin_from(AntiMotor other) {
        float4 anti_wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g0.xyz * float4(other.e3215).xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23), 0.0) + float4(((anti_wedge_g0.yz * other.group0.zx) - (anti_wedge_g0.zx * other.group0.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public Dipole reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual_g0 * other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public Dipole reject_via_origin_from(Circle other) {
        float anti_wedge_g0 = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45);
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
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_dual_g2_w)) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g2_w)) + (right_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g2_w)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w))), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group3.zx) - (this.group1.zx * other.group3.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(anti_wedge_g0[3]) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.zxyw * other.group3.yzxw) + float4(-(anti_wedge_g0.yz * other.group3.zx), anti_wedge_g0[0] * other.e4315 * -1.0, anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g0.xyz)), right_dual_g0[0] * anti_wedge_g0[0]), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23), 0.0) + float4((float3(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0) - (right_dual_g0 * float3(anti_wedge_g1)) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public Dipole reject_via_origin_from(DualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group1.zx) - (this.group1.zx * other.group1.yz)), 0.0)), 0.0);
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zx * other.group1.yz) - (anti_wedge_g0.yz * other.group1.zx)), (anti_wedge_g0[1] * other.e4235) - (anti_wedge_g0[0] * other.e4315), anti_wedge_g0[3] * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + (float3(other.e3215) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public AntiLine reject_via_origin_from(Line other) {
        float anti_wedge_g0 = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public VersorOdd reject_via_origin_from(Motor other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float anti_wedge_g2_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((anti_wedge_g0 * float3(right_dual_g0_w)), anti_wedge_g2_w * right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e15, e25, e35, e1234 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w * other.e12345) * this.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1[3])) + (anti_wedge_g0.yzx * right_dual_g1_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g1_xyz.yzx)), anti_wedge_g2_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group9.zx) - (this.group1.zx * other.group9.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float4 anti_wedge_g3 = float4(other.e12345) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(other.e12345) * this.group1.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (right_dual_g9_w * anti_wedge_g1[3]) + (right_dual_g9_xyz[0] * anti_wedge_g1[0]) + (right_dual_g9_xyz[1] * anti_wedge_g1[1]) + (right_dual_g9_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g5[0] * right_dual_g6_xyz[0]) - (anti_wedge_g5[1] * right_dual_g6_xyz[1]) - (anti_wedge_g5[2] * right_dual_g6_xyz[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g4 * float3(other.e3215)) + (right_dual_g6_xyz * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (right_dual_g9_xyz * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (right_dual_g6_xyz.yzx * anti_wedge_g1.zxy) + (other.group7.yzx * anti_wedge_g3.zxy) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (right_dual_g6_xyz.zxy * anti_wedge_g1.yzx) - (other.group7.zxy * anti_wedge_g3.yzx)), anti_wedge_g0_x * right_dual_g9_w), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public Dipole reject_via_origin_from(Plane other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0)), 0.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zx * other.group0.yz) - (anti_wedge_g0.yz * other.group0.zx)), (anti_wedge_g0[1] * other.e4235) - (anti_wedge_g0[0] * other.e4315), anti_wedge_g0[3] * other.e3215), 
            /* e15, e25, e35 */
            (float3(other.e4235) * float3(this.e25 * other.e4315, this.e15 * other.e4315, this.e15 * other.e4125)) + (float3(other.e4125) * float3(this.e35 * other.e4235, this.e35 * other.e4315, this.e25 * other.e4315)) + (float3(other.e3215) * anti_wedge_g0.xyz) + (float3(this.e45 * other.e3215) * other.group0.xyz) + (float3(other.e4235 * other.e4235, other.e4315 * other.e4315, other.e4125 * other.e4125) * this.group2.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public Dipole reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(anti_wedge_g0[3])) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_dual_g0_xyz.zxy, other.e3215)) + float4(-(right_dual_g0_xyz.yzx * anti_wedge_g0.zxy), anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * anti_wedge_g0.xyz) - (right_dual_g0_xyz * float3(anti_wedge_g1))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float3 anti_wedge_g0 = float3(other.e12345) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group2.xyz;
        float anti_wedge_g2_w = -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(right_dual_g0_w)) + (right_dual_g0_xyz * float3(anti_wedge_g2_w))), anti_wedge_g2_w * right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0_w)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w))), anti_wedge_g2_w * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g2_w) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (anti_wedge_g2_xyz.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g0_xyz.zxy)), anti_wedge_g2_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Dipole {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e23 * other.e4315) - (this.e31 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group3.zx) - (this.group1.zx * other.group3.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g0[3])) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_dual_g3_xyz.zxy, other.e3215)) + float4(-(right_dual_g3_xyz.yzx * anti_wedge_g0.zxy), anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g0.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g1))), anti_wedge_g0[0] * other.e41 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23), 0.0) + float4(((float3(anti_wedge_g1) * other.group0.xyz) + float3(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e45 * this.e3215)) + (right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(-(other.group2.zx * this.group3.yz), other.e315 * this.e4235 * -1.0, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45));
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g2_xyz * float4(anti_wedge_g2[3]).xyz), -(anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g2[3] * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g1[3])) + (float3(anti_wedge_g2[3]) * other.group3.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g2.zxy) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g2.yzx)), anti_wedge_g2[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_via_origin_from(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, other.e3215 * other.e3215, 0.0) * float4((this.group0.xyz * float3(float2(other.e3215 * other.e3215), 1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g0_xyz = (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(other.e321 * this.e45 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz * float4(right_dual_g0[3]).xyz), -(anti_wedge_g0_xyz[0] * right_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_via_origin_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4((float3(this.e1234) * other.group0.xyz), (other.e1 * this.e4235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e321 * this.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e41));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(right_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Plane reject_via_origin_from(AntiLine other) {
        float4 anti_wedge_g0 = float4(((float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0.yzx * anti_wedge_g0.zxy) - (right_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Flector reject_via_origin_from(AntiMotor other) {
        float3 anti_wedge_g1_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(other.e3215) * this.group0.xyz) + (float3(this.e1234) * other.group1.xyz);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1_xyz * float4(other.e3215).xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz.yzx * other.group0.zxy) - (anti_wedge_g1_xyz.zxy * other.group0.yzx)), anti_wedge_g1_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Plane reject_via_origin_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, other.e3 * other.e3 * this.e4125, 0.0) + float4(((float3(other.e1) * float3(other.e2 * this.e4315, other.e2 * this.e4235, other.e3 * this.e4235)) + (float3(other.e3) * float3(other.e1 * this.e4125, other.e2 * this.e4125, other.e2 * this.e4315)) + (float3(other.e5 * this.e1234) * other.group0.xyz) + float3((other.group0.xy * other.group0.xy * this.group3.xy), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g0 * other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz);
        float3 anti_wedge_g2_xyz = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx);
        float3 right_dual_g1_xyz = other.group1.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * right_dual_g1_xyz[0]) - (anti_wedge_g0[1] * right_dual_g1_xyz[1]) - (anti_wedge_g0[2] * right_dual_g1_xyz[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group2.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz * float3(other.e321)) - (anti_wedge_g0.zxy * other.group2.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float anti_wedge_g0_w = other.e321 * this.e45 * -1.0;
        float4 anti_wedge_g1 = float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz)), other.e12345 * this.e45);
        float3 anti_wedge_g2_xyz = float3((other.e315 * this.e4125) - (other.e125 * this.e4315), (other.e125 * this.e4235) - (other.e235 * this.e4125), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + (float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g2_w)) + (right_dual_g1 * float4(anti_wedge_g0_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g2_w)) + (right_dual_g2_xyz * float3(anti_wedge_g0_w))), right_dual_g2_w * other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_dual_g2_w) * anti_wedge_g3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), right_dual_g2_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (right_dual_g0 * float3(other.e45 * this.e3215)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), (other.e41 * this.e4235) - (this.e42 * other.e4315) - (this.e43 * other.e4125) - (other.e1234 * this.e45)) - (other.group3.xyzx * float4(float3(this.e3215), this.e41));
        float4 anti_wedge_g3 = (other.group3.zxyw * this.group1.yzxw) + (other.group3.wwwx * float4(this.group0.xyz, this.e15)) + (this.group2.wwwz * float4(other.group2.xyz, other.e4125)) + float4((float3(((other.group1.zx * this.group3.yz) - (other.group3.yz * this.group1.zx)), (other.e31 * this.e4235) - (other.e4235 * this.e31)) - (float3(this.e3215) * other.group0.xyz)), (other.e4315 * this.e25) - (other.e45 * this.e3215) - (other.e35 * this.e4125)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15));
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(anti_wedge_g2[3]) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.yzxw * float4(anti_wedge_g3.zxy, anti_wedge_g2[3])) - (anti_wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) - (other.e41 * anti_wedge_g3[0]) - (anti_wedge_g1[3] * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e35)) + (anti_wedge_g3.wwwx * float4(other.group0.xyz, other.e15)) + float4(((anti_wedge_g0 * float3(other.e3215)) + float3((anti_wedge_g1.yz * other.group3.zx), anti_wedge_g1[0] * other.e4315)), (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g3[1] * other.e25)) - (anti_wedge_g2.xyzy * float4(float3(other.e1234), other.e4315)) - (anti_wedge_g2.wwwz * float4(other.group2.xyz, other.e4125)) - (anti_wedge_g3.zxyw * other.group1.yzxw) - (other.group3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_via_origin_from(DualNum other) {
        float4 anti_wedge_g0 = float4((this.group0.xyz * float3(float2(other.e12345), other.e12345)), other.e5 * this.e1234);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.e12345 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g3.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * anti_wedge_g0[3]) - (other.e12345 * anti_wedge_g3[3]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_via_origin_from(FlatPoint other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) + (other.e45 * other.e45 * this.e3215) + (this.e4235 * other.e15 * other.e45) + (this.e4315 * other.e25 * other.e45) + (this.e4125 * other.e35 * other.e45)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e45
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz;
        float2 anti_wedge_g1_xy = (this.group3.zx * other.group1.yz) - (this.group3.yz * other.group1.zx);
        float anti_wedge_g1_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group1.xyz);
        float anti_wedge_g2_w = (this.e1234 * other.e45) - (this.e41 * other.e4235) - (this.e42 * other.e4315) - (this.e43 * other.e4125);
        float4 anti_wedge_g3 = (this.group2.wwwy * float4(other.group0.xyz, other.e4315)) + (other.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(((float3(other.e3215) * this.group0.xyz) + float3(-(this.group1.zx * other.group1.yz), this.e31 * other.e4235 * -1.0)), (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2_w * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zx * other.group1.yz) - (anti_wedge_g3.yz * other.group1.zx)), (anti_wedge_g3[1] * other.e4235) - (anti_wedge_g3[0] * other.e4315), anti_wedge_g2_w * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group1.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), (anti_wedge_g2_w * other.e45) + (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g1_z * other.e4315) - (anti_wedge_g2_w * other.e15), -(anti_wedge_g2_w * other.e25) - (anti_wedge_g1_xy[0] * other.e4125), (anti_wedge_g1_xy[0] * other.e4315) - (anti_wedge_g2_w * other.e35) - (anti_wedge_g1_xy[1] * other.e4235), (anti_wedge_g3[0] * other.e15) + (anti_wedge_g3[1] * other.e25) - (anti_wedge_g2_xyz[0] * other.e4235) - (anti_wedge_g2_xyz[1] * other.e4315) - (anti_wedge_g3[3] * other.e45)) + float4(((anti_wedge_g0 * float3(other.e3215)) + float3((float2(anti_wedge_g1_xy[1], anti_wedge_g1_z) * other.group1.zx), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group1.zxy) - (anti_wedge_g0.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group3.xyz) - (float3(other.e415) * other.group0.yyz * this.group3.yxx) - (float3(other.e435) * other.group0.xyy * this.group3.zzy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(((float3(this.e1234) * other.group0.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e1234 * other.e5);
        float anti_wedge_g1_w = this.e45 * other.e12345;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w * this.e1234) * other.group1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group1.xyz)), anti_wedge_g1_w * right_dual_g0_w), 
            /* e15, e25, e35, e1234 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w) * float3(this.e4125 * other.e315, this.e4235 * other.e125, this.e4315 * other.e235)) + (float3(right_dual_g0_w * this.e3215) * other.group0.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group2.xyz) - (float3(right_dual_g0_w) * float3(this.e4315 * other.e125, this.e4125 * other.e235, this.e4235 * other.e315))), right_dual_g0_w * this.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1_w)) + (float3(right_dual_g0_w) * anti_wedge_g3.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g0.zxy)), right_dual_g0_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = (float4(this.e1234) * other.group3) + (this.group3.yzxy * float4(other.group5.zxy, other.e42)) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group9.zx) - (this.group1.zx * other.group9.yz)), (this.e23 * other.e4315) - (this.e31 * other.e4235)) - (float3(this.e3215) * other.group4.xyz) - (other.group5.yzx * this.group3.zxy)), (other.e41 * this.e4235) + (other.e43 * this.e4125) - (this.e41 * other.e4235) - (this.e42 * other.e4315) - (this.e43 * other.e4125)) - (float4(other.e1234) * float4(this.group2.xyz, this.e45));
        float anti_wedge_g2 = (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45);
        float4 anti_wedge_g3 = float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group6.xyz) + (other.group8.yzx * this.group3.zxy) - (other.group8.zxy * this.group3.yzx)), other.e12345 * this.e45);
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group3.yzx) - (other.group7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 anti_wedge_g6 = (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group9.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group3;
        float anti_wedge_g10 = other.e12345 * this.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(Plane other) {
        float2 anti_wedge_g1_xy = (this.group3.zx * other.group0.yz) - (this.group3.yz * other.group0.zx);
        float anti_wedge_g1_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float4 anti_wedge_g3 = (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + float4(((float3(other.e3215) * this.group0.xyz) + float3(-(this.group1.zx * other.group0.yz), this.e31 * other.e4235 * -1.0)), this.e45 * other.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0, 0.0, (anti_wedge_g3[1] * other.e4235) - (anti_wedge_g3[0] * other.e4315), 0.0) + float4(((anti_wedge_g3.zx * other.group0.yz) - (anti_wedge_g3.yz * other.group0.zx)), 0.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group0.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1_xy[1], anti_wedge_g1_z, anti_wedge_g1_xy[0], this.e1234 * other.e3215) * other.group0.zxyw) + float4(((float3(this.e1234 * other.e3215) * other.group0.xyz) + float3(-(float2(anti_wedge_g1_z, anti_wedge_g1_xy[0]) * other.group0.yz), anti_wedge_g1_xy[1] * other.e4235 * -1.0)), (other.e4235 * other.e4235 * this.e3215) - (this.e4235 * other.e4235 * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public Sphere reject_via_origin_from(RoundPoint other) {
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
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g1 = (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group0.zxy, other.e1234));
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g2_w = this.e45 * other.e1234 * -1.0;
        float4 anti_wedge_g3 = (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + float4(((float3(other.e3215) * this.group0.xyz) + float3(-(this.group1.zx * other.group0.yz), this.e31 * other.e4235 * -1.0) - (float3(other.e1234) * this.group2.xyz)), this.e45 * other.e3215);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(anti_wedge_g2_w)) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0_xyz.zxy * anti_wedge_g3.yzx), anti_wedge_g2_w * other.e3215) - (anti_wedge_g3.zxyw * float4(right_dual_g0_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g3.xyz) - (right_dual_g0_xyz * float3(anti_wedge_g3[3]))), anti_wedge_g1[3] * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(this.e1234 * other.e3215), anti_wedge_g1[3])) + float4(((right_dual_g0_xyz.yzx * anti_wedge_g1.zxy) - (anti_wedge_g2_xyz * float3(other.e1234)) - (float3(other.e3215 * other.e1234) * this.group3.xyz) - (right_dual_g0_xyz.zxy * anti_wedge_g1.yzx)), anti_wedge_g2_xyz[0] * right_dual_g0_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = (this.group3.yzxy * float4(other.group0.zxy, other.e2)) + float4(((float3(this.e1234) * other.group1.xyz) + (float3(other.e12345) * this.group0.xyz) + float3(-(this.group3.zx * other.group0.yz), this.e4315 * other.e423 * -1.0)), (this.e1234 * other.e5) + (this.e4235 * other.e1) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412));
        float4 anti_wedge_g1 = float4(((float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz) - (float3(other.e321) * this.group3.xyz)), this.e45 * other.e12345);
        float4 anti_wedge_g2 = float4((float3((this.e4125 * other.e315) - (this.e4315 * other.e125), (this.e4235 * other.e125) - (this.e4125 * other.e235), (this.e4315 * other.e235) - (this.e4235 * other.e315)) + (float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group2.xyz)), this.e1234 * other.e12345);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w) * anti_wedge_g0.xyz)), right_dual_g0_w * anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_dual_g0_w)) + (right_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(right_dual_g0_xyz[0] * anti_wedge_g1[0]) - (right_dual_g0_xyz[1] * anti_wedge_g1[1]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g0[1] * right_dual_g2[2]) - (anti_wedge_g0[2] * right_dual_g2[1]), (anti_wedge_g0[2] * right_dual_g2[0]) - (anti_wedge_g0[0] * right_dual_g2[2]), (anti_wedge_g0[0] * right_dual_g2[1]) - (anti_wedge_g0[1] * right_dual_g2[0])) + (float3(anti_wedge_g0[3]) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group3.xyz) + (right_dual_g0_xyz.yzx * anti_wedge_g2.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g2.yzx)), right_dual_g0_w * this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DipoleInversion {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group3.xyz);
        float anti_wedge_g2_w = (this.e1234 * other.e45) - (this.e41 * other.e4235) - (this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234);
        float4 anti_wedge_g3 = (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + (other.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group3.yz * other.group1.zx) - (this.group1.zx * other.group3.yz)), (this.e4235 * other.e31) - (this.e31 * other.e4235))), (this.e45 * other.e3215) + (this.e35 * other.e4125) - (this.e4235 * other.e15)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group3.zxyz * float4(other.group1.yzx, other.e35)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315));
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g2_w)) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3_xyz.zxy * anti_wedge_g3.yzx), anti_wedge_g2_w * other.e3215) - (anti_wedge_g3.zxyw * float4(right_dual_g3_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g3.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g3[3]))), (anti_wedge_g2_w * other.e45) - (anti_wedge_g0[0] * right_dual_g3_xyz[0]) - (anti_wedge_g0[1] * right_dual_g3_xyz[1]) - (anti_wedge_g0[2] * right_dual_g3_xyz[2]) - (anti_wedge_g1[3] * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(anti_wedge_g3[3]) * other.group0.xyz) + (right_dual_g3_xyz.yzx * anti_wedge_g1.zxy) + float3(((anti_wedge_g3.yz * other.group1.zx) - (anti_wedge_g3.zx * other.group1.yz)), (anti_wedge_g3[0] * other.e31) - (anti_wedge_g3[1] * other.e23)) - (anti_wedge_g2_xyz * float3(other.e1234)) - (float3(anti_wedge_g2_w) * other.group2.xyz) - (right_dual_g3_xyz.zxy * anti_wedge_g1.yzx)), (anti_wedge_g2_xyz[0] * right_dual_g3_xyz[0]) + (anti_wedge_g2_xyz[1] * right_dual_g3_xyz[1]) + (anti_wedge_g2_xyz[2] * right_dual_g3_xyz[2]) + (anti_wedge_g1[3] * other.e3215))
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<DualNum> operator /(DualNum lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e235, e315, e125, e12345 */
            float4((float4(anti_wedge_g2[3]).xyz * other.group2.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) + (anti_wedge_g2[3] * other.scalar) - (right_dual_g0[0] * anti_wedge_g2[0]) - (right_dual_g0[1] * anti_wedge_g2[1]) - (right_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual_g1_xyz * float3(anti_wedge_g2[3])) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g2[3])) + (float3(anti_wedge_g3[3]) * other.group0.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(right_dual_g1_xyz, other.e1)) + float4(((right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), -(other.e423 * anti_wedge_g2[0]) - (other.e431 * anti_wedge_g2[1]) - (other.e412 * anti_wedge_g2[2]) - (anti_wedge_g2[3] * other.e5) - (this.e12345 * right_dual_g2_xyz[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public DualNum reject_via_origin_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public Motor reject_via_origin_from(AntiFlector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual_g0_w * -1.0) * anti_wedge_g1.xyz), (right_dual_g1_xyz[0] * anti_wedge_g1[0]) + (right_dual_g1_xyz[1] * anti_wedge_g1[1]) + (right_dual_g1_xyz[2] * anti_wedge_g1[2]) - (right_dual_g0_w * this.e12345 * other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g0_xyz.yzx * anti_wedge_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e23) + (anti_wedge_g0[1] * other.e31) + (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public Motor reject_via_origin_from(AntiMotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e23) + (anti_wedge_g0[1] * other.e31) + (anti_wedge_g0[2] * other.e12) + (anti_wedge_g0[3] * other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public DualNum reject_via_origin_from(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g1[3] * other.e321) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e12345);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = other.group2 * float4(float2(this.e12345), this.e12345, this.e5);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(right_dual_g2_w) * anti_wedge_g0.xyz), (right_dual_g2_w * anti_wedge_g0[3]) + (anti_wedge_g1[3] * other.e321) - (right_dual_g2_xyz[0] * anti_wedge_g0[0]) - (right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2])), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g2_w) * anti_wedge_g1.xyz) + (float3(anti_wedge_g2[3]) * other.group0.xyz)), right_dual_g2_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2_w) * anti_wedge_g2.xyz) + (float3(anti_wedge_g2[3]) * other.group1.xyz)), right_dual_g2_w * anti_wedge_g2[3]), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) + (anti_wedge_g2[0] * other.e41) + (anti_wedge_g2[1] * other.e42) + (anti_wedge_g2[2] * other.e43) + (anti_wedge_g1[0] * other.e23) + (anti_wedge_g1[1] * other.e31) + (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = this.e5 * other.e1234;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(anti_wedge_g0_w)) + (float3(other.e1234) * anti_wedge_g1.xyz) + (anti_wedge_g0_xyz.zxy * other.group3.yzx) - (anti_wedge_g0_xyz.yzx * other.group3.zxy)), (anti_wedge_g0_xyz[0] * other.e15) + (anti_wedge_g0_xyz[1] * other.e25) + (anti_wedge_g0_xyz[2] * other.e35) + (anti_wedge_g3[3] * other.e1234) - (right_dual_g0[0] * anti_wedge_g2[0]) - (right_dual_g0[1] * anti_wedge_g2[1]) - (right_dual_g0[2] * anti_wedge_g2[2]) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g3[0] * other.e4235) - (anti_wedge_g3[1] * other.e4315)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (float3(anti_wedge_g1[3]) * other.group3.xyz) + (float3(other.e1234) * anti_wedge_g2.xyz) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[1] * other.e4125) - (anti_wedge_g2[2] * other.e4315), (anti_wedge_g2[2] * other.e4235) - (anti_wedge_g2[0] * other.e4125), (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235)) + (float3(other.e3215) * anti_wedge_g1.xyz) - (float3(anti_wedge_g0_w) * other.group2.xyz)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public DualNum reject_via_origin_from(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e12345 * other.e12345 * this.e5) - (other.e5 * other.e12345 * this.e12345), other.e12345 * other.e12345 * this.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public Motor reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group1.xyz), -(anti_wedge_g0[3] * other.e45) - (anti_wedge_g1[0] * other.e4235) - (anti_wedge_g1[1] * other.e4315) - (anti_wedge_g1[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e4315) - (anti_wedge_g0[1] * other.e4235), 0.0) + float4(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public Motor reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float anti_wedge_g1_w = (this.e5 * other.e12345) + (this.e12345 * other.e5);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_dual_g0_w) * anti_wedge_g0.xyz), (right_dual_g0_w * anti_wedge_g0[3]) - (right_dual_g0_xyz[0] * anti_wedge_g0[0]) - (right_dual_g0_xyz[1] * anti_wedge_g0[1]) - (right_dual_g0_xyz[2] * anti_wedge_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1_w)) + (float3(right_dual_g0_w * this.e12345) * other.group1.xyz)), anti_wedge_g1_w * right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public MultiVector reject_via_origin_from(MultiVector other) {
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
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) + (right_dual_g0[0] * this.e12345 * other.e12345) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e4235) - (anti_wedge_g0[1] * other.e4315) - (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public AntiScalar reject_via_origin_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g0[3] * other.e5) - (this.e12345 * other.e4 * other.e5)
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
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public VersorEven reject_via_origin_from(Sphere other) {
        float anti_wedge_g0_w = this.e5 * other.e1234;
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_wedge_g3[0] * right_dual_g0[0]) + (anti_wedge_g3[1] * right_dual_g0[1]) + (anti_wedge_g3[2] * right_dual_g0[2]) + (anti_wedge_g3[3] * right_dual_g0[3]) + (this.e12345 * other.e3215 * other.e1234)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(anti_wedge_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float anti_wedge_g2_w = (this.e5 * other.e12345) + (this.e12345 * other.e5);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_dual_g0_w)) + (anti_wedge_g3.zxyx * float4(right_dual_g0_xyz.yzx, other.e1)) + float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) - (right_dual_g0_xyz.zxy * anti_wedge_g3.yzx)), -(right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2])), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2_w)) + (right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g0_w * this.e12345) * other.group2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), anti_wedge_g2_w * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension DualNum {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = (this.e5 * other.e1234) + (this.e12345 * other.scalar);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2;
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.xyzx * float4(float3(right_dual_g3[3]), other.e23)) + float4(((anti_wedge_g0_xyz.yzx * right_dual_g3.zxy) - (float3(anti_wedge_g0_w) * other.group0.xyz) - (anti_wedge_g0_xyz.zxy * right_dual_g3.yzx)), (anti_wedge_g0_w * other.scalar) + (this.e12345 * right_dual_g3[0] * other.e4235) - (anti_wedge_g0_xyz[0] * right_dual_g2_xyz[0]) - (anti_wedge_g0_xyz[1] * right_dual_g2_xyz[1]) - (anti_wedge_g0_xyz[2] * right_dual_g2_xyz[2]) - (anti_wedge_g1[3] * other.e45)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (float3(right_dual_g3[3]) * anti_wedge_g2.xyz) - (float3(anti_wedge_g0_w) * other.group1.xyz) - (float3(anti_wedge_g1[3]) * right_dual_g3.xyz)), anti_wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[2] * right_dual_g3[1]) - (anti_wedge_g2[1] * right_dual_g3[2]), (anti_wedge_g2[0] * right_dual_g3[2]) - (anti_wedge_g2[2] * right_dual_g3[0]), (anti_wedge_g2[1] * right_dual_g3[0]) - (anti_wedge_g2[0] * right_dual_g3[1])) + (right_dual_g2_xyz * float3(anti_wedge_g0_w)) + (float3(other.e3215) * anti_wedge_g1.xyz)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0_w)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<FlatPoint> operator /(FlatPoint lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
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
public FlatPoint operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_via_origin_from(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e321 * this.e45) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public Flector reject_via_origin_from(AntiFlector other) {
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
public FlatPoint operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_via_origin_from(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public Dipole reject_via_origin_from(Circle other) {
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
public VersorOdd operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g1 = float4(float3(0.0), other.e12345 * this.e45);
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g2_w = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g2_w) * float4(other.group0.xyz, right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g2_w)) + (right_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g2_w) * other.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(other.e1234 * -1.0) * this.group0;
        float anti_wedge_g1 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(anti_wedge_g0[3]) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.zxyw * other.group3.yzxw) + float4(-(anti_wedge_g0.yz * other.group3.zx), anti_wedge_g0[0] * other.e4315 * -1.0, anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g0.xyz)), right_dual_g0[0] * anti_wedge_g0[0]), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23), 0.0) + float4((float3(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0) - (right_dual_g0 * float3(anti_wedge_g1)) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_via_origin_from(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public Flector reject_via_origin_from(Flector other) {
        float anti_wedge_g0_x = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_wedge_g0_x * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public Flector reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(other.e12345 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0_xyz * float4(anti_wedge_g0[3]).xyz), -(right_dual_g0_xyz[0] * anti_wedge_g0[0]) - (right_dual_g0_xyz[1] * anti_wedge_g0[1]) - (right_dual_g0_xyz[2] * anti_wedge_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = float4(other.e1234 * -1.0) * this.group0;
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (right_dual_g9_w * anti_wedge_g1[3]) + (right_dual_g9_xyz[0] * anti_wedge_g1[0]) + (right_dual_g9_xyz[1] * anti_wedge_g1[1]) + (right_dual_g9_xyz[2] * anti_wedge_g1[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g6_xyz * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (right_dual_g9_xyz * float3(anti_wedge_g0_x)) + (right_dual_g6_xyz.yzx * anti_wedge_g1.zxy) + (other.group7.yzx * anti_wedge_g3.zxy) - (right_dual_g7 * float3(anti_wedge_g2)) - (right_dual_g6_xyz.zxy * anti_wedge_g1.yzx) - (other.group7.zxy * anti_wedge_g3.yzx)), anti_wedge_g0_x * right_dual_g9_w), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public FlatPoint reject_via_origin_from(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(0.0, 0.0, other.e4125 * other.e4125 * this.e35, 0.0) + float4(((float3(other.e4235) * float3(this.e25 * other.e4315, this.e15 * other.e4315, this.e15 * other.e4125)) + (float3(other.e4125) * float3(this.e35 * other.e4235, this.e35 * other.e4315, this.e25 * other.e4315)) + (float3(this.e45 * other.e3215) * other.group0.xyz) + float3((other.group0.xy * other.group0.xy * this.group0.xy), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public Dipole reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = float4(other.e1234 * -1.0) * this.group0;
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(anti_wedge_g0[3])) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_dual_g0_xyz.zxy, other.e3215)) + float4(-(right_dual_g0_xyz.yzx * anti_wedge_g0.zxy), anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * anti_wedge_g0.xyz) - (right_dual_g0_xyz * float3(anti_wedge_g1))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g1 = float4(float3(0.0), this.e45 * other.e12345);
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g2_w = -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_dual_g0 * float4(anti_wedge_g2_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0[3])) + (right_dual_g1 * float4(anti_wedge_g2_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0[3])) + (float3(anti_wedge_g2_w) * other.group2.xyz)), anti_wedge_g2_w * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g2_w) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g0.zxy)), anti_wedge_g2_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension FlatPoint {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = float4(other.e1234 * -1.0) * this.group0;
        float anti_wedge_g1 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g0[3])) - (float3(other.e1234) * anti_wedge_g0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g0.yzxw * float4(right_dual_g3_xyz.zxy, other.e3215)) + float4(-(right_dual_g3_xyz.yzx * anti_wedge_g0.zxy), anti_wedge_g1 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g0.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g1))), anti_wedge_g0[0] * other.e41 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e31) - (anti_wedge_g0[1] * other.e23), 0.0) + float4(((float3(anti_wedge_g1) * other.group0.xyz) + float3(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0) - (float3(anti_wedge_g0[3]) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Flector> operator /(Flector lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Sphere reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = other.e41 * this.e4235;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e45 * this.e3215)) + (right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group1.xyz);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group1.zxyx * float4(other.group2.yzx, other.e1)) + float4(-(other.group2.zx * this.group1.yz), other.e315 * this.e4235 * -1.0, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45));
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g2_xyz * float4(anti_wedge_g2[3]).xyz), -(anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g2[3] * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g1[3])) + (float3(anti_wedge_g2[3]) * other.group3.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g2.zxy) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g2.yzx)), anti_wedge_g2[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(AntiFlatPoint other) {
        float4 anti_wedge_g0 = float4(other.e321 * -1.0) * float4(this.group1.xyz, this.e45);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0.xyz * float4(right_dual_g0[3]).xyz), -(anti_wedge_g0[0] * right_dual_g0[0]) - (anti_wedge_g0[1] * right_dual_g0[1]) - (anti_wedge_g0[2] * right_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(AntiFlector other) {
        float anti_wedge_g0_w = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(anti_wedge_g0_w), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g0_w) * other.group1.xyz) - (float3(right_dual_g0[3] * other.e321) * this.group1.xyz)), anti_wedge_g0_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Plane reject_via_origin_from(AntiLine other) {
        float4 anti_wedge_g0 = float4(((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy)), 0.0);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0.yzx * anti_wedge_g0.zxy) - (right_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4((other.group0.zx * this.group1.yz), other.e31 * this.e4235, (other.e3215 * this.e45) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) - (this.group1.zxyx * float4(other.group0.yzx, other.e15));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * float4(other.e3215).xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, anti_wedge_g1[1] * other.e23 * -1.0, 0.0) + (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e15)) + float4(-(anti_wedge_g1.zx * other.group0.yz), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Plane reject_via_origin_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, other.e3 * other.e3 * this.e4125, 0.0) + float4(((float3(other.e1) * float3(other.e2 * this.e4315, other.e2 * this.e4235, other.e3 * this.e4235)) + (float3(other.e3) * float3(other.e1 * this.e4125, other.e2 * this.e4125, other.e2 * this.e4315)) + float3((other.group0.xy * other.group0.xy * this.group1.xy), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group1.xyz);
        float3 anti_wedge_g2_xyz = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group1.zxy) - (other.group2.zxy * this.group1.yzx);
        float3 right_dual_g1_xyz = other.group1.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * right_dual_g1_xyz[0]) - (anti_wedge_g0[1] * right_dual_g1_xyz[1]) - (anti_wedge_g0[2] * right_dual_g1_xyz[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group2.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz * float3(other.e321)) - (anti_wedge_g0.zxy * other.group2.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float anti_wedge_g0_w = other.e321 * this.e45 * -1.0;
        float4 anti_wedge_g1 = float4((float3(this.e3215) * other.group0.xyz), (other.e12345 * this.e45) - (other.e425 * this.e4315) - (other.e435 * this.e4125)) - (other.group1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = float4(0.0, 0.0, (other.e235 * this.e4315) - (other.e315 * this.e4235), 0.0) + float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e3215) * other.group1.xyz) + float3(((other.group2.yz * this.group1.zx) - (other.group2.zx * this.group1.yz)), 0.0)), 0.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group1;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g2_w)) + (right_dual_g1 * float4(anti_wedge_g0_w)), 
            /* e15, e25, e35, e1234 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g0_w)) + (float3(right_dual_g2_w) * anti_wedge_g2.xyz)), right_dual_g2_w * anti_wedge_g2[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_dual_g2_w) * anti_wedge_g3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g2.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g2.yzx)), right_dual_g2_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Sphere reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = other.e41 * this.e4235;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (right_dual_g0 * float3(other.e45 * this.e3215)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(other.e1234 * -1.0) * this.group1.xyz;
        float2 anti_wedge_g1_xy = (other.group3.yz * this.group1.zx) - (other.group3.zx * this.group1.yz);
        float anti_wedge_g1_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        float anti_wedge_g1_w = other.e1234 * this.e3215 * -1.0;
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group3.xyz);
        float anti_wedge_g2_w = (other.e41 * this.e4235) - (other.e1234 * this.e45);
        float4 anti_wedge_g3 = float4((float3((other.group1.zx * this.group1.yz), other.e31 * this.e4235) - (float3(this.e3215) * other.group0.xyz)), (other.e4235 * this.e15) - (other.e45 * this.e3215) - (other.e35 * this.e4125)) - (other.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(other.group1.yzx, other.e15));
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(anti_wedge_g2_w) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.zx * other.group3.yz), anti_wedge_g3[1] * other.e4235, anti_wedge_g2_w * other.e3215) - (anti_wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) - (anti_wedge_g1_w * other.e1234) - (other.e41 * anti_wedge_g3[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g1_z * other.e4315) - (anti_wedge_g2_w * other.e15), -(anti_wedge_g2_w * other.e25) - (anti_wedge_g1_xy[0] * other.e4125), -(anti_wedge_g2_w * other.e35) - (anti_wedge_g1_xy[1] * other.e4235), (anti_wedge_g3[0] * other.e15) - (anti_wedge_g2_xyz[0] * other.e4235) - (anti_wedge_g2_xyz[1] * other.e4315) - (anti_wedge_g2_xyz[2] * other.e4125)) + (float4(anti_wedge_g1_xy[1], anti_wedge_g1_z, anti_wedge_g1_xy[0], anti_wedge_g1_w) * other.group3.zxyw) + (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e35)) + (anti_wedge_g3.wwwy * float4(other.group0.xyz, other.e25)) + float4(((anti_wedge_g0 * float3(other.e3215)) - (anti_wedge_g2_xyz * float3(other.e1234))), 0.0) - (anti_wedge_g3.zxyw * other.group1.yzxw)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public AntiDualNum reject_via_origin_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((other.e45 * other.e45 * this.e3215) + (other.e15 * other.e45 * this.e4235) + (other.e25 * other.e45 * this.e4315) + (other.e35 * other.e45 * this.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (other.e4235 * this.e4315) - (other.e4315 * this.e4235), 0.0) + float4(((other.group1.yz * this.group1.zx) - (other.group1.zx * this.group1.yz)), 0.0, 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(((float3(other.e4235 * this.e15) * other.group1.xyz) - (float3(other.e15 * this.e4235) * other.group1.xyz) - (float3(other.e25 * this.e4315) * other.group1.xyz) - (float3(other.e35 * this.e4125) * other.group1.xyz) - (float3(other.e45 * this.e3215) * other.group1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e4315) - (anti_wedge_g0[1] * other.e4235), 0.0) + float4(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Plane reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = (float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4((float3((this.e4125 * other.e315) - (this.e4315 * other.e125), (this.e4235 * other.e125) - (this.e4125 * other.e235), (this.e4315 * other.e235) - (this.e4235 * other.e315)) + (float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e45 * other.e12345);
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float right_dual_g0_w = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(right_dual_g0_w), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_dual_g0_w) * anti_wedge_g1.xyz) + (float3(anti_wedge_g0[3]) * other.group0.xyz)), right_dual_g0_w * anti_wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = (this.group1.yzxx * float4(other.group5.zxy, other.e41)) + float4((-(float3(this.e3215) * other.group4.xyz) - (float3(other.e1234) * this.group0.xyz) - (other.group5.yzx * this.group1.zxy)), this.e45 * other.e1234 * -1.0);
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45);
        float4 anti_wedge_g3 = float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e3215) * other.group6.xyz) + (other.group8.yzx * this.group1.zxy) - (other.group8.zxy * this.group1.yzx)), other.e12345 * this.e45);
        float3 anti_wedge_g4 = (other.group7.zxy * this.group1.yzx) - (other.group7.yzx * this.group1.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group1.xyz);
        float2 anti_wedge_g6_xy = (this.group1.zx * other.group9.yz) - (this.group1.yz * other.group9.zx);
        float anti_wedge_g6_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float anti_wedge_g6_w = this.e3215 * other.e1234 * -1.0;
        float3 anti_wedge_g7 = float3(other.e1234 * -1.0) * this.group1.xyz;
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group1;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) - (anti_wedge_g6_w * right_dual_g3[3]) - (anti_wedge_g6_z * right_dual_g5[2]) - (anti_wedge_g6_xy[0] * right_dual_g5[0]) - (anti_wedge_g6_xy[1] * right_dual_g5[1]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) + float3((anti_wedge_g6_xy * float2(right_dual_g0[0])), anti_wedge_g6_z * right_dual_g0[0]) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), (anti_wedge_g0_x * other.e45) + (anti_wedge_g6_w * right_dual_g0[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((float2(anti_wedge_g6_z, anti_wedge_g6_xy[0]) * right_dual_g1.yz) - (float2(anti_wedge_g6_xy[1], anti_wedge_g6_z) * right_dual_g1.zx)), (anti_wedge_g6_xy[1] * right_dual_g1[0]) - (anti_wedge_g6_xy[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g6_w * other.e3215) + (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2])), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6_w * right_dual_g1[3]) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Flector reject_via_origin_from(Plane other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e4315 * other.e4235) - (this.e4235 * other.e4315), 0.0) + float4(((this.group1.zx * other.group0.yz) - (this.group1.yz * other.group0.zx)), 0.0, 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e4235 * other.e4235 * this.e15, this.e15 * other.e4235 * other.e4315, this.e15 * other.e4235 * other.e4125, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0.yz * other.group0.zx), anti_wedge_g0[0] * other.e4315, other.e4235 * other.e4235 * this.e3215) - (other.group0.yzxx * float4(anti_wedge_g0.zxy, this.e4235 * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public Sphere reject_via_origin_from(RoundPoint other) {
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
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_via_origin_from(Sphere other) {
        float2 anti_wedge_g1_xy = (this.group1.zx * other.group0.yz) - (this.group1.yz * other.group0.zx);
        float anti_wedge_g1_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float anti_wedge_g1_w = this.e3215 * other.e1234 * -1.0;
        float anti_wedge_g2_w = this.e45 * other.e1234 * -1.0;
        float3 anti_wedge_g3_xyz = float3(other.e1234 * -1.0) * this.group0.xyz;
        float anti_wedge_g3_w = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(anti_wedge_g2_w)) - (anti_wedge_g3_xyz * float3(other.e1234)), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g0_xyz.yzx)), (anti_wedge_g2_w * other.e3215) - (anti_wedge_g3_w * other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g3_xyz * float3(other.e3215)) - (right_dual_g0_xyz * float3(anti_wedge_g3_w))), anti_wedge_g1_w * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e3215 * other.e1234) * other.group0.xyz) + (float3(anti_wedge_g1_z, anti_wedge_g1_xy[0], anti_wedge_g1_xy[1]) * right_dual_g0_xyz.yzx) - (float3(anti_wedge_g1_xy[1], anti_wedge_g1_z, anti_wedge_g1_xy[0]) * right_dual_g0_xyz.zxy) - float3(2.0)*(float3(other.e3215 * other.e1234) * this.group1.xyz)), anti_wedge_g1_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = (this.group1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), -(this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (other.group0.yzxx * float4(this.group1.zxy, this.e15));
        float4 anti_wedge_g1 = (other.group0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group1.xyzx * other.group1.wwwx);
        float4 anti_wedge_g2 = float4(0.0, 0.0, (this.e4315 * other.e235) - (this.e4235 * other.e315), 0.0) + float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group0.xyz) + float3(((this.group1.zx * other.group2.yz) - (this.group1.yz * other.group2.zx)), 0.0)), 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w) * anti_wedge_g0.xyz)), right_dual_g0_w * anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_dual_g0_w)) + (right_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(right_dual_g0_xyz[0] * anti_wedge_g1[0]) - (right_dual_g0_xyz[1] * anti_wedge_g1[1]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g0[1] * right_dual_g2[2]) - (anti_wedge_g0[2] * right_dual_g2[1]), (anti_wedge_g0[2] * right_dual_g2[0]) - (anti_wedge_g0[0] * right_dual_g2[2]), (anti_wedge_g0[0] * right_dual_g2[1]) - (anti_wedge_g0[1] * right_dual_g2[0])) + (float3(anti_wedge_g0[3]) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group1.xyz) + (right_dual_g0_xyz.yzx * anti_wedge_g2.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g2.yzx)), right_dual_g0_w * this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Flector {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = float3(other.e1234 * -1.0) * this.group1.xyz;
        float2 anti_wedge_g1_xy = (this.group1.zx * other.group3.yz) - (this.group1.yz * other.group3.zx);
        float anti_wedge_g1_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float anti_wedge_g1_w = this.e3215 * other.e1234 * -1.0;
        float4 anti_wedge_g2 = (this.group1.xyzx * float4(float3(other.e3215), other.e41)) + float4(-(float3(this.e3215) * other.group3.xyz), this.e45 * other.e1234 * -1.0);
        float4 anti_wedge_g3 = float4((this.group1.yz * other.group1.zx), this.e4235 * other.e31, (this.e15 * other.e4235) - (this.e3215 * other.e45)) - (this.group1.zxyy * float4(other.group1.yzx, other.e25)) - (this.group1.wwwz * float4(other.group0.xyz, other.e35)) - (other.group2.wwwx * float4(this.group0.xyz, this.e4235));
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g2[3])) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3_xyz.zxy * anti_wedge_g3.yzx), anti_wedge_g2[3] * other.e3215) - (anti_wedge_g3.zxyw * float4(right_dual_g3_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g3.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g3[3]))), (anti_wedge_g2[3] * other.e45) - (anti_wedge_g1_w * other.e1234) - (anti_wedge_g0[0] * right_dual_g3_xyz[0]) - (anti_wedge_g0[1] * right_dual_g3_xyz[1]) - (anti_wedge_g0[2] * right_dual_g3_xyz[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(anti_wedge_g3[3]) * other.group0.xyz) + (float3(anti_wedge_g1_z, anti_wedge_g1_xy[0], anti_wedge_g1_xy[1]) * right_dual_g3_xyz.yzx) + float3(((anti_wedge_g3.yz * other.group1.zx) - (anti_wedge_g3.zx * other.group1.yz)), (anti_wedge_g3[0] * other.e31) - (anti_wedge_g3[1] * other.e23)) - (float3(anti_wedge_g2[3]) * other.group2.xyz) - (float3(other.e1234) * anti_wedge_g2.xyz) - (float3(anti_wedge_g1_xy[1], anti_wedge_g1_z, anti_wedge_g1_xy[0]) * right_dual_g3_xyz.zxy)), (anti_wedge_g1_w * other.e3215) + (right_dual_g3_xyz[0] * anti_wedge_g2[0]) + (right_dual_g3_xyz[1] * anti_wedge_g2[1]) + (right_dual_g3_xyz[2] * anti_wedge_g2[2]))
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Line> operator /(Line lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group0.xyz) + (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float anti_wedge_g1 = -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e321)) + (float3(anti_wedge_g1) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx)), anti_wedge_g0_xyz[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g0_xyz = float3(other.e321) * this.group0.xyz;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            anti_wedge_g0_xyz * float3(other.e321), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g0_xyz.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Motor reject_via_origin_from(AntiFlector other) {
        float3 anti_wedge_g0_xyz = float3(other.e321) * this.group0.xyz;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float4(other.e321).xyz), (anti_wedge_g0_xyz[0] * right_dual_g1_xyz[0]) + (anti_wedge_g0_xyz[1] * right_dual_g1_xyz[1]) + (anti_wedge_g0_xyz[2] * right_dual_g1_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(AntiLine other) {
        float anti_wedge_g0 = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Motor reject_via_origin_from(AntiMotor other) {
        float anti_wedge_g0_w = -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_wedge_g0_w) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0_w) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Line(
            /* e415, e425, e435 */
            float3(right_dual_g0 * other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0 * other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Circle reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0_xyz = (float3(other.e321) * this.group0.xyz) + (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float anti_wedge_g1 = -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e321)) + (float3(anti_wedge_g1) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0_xyz.yzx * other.group2.zxy) - (anti_wedge_g0_xyz.zxy * other.group2.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float3 anti_wedge_g3_xyz = (float3(other.e321) * this.group0.xyz) + (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(anti_wedge_g2_w) * other.group1.xyz) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy)), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e321)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g2_w * other.e12345) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2_w * other.e12345) * this.group1.xyz) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g2_w)), anti_wedge_g2_w * right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Circle reject_via_origin_from(Dipole other) {
        float anti_wedge_g0 = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float anti_wedge_g1_w = -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.zxy * other.group3.yzx) - (anti_wedge_g0.yzx * other.group3.zxy)), anti_wedge_g1_w * other.e45 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(other.e1234)) + (float3(anti_wedge_g1_w) * other.group3.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.yzx * other.group3.zxy) - (anti_wedge_g2_xyz.zxy * other.group3.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Motor reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0_xyz = (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group1.zxy) - (this.group1.zxy * other.group1.yzx);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * other.group1.zxy) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(RejectViaOriginFromInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public AntiFlatPoint reject_via_origin_from(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((-(float3(other.e235 * this.e415) * other.group0.xyz) - (float3(other.e315 * this.e425) * other.group0.xyz) - (float3(other.e125 * this.e435) * other.group0.xyz) - (other.group0.xyz * other.group0.xyz * this.group1.xyz) - (float3(other.e415) * other.group0.yyz * this.group1.yxx) - (float3(other.e435) * other.group0.xyy * this.group1.zzy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Motor reject_via_origin_from(Motor other) {
        float3 anti_wedge_g0_xyz = float3(other.e12345) * this.group0.xyz;
        float anti_wedge_g1_w = -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float3(right_dual_g0_w)), -(anti_wedge_g0_xyz[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1_w)) + (float3(right_dual_g0_w * other.e12345) * this.group1.xyz)), anti_wedge_g1_w * right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43);
        float4 anti_wedge_g1 = float4(((float3(other.e321) * this.group0.xyz) + (this.group1.yzx * other.group7.zxy) - (this.group1.zxy * other.group7.yzx)), 0.0);
        float anti_wedge_g2 = -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4(((float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group9.zxy) - (this.group1.zxy * other.group9.yzx)), 0.0);
        float3 anti_wedge_g4 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(other.e1234) * this.group1.xyz;
        float4 anti_wedge_g6 = float4(1.0, 1.0, other.e12345, 0.0) * float4((this.group0.xyz * float3(float2(other.e12345), 1.0)), 0.0);
        float3 anti_wedge_g8 = float3(other.e12345) * this.group1.xyz;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(RejectViaOriginFromInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Line reject_via_origin_from(Plane other) {
        float3 anti_wedge_g0_xyz = (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        return Line(
            /* e415, e425, e435 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * other.group0.zxy) - (anti_wedge_g0_xyz.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(RejectViaOriginFromInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public Circle reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g0_xyz.yzx), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g2 * float3(other.e1234)) + (right_dual_g0_xyz * float3(this.e415 * other.e4235)) + (right_dual_g0_xyz * float3(this.e425 * other.e4315)) + (right_dual_g0_xyz * float3(this.e435 * other.e4125))), 0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g2.yzx * right_dual_g0_xyz.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float3 anti_wedge_g1_xyz = float3(other.e12345) * this.group0.xyz;
        float3 anti_wedge_g2_xyz = float3(other.e12345) * this.group1.xyz;
        float anti_wedge_g2_w = -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412);
        float3 anti_wedge_g3_xyz = (float3(other.e321) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (anti_wedge_g3_xyz.zxy * right_dual_g0_xyz.yzx) - (anti_wedge_g3_xyz.yzx * right_dual_g0_xyz.zxy)), (anti_wedge_g3_xyz[0] * other.e1) - (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2]) - (anti_wedge_g2_xyz[0] * right_dual_g0_xyz[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0_xyz[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0_xyz[2])), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g0_w)) + (anti_wedge_g3_xyz * float3(other.e321)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w))), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0_w)) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), 0.0), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g3_xyz * float3(right_dual_g0_w)), anti_wedge_g2_w * right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Line {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = float3(other.e1234) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(other.e1234)) + (anti_wedge_g0.yzx * right_dual_g3_xyz.zxy) - (anti_wedge_g0.zxy * right_dual_g3_xyz.yzx)), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(other.e1234)) + (right_dual_g3_xyz * float3(this.e415 * other.e4235)) + (right_dual_g3_xyz * float3(this.e425 * other.e4315)) + (right_dual_g3_xyz * float3(this.e435 * other.e4125))), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.zxy * right_dual_g3_xyz.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g3_xyz.zxy)), 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Motor> operator /(Motor lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w * -1.0) * other.group2.xyz), (anti_wedge_g2_w * other.scalar) + (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float3 anti_wedge_g3_xyz = (float3(other.e321) * this.group0.xyz) + (float3(this.e12345) * other.group3.xyz) + (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        float anti_wedge_g3_w = other.e5 * this.e12345;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e321)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(anti_wedge_g3_w) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3_w)) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), (anti_wedge_g3_xyz[0] * other.e1) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public DualNum reject_via_origin_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g1_xyz = float3(other.e321) * this.group0.xyz;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e321) * float4(anti_wedge_g1_xyz, other.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(AntiFlector other) {
        float3 anti_wedge_g1_xyz = (float3(other.e321) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g1_xyz * float3(right_dual_g0_w * -1.0)), (anti_wedge_g1_xyz[0] * right_dual_g1_xyz[0]) + (anti_wedge_g1_xyz[1] * right_dual_g1_xyz[1]) + (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2]) - (right_dual_g0_w * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(AntiLine other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_dual_g0 * float3(anti_wedge_g0_w)), -(anti_wedge_g0_xyz[0] * right_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group0.xyz), (anti_wedge_g0_w * other.scalar) + (anti_wedge_g0_xyz[0] * other.e23) + (anti_wedge_g0_xyz[1] * other.e31) + (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public AntiScalar reject_via_origin_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual_g0 * other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public CircleRotor reject_via_origin_from(Circle other) {
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435);
        float3 anti_wedge_g3_xyz = (float3(other.e321) * this.group0.xyz) + (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e321)) + (float3(anti_wedge_g2_w) * other.group2.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g3_xyz.yzx * other.group2.zxy) - (anti_wedge_g3_xyz.zxy * other.group2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float anti_wedge_g2_w = other.e12345 * this.e5;
        float4 anti_wedge_g3 = float4(((float3(other.e321) * this.group0.xyz) + (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy)), 0.0);
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g2_w * this.e12345) * other.group0.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx)), right_dual_g2_w * other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(anti_wedge_g2_w) * other.group0.xyz) + (float3(other.e321) * anti_wedge_g3.xyz) + (float3(right_dual_g2_w * other.e12345) * this.group0.xyz) + (float3(right_dual_g2_w * this.e12345) * other.group1.xyz)), right_dual_g2_w * other.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g2_w * other.e12345) * this.group1.xyz) + (float3(right_dual_g2_w * this.e12345) * other.group2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), anti_wedge_g2_w * right_dual_g2_w), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public CircleRotor reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((right_dual_g2 * float3(anti_wedge_g2_w)), -(anti_wedge_g0[0] * right_dual_g2[0]) - (anti_wedge_g0[1] * right_dual_g2[1]) - (anti_wedge_g0[2] * right_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0_xyz = (float3(other.e1234) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz);
        float anti_wedge_g0_w = other.e1234 * this.e5;
        float3 anti_wedge_g1_xyz = (float3(other.e1234) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz);
        float anti_wedge_g1_w = other.e45 * this.e12345;
        float3 anti_wedge_g2_xyz = float3((other.e4125 * this.e315) - (other.e4315 * this.e125), (other.e4235 * this.e125) - (other.e4125 * this.e235), (other.e4315 * this.e235) - (other.e4235 * this.e315)) + (float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group2.wwwz * float4(anti_wedge_g1_xyz, anti_wedge_g0_xyz[2])) + float4(((right_dual_g0 * float3(anti_wedge_g0_w)) + (anti_wedge_g0_xyz.zxy * other.group3.yzx) - (anti_wedge_g0_xyz.yzx * other.group3.zxy)), (anti_wedge_g0_xyz[0] * other.e15) + (anti_wedge_g0_xyz[1] * other.e25) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g1_w * other.e45) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g3[0] * other.e4235) - (anti_wedge_g3[1] * other.e4315)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g0_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz * float3(other.e1234)) + (float3(anti_wedge_g1_w) * other.group3.xyz) - (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.yzx * other.group3.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g2_xyz.zxy * other.group3.yzx)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0, 
            /* e235, e315, e125, e5 */
            float4((this.group1.xyz * float3(float2(other.e12345 * other.e12345 * -1.0), other.e12345 * other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e12345 * other.e12345 * this.e5) - (other.e5 * other.e12345 * this.e12345))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public AntiScalar reject_via_origin_from(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * other.e45 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0_xyz = float3((other.e4125 * this.e315) - (other.e4315 * this.e125), (other.e4235 * this.e125) - (other.e4125 * this.e235), (other.e4315 * this.e235) - (other.e4235 * this.e315)) + (float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz);
        float anti_wedge_g0_w = other.e45 * this.e12345;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), -(anti_wedge_g0_w * other.e45) - (anti_wedge_g1[0] * other.e4235) - (anti_wedge_g1[1] * other.e4315) - (anti_wedge_g1[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * other.group1.zxy) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((-(float3(other.e235 * this.e415) * other.group0.xyz) - (float3(other.e315 * this.e425) * other.group0.xyz) - (float3(other.e125 * this.e435) * other.group0.xyz) - (other.group0.xyz * other.group0.xyz * this.group1.xyz) - (float3(other.e415) * other.group0.yyz * this.group1.yxx) - (float3(other.e435) * other.group0.xyy * this.group1.zzy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Motor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float3(right_dual_g0_w)), (right_dual_g0_w * other.e12345 * this.e12345) - (anti_wedge_g0_xyz[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0_xyz[2])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1[3])) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz)), right_dual_g0_w * anti_wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) + (this.e5 * other.e1234) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 anti_wedge_g1 = float4(((float3(this.e12345) * other.group1.xyz) + (float3(other.e321) * this.group0.xyz) + (other.group7.zxy * this.group1.yzx) - (other.group7.yzx * this.group1.zxy)), this.e12345 * other.e4);
        float anti_wedge_g2 = (other.e12345 * this.e5) + (this.e12345 * other.e5) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float4 anti_wedge_g3 = float4((float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(this.e12345) * other.group3.xyz) + (float3(other.e3215) * this.group0.xyz)), this.e12345 * other.e45);
        float3 anti_wedge_g4 = (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group0.xyz);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group5.xyz) + (float3(other.e1234) * this.group1.xyz);
        float3 anti_wedge_g6_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group6.xyz);
        float anti_wedge_g6_w = this.e12345 * other.e321;
        float3 anti_wedge_g7 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g8 = (float3(other.e12345) * this.group1.xyz) + (float3(this.e12345) * other.group8.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = this.e12345 * other.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) + (right_dual_g0[0] * other.e12345 * this.e12345) - (anti_wedge_g6_w * right_dual_g3[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g6_xyz[0] * right_dual_g5[0]) - (anti_wedge_g6_xyz[1] * right_dual_g5[1]) - (anti_wedge_g6_xyz[2] * right_dual_g5[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (float4(right_dual_g0[0]) * float4(anti_wedge_g6_xyz, anti_wedge_g6_w)) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(anti_wedge_g0_x)) + (right_dual_g1.yzxy * float4(anti_wedge_g6_xyz.zxy, anti_wedge_g8[1])) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (float3(right_dual_g0[0]) * anti_wedge_g9.xyz) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (anti_wedge_g6_xyz.yzx * right_dual_g1.zxy) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g6_w * other.e3215) + (right_dual_g0[0] * anti_wedge_g9[3]) + (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[2] * right_dual_g1[2])), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6_w * right_dual_g1[3]) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public Motor reject_via_origin_from(Plane other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (this.e235 * other.e4315) - (this.e315 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0)), 0.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz), -(anti_wedge_g1[0] * other.e4235) - (anti_wedge_g1[1] * other.e4315) - (anti_wedge_g1[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e4315) - (anti_wedge_g0[1] * other.e4235), 0.0) + float4(((anti_wedge_g0.yz * other.group0.zx) - (anti_wedge_g0.zx * other.group0.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public AntiScalar reject_via_origin_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g0[3] * other.e5) - (this.e12345 * other.e4 * other.e5)
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
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public VersorEven reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = float4(other.e1234) * float4(this.group0.xyz, this.e5);
        float3 anti_wedge_g1_xyz = float3(other.e1234) * this.group1.xyz;
        float3 anti_wedge_g2_xyz = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(other.e3215) * this.group0.xyz);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual_g0.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(((anti_wedge_g1_xyz * float3(right_dual_g0[3])) + float3(-(anti_wedge_g0.zx * right_dual_g0.yz), anti_wedge_g0[1] * right_dual_g0[0] * -1.0)), anti_wedge_g3[0] * right_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0[3])) + (float3(other.e3215) * anti_wedge_g0.xyz) + (float3(this.e415 * other.e4235) * right_dual_g0.xyz) + (float3(this.e425 * other.e4315) * right_dual_g0.xyz) + (float3(this.e435 * other.e4125) * right_dual_g0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g0.zxy)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float4 anti_wedge_g2 = (float4(this.e12345) * other.group2) + (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435));
        float4 anti_wedge_g3 = float4((float3((this.e315 * other.e412) - (this.e125 * other.e431), (this.e125 * other.e423) - (this.e235 * other.e412), (this.e235 * other.e431) - (this.e315 * other.e423)) + (float3(this.e12345) * other.group3.xyz) + (float3(other.e321) * this.group0.xyz)), this.e12345 * other.e4);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_dual_g0_w)) + (anti_wedge_g3.zxyx * float4(right_dual_g0_xyz.yzx, other.e1)) + float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) - (right_dual_g0_xyz.zxy * anti_wedge_g3.yzx)), -(right_dual_g2_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g2[3] * other.e4)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2[3])) + (right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(other.e321) * anti_wedge_g3.xyz) + (float3(right_dual_g0_w * this.e12345) * other.group1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group0.xyz)), right_dual_g0_w * this.e12345 * other.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2[3])) + (float3(right_dual_g0_w) * anti_wedge_g2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), right_dual_g0_w * anti_wedge_g2[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Motor {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = (this.group0 * float4(float3(other.e1234), other.scalar)) + float4((float3(this.e12345) * other.group0.xyz), (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43));
        float3 anti_wedge_g1_xyz = (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group1.xyz);
        float anti_wedge_g1_w = this.e12345 * other.e45;
        float3 anti_wedge_g2_xyz = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group0.xyz);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g3[3])) + float3(((anti_wedge_g0.yz * right_dual_g3.zx) - (anti_wedge_g0.zx * right_dual_g3.yz)), (anti_wedge_g0[0] * right_dual_g3[1]) - (anti_wedge_g0[1] * right_dual_g3[0]))), (anti_wedge_g1_xyz[0] * other.e23) + (right_dual_g3[0] * this.e12345 * other.e4235) - (anti_wedge_g1_w * other.e45) - (right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2])) - (anti_wedge_g0.wwwx * float4(other.group0.xyz, right_dual_g2_xyz[0])), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g3[3])) + (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g1_w) * right_dual_g3.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (right_dual_g2_xyz * float3(anti_wedge_g0[3])) + (anti_wedge_g2_xyz.zxy * right_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g3.zxy)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<MultiVector> operator /(MultiVector lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiCircleRotor other) {
        float anti_wedge_g0_x = (this.e12345 * other.scalar) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321);
        float3 anti_wedge_g1_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g1_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group2.xyz, other.e45);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group1.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g4[0] * other.e15) + (anti_wedge_g4[1] * other.e25) + (anti_wedge_g4[2] * other.e35) - (anti_wedge_g5[0] * right_dual_g1[0]) - (anti_wedge_g5[1] * right_dual_g1[1]) - (anti_wedge_g5[2] * right_dual_g1[2]) - (right_dual_g0[0] * anti_wedge_g3[0]) - (right_dual_g0[1] * anti_wedge_g3[1]) - (right_dual_g0[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_dual_g1[3])), 
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
            right_dual_g1 * float4(anti_wedge_g0_x), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g0_x), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x * -1.0) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e45 * this.e3215)) + (right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (anti_wedge_g1_xyz.zxy * right_dual_g1.yzx) - (float3(anti_wedge_g1_w) * other.group2.xyz) - (anti_wedge_g1_xyz.yzx * right_dual_g1.zxy)), 0.0), 
            /* e1234 */
            (anti_wedge_g1_w * right_dual_g1[3]) + (anti_wedge_g1_xyz[0] * right_dual_g0[0]) + (anti_wedge_g1_xyz[1] * right_dual_g0[1]) + (anti_wedge_g1_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiDipoleInversion other) {
        float anti_wedge_g0_x = (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (other.e321 * this.e45);
        float3 anti_wedge_g1_xyz = (float3(this.e12345) * other.group3.xyz) + (float3(other.e321) * this.group6.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group8.yzx) + (this.group7.zxy * other.group2.yzx) - (other.group0.yzx * this.group8.zxy) - (this.group7.yzx * other.group2.zxy);
        float anti_wedge_g1_w = this.e12345 * other.e4;
        float anti_wedge_g2 = (this.e12345 * other.e5) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4(0.0, 0.0, (other.e235 * this.e4315) - (other.e315 * this.e4235), 0.0) + float4(((float3(this.e3215) * other.group1.xyz) + float3(((other.group2.yz * this.group9.zx) - (other.group2.zx * this.group9.yz)), 0.0)), 0.0);
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g7 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g8 = float3(this.e12345) * other.group2.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1_w * right_dual_g3_w) + (anti_wedge_g2 * right_dual_g2_w) + (anti_wedge_g1_xyz[0] * right_dual_g3_xyz[0]) + (anti_wedge_g1_xyz[1] * right_dual_g3_xyz[1]) + (anti_wedge_g1_xyz[2] * right_dual_g3_xyz[2]) - (right_dual_g1_w * anti_wedge_g6[3]) - (anti_wedge_g7[0] * right_dual_g2_xyz[0]) - (anti_wedge_g7[1] * right_dual_g2_xyz[1]) - (anti_wedge_g7[2] * right_dual_g2_xyz[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g6[0]) - (right_dual_g1_xyz[1] * anti_wedge_g6[1]) - (right_dual_g1_xyz[2] * anti_wedge_g6[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0_x) * float4(right_dual_g2_xyz, right_dual_g1_w), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e23, e31, e12 */
            right_dual_g1_xyz * float3(anti_wedge_g0_x), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g1_w)) + (float3(anti_wedge_g2) * other.group0.xyz) - (anti_wedge_g1_xyz * float3(right_dual_g1_w))), 0.0), 
            /* e423, e431, e412 */
            (right_dual_g1_xyz * float3(anti_wedge_g1_w)) + (anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy), 
            /* e235, e315, e125 */
            (right_dual_g1_xyz * float3(anti_wedge_g2)) + (anti_wedge_g1_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g2_xyz.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g5 * float3(right_dual_g1_w)) + (right_dual_g1_xyz * float3(anti_wedge_g3[3])) + (right_dual_g3_xyz * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g3.zxy) - (anti_wedge_g4.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g3.yzx)), anti_wedge_g0_x * right_dual_g3_w), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g2_w) - (anti_wedge_g4[0] * right_dual_g1_xyz[0]) - (anti_wedge_g4[1] * right_dual_g1_xyz[1]) - (anti_wedge_g4[2] * right_dual_g1_xyz[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiDualNum other) {
        float anti_wedge_g0_x = (other.e3215 * this.e4) + (other.scalar * this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, anti_wedge_g0_x * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, other.e3215 * other.e3215, 0.0) * float4((this.group4.xyz * float3(float2(other.e3215 * other.e3215), 1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215 * other.e3215) * this.group7.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e1234), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g1_xyz = (float3(other.e321) * this.group6.xyz) + (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e12345 * right_dual_g0[3] * other.e321 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            -(right_dual_g0 * float4(this.e41 * other.e235)) - (right_dual_g0 * float4(this.e42 * other.e315)) - (right_dual_g0 * float4(this.e43 * other.e125)) - (right_dual_g0 * float4(other.e321 * this.e45)), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1_xyz * float4(right_dual_g0[3]).xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz.yzx * right_dual_g0.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_dual_g0[3] * this.e1234) * other.group0.xyz) - (float3(right_dual_g0[3] * other.e321) * this.group9.xyz)), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiFlector other) {
        float anti_wedge_g0_x = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (other.e321 * this.e45);
        float3 anti_wedge_g1_xyz = (float3(this.e12345) * other.group1.xyz) + (float3(other.e321) * this.group6.xyz) + (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float3 right_dual_g1_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1_xyz[0] * right_dual_g1_xyz[0]) + (anti_wedge_g1_xyz[1] * right_dual_g1_xyz[1]) + (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2]) - (this.e12345 * right_dual_g0[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(anti_wedge_g0_x), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1_xyz * float4(right_dual_g0[3]).xyz * float3(-1.0)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz.yzx * right_dual_g0.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g0_x)) + (float3(right_dual_g0[3] * this.e1234) * other.group0.xyz) - (float3(right_dual_g0[3] * other.e321) * this.group9.xyz)), anti_wedge_g0_x * other.e5 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiLine other) {
        float anti_wedge_g0_x = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 anti_wedge_g1_xyz = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = float3(this.e12345) * other.group0.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g5[0] * right_dual_g0[0]) - (anti_wedge_g5[1] * right_dual_g0[1]) - (anti_wedge_g5[2] * right_dual_g0[2])), 
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
            float4((right_dual_g0 * float3(anti_wedge_g0_x)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x * -1.0) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g1_xyz.yzx * right_dual_g0.zxy)), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiMotor other) {
        float anti_wedge_g0_x = (this.e12345 * other.scalar) + (other.e3215 * this.e4) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435);
        float4 anti_wedge_g1 = float4(0.0, 0.0, (other.e31 * this.e4235) - (other.e23 * this.e4315), 0.0) + float4(((float3(other.e3215) * this.group4.xyz) + (float3(this.e1234) * other.group1.xyz) + float3(((other.group0.zx * this.group9.yz) - (other.group0.yz * this.group9.zx)), 0.0)), 0.0);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group0.xyz) + (float3(other.e3215) * this.group7.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * float4(other.e3215).xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0_x * -1.0) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) - (float3(anti_wedge_g0_x) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g1[1] * other.e12) - (anti_wedge_g1[2] * other.e31), (anti_wedge_g1[2] * other.e23) - (anti_wedge_g1[0] * other.e12), (anti_wedge_g1[0] * other.e31) - (anti_wedge_g1[1] * other.e23)) - (float3(anti_wedge_g1[3]) * other.group1.xyz)), other.e3215 * other.e3215 * this.e1234), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiPlane other) {
        float4 anti_wedge_g1 = float4(1.0, 1.0, this.e12345, 0.0) * float4((other.group0.xyz * float3(float2(this.e12345), 1.0)), 0.0);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1[0] * right_dual_g0[0]) + (anti_wedge_g1[1] * right_dual_g0[1]) + (anti_wedge_g1[2] * right_dual_g0[2]) + (anti_wedge_g1[3] * right_dual_g0[3])), 
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
            (right_dual_g0 * float4(other.e1 * this.e4235)) + (right_dual_g0 * float4(other.e2 * this.e4315)) + (right_dual_g0 * float4(other.e3 * this.e4125)) + (right_dual_g0 * float4(other.e5 * this.e1234)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0 * other.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e5 */
            right_dual_g0 * other.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0 * other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual_g0 * other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0 * other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0 * other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual_g0 * other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0 * other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0 * other.e12345) * this.group9, 
            /* e1234 */
            right_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Circle other) {
        float anti_wedge_g0_x = -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (other.e321 * this.e45);
        float3 anti_wedge_g1_xyz = (float3(other.e321) * this.group6.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group8.yzx) + (other.group2.yzx * this.group7.zxy) - (other.group0.yzx * this.group8.zxy) - (other.group2.zxy * this.group7.yzx);
        float anti_wedge_g2 = -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435);
        float3 anti_wedge_g3_xyz = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group9.zxy) - (other.group2.zxy * this.group9.yzx);
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g7 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g8 = float3(this.e12345) * other.group2.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual_g1_w * anti_wedge_g6[3]) - (anti_wedge_g7[0] * other.e235) - (anti_wedge_g7[1] * other.e315) - (anti_wedge_g7[2] * other.e125) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g6[0]) - (right_dual_g1_xyz[1] * anti_wedge_g6[1]) - (right_dual_g1_xyz[2] * anti_wedge_g6[2])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0_x) * float4(other.group2.xyz, right_dual_g1_w), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e23, e31, e12 */
            right_dual_g1_xyz * float3(anti_wedge_g0_x), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g2) * other.group0.xyz) - (anti_wedge_g1_xyz * float3(right_dual_g1_w))), 0.0), 
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy), 
            /* e235, e315, e125 */
            (right_dual_g1_xyz * float3(anti_wedge_g2)) + (anti_wedge_g1_xyz.yzx * other.group2.zxy) - (anti_wedge_g1_xyz.zxy * other.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g5 * float3(right_dual_g1_w)) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) + (anti_wedge_g4.yzx * other.group2.zxy) - (anti_wedge_g3_xyz.yzx * other.group0.zxy) - (anti_wedge_g4.zxy * other.group2.yzx)), 0.0), 
            /* e1234 */
            -(anti_wedge_g4[0] * right_dual_g1_xyz[0]) - (anti_wedge_g4[1] * right_dual_g1_xyz[1]) - (anti_wedge_g4[2] * right_dual_g1_xyz[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(CircleRotor other) {
        float anti_wedge_g0_x = (this.scalar * other.e12345) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (other.e321 * this.e45);
        float4 anti_wedge_g1 = float4(((float3(other.e321) * this.group6.xyz) + (float3(other.e12345) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group8.yzx) + (this.group7.zxy * other.group2.yzx) - (other.group0.yzx * this.group8.zxy) - (this.group7.yzx * other.group2.zxy)), other.e12345 * this.e4);
        float anti_wedge_g2 = (other.e12345 * this.e5) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float4 anti_wedge_g3 = float4((float3((other.e315 * this.e4125) - (other.e125 * this.e4315), (other.e125 * this.e4235) - (other.e235 * this.e4125), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + (float3(other.e12345) * this.group3.xyz) + (float3(this.e3215) * other.group1.xyz)), other.e12345 * this.e45);
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group5.xyz) + (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group6);
        float3 anti_wedge_g7 = (float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group7.xyz);
        float3 anti_wedge_g8 = (float3(this.e12345) * other.group2.xyz) + (float3(other.e12345) * this.group8.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group9;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g2_w, (right_dual_g2_w * this.e12345 * other.e12345) - (right_dual_g1_w * anti_wedge_g6[3]) - (anti_wedge_g7[0] * right_dual_g2_xyz[0]) - (anti_wedge_g7[1] * right_dual_g2_xyz[1]) - (anti_wedge_g7[2] * right_dual_g2_xyz[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g6[0]) - (right_dual_g1_xyz[1] * anti_wedge_g6[1]) - (right_dual_g1_xyz[2] * anti_wedge_g6[2])), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(right_dual_g2_w), 
            /* e5 */
            anti_wedge_g2 * right_dual_g2_w, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(right_dual_g2_w)) + (float4(anti_wedge_g0_x) * float4(right_dual_g2_xyz, right_dual_g1_w)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_dual_g2_w)) + (right_dual_g1_xyz * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g1[3])) + (float3(anti_wedge_g2) * other.group0.xyz) + (float3(right_dual_g2_w) * anti_wedge_g6.xyz) - (float3(right_dual_g1_w) * anti_wedge_g1.xyz)), right_dual_g2_w * anti_wedge_g6[3]), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(right_dual_g2_w)) + (right_dual_g1_xyz * float3(anti_wedge_g1[3])) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(right_dual_g2_w)) + (right_dual_g1_xyz * float3(anti_wedge_g2)) + (right_dual_g2_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g5 * float3(right_dual_g1_w)) + (right_dual_g1_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g2_w) * anti_wedge_g9.xyz) + (anti_wedge_g4.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g3.zxy) - (anti_wedge_g4.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g3.yzx)), right_dual_g2_w * anti_wedge_g9[3]), 
            /* e1234 */
            (right_dual_g2_w * other.e12345 * this.e1234) - (anti_wedge_g4[0] * right_dual_g1_xyz[0]) - (anti_wedge_g4[1] * right_dual_g1_xyz[1]) - (anti_wedge_g4[2] * right_dual_g1_xyz[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Dipole other) {
        float anti_wedge_g0_x = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321);
        float3 anti_wedge_g1_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g1_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group2.xyz, other.e45);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group1.xyz;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g4[0] * right_dual_g2[0]) - (anti_wedge_g4[1] * right_dual_g2[1]) - (anti_wedge_g4[2] * right_dual_g2[2]) - (anti_wedge_g5[0] * right_dual_g1[0]) - (anti_wedge_g5[1] * right_dual_g1[1]) - (anti_wedge_g5[2] * right_dual_g1[2]) - (right_dual_g0[0] * anti_wedge_g3[0]) - (right_dual_g0[1] * anti_wedge_g3[1]) - (right_dual_g0[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * right_dual_g1[3])), 
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
            right_dual_g1 * float4(anti_wedge_g0_x), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g0_x), 
            /* e235, e315, e125 */
            right_dual_g2 * float3(anti_wedge_g0_x), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (right_dual_g0 * float3(other.e45 * this.e3215)) + (right_dual_g2 * float3(anti_wedge_g1_w)) + (anti_wedge_g1_xyz.zxy * right_dual_g1.yzx) - (anti_wedge_g1_xyz.yzx * right_dual_g1.zxy)), 0.0), 
            /* e1234 */
            (anti_wedge_g1_w * right_dual_g1[3]) + (anti_wedge_g1_xyz[0] * right_dual_g0[0]) + (anti_wedge_g1_xyz[1] * right_dual_g0[1]) + (anti_wedge_g1_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(DipoleInversion other) {
        float anti_wedge_g0_x = (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + float4(((float3(other.e3215) * this.group4.xyz) + (this.group5.yzx * other.group3.zxy) + float3(((other.group1.zx * this.group9.yz) - (other.group1.yz * this.group9.zx)), (other.e31 * this.e4235) - (other.e23 * this.e4315)) - (float3(this.e3215) * other.group0.xyz)), (other.e41 * this.e4235) + (other.e42 * this.e4315) + (other.e43 * this.e4125) - (this.e41 * other.e4235) - (this.e43 * other.e4125)) - (float4(other.e1234) * this.group3) - (other.group3.yzxy * float4(this.group5.zxy, this.e42));
        float anti_wedge_g2 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125);
        float3 anti_wedge_g3_xyz = (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group6.xyz) + (this.group8.yzx * other.group3.zxy) - (this.group8.zxy * other.group3.yzx);
        float anti_wedge_g3_w = this.e12345 * other.e45;
        float3 anti_wedge_g4 = (float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group6.xyz) + (this.group7.zxy * other.group3.yzx) - (this.group7.yzx * other.group3.zxy);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group3.xyz);
        float4 anti_wedge_g6 = (other.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group3;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g4[0] * other.e15) + (anti_wedge_g4[1] * other.e25) + (anti_wedge_g4[2] * other.e35) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g9[3] * other.e1234) + (this.e12345 * other.e1234 * other.e3215) - (anti_wedge_g3_w * other.e45) - (anti_wedge_g3_xyz[0] * right_dual_g0[0]) - (anti_wedge_g3_xyz[1] * right_dual_g0[1]) - (anti_wedge_g3_xyz[2] * right_dual_g0[2]) - (anti_wedge_g9[0] * other.e4235) - (anti_wedge_g9[1] * other.e4315) - (anti_wedge_g9[2] * other.e4125)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * float4((other.group3.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(((float3(anti_wedge_g2) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.e3215), 
            /* e41, e42, e43 */
            -(float3(anti_wedge_g1[3]) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[2] * other.e4315) - (anti_wedge_g1[1] * other.e4125), (anti_wedge_g1[0] * other.e4125) - (anti_wedge_g1[2] * other.e4235), (anti_wedge_g1[1] * other.e4235) - (anti_wedge_g1[0] * other.e4315)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e1234)) + (anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g3_w) * other.group3.xyz) - (float3(anti_wedge_g0_x) * other.group1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e1234)) + (right_dual_g0 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.zxy * other.group3.yzx) - (anti_wedge_g4.yzx * other.group3.zxy), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g3_xyz.yzx * other.group3.zxy) - (float3(anti_wedge_g0_x) * other.group2.xyz) - (anti_wedge_g3_xyz.zxy * other.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e25)) + float4(((anti_wedge_g7 * float3(other.e3215)) + float3(((anti_wedge_g6.yz * other.group3.zx) - (anti_wedge_g1.zx * other.group1.yz) - (anti_wedge_g6.zx * other.group3.yz)), (anti_wedge_g6[0] * other.e4315) - (anti_wedge_g1[1] * other.e23) - (anti_wedge_g6[1] * other.e4235)) - (anti_wedge_g8 * float3(other.e1234)) - (float3(anti_wedge_g1[3]) * other.group2.xyz)), (anti_wedge_g1[0] * other.e15) + (anti_wedge_g1[2] * other.e35) - (anti_wedge_g8[0] * other.e4235) - (anti_wedge_g8[1] * other.e4315) - (anti_wedge_g8[2] * other.e4125)) - (float4(anti_wedge_g2) * float4(right_dual_g0, other.e45)), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e4235) + (anti_wedge_g7[1] * other.e4315) + (anti_wedge_g7[2] * other.e4125) + (right_dual_g0[0] * anti_wedge_g1[0]) + (right_dual_g0[1] * anti_wedge_g1[1]) + (right_dual_g0[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6[3] * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(DualNum other) {
        float anti_wedge_g0_x = (other.e5 * this.e1234) + (other.e12345 * this.scalar);
        float4 anti_wedge_g1 = float4(other.e12345) * this.group1;
        float4 anti_wedge_g9 = float4(other.e12345) * this.group9;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.e12345 * -1.0, -(other.e5 * anti_wedge_g1[3]) - (other.e12345 * other.e12345 * this.e12345)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.e12345 * -1.0), 
            /* e5 */
            -(other.e12345 * other.e12345 * this.e5) - (other.e5 * other.e12345 * this.e12345), 
            /* e15, e25, e35, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group3, 
            /* e41, e42, e43 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group6, 
            /* e423, e431, e412 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345 * other.e12345 * -1.0) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g9.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(anti_wedge_g0_x * other.e5) - (other.e12345 * anti_wedge_g9[3])), 
            /* e1234 */
            other.e12345 * other.e12345 * this.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(FlatPoint other) {
        float anti_wedge_g0_x = -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e45 * this.e321);
        float4 anti_wedge_g1 = float4(this.e1234) * other.group0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e45 * other.e45 * this.e12345 * -1.0), 
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
            float4(float3(0.0), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x * -1.0) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float4(anti_wedge_g1[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e15) + (anti_wedge_g1[1] * other.e25) + (anti_wedge_g1[2] * other.e35) + (other.e45 * other.e45 * this.e3215) + (other.e15 * other.e45 * this.e4235) + (other.e25 * other.e45 * this.e4315) + (other.e35 * other.e45 * this.e4125)), 
            /* e1234 */
            anti_wedge_g1[3] * other.e45
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Flector other) {
        float anti_wedge_g0_x = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e45 * this.e321);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group4.xyz) + (float3(this.e1234) * other.group0.xyz) + (this.group5.yzx * other.group1.zxy) - (this.group5.zxy * other.group1.yzx);
        float anti_wedge_g1_w = other.e45 * this.e1234;
        float anti_wedge_g2 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float3 anti_wedge_g3_xyz = (float3(this.e12345) * other.group0.xyz) + (float3(other.e3215) * this.group6.xyz) + (this.group8.yzx * other.group1.zxy) - (this.group8.zxy * other.group1.yzx);
        float anti_wedge_g3_w = this.e12345 * other.e45;
        float3 anti_wedge_g4 = (this.group7.zxy * other.group1.yzx) - (this.group7.yzx * other.group1.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group1.xyz);
        float2 anti_wedge_g6_xy = (other.group1.yz * this.group9.zx) - (other.group1.zx * this.group9.yz);
        float anti_wedge_g6_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        float3 anti_wedge_g7 = float3(this.e1234) * other.group1.xyz;
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group1.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group1;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g4[0] * other.e15) + (anti_wedge_g4[1] * other.e25) + (anti_wedge_g4[2] * other.e35) - (anti_wedge_g3_w * other.e45) - (anti_wedge_g9[0] * other.e4235) - (anti_wedge_g9[1] * other.e4315) - (anti_wedge_g9[2] * other.e4125)), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_x * -1.0) * other.group1.xyz), 0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group1.xyz)), anti_wedge_g1_w * other.e3215), 
            /* e41, e42, e43 */
            float3(anti_wedge_g1_w * -1.0) * other.group1.xyz, 
            /* e23, e31, e12 */
            (anti_wedge_g1_xyz.zxy * other.group1.yzx) - (anti_wedge_g1_xyz.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            (other.group1.wwwx * float4(anti_wedge_g4, anti_wedge_g5[0])) + float4((float3(anti_wedge_g3_w) * other.group1.xyz), (anti_wedge_g0_x * other.e45) + (anti_wedge_g5[1] * other.e4315) + (anti_wedge_g5[2] * other.e4125)), 
            /* e423, e431, e412 */
            (anti_wedge_g4.zxy * other.group1.yzx) - (anti_wedge_g4.yzx * other.group1.zxy), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g3_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_x) * other.group0.xyz) - (anti_wedge_g3_xyz.zxy * other.group1.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g7 * float3(other.e3215)) + float3((float2(anti_wedge_g6_xy[1], anti_wedge_g6_z) * other.group1.zx), anti_wedge_g6_xy[0] * other.e4315)), (anti_wedge_g1_xyz[0] * other.e15) + (anti_wedge_g1_xyz[1] * other.e25) - (anti_wedge_g8[1] * other.e4315) - (anti_wedge_g8[2] * other.e4125)) - (float4(anti_wedge_g6_z, anti_wedge_g6_xy[0], anti_wedge_g6_xy[1], anti_wedge_g8[0]) * other.group1.yzxx) - (other.group0 * float4(float3(anti_wedge_g1_w), anti_wedge_g2)), 
            /* e1234 */
            (anti_wedge_g1_w * other.e45) + (anti_wedge_g7[0] * other.e4235) + (anti_wedge_g7[1] * other.e4315) + (anti_wedge_g7[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Line other) {
        float anti_wedge_g0_x = -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43);
        float3 anti_wedge_g1_xyz = (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group7.zxy) - (other.group1.zxy * this.group7.yzx);
        float3 anti_wedge_g4 = float3(this.e1234) * other.group0.xyz;
        float4 anti_wedge_g6 = float4(1.0, 1.0, this.e12345, 0.0) * float4((other.group0.xyz * float3(float2(this.e12345), 1.0)), 0.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e415 * anti_wedge_g6[0]) - (other.e425 * anti_wedge_g6[1]) - (other.e435 * anti_wedge_g6[2])), 
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
            float4(float3(0.0), -(anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435)), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz.yzx * other.group1.zxy) - (float3(other.e415) * float3(other.e425 * this.e315, other.e425 * this.e235, other.e435 * this.e235)) - (float3(other.e435) * float3(other.e415 * this.e125, other.e425 * this.e125, other.e425 * this.e315)) - (float3(other.e235 * this.e415) * other.group0.xyz) - (float3(other.e315 * this.e425) * other.group0.xyz) - (float3(other.e125 * this.e435) * other.group0.xyz) - (anti_wedge_g1_xyz.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g4.yzx * other.group1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            -(anti_wedge_g4[0] * other.e415) - (anti_wedge_g4[1] * other.e425) - (anti_wedge_g4[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Motor other) {
        float anti_wedge_g0_x = (this.scalar * other.e12345) + (other.e5 * this.e1234) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435);
        float4 anti_wedge_g1 = float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e321) * other.group0.xyz) + (this.group7.zxy * other.group1.yzx) - (this.group7.yzx * other.group1.zxy)), other.e12345 * this.e4);
        float anti_wedge_g2 = (this.e12345 * other.e5) + (other.e12345 * this.e5) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435);
        float anti_wedge_g3_w = other.e12345 * this.e45;
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group0.xyz);
        float3 anti_wedge_g6_xyz = (float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group6.xyz);
        float3 anti_wedge_g7 = float3(other.e12345) * this.group7.xyz;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0_w, (right_dual_g1_w * anti_wedge_g1[3]) + (right_dual_g0_w * this.e12345 * other.e12345) - (anti_wedge_g6_xyz[0] * right_dual_g0_xyz[0]) - (anti_wedge_g6_xyz[1] * right_dual_g0_xyz[1]) - (anti_wedge_g6_xyz[2] * right_dual_g0_xyz[2]) - (anti_wedge_g7[0] * right_dual_g1_xyz[0]) - (anti_wedge_g7[1] * right_dual_g1_xyz[1]) - (anti_wedge_g7[2] * right_dual_g1_xyz[2])), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(right_dual_g0_w), 
            /* e5 */
            anti_wedge_g2 * right_dual_g0_w, 
            /* e15, e25, e35, e45 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g0_x)) + (float3(right_dual_g0_w) * float3(other.e315 * this.e4125, other.e125 * this.e4235, other.e235 * this.e4315)) + (float3(right_dual_g0_w * other.e12345) * this.group3.xyz) + (float3(right_dual_g0_w * this.e3215) * other.group0.xyz) - (float3(right_dual_g0_w) * float3(other.e125 * this.e4315, other.e235 * this.e4125, other.e315 * this.e4235))), anti_wedge_g3_w * right_dual_g0_w), 
            /* e41, e42, e43 */
            anti_wedge_g4 * float3(right_dual_g0_w), 
            /* e23, e31, e12 */
            (right_dual_g0_xyz * float3(anti_wedge_g0_x)) + (float3(right_dual_g0_w * other.e12345) * this.group5.xyz) + (float3(right_dual_g0_w * this.e1234) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g6_xyz * float3(right_dual_g0_w)) + (right_dual_g1_xyz * float3(anti_wedge_g1[3]))), right_dual_g0_w * other.e12345 * this.e321), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(right_dual_g0_w)) + (right_dual_g0_xyz * float3(anti_wedge_g1[3])), 
            /* e235, e315, e125 */
            (right_dual_g0_xyz * float3(anti_wedge_g2)) + (float3(right_dual_g0_w * this.e12345) * other.group1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group8.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g3_w)) + (float3(right_dual_g0_w * other.e12345) * this.group9.xyz) + (anti_wedge_g4.yzx * right_dual_g1_xyz.zxy) - (anti_wedge_g4.zxy * right_dual_g1_xyz.yzx)), (anti_wedge_g0_x * right_dual_g1_w) + (right_dual_g0_w * other.e12345 * this.e3215)), 
            /* e1234 */
            (right_dual_g0_w * other.e12345 * this.e1234) - (anti_wedge_g4[0] * right_dual_g0_xyz[0]) - (anti_wedge_g4[1] * right_dual_g0_xyz[1]) - (anti_wedge_g4[2] * right_dual_g0_xyz[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e5 * this.e1234) + (other.e1234 * this.e5) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (other.e45 * this.e321) - (other.e321 * this.e45);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + (this.group9.yzxx * float4(other.group5.zxy, other.e41)) + float4(((float3(other.e321) * this.group6.xyz) + (float3(other.e3215) * this.group4.xyz) + (float3(this.e321) * other.group6.xyz) + (float3(this.e1234) * other.group3.xyz) + (other.group7.zxy * this.group8.yzx) + (other.group8.yzx * this.group7.zxy) + (this.group5.yzx * other.group9.zxy) - (float3(this.e3215) * other.group4.xyz) - (float3(other.e1234) * this.group3.xyz) - (other.group5.yzx * this.group9.zxy) - (other.group7.yzx * this.group8.zxy) - (other.group8.zxy * this.group7.yzx) - (this.group5.zxy * other.group9.yzx)), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234));
        float anti_wedge_g2 = (other.e12345 * this.e5) + (this.e12345 * other.e5) + (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = (float4(other.e12345) * this.group3) + (float4(this.e12345) * other.group3) + float4(((float3(other.e3215) * this.group6.xyz) + (float3(this.e3215) * other.group6.xyz) + (other.group8.yzx * this.group9.zxy) + (this.group8.yzx * other.group9.zxy) - (other.group8.zxy * this.group9.yzx) - (this.group8.zxy * other.group9.yzx)), 0.0);
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group4.xyz) + (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group6.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group9.yzx) + (this.group7.zxy * other.group9.yzx) - (other.group7.yzx * this.group9.zxy) - (this.group7.yzx * other.group9.zxy);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group5.xyz) + (float3(this.e12345) * other.group5.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group9.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group6) + (other.group9.yzxw * float4(this.group9.zxy, this.e1234)) + float4(-(other.group9.zx * this.group9.yz), other.e4315 * this.e4235 * -1.0, this.e3215 * other.e1234 * -1.0);
        float3 anti_wedge_g7 = (float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group8.xyz) + (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = (float4(other.e12345) * this.group9) + (float4(this.e12345) * other.group9);
        float anti_wedge_g10 = (other.e12345 * this.e1234) + (this.e12345 * other.e1234);
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) + (right_dual_g0[0] * other.e12345 * this.e12345) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Plane other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group4.xyz) + (this.group5.yzx * other.group0.zxy) - (this.group5.zxy * other.group0.yzx);
        float3 anti_wedge_g3_xyz = (float3(other.e3215) * this.group6.xyz) + (this.group8.yzx * other.group0.zxy) - (this.group8.zxy * other.group0.yzx);
        float3 anti_wedge_g4 = (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy);
        float2 anti_wedge_g6_xy = (this.group9.zx * other.group0.yz) - (this.group9.yz * other.group0.zx);
        float anti_wedge_g6_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float3 anti_wedge_g7 = float3(this.e1234) * other.group0.xyz;
        float4 anti_wedge_g9 = float4(this.e12345) * other.group0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g9[0] * other.e4235) - (anti_wedge_g9[1] * other.e4315) - (anti_wedge_g9[2] * other.e4125)), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_x * -1.0) * other.group0.xyz), 0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(0.0, 0.0, other.e4125 * other.e4125 * this.e35, 0.0) + float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (float3(other.e4235) * float3(this.e25 * other.e4315, this.e15 * other.e4315, this.e15 * other.e4125)) + (float3(other.e4125) * float3(this.e35 * other.e4235, this.e35 * other.e4315, this.e25 * other.e4315)) + (float3(this.e45 * other.e3215) * other.group0.xyz) + float3((other.group0.xy * other.group0.xy * this.group3.xy), 0.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g4 * float4(other.e3215).xyz), 0.0), 
            /* e423, e431, e412 */
            (anti_wedge_g4.zxy * other.group0.yzx) - (anti_wedge_g4.yzx * other.group0.zxy), 
            /* e235, e315, e125 */
            (float3(other.e3215 * other.e3215) * this.group7.xyz) + (anti_wedge_g3_xyz.yzx * other.group0.zxy) - (float3(this.e321 * other.e3215) * other.group0.xyz) - (anti_wedge_g3_xyz.zxy * other.group0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g6_xy[0] * other.e4315) - (anti_wedge_g6_xy[1] * other.e4235), 0.0) + float4(((anti_wedge_g7 * float3(other.e3215)) + float3(((float2(anti_wedge_g6_xy[1], anti_wedge_g6_z) * other.group0.zx) - (float2(anti_wedge_g6_z, anti_wedge_g6_xy[0]) * other.group0.yz)), 0.0)), 0.0), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e4235) + (anti_wedge_g7[1] * other.e4315) + (anti_wedge_g7[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(RoundPoint other) {
        float anti_wedge_g0_x = (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group0;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1[0] * right_dual_g0[0]) + (anti_wedge_g1[1] * right_dual_g0[1]) + (anti_wedge_g1[2] * right_dual_g0[2]) + (anti_wedge_g1[3] * right_dual_g0[3]) + (right_dual_g1 * this.e12345 * other.e5)), 
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
            right_dual_g0 * float4(anti_wedge_g0_x), 
            /* e1234 */
            anti_wedge_g0_x * right_dual_g1
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
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(Sphere other) {
        float anti_wedge_g0_x = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        float4 anti_wedge_g1 = float4(((float3(other.e3215) * this.group4.xyz) + (this.group5.yzx * other.group0.zxy) - (float3(other.e1234) * this.group3.xyz) - (this.group5.zxy * other.group0.yzx)), this.e45 * other.e1234 * -1.0);
        float3 anti_wedge_g3_xyz = (float3(other.e3215) * this.group6.xyz) + (this.group8.yzx * other.group0.zxy) - (this.group8.zxy * other.group0.yzx);
        float3 anti_wedge_g4 = (float3(other.e1234) * this.group6.xyz) + (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) - (float3(this.e321) * other.group0.xyz);
        float4 anti_wedge_g6 = float4((this.group9.zx * other.group0.yz), this.e4315 * other.e4235, other.e3215 * this.e1234) - (this.group9.yzxw * float4(other.group0.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group0;
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * right_dual_g0[0]) + (anti_wedge_g9[1] * right_dual_g0[1]) + (anti_wedge_g9[2] * right_dual_g0[2]) + (anti_wedge_g9[3] * right_dual_g0[3]) + (this.e12345 * other.e3215 * other.e1234)), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(anti_wedge_g0_x), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) - (right_dual_g0 * float4(this.e15 * other.e4235)) - (right_dual_g0 * float4(this.e25 * other.e4315)) - (right_dual_g0 * float4(this.e35 * other.e4125)) - (right_dual_g0 * float4(this.e45 * other.e3215)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g1[3]) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g0[2]) - (anti_wedge_g1[2] * right_dual_g0[1]), (anti_wedge_g1[2] * right_dual_g0[0]) - (anti_wedge_g1[0] * right_dual_g0[2]), (anti_wedge_g1[0] * right_dual_g0[1]) - (anti_wedge_g1[1] * right_dual_g0[0])), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(right_dual_g0[3])) + (anti_wedge_g4 * float3(other.e3215))), 0.0), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g0[3])) + (anti_wedge_g4.yzx * right_dual_g0.zxy) - (anti_wedge_g4.zxy * right_dual_g0.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g3_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g3_xyz.yzx * right_dual_g0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g7 * float3(other.e3215)) + float3(((anti_wedge_g6.zx * right_dual_g0.yz) - (anti_wedge_g6.yz * right_dual_g0.zx)), (anti_wedge_g6[1] * right_dual_g0[0]) - (anti_wedge_g6[0] * right_dual_g0[1])) - (anti_wedge_g8 * float3(right_dual_g0[3]))), (anti_wedge_g8[0] * right_dual_g0[0]) + (anti_wedge_g8[1] * right_dual_g0[1])), 
            /* e1234 */
            -(anti_wedge_g7[0] * right_dual_g0[0]) - (anti_wedge_g7[1] * right_dual_g0[1]) - (anti_wedge_g7[2] * right_dual_g0[2]) - (anti_wedge_g6[3] * right_dual_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(VersorEven other) {
        float anti_wedge_g0_x = (this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (other.e5 * this.e1234) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321);
        float4 anti_wedge_g1 = (float4(this.e12345) * other.group3) + float4(((float3(this.e321) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz) + (float3(other.e321) * this.group6.xyz) + (this.group7.zxy * other.group2.yzx) + (this.group8.yzx * other.group0.zxy) - (this.group7.yzx * other.group2.zxy) - (this.group8.zxy * other.group0.yzx)), this.e4 * other.e12345);
        float anti_wedge_g2 = (this.e12345 * other.e5) + (other.e12345 * this.e5) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125);
        float4 anti_wedge_g3 = float4((float3((this.e4125 * other.e315) - (this.e4315 * other.e125), (this.e4235 * other.e125) - (this.e4125 * other.e235), (this.e4315 * other.e235) - (this.e4235 * other.e315)) + (float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group3.xyz)), this.e45 * other.e12345);
        float3 anti_wedge_g4 = float3((this.e4315 * other.e412) - (this.e4125 * other.e431), (this.e4125 * other.e423) - (this.e4235 * other.e412), (this.e4235 * other.e431) - (this.e4315 * other.e423)) + (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group1.xyz);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group5.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz);
        float4 anti_wedge_g6 = (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group6);
        float3 anti_wedge_g7 = (float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group7.xyz);
        float3 anti_wedge_g8 = (float3(this.e12345) * other.group2.xyz) + (float3(other.e12345) * this.group8.xyz);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0_w, (anti_wedge_g2 * right_dual_g2_w) + (anti_wedge_g1[0] * right_dual_g3[0]) + (anti_wedge_g1[1] * right_dual_g3[1]) + (anti_wedge_g1[2] * right_dual_g3[2]) + (anti_wedge_g1[3] * right_dual_g3[3]) + (right_dual_g0_w * this.e12345 * other.e12345) - (right_dual_g1_w * anti_wedge_g6[3]) - (anti_wedge_g7[0] * right_dual_g2_xyz[0]) - (anti_wedge_g7[1] * right_dual_g2_xyz[1]) - (anti_wedge_g7[2] * right_dual_g2_xyz[2]) - (anti_wedge_g8[0] * right_dual_g0_xyz[0]) - (anti_wedge_g8[1] * right_dual_g0_xyz[1]) - (anti_wedge_g8[2] * right_dual_g0_xyz[2]) - (right_dual_g1_xyz[0] * anti_wedge_g6[0]) - (right_dual_g1_xyz[1] * anti_wedge_g6[1]) - (right_dual_g1_xyz[2] * anti_wedge_g6[2])), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(right_dual_g0_w), 
            /* e5 */
            anti_wedge_g2 * right_dual_g0_w, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(right_dual_g0_w)) + (float4(anti_wedge_g0_x) * float4(right_dual_g2_xyz, right_dual_g1_w)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0_w)) + (right_dual_g0_xyz * float3(anti_wedge_g0_x)), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(right_dual_g0_w)) + (right_dual_g1_xyz * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2)) + (right_dual_g2_xyz * float3(anti_wedge_g1[3])) + (float3(right_dual_g0_w) * anti_wedge_g6.xyz) - (float3(right_dual_g1_w) * anti_wedge_g1.xyz)), right_dual_g0_w * anti_wedge_g6[3]), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(right_dual_g0_w)) + (right_dual_g1_xyz * float3(anti_wedge_g1[3])) + (right_dual_g0_xyz.yzx * anti_wedge_g1.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(right_dual_g0_w)) + (right_dual_g1_xyz * float3(anti_wedge_g2)) + (right_dual_g2_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g3 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g1_w)) + (right_dual_g1_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w * other.e12345) * this.group9.xyz) + (anti_wedge_g4.yzx * right_dual_g2_xyz.zxy) + (right_dual_g0_xyz.yzx * anti_wedge_g3.zxy) - (anti_wedge_g4.zxy * right_dual_g2_xyz.yzx) - (right_dual_g0_xyz.zxy * anti_wedge_g3.yzx)), right_dual_g0_w * this.e3215 * other.e12345), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g2_w) + (right_dual_g0_w * other.e12345 * this.e1234) - (anti_wedge_g4[0] * right_dual_g1_xyz[0]) - (anti_wedge_g4[1] * right_dual_g1_xyz[1]) - (anti_wedge_g4[2] * right_dual_g1_xyz[2]) - (anti_wedge_g5[0] * right_dual_g0_xyz[0]) - (anti_wedge_g5[1] * right_dual_g0_xyz[1]) - (anti_wedge_g5[2] * right_dual_g0_xyz[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension MultiVector {
    public MultiVector reject_via_origin_from(VersorOdd other) {
        float anti_wedge_g0_x = (this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (other.e1234 * this.e5) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(other.group2.xyz, other.e45)) + (this.group9.yzxz * float4(other.group1.zxy, other.e43)) + float4(((float3(other.e3215) * this.group4.xyz) + (this.group5.yzx * other.group3.zxy) + float3(-(this.group9.zx * other.group1.yz), this.e4315 * other.e23 * -1.0) - (float3(this.e3215) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz)), (this.e4235 * other.e41) + (this.e4315 * other.e42) - (this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.yzxx * float4(this.group5.zxy, this.e41));
        float anti_wedge_g2 = (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45);
        float3 anti_wedge_g3_xyz = (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group6.xyz) + (this.group8.yzx * other.group3.zxy) - (this.group8.zxy * other.group3.yzx);
        float anti_wedge_g3_w = this.e12345 * other.e45;
        float3 anti_wedge_g4 = (float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group6.xyz) + (this.group7.zxy * other.group3.yzx) - (this.group7.yzx * other.group3.zxy);
        float3 anti_wedge_g5 = (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group3.xyz);
        float4 anti_wedge_g6 = float4((this.group9.zx * other.group3.yz), this.e4315 * other.e4235, other.e3215 * this.e1234) - (this.group9.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group3;
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.scalar) + (anti_wedge_g3_xyz[0] * other.e41) + (anti_wedge_g3_xyz[1] * other.e42) + (anti_wedge_g3_xyz[2] * other.e43) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g9[0] * right_dual_g3[0]) + (anti_wedge_g9[1] * right_dual_g3[1]) + (anti_wedge_g9[2] * right_dual_g3[2]) + (anti_wedge_g9[3] * right_dual_g3[3]) + (this.e12345 * other.e1234 * other.e3215) - (anti_wedge_g3_w * other.e45) - (anti_wedge_g4[0] * right_dual_g2_xyz[0]) - (anti_wedge_g4[1] * right_dual_g2_xyz[1]) - (anti_wedge_g4[2] * right_dual_g2_xyz[2])), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0_x), 
            /* e5 */
            anti_wedge_g0_x * other.e3215, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) - (right_dual_g3 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) - (anti_wedge_g1[2] * right_dual_g3[1]), (anti_wedge_g1[2] * right_dual_g3[0]) - (anti_wedge_g1[0] * right_dual_g3[2]), (anti_wedge_g1[0] * right_dual_g3[1]) - (anti_wedge_g1[1] * right_dual_g3[0])), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(right_dual_g3[3])) + (anti_wedge_g4 * float3(other.e3215)) - (float3(anti_wedge_g0_x) * other.group1.xyz) - (float3(anti_wedge_g3_w) * right_dual_g3.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) - (float3(anti_wedge_g0_x) * other.group0.xyz) - (anti_wedge_g4.zxy * right_dual_g3.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e3215)) + (right_dual_g2_xyz * float3(anti_wedge_g0_x)) + (anti_wedge_g3_xyz.zxy * right_dual_g3.yzx) - (anti_wedge_g3_xyz.yzx * right_dual_g3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g7 * float3(other.e3215)) + (right_dual_g2_xyz * float3(anti_wedge_g1[3])) + (float3(anti_wedge_g2) * other.group0.xyz) + float3(((anti_wedge_g1.yz * other.group1.zx) + (anti_wedge_g6.zx * right_dual_g3.yz) - (anti_wedge_g1.zx * other.group1.yz) - (anti_wedge_g6.yz * right_dual_g3.zx)), (anti_wedge_g1[0] * other.e31) + (anti_wedge_g6[1] * right_dual_g3[0]) - (anti_wedge_g1[1] * other.e23) - (anti_wedge_g6[0] * right_dual_g3[1])) - (anti_wedge_g8 * float3(right_dual_g3[3]))), (anti_wedge_g8[0] * right_dual_g3[0]) + (anti_wedge_g8[1] * right_dual_g3[1]) + (anti_wedge_g8[2] * right_dual_g3[2]) + (anti_wedge_g6[3] * other.e3215) - (right_dual_g2_xyz[2] * anti_wedge_g1[2])), 
            /* e1234 */
            (anti_wedge_g1[3] * other.e45) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43) - (anti_wedge_g6[3] * right_dual_g3[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Plane> operator /(Plane lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Sphere reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = other.e41 * this.e4235;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e45 * this.e3215)) + (right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz);
        float4 anti_wedge_g2 = float4(0.0, 0.0, other.e315 * this.e4235 * -1.0, 0.0) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1)) + float4(-(other.group2.zx * this.group0.yz), 0.0, 0.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g2_xyz * float4(anti_wedge_g2[3]).xyz), -(anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g2[3] * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g1[3])) + (float3(anti_wedge_g2[3]) * other.group3.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g2.zxy) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g2.yzx)), anti_wedge_g2[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g0 = float3(other.e321 * -1.0) * this.group0.xyz;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float4(other.e321).xyz * float3(-1.0)), -(anti_wedge_g0[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0[2] * right_dual_g0_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Flector reject_via_origin_from(AntiFlector other) {
        float anti_wedge_g0_w = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(anti_wedge_g0_w), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g0_w) * other.group1.xyz) - (float3(right_dual_g0[3] * other.e321) * this.group0.xyz)), anti_wedge_g0_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(AntiLine other) {
        float4 anti_wedge_g0 = float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0.yzx * anti_wedge_g0.zxy) - (right_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Flector reject_via_origin_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4(0.0, 0.0, (other.e31 * this.e4235) - (other.e23 * this.e4315), 0.0) + float4(((other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx)), 0.0, 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * float4(other.e3215).xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, anti_wedge_g1[1] * other.e23 * -1.0, 0.0) + (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e15)) + float4(-(anti_wedge_g1.zx * other.group0.yz), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, other.e3 * other.e3 * this.e4125, 0.0) + float4(((float3(other.e1) * float3(other.e2 * this.e4315, other.e2 * this.e4235, other.e3 * this.e4235)) + (float3(other.e3) * float3(other.e1 * this.e4125, other.e2 * this.e4125, other.e2 * this.e4315)) + float3((other.group0.xy * other.group0.xy * this.group0.xy), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Sphere reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz);
        float3 anti_wedge_g2 = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx);
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group2.zxy) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g1_xyz * float3(other.e321)) - (anti_wedge_g0.zxy * other.group2.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * right_dual_g1_xyz[0]) - (anti_wedge_g0[1] * right_dual_g1_xyz[1]) - (anti_wedge_g0[2] * right_dual_g1_xyz[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0 = (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4(((float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz)), 0.0);
        float4 anti_wedge_g2 = float4(0.0, 0.0, (other.e235 * this.e4315) - (other.e315 * this.e4235), 0.0) + float4(((float3(this.e3215) * other.group1.xyz) + float3(((other.group2.yz * this.group0.zx) - (other.group2.zx * this.group0.yz)), 0.0)), 0.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(right_dual_g2_w), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g2_w) * anti_wedge_g2.xyz), (right_dual_g2_w * anti_wedge_g2[3]) - (anti_wedge_g0[0] * right_dual_g1_xyz[0]) - (anti_wedge_g0[1] * right_dual_g1_xyz[1]) - (anti_wedge_g0[2] * right_dual_g1_xyz[2]) - (other.e423 * anti_wedge_g1[0]) - (other.e431 * anti_wedge_g1[1]) - (other.e412 * anti_wedge_g1[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g1[3])) + (float3(right_dual_g2_w) * anti_wedge_g3.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g2.zxy) - (float3(other.e321) * anti_wedge_g1.xyz) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g2.yzx)), right_dual_g2_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Sphere reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = other.e41 * this.e4235;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (right_dual_g0 * float3(other.e45 * this.e3215)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float2 anti_wedge_g1_xy = (other.group3.yz * this.group0.zx) - (other.group3.zx * this.group0.yz);
        float anti_wedge_g1_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        float anti_wedge_g1_w = other.e1234 * this.e3215 * -1.0;
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group3.xyz);
        float anti_wedge_g2_w = other.e41 * this.e4235;
        float3 anti_wedge_g3_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g3_w = other.e35 * this.e4125 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            -(anti_wedge_g3_xyz * float3(other.e1234)) - (float3(anti_wedge_g2_w) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.yzxw * float4(anti_wedge_g3_xyz.zxy, anti_wedge_g2_w)) + float4(-(anti_wedge_g3_xyz.yzx * other.group3.zxy), anti_wedge_g3_w * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(float3(anti_wedge_g3_w), anti_wedge_g0[0])) + (other.group3.wwwy * float4(anti_wedge_g3_xyz, anti_wedge_g0[1])) + float4(float3(0.0), (anti_wedge_g0[2] * other.e4125) - (anti_wedge_g1_w * other.e1234) - (anti_wedge_g3_xyz[0] * other.e41)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g3_w * other.e41) + (anti_wedge_g1_xy[1] * other.e4125) - (anti_wedge_g1_z * other.e4315) - (anti_wedge_g2_w * other.e15), (anti_wedge_g1_z * other.e4235) + (anti_wedge_g3_w * other.e42) - (anti_wedge_g2_w * other.e25) - (anti_wedge_g1_xy[0] * other.e4125), (anti_wedge_g3_w * other.e43) + (anti_wedge_g1_xy[0] * other.e4315) - (anti_wedge_g2_w * other.e35) - (anti_wedge_g1_xy[1] * other.e4235), (anti_wedge_g1_w * other.e3215) + (anti_wedge_g3_xyz[0] * other.e15) + (anti_wedge_g3_xyz[1] * other.e25) + (anti_wedge_g3_xyz[2] * other.e35) - (anti_wedge_g3_w * other.e45) - (anti_wedge_g2_xyz[0] * other.e4235) - (anti_wedge_g2_xyz[1] * other.e4315) - (anti_wedge_g2_xyz[2] * other.e4125)) + float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g3_xyz.yzx * other.group1.zxy) - (anti_wedge_g2_xyz * float3(other.e1234)) - (anti_wedge_g3_xyz.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<Plane> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public AntiDualNum reject_via_origin_from(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((other.e45 * other.e45 * this.e3215) + (other.e15 * other.e45 * this.e4235) + (other.e25 * other.e45 * this.e4315) + (other.e35 * other.e45 * this.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Flector reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (other.e4235 * this.e4315) - (other.e4315 * this.e4235), 0.0) + float4(((other.group1.yz * this.group0.zx) - (other.group1.zx * this.group0.yz)), 0.0, 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float4(other.e45).xyz * float4(this.e3215).xyz * other.group1.xyz * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g0[0] * other.e4315) - (anti_wedge_g0[1] * other.e4235), 0.0) + float4(((anti_wedge_g0.yz * other.group1.zx) - (anti_wedge_g0.zx * other.group1.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(Line other) {
        float3 anti_wedge_g0_xyz = (float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Flector reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(0.0, 0.0, (other.e235 * this.e4315) - (other.e315 * this.e4235), 0.0) + float4(((float3(this.e3215) * other.group0.xyz) + float3(((other.group1.yz * this.group0.zx) - (other.group1.zx * this.group0.yz)), 0.0)), 0.0);
        float4 anti_wedge_g1 = float4(other.e12345) * this.group0;
        float right_dual_g0_w = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(right_dual_g0_w), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_dual_g0_w) * anti_wedge_g1.xyz) + (float3(anti_wedge_g0[3]) * other.group0.xyz)), right_dual_g0_w * anti_wedge_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215);
        float4 anti_wedge_g1 = float4(((other.group5.zxy * this.group0.yzx) - (float3(this.e3215) * other.group4.xyz) - (other.group5.yzx * this.group0.zxy)), other.e41 * this.e4235);
        float anti_wedge_g2 = -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = float4(((float3(this.e3215) * other.group6.xyz) + (other.group8.yzx * this.group0.zxy) - (other.group8.zxy * this.group0.yzx)), 0.0);
        float3 anti_wedge_g4 = (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group0.xyz);
        float2 anti_wedge_g6_xy = (other.group9.yz * this.group0.zx) - (other.group9.zx * this.group0.yz);
        float anti_wedge_g6_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        float anti_wedge_g6_w = this.e3215 * other.e1234 * -1.0;
        float3 anti_wedge_g7 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group0;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) - (anti_wedge_g6_w * right_dual_g3[3]) - (anti_wedge_g6_z * right_dual_g5[2]) - (anti_wedge_g6_xy[0] * right_dual_g5[0]) - (anti_wedge_g6_xy[1] * right_dual_g5[1]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) + float3((anti_wedge_g6_xy * float2(right_dual_g0[0])), anti_wedge_g6_z * right_dual_g0[0]) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), (anti_wedge_g0_x * other.e45) + (anti_wedge_g6_w * right_dual_g0[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((float2(anti_wedge_g6_z, anti_wedge_g6_xy[0]) * right_dual_g1.yz) - (float2(anti_wedge_g6_xy[1], anti_wedge_g6_z) * right_dual_g1.zx)), (anti_wedge_g6_xy[1] * right_dual_g1[0]) - (anti_wedge_g6_xy[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g6_w * other.e3215) + (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2])), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g6_w * right_dual_g1[3]) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2])
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Plane reject_via_origin_from(Plane other) {
        float3 anti_wedge_g0 = float3((other.e4315 * this.e4125) - (other.e4125 * this.e4315), (other.e4125 * this.e4235) - (other.e4235 * this.e4125), (other.e4235 * this.e4315) - (other.e4315 * this.e4235));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (other.group0.zxyx * float4(anti_wedge_g0.yzx, other.e4235 * this.e3215)) - (other.group0.yzxx * float4(anti_wedge_g0.zxy, other.e3215 * this.e4235))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Sphere reject_via_origin_from(RoundPoint other) {
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
public Sphere operator /(RejectViaOriginFromInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public Sphere reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float2 anti_wedge_g1_xy = (this.group0.zx * other.group0.yz) - (this.group0.yz * other.group0.zx);
        float anti_wedge_g1_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(anti_wedge_g1_z, anti_wedge_g1_xy[0], anti_wedge_g1_xy[1]) * right_dual_g0_xyz.yzx) - (anti_wedge_g2 * float3(other.e1234)) - (float3(anti_wedge_g1_xy[1], anti_wedge_g1_z, anti_wedge_g1_xy[0]) * right_dual_g0_xyz.zxy)), (anti_wedge_g2[0] * right_dual_g0_xyz[0]) + (anti_wedge_g2[1] * right_dual_g0_xyz[1])), 
            /* e1234 */
            (this.e3215 * other.e1234 * other.e1234) - (anti_wedge_g0[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0[2] * right_dual_g0_xyz[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float2 anti_wedge_g0_xy = (this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz);
        float anti_wedge_g0_z = (this.e4235 * other.e431) - (this.e4315 * other.e423);
        float anti_wedge_g0_w = this.e4235 * other.e1;
        float4 anti_wedge_g1 = float4(((float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz)), 0.0);
        float4 anti_wedge_g2 = float4(0.0, 0.0, (this.e4315 * other.e235) - (this.e4235 * other.e315), 0.0) + float4(((float3(this.e3215) * other.group1.xyz) + float3(((this.group0.zx * other.group2.yz) - (this.group0.yz * other.group2.zx)), 0.0)), 0.0);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0_w)) + float3((anti_wedge_g0_xy * float2(right_dual_g0_w)), anti_wedge_g0_z * right_dual_g0_w)), anti_wedge_g0_w * right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g0_w)), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_dual_g0_w)) + (right_dual_g2 * float4(anti_wedge_g0_w)) + float4(float3(0.0), -(anti_wedge_g0_z * right_dual_g1[2]) - (anti_wedge_g0_xy[0] * right_dual_g1[0]) - (anti_wedge_g0_xy[1] * right_dual_g1[1]) - (right_dual_g0_xyz[0] * anti_wedge_g1[0]) - (right_dual_g0_xyz[1] * anti_wedge_g1[1]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((right_dual_g0_w * anti_wedge_g3[0]) + (anti_wedge_g0_xy[1] * right_dual_g2[2]) - (anti_wedge_g0_z * right_dual_g2[1]), (anti_wedge_g0_z * right_dual_g2[0]) + (right_dual_g0_w * anti_wedge_g3[1]) - (anti_wedge_g0_xy[0] * right_dual_g2[2]), (right_dual_g0_w * anti_wedge_g3[2]) + (anti_wedge_g0_xy[0] * right_dual_g2[1]) - (anti_wedge_g0_xy[1] * right_dual_g2[0])) + (float3(anti_wedge_g0_w) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (right_dual_g0_xyz.yzx * anti_wedge_g2.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g2.yzx)), right_dual_g0_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Plane {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = float3(other.e1234 * -1.0) * this.group0.xyz;
        float2 anti_wedge_g1_xy = (this.group0.zx * other.group3.yz) - (this.group0.yz * other.group3.zx);
        float anti_wedge_g1_z = (this.e4315 * other.e4235) - (this.e4235 * other.e4315);
        float anti_wedge_g1_w = this.e3215 * other.e1234 * -1.0;
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group3.xyz);
        float anti_wedge_g2_w = this.e4235 * other.e41;
        float3 anti_wedge_g3_xyz = float3((this.e4315 * other.e12) - (this.e4125 * other.e31), (this.e4125 * other.e23) - (this.e4235 * other.e12), (this.e4235 * other.e31) - (this.e4315 * other.e23)) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g3_w = this.e3215 * other.e45 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g2_w)) - (anti_wedge_g3_xyz * float3(other.e1234)), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3_xyz.yzx * right_dual_g3_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g3_xyz.yzx)), (anti_wedge_g2_w * other.e3215) - (anti_wedge_g3_w * other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g3_xyz * float3(other.e3215)) - (right_dual_g3_xyz * float3(anti_wedge_g3_w))), (anti_wedge_g2_w * other.e45) - (anti_wedge_g1_w * other.e1234) - (anti_wedge_g0[0] * right_dual_g3_xyz[0]) - (anti_wedge_g0[1] * right_dual_g3_xyz[1]) - (anti_wedge_g0[2] * right_dual_g3_xyz[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_z * right_dual_g3_xyz[1]) + (anti_wedge_g2_w * right_dual_g2_xyz[0]) + (anti_wedge_g3_w * other.e41), (anti_wedge_g2_w * right_dual_g2_xyz[1]) + (anti_wedge_g3_w * other.e42) + (anti_wedge_g1_xy[0] * right_dual_g3_xyz[2]), (anti_wedge_g2_w * right_dual_g2_xyz[2]) + (anti_wedge_g3_w * other.e43) + (anti_wedge_g1_xy[1] * right_dual_g3_xyz[0]), (anti_wedge_g1_w * other.e3215) + (anti_wedge_g2_xyz[0] * right_dual_g3_xyz[0]) + (anti_wedge_g2_xyz[1] * right_dual_g3_xyz[1]) + (anti_wedge_g2_xyz[2] * right_dual_g3_xyz[2])) + float4(((anti_wedge_g0 * float3(other.e3215)) + (anti_wedge_g3_xyz.yzx * other.group1.zxy) - (anti_wedge_g2_xyz * float3(other.e1234)) - (float3(anti_wedge_g1_xy[1], anti_wedge_g1_z, anti_wedge_g1_xy[0]) * right_dual_g3_xyz.zxy) - (anti_wedge_g3_xyz.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<RoundPoint> operator /(RoundPoint lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public DualNum reject_via_origin_from(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * this.e4) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public Motor reject_via_origin_from(AntiMotor other) {
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
public RoundPoint operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public RoundPoint reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e5 */
            right_dual_g0 * other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float anti_wedge_g1 = other.e12345 * this.e5;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_dual_g1_xyz * float3(anti_wedge_g0[3])) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g0[3])) + (float3(anti_wedge_g1) * other.group0.xyz) + (float3(other.e321) * anti_wedge_g0.xyz)), 0.0), 
            /* e235, e315, e125, e4 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g1)) + (right_dual_g2_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g0.zxy)), right_dual_g2_w * anti_wedge_g0[3]), 
            /* e1, e2, e3, e5 */
            float4((float3(right_dual_g2_w) * anti_wedge_g0.xyz), anti_wedge_g1 * right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion reject_via_origin_from(DipoleInversion other) {
        float anti_wedge_g0 = (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group0.xyz, 
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
public VersorEven operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_via_origin_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e5 * anti_wedge_g0[3] * -1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e12345 * other.e12345 * this.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public AntiFlector reject_via_origin_from(Flector other) {
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
public VersorEven operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float anti_wedge_g1 = other.e12345 * this.e5;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0[3]) * float4(right_dual_g0_xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g1_xyz * float4(anti_wedge_g0[3]).xyz), -(right_dual_g0_xyz[0] * anti_wedge_g0[0]) - (right_dual_g0_xyz[1] * anti_wedge_g0[1]) - (right_dual_g0_xyz[2] * anti_wedge_g0[2])), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1)) + (right_dual_g1_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g0.zxy)), anti_wedge_g1 * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5);
        float4 anti_wedge_g1 = float4(other.e12345) * this.group0;
        float anti_wedge_g2 = other.e12345 * this.e5;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g2 * right_dual_g10) + (right_dual_g9_w * anti_wedge_g1[3]) + (right_dual_g9_xyz[0] * anti_wedge_g1[0]) + (right_dual_g9_xyz[1] * anti_wedge_g1[1]) + (right_dual_g9_xyz[2] * anti_wedge_g1[2])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g6_xyz * float3(anti_wedge_g0_x)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (other.group7.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) - (anti_wedge_g1[2] * right_dual_g3[1]), (anti_wedge_g1[2] * right_dual_g3[0]) - (anti_wedge_g1[0] * right_dual_g3[2]), (anti_wedge_g1[0] * right_dual_g3[1]) - (anti_wedge_g1[1] * right_dual_g3[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g8 * float3(anti_wedge_g1[3])) + (right_dual_g9_xyz * float3(anti_wedge_g0_x)) + (right_dual_g6_xyz.yzx * anti_wedge_g1.zxy) - (right_dual_g7 * float3(anti_wedge_g2)) - (right_dual_g6_xyz.zxy * anti_wedge_g1.yzx)), anti_wedge_g0_x * right_dual_g9_w), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public AntiPlane reject_via_origin_from(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(0.0, 0.0, other.e4125 * other.e4125 * this.e3 * -1.0, 0.0) + float4((float3(-(other.group0.xy * other.group0.xy * this.group0.xy), 0.0) - (float3(other.e4235) * float3(other.e4315 * this.e2, other.e4315 * this.e1, other.e4125 * this.e1)) - (float3(other.e4125) * float3(other.e4235 * this.e3, other.e4315 * this.e3, other.e4315 * this.e2)) - (float3(other.e3215 * this.e4) * other.group0.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public RoundPoint reject_via_origin_from(Sphere other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            anti_wedge_g0 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float anti_wedge_g1 = other.e12345 * this.e5;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float3 right_dual_g3_xyz = other.group3.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.zxyx * float4(right_dual_g0_xyz.yzx, right_dual_g3_xyz[0])) + (anti_wedge_g0.wwwy * float4(right_dual_g1_xyz, right_dual_g3_xyz[1])) + float4(-(right_dual_g0_xyz.zxy * anti_wedge_g0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1)) + (right_dual_g2_xyz * float3(anti_wedge_g0[3])) + (float3(other.e321) * anti_wedge_g0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g1)) + (right_dual_g2_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g0.zxy)), anti_wedge_g1 * right_dual_g0_w), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension RoundPoint {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float anti_wedge_g0 = (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (other.e1234 * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<Scalar> operator /(Scalar lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public Scalar reject_via_origin_from(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public AntiCircleRotor reject_via_origin_from(CircleRotor other) {
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
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public AntiDualNum reject_via_origin_from(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * this.scalar * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public AntiMotor reject_via_origin_from(Motor other) {
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
public MultiVector operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0 = other.e12345 * this.scalar;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * float4((other.group9.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            anti_wedge_g0 * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * float4((other.group5.xyz * float3(-1.0)), other.e45), 
            /* e423, e431, e412 */
            float3(anti_wedge_g0 * -1.0) * other.group4.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0 * -1.0) * other.group3.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Scalar {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float anti_wedge_g0 = other.e12345 * this.scalar;
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
public RejectViaOriginFromInfixPartial<Sphere> operator /(Sphere lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e45 * this.e3215)) + (right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group0.xyz);
        float4 anti_wedge_g2 = float4(0.0, 0.0, other.e315 * this.e4235 * -1.0, 0.0) + (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1)) + float4(-(other.group2.zx * this.group0.yz), 0.0, 0.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g2_xyz * float4(anti_wedge_g2[3]).xyz), -(anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g2[3] * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g1[3])) + (float3(anti_wedge_g2[3]) * other.group3.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g2.zxy) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g2.yzx)), anti_wedge_g2[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public AntiDualNum reject_via_origin_from(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * other.e3215 * this.e1234, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Plane reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float4(other.e321).xyz * float3(-1.0)), -(anti_wedge_g0[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0[2] * right_dual_g0_xyz[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Flector reject_via_origin_from(AntiFlector other) {
        float anti_wedge_g0_w = other.e1 * this.e4235;
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(anti_wedge_g0_w), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g0_w) * other.group1.xyz) + (float3(right_dual_g0[3] * this.e1234) * other.group0.xyz) - (float3(right_dual_g0[3] * other.e321) * this.group0.xyz)), anti_wedge_g0_w * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Plane reject_via_origin_from(AntiLine other) {
        float4 anti_wedge_g0 = float4(((float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0.yzx * anti_wedge_g0.zxy) - (right_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Flector reject_via_origin_from(AntiMotor other) {
        float4 anti_wedge_g1 = float4(0.0, 0.0, (other.e31 * this.e4235) - (other.e23 * this.e4315), 0.0) + float4(((float3(this.e1234) * other.group1.xyz) + float3(((other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx)), 0.0)), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * float4(other.e3215).xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, anti_wedge_g1[1] * other.e23 * -1.0, 0.0) + (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e15)) + float4(-(anti_wedge_g1.zx * other.group0.yz), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Plane reject_via_origin_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, other.e3 * other.e3 * this.e4125, 0.0) + float4(((float3(other.e1) * float3(other.e2 * this.e4315, other.e2 * this.e4235, other.e3 * this.e4235)) + (float3(other.e3) * float3(other.e1 * this.e4125, other.e2 * this.e4125, other.e2 * this.e4315)) + (float3(other.e5 * this.e1234) * other.group0.xyz) + float3((other.group0.xy * other.group0.xy * this.group0.xy), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e1234 */
            right_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group0.xyz);
        float3 anti_wedge_g2 = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx);
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group2.zxy) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g1_xyz * float3(other.e321)) - (anti_wedge_g0.zxy * other.group2.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * right_dual_g1_xyz[0]) - (anti_wedge_g0[1] * right_dual_g1_xyz[1]) - (anti_wedge_g0[2] * right_dual_g1_xyz[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4(((float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group0.xyz)), 0.0);
        float3 anti_wedge_g2_xyz = float3((other.e315 * this.e4125) - (other.e125 * this.e4315), (other.e125 * this.e4235) - (other.e235 * this.e4125), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + (float3(this.e3215) * other.group1.xyz);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(right_dual_g2_w), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_xyz * float3(right_dual_g2_w)), (right_dual_g2_w * other.e12345 * this.e1234) - (anti_wedge_g0[0] * right_dual_g1_xyz[0]) - (anti_wedge_g0[1] * right_dual_g1_xyz[1]) - (anti_wedge_g0[2] * right_dual_g1_xyz[2]) - (other.e423 * anti_wedge_g1[0]) - (other.e431 * anti_wedge_g1[1]) - (other.e412 * anti_wedge_g1[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g1[3])) + (float3(right_dual_g2_w) * anti_wedge_g3.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (float3(other.e321) * anti_wedge_g1.xyz) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), right_dual_g2_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (right_dual_g0 * float3(other.e45 * this.e3215)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group3.xyz);
        float anti_wedge_g2_w = other.e41 * this.e4235;
        float4 anti_wedge_g3 = float4(0.0, 0.0, (other.e31 * this.e4235) - (other.e23 * this.e4315), 0.0) + float4(((float3(this.e1234) * other.group2.xyz) + float3(((other.group1.zx * this.group0.yz) - (other.group1.yz * this.group0.zx)), 0.0) - (float3(this.e3215) * other.group0.xyz)), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(anti_wedge_g2_w) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.zx * other.group3.yz), anti_wedge_g3[1] * other.e4235, anti_wedge_g2_w * other.e3215) - (anti_wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) - (other.e41 * anti_wedge_g3[0]) - (anti_wedge_g1[3] * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e35)) + (anti_wedge_g3.wwwx * float4(other.group0.xyz, other.e15)) + float4(((anti_wedge_g0 * float3(other.e3215)) + float3(((anti_wedge_g1.yz * other.group3.zx) - (anti_wedge_g1.zx * other.group3.yz)), (anti_wedge_g1[0] * other.e4315) - (anti_wedge_g1[1] * other.e4235)) - (anti_wedge_g2_xyz * float3(other.e1234)) - (float3(anti_wedge_g2_w) * other.group2.xyz)), (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g3[1] * other.e25) - (anti_wedge_g2_xyz[0] * other.e4235) - (anti_wedge_g2_xyz[1] * other.e4315) - (anti_wedge_g2_xyz[2] * other.e4125)) - (anti_wedge_g3.zxyw * other.group1.yzxw)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public VersorOdd reject_via_origin_from(DualNum other) {
        float anti_wedge_g0_w = other.e5 * this.e1234;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), anti_wedge_g0_w * other.e12345 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e12345 * other.e12345 * this.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g3.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(anti_wedge_g0_w * other.e5) - (other.e12345 * anti_wedge_g3[3]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(FlatPoint other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) + (other.e45 * other.e45 * this.e3215) + (other.e15 * other.e45 * this.e4235) + (other.e25 * other.e45 * this.e4315) + (other.e35 * other.e45 * this.e4125)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e45
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz;
        float2 anti_wedge_g1_xy = (other.group1.yz * this.group0.zx) - (other.group1.zx * this.group0.yz);
        float anti_wedge_g1_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group1.xyz);
        float anti_wedge_g2_w = other.e45 * this.e1234;
        float3 anti_wedge_g3_xyz = float3(this.e1234) * other.group0.xyz;
        float anti_wedge_g3_w = -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2_w * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3_xyz.zxy * other.group1.yzx) - (anti_wedge_g3_xyz.yzx * other.group1.zxy)), anti_wedge_g2_w * other.e3215), 
            /* e15, e25, e35, e1234 */
            (other.group1.wwwx * float4(anti_wedge_g3_xyz, anti_wedge_g0[0])) + float4((float3(anti_wedge_g3_w) * other.group1.xyz), (anti_wedge_g2_w * other.e45) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g1_z * other.e4315) - (anti_wedge_g2_w * other.e15), -(anti_wedge_g2_w * other.e25) - (anti_wedge_g1_xy[0] * other.e4125), (anti_wedge_g1_xy[0] * other.e4315) - (anti_wedge_g2_w * other.e35) - (anti_wedge_g1_xy[1] * other.e4235), (anti_wedge_g3_xyz[0] * other.e15) + (anti_wedge_g3_xyz[1] * other.e25) - (anti_wedge_g3_w * other.e45) - (anti_wedge_g2_xyz[0] * other.e4235) - (anti_wedge_g2_xyz[1] * other.e4315)) + float4(((anti_wedge_g0 * float3(other.e3215)) + float3((float2(anti_wedge_g1_xy[1], anti_wedge_g1_z) * other.group1.zx), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group1.zxy) - (anti_wedge_g0.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group0.xyz) - (float3(other.e415) * other.group0.yyz * this.group0.yxx) - (float3(other.e435) * other.group0.xyy * this.group0.zzy)), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public VersorOdd reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = float4(this.e1234) * float4(other.group0.xyz, other.e5);
        float anti_wedge_g1_w = -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w * this.e1234) * other.group1.xyz)), anti_wedge_g1_w * right_dual_g0_w), 
            /* e15, e25, e35, e1234 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w) * float3(other.e315 * this.e4125, other.e125 * this.e4235, other.e235 * this.e4315)) + (float3(right_dual_g0_w * this.e3215) * other.group0.xyz) - (float3(right_dual_g0_w) * float3(other.e125 * this.e4315, other.e235 * this.e4125, other.e315 * this.e4235))), right_dual_g0_w * other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1_w)) + (float3(right_dual_g0_w) * anti_wedge_g3.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g0.zxy)), right_dual_g0_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234);
        float4 anti_wedge_g1 = (this.group0.yzxx * float4(other.group5.zxy, other.e41)) + float4(((float3(this.e1234) * other.group3.xyz) - (float3(this.e3215) * other.group4.xyz) - (other.group5.yzx * this.group0.zxy)), other.e42 * this.e4315);
        float anti_wedge_g2 = -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = float4(((float3(this.e3215) * other.group6.xyz) + (other.group8.yzx * this.group0.zxy) - (other.group8.zxy * this.group0.yzx)), 0.0);
        float3 anti_wedge_g4 = (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group0.xyz);
        float4 anti_wedge_g6 = (other.group9.yzxw * float4(this.group0.zxy, this.e1234)) + float4(-(other.group9.zx * this.group0.yz), other.e4315 * this.e4235 * -1.0, this.e3215 * other.e1234 * -1.0);
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group0.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group0;
        float anti_wedge_g10 = other.e12345 * this.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(Plane other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        float2 anti_wedge_g1_xy = (other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz);
        float anti_wedge_g1_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (anti_wedge_g1_xy[0] * other.e4315) - (anti_wedge_g1_xy[1] * other.e4235), 0.0) + float4(((anti_wedge_g0 * float3(other.e3215)) + float3(((float2(anti_wedge_g1_xy[1], anti_wedge_g1_z) * other.group0.zx) - (float2(anti_wedge_g1_z, anti_wedge_g1_xy[0]) * other.group0.yz)), 0.0)), 0.0), 
            /* e1234 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(RoundPoint other) {
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
public Sphere operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public Sphere reject_via_origin_from(Sphere other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group0.xyz);
        float4 anti_wedge_g1 = (other.group0.yzxw * float4(this.group0.zxy, this.e1234)) + float4(-(other.group0.zx * this.group0.yz), other.e4315 * this.e4235 * -1.0, this.e3215 * other.e1234 * -1.0);
        float3 anti_wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (right_dual_g0_xyz.yzx * anti_wedge_g1.zxy) - (anti_wedge_g2 * float3(other.e1234)) - (right_dual_g0_xyz.zxy * anti_wedge_g1.yzx)), (anti_wedge_g2[0] * right_dual_g0_xyz[0]) + (anti_wedge_g2[1] * right_dual_g0_xyz[1])), 
            /* e1234 */
            -(anti_wedge_g0[0] * right_dual_g0_xyz[0]) - (anti_wedge_g0[1] * right_dual_g0_xyz[1]) - (anti_wedge_g0[2] * right_dual_g0_xyz[2]) - (anti_wedge_g1[3] * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(other.group0.zxy, other.e1)) + float4(((float3(this.e1234) * other.group1.xyz) + float3(-(this.group0.zx * other.group0.yz), this.e4315 * other.e423 * -1.0)), this.e4315 * other.e2);
        float4 anti_wedge_g1 = float4(((float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group0.xyz)), 0.0);
        float4 anti_wedge_g2 = float4((float3((this.e4125 * other.e315) - (this.e4315 * other.e125), (this.e4235 * other.e125) - (this.e4125 * other.e235), (this.e4315 * other.e235) - (this.e4235 * other.e315)) + (float3(this.e3215) * other.group1.xyz)), other.e12345 * this.e1234);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w) * anti_wedge_g0.xyz)), right_dual_g0_w * anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_dual_g0_w)) + (right_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(right_dual_g0_xyz[0] * anti_wedge_g1[0]) - (right_dual_g0_xyz[1] * anti_wedge_g1[1]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g0[1] * right_dual_g2[2]) - (anti_wedge_g0[2] * right_dual_g2[1]), (anti_wedge_g0[2] * right_dual_g2[0]) - (anti_wedge_g0[0] * right_dual_g2[2]), (anti_wedge_g0[0] * right_dual_g2[1]) - (anti_wedge_g0[1] * right_dual_g2[0])) + (float3(anti_wedge_g0[3]) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group0.xyz) + (right_dual_g0_xyz.yzx * anti_wedge_g2.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g2.yzx)), right_dual_g0_w * this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension Sphere {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz);
        float4 anti_wedge_g1 = float4((this.group0.zx * other.group3.yz), this.e4315 * other.e4235, other.e3215 * this.e1234) - (this.group0.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group3.xyz);
        float anti_wedge_g2_w = this.e4235 * other.e41;
        float4 anti_wedge_g3 = float4(0.0, 0.0, (this.e4235 * other.e31) - (this.e4315 * other.e23), 0.0) + float4(((float3(this.e1234) * other.group2.xyz) + float3(((this.group0.yz * other.group1.zx) - (this.group0.zx * other.group1.yz)), 0.0) - (float3(this.e3215) * other.group0.xyz)), 0.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g2_w)) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3_xyz.zxy * anti_wedge_g3.yzx), anti_wedge_g2_w * other.e3215) - (anti_wedge_g3.zxyw * float4(right_dual_g3_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g3.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g3[3]))), (anti_wedge_g2_w * other.e45) - (anti_wedge_g0[0] * right_dual_g3_xyz[0]) - (anti_wedge_g0[1] * right_dual_g3_xyz[1]) - (anti_wedge_g0[2] * right_dual_g3_xyz[2]) - (anti_wedge_g1[3] * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(anti_wedge_g3[3]) * other.group0.xyz) + (right_dual_g3_xyz.yzx * anti_wedge_g1.zxy) + float3(((anti_wedge_g3.yz * other.group1.zx) - (anti_wedge_g3.zx * other.group1.yz)), (anti_wedge_g3[0] * other.e31) - (anti_wedge_g3[1] * other.e23)) - (anti_wedge_g2_xyz * float3(other.e1234)) - (float3(anti_wedge_g2_w) * other.group2.xyz) - (right_dual_g3_xyz.zxy * anti_wedge_g1.yzx)), (anti_wedge_g2_xyz[0] * right_dual_g3_xyz[0]) + (anti_wedge_g2_xyz[1] * right_dual_g3_xyz[1]) + (anti_wedge_g2_xyz[2] * right_dual_g3_xyz[2]) + (anti_wedge_g1[3] * other.e3215))
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<VersorEven> operator /(VersorEven lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w * -1.0) * other.group2.xyz), (anti_wedge_g2_w * other.scalar) + (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_via_origin_from(AntiDipoleInversion other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float3 anti_wedge_g3_xyz = float3((other.e315 * this.e412) - (other.e125 * this.e431), (other.e125 * this.e423) - (other.e235 * this.e412), (other.e235 * this.e431) - (other.e315 * this.e423)) + (float3(other.e321) * this.group1.xyz) + (float3(this.e12345) * other.group3.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) - (other.group0.yzx * this.group2.zxy);
        float anti_wedge_g3_w = other.e5 * this.e12345;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e321)) + (right_dual_g2_xyz * float3(anti_wedge_g2_w)) + (float3(anti_wedge_g3_w) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3_w)) + (anti_wedge_g3_xyz.yzx * right_dual_g2_xyz.zxy) - (anti_wedge_g3_xyz.zxy * right_dual_g2_xyz.yzx)), (anti_wedge_g3_xyz[0] * other.e1) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (right_dual_g1_xyz[0] * anti_wedge_g1[0]) - (right_dual_g1_xyz[1] * anti_wedge_g1[1]) - (right_dual_g1_xyz[2] * anti_wedge_g1[2]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public Motor reject_via_origin_from(AntiDualNum other) {
        float4 anti_wedge_g0 = float4((this.group0.xyz * float3(float2(other.e3215), other.e3215)), (other.e3215 * this.e4) + (other.scalar * this.e12345));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * anti_wedge_g0[3]), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public Motor reject_via_origin_from(AntiFlatPoint other) {
        float4 anti_wedge_g1 = float4(0.0, 0.0, (other.e235 * this.e431) - (other.e315 * this.e423), 0.0) + float4(((float3(other.e321) * this.group1.xyz) + float3(((other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz)), 0.0)), 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g1.xyz * float4(other.e321).xyz), other.e321 * other.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz.zxy * anti_wedge_g1.yzx) - (right_dual_g0_xyz.yzx * anti_wedge_g1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public Motor reject_via_origin_from(AntiFlector other) {
        float3 anti_wedge_g1_xyz = float3((other.e315 * this.e412) - (other.e125 * this.e431), (other.e125 * this.e423) - (other.e235 * this.e412), (other.e235 * this.e431) - (other.e315 * this.e423)) + (float3(other.e321) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g1_xyz * float3(right_dual_g0_w * -1.0)), (anti_wedge_g1_xyz[0] * right_dual_g1_xyz[0]) + (anti_wedge_g1_xyz[1] * right_dual_g1_xyz[1]) + (anti_wedge_g1_xyz[2] * right_dual_g1_xyz[2]) - (right_dual_g0_w * other.e321 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz.yzx * right_dual_g0_xyz.zxy) - (anti_wedge_g1_xyz.zxy * right_dual_g0_xyz.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public Motor reject_via_origin_from(AntiLine other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_dual_g0 * float3(anti_wedge_g0_w)), -(anti_wedge_g0_xyz[0] * right_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0[2])), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public Motor reject_via_origin_from(AntiMotor other) {
        float4 anti_wedge_g0 = (float4(other.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e23) + (anti_wedge_g0[1] * other.e31) + (anti_wedge_g0[2] * other.e12) + (anti_wedge_g0[3] * other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public AntiScalar reject_via_origin_from(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual_g0 * other.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_via_origin_from(Circle other) {
        float anti_wedge_g2_w = -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float3 anti_wedge_g3_xyz = (float3(other.e321) * this.group1.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) + (other.group2.yzx * this.group0.zxy) - (other.group0.yzx * this.group2.zxy) - (other.group2.zxy * this.group0.yzx);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (anti_wedge_g3_xyz.zxy * other.group0.yzx) - (anti_wedge_g3_xyz.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g3_xyz * float3(other.e321)) + (float3(anti_wedge_g2_w) * other.group2.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g3_xyz.yzx * other.group2.zxy) - (anti_wedge_g3_xyz.zxy * other.group2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(CircleRotor other) {
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1);
        float anti_wedge_g2_w = other.e12345 * this.e5;
        float4 anti_wedge_g3 = float4((float3((other.e315 * this.e412) - (other.e125 * this.e431), (other.e125 * this.e423) - (other.e235 * this.e412), (other.e235 * this.e431) - (other.e315 * this.e423)) + (float3(other.e321) * this.group1.xyz) + (float3(other.e12345) * this.group3.xyz) + (float3(this.e321) * other.group1.xyz) + (other.group0.zxy * this.group2.yzx) - (other.group0.yzx * this.group2.zxy)), other.e12345 * this.e4);
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g2_w * other.e12345) * this.group0.xyz) + (float3(right_dual_g2_w * this.e12345) * other.group0.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx)), right_dual_g2_w * other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(anti_wedge_g2_w) * other.group0.xyz) + (float3(right_dual_g2_w) * anti_wedge_g1.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), right_dual_g2_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2_w)) + (float3(right_dual_g2_w * other.e12345) * this.group2.xyz) + (float3(right_dual_g2_w * this.e12345) * other.group2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), anti_wedge_g2_w * right_dual_g2_w), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1;
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(anti_wedge_g2_w), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(anti_wedge_g2_w), 
            /* e235, e315, e125, e12345 */
            float4((right_dual_g2 * float3(anti_wedge_g2_w)), -(anti_wedge_g0[0] * right_dual_g2[0]) - (anti_wedge_g0[1] * right_dual_g2[1]) - (anti_wedge_g0[2] * right_dual_g2[2]) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g1[0] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g1[2]) - (anti_wedge_g1[3] * right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(DipoleInversion other) {
        float4 anti_wedge_g0 = (other.group3.yzxy * float4(this.group0.zxy, this.e2)) + float4(((float3(other.e1234) * this.group1.xyz) + (float3(this.e12345) * other.group0.xyz) + float3(-(other.group3.zx * this.group0.yz), other.e4315 * this.e423 * -1.0)), (other.e1234 * this.e5) + (other.e4235 * this.e1) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412));
        float3 anti_wedge_g1_xyz = (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz) - (float3(this.e321) * other.group3.xyz);
        float anti_wedge_g1_w = other.e45 * this.e12345;
        float3 anti_wedge_g2_xyz = float3((other.e4125 * this.e315) - (other.e4315 * this.e125), (other.e4235 * this.e125) - (other.e4125 * this.e235), (other.e4315 * this.e235) - (other.e4235 * this.e315)) + (float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group2.xyz);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3;
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(other.e1234)) + (right_dual_g0 * float3(anti_wedge_g0[3])) + float3(((anti_wedge_g0.zx * other.group3.yz) - (anti_wedge_g0.yz * other.group3.zx)), (anti_wedge_g0[1] * other.e4235) - (anti_wedge_g0[0] * other.e4315))), (anti_wedge_g1_xyz[0] * other.e23) + (anti_wedge_g1_xyz[1] * other.e31) + (anti_wedge_g1_xyz[2] * other.e12) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g1_w * other.e45) - (anti_wedge_g2_xyz[0] * right_dual_g0[0]) - (anti_wedge_g2_xyz[1] * right_dual_g0[1]) - (anti_wedge_g2_xyz[2] * right_dual_g0[2]) - (anti_wedge_g3[0] * other.e4235) - (anti_wedge_g3[1] * other.e4315)), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g2_xyz * float3(other.e1234)) + (float3(anti_wedge_g1_w) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.yzx * other.group3.zxy) - (float3(anti_wedge_g0[3]) * other.group2.xyz) - (anti_wedge_g2_xyz.zxy * other.group3.yzx)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e12345) * this.group0;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * anti_wedge_g3[3]) - (other.e12345 * anti_wedge_g0[3])), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4((this.group2.xyz * float3(float2(other.e12345 * other.e12345 * -1.0), other.e12345 * other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e12345 * other.e12345 * this.e5) - (other.e5 * other.e12345 * this.e12345)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_via_origin_from(FlatPoint other) {
        float anti_wedge_g2_w = -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * right_dual_g0[3]), 
            /* e235, e315, e125, e12345 */
            right_dual_g0 * float4(float3(anti_wedge_g2_w), other.e45 * this.e12345) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(Flector other) {
        float4 anti_wedge_g0 = (other.group1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), -(other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15));
        float4 anti_wedge_g1 = (this.group0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4(0.0, 0.0, (other.e4315 * this.e235) - (other.e4235 * this.e315), 0.0) + float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group0.xyz) + float3(((other.group1.zx * this.group2.yz) - (other.group1.yz * this.group2.zx)), 0.0)), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4(-(anti_wedge_g0.yz * other.group1.zx), anti_wedge_g0[0] * other.e4315 * -1.0, -(anti_wedge_g1[3] * other.e45) - (other.e4235 * other.e4235 * this.e12345)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e3215), other.e45)) + (anti_wedge_g1.wwwx * other.group1.xyzx) + float4(float3(0.0), (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[1] * other.e4125) - (anti_wedge_g2[2] * other.e4315), (anti_wedge_g2[2] * other.e4235) - (anti_wedge_g2[0] * other.e4125), (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235)) + (float3(other.e3215) * anti_wedge_g1.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public CircleRotor reject_via_origin_from(Line other) {
        float anti_wedge_g2_w = -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412);
        float3 anti_wedge_g3_xyz = (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3_xyz[0] * other.e415) - (anti_wedge_g3_xyz[1] * other.e425) - (anti_wedge_g3_xyz[2] * other.e435)), 
            /* e235, e315, e125, e12345 */
            float4(((anti_wedge_g3_xyz.yzx * other.group1.zxy) - (anti_wedge_g3_xyz.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(Motor other) {
        float4 anti_wedge_g2 = (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 anti_wedge_g3 = float4((float3((other.e315 * this.e412) - (other.e125 * this.e431), (other.e125 * this.e423) - (other.e235 * this.e412), (other.e235 * this.e431) - (other.e315 * this.e423)) + (float3(other.e12345) * this.group3.xyz) + (float3(this.e321) * other.group0.xyz)), other.e12345 * this.e4);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w * other.e12345) * this.group0.xyz)), right_dual_g0_w * other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w * other.e12345) * this.group1.xyz) + (float3(right_dual_g0_w * this.e12345) * other.group0.xyz)), right_dual_g0_w * other.e12345 * this.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2[3])) + (float3(right_dual_g0_w) * anti_wedge_g2.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g3.zxy)), right_dual_g0_w * anti_wedge_g2[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (this.e5 * other.e1234) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group3) + float4(((float3(other.e321) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz) + (float3(this.e321) * other.group6.xyz) + (other.group7.zxy * this.group2.yzx) + (other.group8.yzx * this.group0.zxy) - (other.group7.yzx * this.group2.zxy) - (other.group8.zxy * this.group0.yzx)), other.e4 * this.e12345);
        float anti_wedge_g2 = (other.e12345 * this.e5) + (this.e12345 * other.e5) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125);
        float4 anti_wedge_g3 = float4((float3((other.e4125 * this.e315) - (other.e4315 * this.e125), (other.e4235 * this.e125) - (other.e4125 * this.e235), (other.e4315 * this.e235) - (other.e4235 * this.e315)) + (float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group3.xyz)), other.e45 * this.e12345);
        float3 anti_wedge_g4 = float3((other.e4315 * this.e412) - (other.e4125 * this.e431), (other.e4125 * this.e423) - (other.e4235 * this.e412), (other.e4235 * this.e431) - (other.e4315 * this.e423)) + (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group1.xyz);
        float3 anti_wedge_g5 = (float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group5.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz);
        float4 anti_wedge_g6 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group6);
        float3 anti_wedge_g7 = (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz);
        float3 anti_wedge_g8 = (float3(other.e12345) * this.group2.xyz) + (float3(this.e12345) * other.group8.xyz);
        float4 anti_wedge_g9 = float4(this.e12345) * other.group9;
        float anti_wedge_g10 = this.e12345 * other.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) + (right_dual_g0[0] * other.e12345 * this.e12345) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(Plane other) {
        float2 anti_wedge_g0_xy = (other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz);
        float anti_wedge_g0_z = (other.e4235 * this.e431) - (other.e4315 * this.e423);
        float anti_wedge_g0_w = other.e4235 * this.e1;
        float4 anti_wedge_g2 = float4(0.0, 0.0, (other.e4315 * this.e235) - (other.e4235 * this.e315), 0.0) + float4(((float3(other.e3215) * this.group1.xyz) + float3(((other.group0.zx * this.group2.yz) - (other.group0.yz * this.group2.zx)), 0.0)), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float2(anti_wedge_g0_z, anti_wedge_g0_xy[0]) * other.group0.yz) - (float2(anti_wedge_g0_xy[1], anti_wedge_g0_z) * other.group0.zx)), (anti_wedge_g0_xy[1] * other.e4235) - (anti_wedge_g0_xy[0] * other.e4315), other.e4235 * other.e4235 * this.e12345 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(1.0, 1.0, anti_wedge_g0_z, 0.0) * float4((float4(other.e3215).xyz * float3(anti_wedge_g0_xy, 1.0)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3((anti_wedge_g2[1] * other.e4125) - (anti_wedge_g2[2] * other.e4315), (anti_wedge_g2[2] * other.e4235) - (anti_wedge_g2[0] * other.e4125), (anti_wedge_g2[0] * other.e4315) - (anti_wedge_g2[1] * other.e4235)) + (float3(other.e3215 * other.e3215) * this.group0.xyz) - (float3(other.e3215 * this.e321) * other.group0.xyz)), anti_wedge_g0_w * other.e3215), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_w * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public AntiScalar reject_via_origin_from(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0;
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * anti_wedge_g0[0]) + (right_dual_g0_xyz[1] * anti_wedge_g0[1]) + (right_dual_g0_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g0[3] * other.e5) - (other.e4 * this.e12345 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public AntiScalar reject_via_origin_from(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g0 = (other.group0.yzxx * float4(this.group0.zxy, this.e1)) + float4(((float3(other.e1234) * this.group1.xyz) + float3(-(other.group0.zx * this.group0.yz), other.e4315 * this.e423 * -1.0)), other.e4315 * this.e2);
        float3 anti_wedge_g1_xyz = (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group0.xyz);
        float3 anti_wedge_g2_xyz = float3((other.e4125 * this.e315) - (other.e4315 * this.e125), (other.e4235 * this.e125) - (other.e4125 * this.e235), (other.e4315 * this.e235) - (other.e4235 * this.e315)) + (float3(other.e3215) * this.group1.xyz);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group0;
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual_g0.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(((anti_wedge_g1_xyz * float3(right_dual_g0[3])) + float3(-(anti_wedge_g0.zx * right_dual_g0.yz), anti_wedge_g0[1] * right_dual_g0[0] * -1.0)), anti_wedge_g3[0] * right_dual_g0[0]), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g0[3])) + (float3(other.e3215) * anti_wedge_g0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (anti_wedge_g2_xyz.zxy * right_dual_g0.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g0.zxy)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345);
        float4 anti_wedge_g1 = (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1);
        float4 anti_wedge_g2 = (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435));
        float4 anti_wedge_g3 = (other.group0.zxyw * float4(this.group2.yzx, this.e4)) + float4(((float3(other.e12345) * this.group3.xyz) + (float3(other.e321) * this.group1.xyz) + (float3(this.e12345) * other.group3.xyz) + (float3(this.e321) * other.group1.xyz) + float3(((other.group2.yz * this.group0.zx) - (other.group0.yz * this.group2.zx) - (other.group2.zx * this.group0.yz)), (other.e235 * this.e431) - (other.e423 * this.e315) - (other.e315 * this.e423))), other.e4 * this.e12345);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(right_dual_g0_w)) + (anti_wedge_g3.zxyx * float4(right_dual_g0_xyz.yzx, other.e1)) + float4(((right_dual_g1_xyz * float3(anti_wedge_g3[3])) - (right_dual_g0_xyz.zxy * anti_wedge_g3.yzx)), -(right_dual_g1_xyz[2] * anti_wedge_g1[2]) - (right_dual_g2_xyz[0] * anti_wedge_g0[0]) - (right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2]) - (anti_wedge_g2[3] * other.e4)), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g2[3])) + (right_dual_g2_xyz * float3(anti_wedge_g3[3])) + (float3(right_dual_g0_w) * anti_wedge_g1.xyz) + (float3(other.e321) * anti_wedge_g3.xyz)), right_dual_g0_w * anti_wedge_g1[3]), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g2[3])) + (float3(right_dual_g0_w) * anti_wedge_g2.xyz) + (right_dual_g2_xyz.zxy * anti_wedge_g3.yzx) - (right_dual_g2_xyz.yzx * anti_wedge_g3.zxy)), right_dual_g0_w * anti_wedge_g2[3]), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(RejectViaOriginFromInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorEven {
    public VersorEven reject_via_origin_from(VersorOdd other) {
        float4 anti_wedge_g0 = (this.group0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group1.xyz)), (this.e5 * other.e1234) + (this.e1 * other.e4235) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (this.group0.yzxx * float4(other.group3.zxy, other.e15));
        float3 anti_wedge_g1_xyz = (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e321) * other.group3.xyz);
        float anti_wedge_g1_w = this.e12345 * other.e45;
        float3 anti_wedge_g2_xyz = float3((this.e315 * other.e4125) - (this.e125 * other.e4315), (this.e125 * other.e4235) - (this.e235 * other.e4125), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + (float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g3[3])) + float3(((anti_wedge_g0.yz * right_dual_g3.zx) - (anti_wedge_g0.zx * right_dual_g3.yz)), (anti_wedge_g0[0] * right_dual_g3[1]) - (anti_wedge_g0[1] * right_dual_g3[0]))), (anti_wedge_g1_xyz[0] * other.e23) + (right_dual_g3[0] * this.e12345 * other.e4235) - (anti_wedge_g1_w * other.e45) - (right_dual_g2_xyz[1] * anti_wedge_g0[1]) - (right_dual_g2_xyz[2] * anti_wedge_g0[2])) - (anti_wedge_g0.wwwx * float4(other.group0.xyz, right_dual_g2_xyz[0])), 
            /* e415, e425, e435, e321 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g3[3])) + (float3(other.e3215) * anti_wedge_g0.xyz) - (float3(anti_wedge_g1_w) * right_dual_g3.xyz) - (float3(anti_wedge_g0[3]) * other.group1.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e3215)) + (right_dual_g2_xyz * float3(anti_wedge_g0[3])) + (anti_wedge_g2_xyz.zxy * right_dual_g3.yzx) - (anti_wedge_g2_xyz.yzx * right_dual_g3.zxy)), anti_wedge_g0[3] * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(anti_wedge_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public RejectViaOriginFromInfixPartial<VersorOdd> operator /(VersorOdd lhs, RejectViaOriginFromInfix rhs) {
    return RejectViaOriginFromInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_via_origin_from(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e45 * this.e3215)) + (right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(-(other.group2.zx * this.group3.yz), other.e315 * this.e4235 * -1.0, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43));
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(anti_wedge_g2[3]), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g2_xyz * float4(anti_wedge_g2[3]).xyz), -(anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g2[3] * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz * float3(right_dual_g1[3])) + (float3(anti_wedge_g2[3]) * other.group3.xyz) + (anti_wedge_g0.yzx * right_dual_g2_xyz.zxy) + (other.group0.yzx * anti_wedge_g2.zxy) - (anti_wedge_g0.zxy * right_dual_g2_xyz.yzx) - (other.group0.zxy * anti_wedge_g2.yzx)), anti_wedge_g2[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Flector reject_via_origin_from(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, other.e3215 * other.e3215, 0.0) * float4((this.group0.xyz * float3(float2(other.e3215 * other.e3215), 1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Flector reject_via_origin_from(AntiFlatPoint other) {
        float3 anti_wedge_g0_xyz = (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(other.e321 * this.e45 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz * float4(right_dual_g0[3]).xyz), -(anti_wedge_g0_xyz[0] * right_dual_g0[0]) - (anti_wedge_g0_xyz[1] * right_dual_g0[1]) - (anti_wedge_g0_xyz[2] * right_dual_g0[2]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Flector reject_via_origin_from(AntiFlector other) {
        float4 anti_wedge_g0 = float4((float3(this.e1234) * other.group0.xyz), (other.e1 * this.e4235) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (other.group0.wwwx * float4(this.group3.xyz, this.e41));
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(anti_wedge_g0[3]), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(right_dual_g0[3]) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Plane reject_via_origin_from(AntiLine other) {
        float4 anti_wedge_g0 = float4(((float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0.yzx * anti_wedge_g0.zxy) - (right_dual_g0.zxy * anti_wedge_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Flector reject_via_origin_from(AntiMotor other) {
        float3 anti_wedge_g1_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(other.e3215) * this.group0.xyz) + (float3(this.e1234) * other.group1.xyz);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1_xyz * float4(other.e3215).xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g1_xyz.yzx * other.group0.zxy) - (anti_wedge_g1_xyz.zxy * other.group0.yzx)), anti_wedge_g1_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Plane reject_via_origin_from(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, other.e3 * other.e3 * this.e4125, 0.0) + float4(((float3(other.e1) * float3(other.e2 * this.e4315, other.e2 * this.e4235, other.e3 * this.e4235)) + (float3(other.e3) * float3(other.e1 * this.e4125, other.e2 * this.e4125, other.e2 * this.e4315)) + (float3(other.e5 * this.e1234) * other.group0.xyz) + float3((other.group0.xy * other.group0.xy * this.group3.xy), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_via_origin_from(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0 * other.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0 * other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g0 * other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0 * other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(Circle other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float3 anti_wedge_g1_xyz = (float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz);
        float3 anti_wedge_g2_xyz = (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group3.zxy) - (other.group2.zxy * this.group3.yzx);
        float3 right_dual_g1_xyz = other.group1.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * right_dual_g1_xyz[0]) - (anti_wedge_g0[1] * right_dual_g1_xyz[1]) - (anti_wedge_g0[2] * right_dual_g1_xyz[2]) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group2.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz * float3(other.e321)) - (anti_wedge_g0.zxy * other.group2.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_via_origin_from(CircleRotor other) {
        float3 anti_wedge_g0_xyz = (float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy);
        float anti_wedge_g0_w = other.e12345 * this.scalar;
        float4 anti_wedge_g1 = float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz)), other.e12345 * this.e45);
        float3 anti_wedge_g2_xyz = float3((other.e315 * this.e4125) - (other.e125 * this.e4315), (other.e125 * this.e4235) - (other.e235 * this.e4125), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + (float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz);
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0_xyz * float3(right_dual_g2_w)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g2_w)) + (right_dual_g1 * float4(anti_wedge_g0_w)), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(right_dual_g2_w)) + (right_dual_g2_xyz * float3(anti_wedge_g0_w))), right_dual_g2_w * other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_dual_g2_w) * anti_wedge_g3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (anti_wedge_g0_xyz.yzx * right_dual_g2_xyz.zxy) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g0_xyz.zxy * right_dual_g2_xyz.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), right_dual_g2_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_via_origin_from(Dipole other) {
        float3 anti_wedge_g0_xyz = float3((other.e12 * this.e4315) - (other.e31 * this.e4125), (other.e23 * this.e4125) - (other.e12 * this.e4235), (other.e31 * this.e4235) - (other.e23 * this.e4315)) + (float3(this.e1234) * other.group2.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g0_w = (other.e41 * this.e4235) + (other.e42 * this.e4315);
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0 * float3(other.e15 * this.e4235)) + (right_dual_g0 * float3(other.e25 * this.e4315)) + (right_dual_g0 * float3(other.e35 * this.e4125)) + (right_dual_g0 * float3(other.e45 * this.e3215)) + (anti_wedge_g0_xyz.yzx * other.group1.zxy) - (float3(anti_wedge_g0_w) * other.group2.xyz) - (anti_wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            (anti_wedge_g0_w * other.e45) + (anti_wedge_g0_xyz[0] * right_dual_g0[0]) + (anti_wedge_g0_xyz[1] * right_dual_g0[1]) + (anti_wedge_g0_xyz[2] * right_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(DipoleInversion other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group3.xyz);
        float anti_wedge_g2_w = (other.e41 * this.e4235) - (other.e1234 * this.e45) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43);
        float4 anti_wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4((float3(((other.group1.zx * this.group3.yz) - (other.group3.yz * this.group1.zx)), (other.e31 * this.e4235) - (other.e4235 * this.e31)) - (float3(this.e3215) * other.group0.xyz)), (other.e4235 * this.e15) - (other.e45 * this.e3215) - (other.e35 * this.e4125)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15));
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(anti_wedge_g2_w) * other.group3.xyz) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g3.zx * other.group3.yz), anti_wedge_g3[1] * other.e4235, anti_wedge_g2_w * other.e3215) - (anti_wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group3.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) - (other.e41 * anti_wedge_g3[0]) - (anti_wedge_g1[3] * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e35)) + (anti_wedge_g3.wwwx * float4(other.group0.xyz, other.e15)) + float4(((anti_wedge_g0 * float3(other.e3215)) + float3(((anti_wedge_g1.yz * other.group3.zx) - (anti_wedge_g1.zx * other.group3.yz)), (anti_wedge_g1[0] * other.e4315) - (anti_wedge_g1[1] * other.e4235)) - (anti_wedge_g2_xyz * float3(other.e1234)) - (float3(anti_wedge_g2_w) * other.group2.xyz)), (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g3[1] * other.e25) - (anti_wedge_g2_xyz[0] * other.e4235) - (anti_wedge_g2_xyz[1] * other.e4315) - (anti_wedge_g2_xyz[2] * other.e4125)) - (anti_wedge_g3.zxyw * other.group1.yzxw)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_via_origin_from(DualNum other) {
        float4 anti_wedge_g0 = float4((this.group0.xyz * float3(float2(other.e12345), other.e12345)), (other.e5 * this.e1234) + (other.e12345 * this.scalar));
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.e12345 * -1.0), 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345 * other.e12345 * -1.0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g3.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * anti_wedge_g0[3]) - (other.e12345 * anti_wedge_g3[3]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_via_origin_from(FlatPoint other) {
        float4 anti_wedge_g0 = float4(this.e1234) * other.group0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(anti_wedge_g0[3]).xyz * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e15) + (anti_wedge_g0[1] * other.e25) + (anti_wedge_g0[2] * other.e35) + (other.e45 * other.e45 * this.e3215) + (other.e15 * other.e45 * this.e4235) + (other.e25 * other.e45 * this.e4315) + (other.e35 * other.e45 * this.e4125)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e45
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(Flector other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group1.xyz;
        float2 anti_wedge_g1_xy = (other.group1.yz * this.group3.zx) - (other.group1.zx * this.group3.yz);
        float anti_wedge_g1_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        float4 anti_wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), (other.e45 * this.e1234) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - (other.group1.xyzx * float4(float3(this.e3215), this.e41));
        float4 anti_wedge_g3 = (other.group1.zxyy * float4(this.group1.yzx, this.e25)) + (other.group1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(other.group0.xyz, other.e4235)) + float4(-(other.group1.yz * this.group1.zx), other.e4235 * this.e31 * -1.0, -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zx * other.group1.yz) - (anti_wedge_g3.yz * other.group1.zx)), (anti_wedge_g3[1] * other.e4235) - (anti_wedge_g3[0] * other.e4315), anti_wedge_g2[3] * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group1.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g2[3] * other.e45)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + float3(((float2(anti_wedge_g1_xy[1], anti_wedge_g1_z) * other.group1.zx) - (float2(anti_wedge_g1_z, anti_wedge_g1_xy[0]) * other.group1.yz)), (anti_wedge_g1_xy[0] * other.e4315) - (anti_wedge_g1_xy[1] * other.e4235))), (anti_wedge_g3[0] * other.e15) + (anti_wedge_g3[1] * other.e25) - (anti_wedge_g2[0] * other.e4235) - (anti_wedge_g3[3] * other.e45)) - (anti_wedge_g2.wwwy * float4(other.group0.xyz, other.e4315))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(Line other) {
        float3 anti_wedge_g0 = float3(this.e1234) * other.group0.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0.yzx * other.group1.zxy) - (anti_wedge_g0.zxy * other.group1.yzx) - (other.group0.xyz * other.group0.xyz * this.group3.xyz) - (float3(other.e415) * other.group0.yyz * this.group3.yxx) - (float3(other.e435) * other.group0.xyy * this.group3.zzy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_via_origin_from(Motor other) {
        float4 anti_wedge_g0 = (other.group0 * float4(float3(this.e1234), this.scalar)) + float4((float3(other.e12345) * this.group0.xyz), (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43));
        float anti_wedge_g1_w = other.e12345 * this.e45;
        float4 anti_wedge_g3 = float4(other.e12345) * this.group3;
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(right_dual_g0_w), 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w * other.e12345) * this.group1.xyz) + (float3(right_dual_g0_w * this.e1234) * other.group1.xyz)), anti_wedge_g1_w * right_dual_g0_w), 
            /* e15, e25, e35, e1234 */
            float4(((right_dual_g1_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w) * float3(other.e315 * this.e4125, other.e125 * this.e4235, other.e235 * this.e4315)) + (float3(right_dual_g0_w * other.e12345) * this.group2.xyz) + (float3(right_dual_g0_w * this.e3215) * other.group0.xyz) - (float3(right_dual_g0_w) * float3(other.e125 * this.e4315, other.e235 * this.e4125, other.e315 * this.e4235))), right_dual_g0_w * other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g1_w)) + (float3(right_dual_g0_w) * anti_wedge_g3.xyz) + (right_dual_g1_xyz.zxy * anti_wedge_g0.yzx) - (right_dual_g1_xyz.yzx * anti_wedge_g0.zxy)), right_dual_g0_w * anti_wedge_g3[3])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public MultiVector reject_via_origin_from(MultiVector other) {
        float anti_wedge_g0_x = (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (this.e1234 * other.e5) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45);
        float4 anti_wedge_g1 = (this.group3.yzxx * float4(other.group5.zxy, other.e41)) + float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e1234) * other.group3.xyz) + float3((other.group9.zx * this.group1.yz), other.e4315 * this.e23) - (float3(this.e3215) * other.group4.xyz) - (other.group5.yzx * this.group3.zxy)), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e4235 * this.e41) - (other.e4315 * this.e42)) - (float4(other.e1234) * float4(this.group2.xyz, this.e45)) - (other.group9.yzxz * float4(this.group1.zxy, this.e43));
        float anti_wedge_g2 = (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215);
        float4 anti_wedge_g3 = float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group6.xyz) + (other.group8.yzx * this.group3.zxy) - (other.group8.zxy * this.group3.yzx)), other.e12345 * this.e45);
        float3 anti_wedge_g4 = (float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group3.yzx) - (other.group7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group3.xyz);
        float4 anti_wedge_g6 = (other.group9.yzxw * float4(this.group3.zxy, this.e1234)) + float4(-(other.group9.zx * this.group3.yz), other.e4315 * this.e4235 * -1.0, this.e3215 * other.e1234 * -1.0);
        float3 anti_wedge_g7 = (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz);
        float4 anti_wedge_g9 = float4(other.e12345) * this.group3;
        float anti_wedge_g10 = other.e12345 * this.e1234;
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * right_dual_g0[0], (anti_wedge_g0_x * right_dual_g0[1]) + (anti_wedge_g10 * other.e3215) + (anti_wedge_g2 * right_dual_g10) + (anti_wedge_g5[0] * other.e23) + (anti_wedge_g5[1] * other.e31) + (anti_wedge_g5[2] * other.e12) + (anti_wedge_g1[0] * right_dual_g9[0]) + (anti_wedge_g1[1] * right_dual_g9[1]) + (anti_wedge_g1[2] * right_dual_g9[2]) + (anti_wedge_g1[3] * right_dual_g9[3]) + (anti_wedge_g9[0] * right_dual_g1[0]) + (anti_wedge_g9[1] * right_dual_g1[1]) + (anti_wedge_g9[2] * right_dual_g1[2]) + (anti_wedge_g9[3] * right_dual_g1[3]) - (anti_wedge_g4[0] * right_dual_g8[0]) - (anti_wedge_g4[1] * right_dual_g8[1]) - (anti_wedge_g4[2] * right_dual_g8[2]) - (anti_wedge_g7[0] * right_dual_g3[0]) - (anti_wedge_g7[1] * right_dual_g3[1]) - (anti_wedge_g7[2] * right_dual_g3[2]) - (anti_wedge_g8[0] * other.e423) - (anti_wedge_g8[1] * other.e431) - (anti_wedge_g8[2] * other.e412) - (right_dual_g5[0] * anti_wedge_g6[0]) - (right_dual_g5[1] * anti_wedge_g6[1]) - (right_dual_g5[2] * anti_wedge_g6[2]) - (right_dual_g7[0] * anti_wedge_g3[0]) - (right_dual_g7[1] * anti_wedge_g3[1]) - (right_dual_g7[2] * anti_wedge_g3[2]) - (anti_wedge_g3[3] * other.e45) - (anti_wedge_g6[3] * right_dual_g3[3])), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(right_dual_g0[0])) + (right_dual_g1 * float4(anti_wedge_g0_x)), 
            /* e5 */
            (anti_wedge_g0_x * other.e3215) + (anti_wedge_g2 * right_dual_g0[0]), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g1 * float4(other.e3215)) + (anti_wedge_g3 * float4(right_dual_g0[0])) + (right_dual_g3 * float4(anti_wedge_g0_x)) - (right_dual_g1 * float4(anti_wedge_g2)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(right_dual_g0[0])) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * anti_wedge_g1.xyz), 
            /* e23, e31, e12 */
            float3((anti_wedge_g1[1] * right_dual_g1[2]) - (anti_wedge_g1[2] * right_dual_g1[1]), (anti_wedge_g1[2] * right_dual_g1[0]) - (anti_wedge_g1[0] * right_dual_g1[2]), (anti_wedge_g1[0] * right_dual_g1[1]) - (anti_wedge_g1[1] * right_dual_g1[0])) + (anti_wedge_g5 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g0_x)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(right_dual_g0[0])) + float4(((anti_wedge_g4 * float3(other.e3215)) + (float3(anti_wedge_g2) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g3.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g3.xyz) - (float3(anti_wedge_g0_x) * other.group5.xyz) - (float3(anti_wedge_g3[3]) * right_dual_g1.xyz) - (float3(right_dual_g3[3]) * anti_wedge_g1.xyz)), anti_wedge_g0_x * other.e45), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(right_dual_g1[3])) + (anti_wedge_g7 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g1[3])) + (right_dual_g7 * float3(anti_wedge_g0_x)) + (anti_wedge_g4.yzx * right_dual_g1.zxy) + (other.group7.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * right_dual_g1.yzx) - (other.group7.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            float3((anti_wedge_g1[1] * right_dual_g3[2]) + (anti_wedge_g3[2] * right_dual_g1[1]) - (anti_wedge_g1[2] * right_dual_g3[1]) - (anti_wedge_g3[1] * right_dual_g1[2]), (anti_wedge_g1[2] * right_dual_g3[0]) + (anti_wedge_g3[0] * right_dual_g1[2]) - (anti_wedge_g1[0] * right_dual_g3[2]) - (anti_wedge_g3[2] * right_dual_g1[0]), (anti_wedge_g1[0] * right_dual_g3[1]) + (anti_wedge_g3[1] * right_dual_g1[0]) - (anti_wedge_g1[1] * right_dual_g3[0]) - (anti_wedge_g3[0] * right_dual_g1[1])) + (anti_wedge_g5 * float3(other.e3215)) + (anti_wedge_g8 * float3(right_dual_g0[0])) + (right_dual_g5 * float3(anti_wedge_g2)) + (right_dual_g8 * float3(anti_wedge_g0_x)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(right_dual_g0[0])) + (right_dual_g9 * float4(anti_wedge_g0_x)) + float4(((anti_wedge_g5 * float3(right_dual_g3[3])) + (anti_wedge_g7 * float3(other.e3215)) + (right_dual_g5 * float3(anti_wedge_g3[3])) + (right_dual_g8 * float3(anti_wedge_g1[3])) + (anti_wedge_g4.yzx * right_dual_g3.zxy) + (other.group5.zxy * anti_wedge_g1.yzx) + (other.group7.yzx * anti_wedge_g3.zxy) + float3(((anti_wedge_g6.zx * right_dual_g1.yz) - (anti_wedge_g6.yz * right_dual_g1.zx)), (anti_wedge_g6[1] * right_dual_g1[0]) - (anti_wedge_g6[0] * right_dual_g1[1])) - (anti_wedge_g8 * float3(right_dual_g1[3])) - (right_dual_g7 * float3(anti_wedge_g2)) - (anti_wedge_g4.zxy * right_dual_g3.yzx) - (other.group5.yzx * anti_wedge_g1.zxy) - (other.group7.zxy * anti_wedge_g3.yzx)), (anti_wedge_g8[0] * right_dual_g1[0]) + (anti_wedge_g8[1] * right_dual_g1[1]) + (anti_wedge_g8[2] * right_dual_g1[2]) + (anti_wedge_g6[3] * other.e3215)), 
            /* e1234 */
            (anti_wedge_g0_x * right_dual_g10) + (anti_wedge_g10 * right_dual_g0[0]) + (right_dual_g7[0] * anti_wedge_g1[0]) + (right_dual_g7[1] * anti_wedge_g1[1]) + (right_dual_g7[2] * anti_wedge_g1[2]) + (anti_wedge_g1[3] * other.e45) - (anti_wedge_g4[0] * right_dual_g5[0]) - (anti_wedge_g4[1] * right_dual_g5[1]) - (anti_wedge_g4[2] * right_dual_g5[2]) - (anti_wedge_g5[0] * other.e423) - (anti_wedge_g5[1] * other.e431) - (anti_wedge_g5[2] * other.e412) - (anti_wedge_g7[0] * right_dual_g1[0]) - (anti_wedge_g7[1] * right_dual_g1[1]) - (anti_wedge_g7[2] * right_dual_g1[2]) - (anti_wedge_g6[3] * right_dual_g1[3])
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(Plane other) {
        float2 anti_wedge_g1_xy = (other.group0.yz * this.group3.zx) - (other.group0.zx * this.group3.yz);
        float anti_wedge_g1_z = (other.e4235 * this.e4315) - (other.e4315 * this.e4235);
        float4 anti_wedge_g3 = float4(0.0, 0.0, other.e4235 * this.e31 * -1.0, 0.0) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25)) + float4(-(other.group0.yz * this.group1.zx), 0.0, 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0, 0.0, (anti_wedge_g3[1] * other.e4235) - (anti_wedge_g3[0] * other.e4315), 0.0) + float4(((anti_wedge_g3.zx * other.group0.yz) - (anti_wedge_g3.yz * other.group0.zx)), 0.0, 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g3[3]) * other.group0.xyz) + (float3(other.e3215) * anti_wedge_g3.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215 * this.e1234) * other.group0) + (float4(anti_wedge_g1_xy[1], anti_wedge_g1_z, anti_wedge_g1_xy[0], other.e4235 * this.e3215) * other.group0.zxyx) - (float4(anti_wedge_g1_z, anti_wedge_g1_xy[0], anti_wedge_g1_xy[1], other.e3215 * this.e4235) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public Sphere reject_via_origin_from(RoundPoint other) {
        float anti_wedge_g0 = (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (this.e1234 * other.e5);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            anti_wedge_g0 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(Sphere other) {
        float4 anti_wedge_g1 = (other.group0.yzxw * float4(this.group3.zxy, this.e1234)) + float4(-(other.group0.zx * this.group3.yz), other.e4315 * this.e4235 * -1.0, this.e3215 * other.e1234 * -1.0);
        float3 anti_wedge_g2_xyz = (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group0.xyz);
        float anti_wedge_g2_w = this.e45 * other.e1234 * -1.0;
        float4 anti_wedge_g3 = float4(0.0, 0.0, other.e4235 * this.e31 * -1.0, 0.0) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + (other.group0.wwwy * float4(this.group0.xyz, this.e25)) + float4((float3(-(other.group0.yz * this.group1.zx), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0);
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(anti_wedge_g2_w)) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0_xyz.zxy * anti_wedge_g3.yzx), anti_wedge_g2_w * other.e3215) - (anti_wedge_g3.zxyw * float4(right_dual_g0_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g3.xyz) - (right_dual_g0_xyz * float3(anti_wedge_g3[3]))), anti_wedge_g1[3] * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(other.e4235 * this.e1234, other.e4315 * this.e1234, other.e4125 * this.e1234, anti_wedge_g1[3])) + float4(((right_dual_g0_xyz.yzx * anti_wedge_g1.zxy) - (anti_wedge_g2_xyz * float3(other.e1234)) - (float3(other.e3215 * other.e1234) * this.group3.xyz) - (right_dual_g0_xyz.zxy * anti_wedge_g1.yzx)), anti_wedge_g2_xyz[0] * right_dual_g0_xyz[0])
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public VersorOdd reject_via_origin_from(VersorEven other) {
        float4 anti_wedge_g0 = (other.group0.zxyw * float4(this.group3.yzx, this.scalar)) + float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group1.xyz)), (other.e5 * this.e1234) + (other.e1 * this.e4235) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (other.group0.yzxx * float4(this.group3.zxy, this.e15));
        float4 anti_wedge_g1 = float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group2.xyz) + (float3(this.e3215) * other.group0.xyz) - (float3(other.e321) * this.group3.xyz)), other.e12345 * this.e45);
        float4 anti_wedge_g2 = float4((float3((other.e315 * this.e4125) - (other.e125 * this.e4315), (other.e125 * this.e4235) - (other.e235 * this.e4125), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + (float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz)), other.e12345 * this.e1234);
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_dual_g0_xyz * float3(anti_wedge_g0[3])) + (float3(right_dual_g0_w) * anti_wedge_g0.xyz)), right_dual_g0_w * anti_wedge_g0[3]), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(right_dual_g0_w)) + (right_dual_g1 * float4(anti_wedge_g0[3])), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(right_dual_g0_w)) + (right_dual_g2 * float4(anti_wedge_g0[3])) + float4(float3(0.0), -(right_dual_g0_xyz[0] * anti_wedge_g1[0]) - (right_dual_g0_xyz[1] * anti_wedge_g1[1]) - (right_dual_g0_xyz[2] * anti_wedge_g1[2]) - (anti_wedge_g0[0] * right_dual_g1[0]) - (anti_wedge_g0[1] * right_dual_g1[1]) - (anti_wedge_g0[2] * right_dual_g1[2])), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((anti_wedge_g0[1] * right_dual_g2[2]) - (anti_wedge_g0[2] * right_dual_g2[1]), (anti_wedge_g0[2] * right_dual_g2[0]) - (anti_wedge_g0[0] * right_dual_g2[2]), (anti_wedge_g0[0] * right_dual_g2[1]) - (anti_wedge_g0[1] * right_dual_g2[0])) + (float3(anti_wedge_g0[3]) * other.group3.xyz) + (float3(anti_wedge_g1[3]) * right_dual_g1.xyz) + (float3(right_dual_g1[3]) * anti_wedge_g1.xyz) + (float3(right_dual_g0_w * other.e12345) * this.group3.xyz) + (right_dual_g0_xyz.yzx * anti_wedge_g2.zxy) - (right_dual_g0_xyz.zxy * anti_wedge_g2.yzx)), right_dual_g0_w * other.e12345 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(RejectViaOriginFromInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.reject_via_origin_from(rhs);
}
public extension VersorOdd {
    public DipoleInversion reject_via_origin_from(VersorOdd other) {
        float3 anti_wedge_g0 = (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz);
        float4 anti_wedge_g1 = (other.group3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(float3(other.e3215), other.e41)) + float4(-(float3(this.e3215) * other.group3.xyz), -(other.e1234 * this.e45) - (other.e4235 * this.e41) - (other.e4315 * this.e42) - (other.e4125 * this.e43));
        float4 anti_wedge_g3 = (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + float4(((other.group1.zx * this.group3.yz) - (other.group3.yz * this.group1.zx)), (other.e31 * this.e4235) - (other.e4235 * this.e31), (other.e4235 * this.e15) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15));
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(anti_wedge_g2[3])) - (float3(other.e1234) * anti_wedge_g3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3_xyz.zxy * anti_wedge_g3.yzx), anti_wedge_g2[3] * other.e3215) - (anti_wedge_g3.zxyw * float4(right_dual_g3_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * anti_wedge_g3.xyz) - (right_dual_g3_xyz * float3(anti_wedge_g3[3]))), (anti_wedge_g2[3] * other.e45) - (anti_wedge_g0[0] * right_dual_g3_xyz[0]) - (anti_wedge_g0[1] * right_dual_g3_xyz[1]) - (anti_wedge_g0[2] * right_dual_g3_xyz[2]) - (anti_wedge_g1[3] * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0 * float3(other.e3215)) + (float3(anti_wedge_g3[3]) * other.group0.xyz) + (right_dual_g3_xyz.yzx * anti_wedge_g1.zxy) + float3(((anti_wedge_g3.yz * other.group1.zx) - (anti_wedge_g3.zx * other.group1.yz)), (anti_wedge_g3[0] * other.e31) - (anti_wedge_g3[1] * other.e23)) - (float3(anti_wedge_g2[3]) * other.group2.xyz) - (float3(other.e1234) * anti_wedge_g2.xyz) - (right_dual_g3_xyz.zxy * anti_wedge_g1.yzx)), (right_dual_g3_xyz[0] * anti_wedge_g2[0]) + (right_dual_g3_xyz[1] * anti_wedge_g2[1]) + (right_dual_g3_xyz[2] * anti_wedge_g2[2]) + (anti_wedge_g1[3] * other.e3215))
        );
    }
}
