implementing cga3d;
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2_w) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), 0.0) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g3_w * this.e45) + (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g1) * other.group3.xyz), 0.0) - (anti_wedge_g0 * float4(float3(other.e5), other.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_via_horizon_onto(AntiDualNum other) {
        float4 anti_wedge_g2 = float4(other.scalar) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4((float3(other.scalar) * this.group0.xyz), anti_wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * anti_wedge_g2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_wedge_g2[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e42) - (right_dual_g1_xyz[2] * this.e43)) + float4((float3(right_dual_g1_w) * this.group0.xyz), 0.0) + float4((right_dual_g1_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g1_xyz.yzx * this.group1.zxy), right_dual_g1_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g1_w * this.e45) + (right_dual_g1_xyz[0] * this.e15) + (right_dual_g1_xyz[1] * this.e25) + (right_dual_g1_xyz[2] * this.e35);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) + float3(float2(0.0), anti_wedge_g0[0] * other.e2 * -1.0) - float3((anti_wedge_g0.yz * other.group1.zx), 0.0)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e321 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiLine anti_project_via_horizon_onto(AntiLine other) {
        float anti_wedge_g0 = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_via_horizon_onto(AntiMotor other) {
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(anti_wedge_g0, anti_wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g2_w) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.zxyw * float4(anti_wedge_g0.yzx, anti_wedge_g2_w)) + float4(float3(0.0), -(anti_wedge_g2_xyz[0] * other.e23) - (anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0) - (other.group1.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public Dipole anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e41);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) + float3(float2(0.0), anti_wedge_g0[0] * other.e2 * -1.0) - float3((anti_wedge_g0.yz * other.group0.zx), 0.0)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35 */
            (right_dual_g0_xyz * float3(this.e15 * other.e1, this.e25 * other.e2, this.e35 * other.e3)) + (float3(right_dual_g0_w * this.e45) * other.group0.xyz) + (float3(this.e25 * other.e1, this.e15 * other.e2, this.e15 * other.e3) * right_dual_g0_xyz.yxx) + (float3(this.e35 * other.e1, this.e35 * other.e2, this.e25 * other.e3) * right_dual_g0_xyz.zzy) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public Dipole anti_project_via_horizon_onto(Dipole other) {
        float anti_wedge_g0 = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float anti_wedge_g0 = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint anti_project_via_horizon_onto(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * other.e5) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint anti_project_via_horizon_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            (float4(this.e41 * other.e15, this.e42 * other.e25, this.e43 * other.e35, this.e41 * other.e15) * other.group0) + (float4(this.e42 * other.e25, this.e41 * other.e15, this.e41 * other.e15, this.e42 * other.e25) * other.group0) + (other.group0 * float4(float2(this.e43 * other.e35), this.e42 * other.e25, this.e43 * other.e35)) - (float4(this.e45 * other.e45) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float anti_wedge_g0 = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float4 anti_wedge_g0 = float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float anti_wedge_g0_x = (right_dual_g0[1] * this.scalar) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (this.e45 * other.e45);
        float4 anti_wedge_g1 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g9_xyz[1] * this.e42) - (right_dual_g9_xyz[2] * this.e43)) + float4((float3(right_dual_g9_w) * this.group0.xyz), 0.0) + float4((right_dual_g9_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g9_xyz.yzx * this.group1.zxy), right_dual_g9_xyz[0] * this.e41);
        float anti_wedge_g2 = (right_dual_g9_w * this.e45) + (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35);
        float4 anti_wedge_g3 = float4(right_dual_g0[1]) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(right_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(right_dual_g0[1]) * this.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public Dipole anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g0_w * this.e45) + (right_dual_g0_xyz[0] * this.e15) + (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1) * other.group0.yzxw) - (anti_wedge_g0.yzxw * float4(other.group0.zxy, other.e5)), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g1) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_project_via_horizon_onto(Scalar other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.scalar * other.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g3_w * this.e45) + (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1) * other.group3.yzxw) - (float4(other.e3, other.e1, other.e2, other.e5) * anti_wedge_g0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g1) * other.group3.xyz), 0.0) - (anti_wedge_g0 * float4(float3(other.e5), other.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float4 anti_wedge_g2 = float4((float3(other.scalar) * this.group2.xyz), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (right_dual_g2_xyz[0] * this.e41) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2[3]) * other.group0.xyz)), anti_wedge_g2[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(float3(other.scalar), other.e1234)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)) + float4((float3(anti_wedge_g2[3]) * other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23)) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (other.e41 * this.e415) + (other.e42 * this.e425) + (other.e43 * this.e435) + (other.e23 * this.e423) + (other.e31 * this.e431) + (other.e12 * this.e412) + (other.scalar * this.e4);
        float4 anti_wedge_g3 = (float4(other.scalar) * this.group3) + (float4(other.e42, other.e43, other.e41, other.e23) * this.group2.zxyx) + (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) + (other.group1.wwwy * float4(this.group1.xyz, this.e315)) + float4(float3(0.0), (other.e12 * this.e125) + (other.e25 * this.e425) + (other.e35 * this.e435)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g0 * float3(other.scalar)), 0.0) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g0[0] * other.e15) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            (other.group2 * float4(float3(anti_wedge_g2_w), anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) + float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (float3(right_dual_g2_w) * this.group1.xyz) + (right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e425) - (right_dual_g3_xyz[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g3_xyz, right_dual_g3_xyz[0]));
        float4 anti_wedge_g2 = (this.group1 * float4(float3(right_dual_g3_w), other.e321)) + (this.group2.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g3_w)) + float4(float3(0.0), (right_dual_g2_w * this.e5) + (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (other.e431 * this.e315) - (other.e412 * this.e125)) - (this.group2.zxyx * float4(right_dual_g3_xyz.yzx, other.e423));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group3.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_via_horizon_onto(AntiDualNum other) {
        float4 anti_wedge_g2 = float4(other.scalar) * this.group2;
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            other.group0.xy.yyyx * float4((float3(other.scalar) * this.group0.xyz), anti_wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * float4(anti_wedge_g2.xyz, anti_wedge_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiFlatPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            -(float4(this.e321 * other.e321 * -1.0) * other.group0) - (float4(right_dual_g0_xyz[0] * this.e423) * other.group0) - (float4(right_dual_g0_xyz[1] * this.e431) * other.group0) - (float4(right_dual_g0_xyz[2] * this.e412) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g1_xyz.yzx * this.group0.zxy) - (right_dual_g1_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435)) + float4((float3(right_dual_g1_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g1_xyz, right_dual_g1_xyz[0]));
        float4 anti_wedge_g2 = (this.group1 * float4(float3(right_dual_g1_w), other.e321)) + (this.group2.yzxw * float4(right_dual_g1_xyz.zxy, right_dual_g1_w)) + float4(float3(0.0), (right_dual_g1_xyz[0] * this.e1) + (right_dual_g1_xyz[1] * this.e2) + (right_dual_g1_xyz[2] * this.e3) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412)) - float4((right_dual_g1_xyz.yzx * this.group2.zxy), right_dual_g0_xyz[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)) + float4((anti_wedge_g0.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group1.yz), 0.0) - float3((anti_wedge_g2.yz * other.group1.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Circle anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) + float4((right_dual_g0 * float3(this.e321)), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - float4((this.group0.zxy * other.group1.yzx), right_dual_g0[0] * this.e423);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125 */
            (float3(this.e415 * other.e15) * other.group0.xyz) + (float3(this.e425 * other.e25) * other.group0.xyz) + (float3(this.e435 * other.e35) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g0.yzx) - (float3(right_dual_g0[0] * this.e235) * other.group0.xyz) - (float3(right_dual_g0[1] * this.e315) * other.group0.xyz) - (float3(right_dual_g0[2] * this.e125) * other.group0.xyz) - (other.group1.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float anti_wedge_g2_w = (right_dual_g0[3] * this.e4) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412);
        float4 anti_wedge_g3 = (right_dual_g0 * float4(float3(this.e321), this.e5)) + (float4(this.e431, this.e412, this.e423, this.e415) * other.group1.zxyx) + float4(float3(0.0), (this.e425 * other.e25) + (this.e435 * other.e35) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) + float4((float3(right_dual_g0[3]) * this.group3.xyz), 0.0) - float4((this.group0.zxy * other.group1.yzx), right_dual_g0[0] * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2_w) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g0 * float3(other.scalar)), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g3[3]) * other.group0.xyz) + (float3(right_dual_g0[3] * other.scalar) * this.group2.xyz) + (anti_wedge_g3.yzx * other.group1.zxy) + float3(float2(0.0), anti_wedge_g3[1] * other.e15 * -1.0) - float3((anti_wedge_g3.zx * other.group1.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g0_xyz[0]));
        float4 anti_wedge_g2 = (float4(right_dual_g0_w) * float4(this.group1.xyz, this.e4)) + float4(float3(0.0), (right_dual_g0_xyz[1] * this.e2) + (right_dual_g0_xyz[2] * this.e3)) + float4((right_dual_g0_xyz.zxy * this.group2.yzx), right_dual_g0_xyz[0] * this.e1) - float4((right_dual_g0_xyz.yzx * this.group2.zxy), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e5) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g2.yzx * other.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g2[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Circle anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float anti_wedge_g0 = (this.e321 * other.e321) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float anti_wedge_g0 = (this.e321 * other.e321) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Circle anti_project_via_horizon_onto(Dipole other) {
        float4 anti_wedge_g0 = (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((other.group0.yzx * this.group2.zxy), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0);
        float anti_wedge_g1 = (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float4 anti_wedge_g0 = (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((other.group0.yzx * this.group2.zxy), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0);
        float anti_wedge_g1 = (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g1) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215)) - float4((anti_wedge_g0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Motor anti_project_via_horizon_onto(DualNum other) {
        float4 anti_wedge_g0 = float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e4);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Line anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g0 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0);
        return Line(
            /* e415, e425, e435 */
            float3(other.e45 * -1.0) * anti_wedge_g0.xyz, 
            /* e235, e315, e125 */
            (anti_wedge_g0.yzx * other.group0.zxy) + float3(float2(0.0), (anti_wedge_g0[1] * other.e15) * -1.0) - float3((anti_wedge_g0.zx * other.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * anti_wedge_g0.xyz), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Line anti_project_via_horizon_onto(Line other) {
        float anti_wedge_g0 = -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float3 anti_wedge_g0_xyz = float3(right_dual_g1_w) * this.group0.xyz;
        float anti_wedge_g0_w = (right_dual_g1_w * this.e4) - (right_dual_g0_xyz[0] * this.e415) - (right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435) - (right_dual_g1_xyz[0] * this.e423) - (right_dual_g1_xyz[1] * this.e431) - (right_dual_g1_xyz[2] * this.e412);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(anti_wedge_g0_w) * other.group0.xy), anti_wedge_g0_w * other.e435, (anti_wedge_g0_w * other.e12345) - (anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e5)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g10 * this.e5) + (right_dual_g9_w * this.e4) + (right_dual_g9_xyz[0] * this.e1) + (right_dual_g9_xyz[1] * this.e2) + (right_dual_g9_xyz[2] * this.e3) + (this.e321 * other.e321) - (right_dual_g5[0] * this.e415) - (right_dual_g5[1] * this.e425) - (right_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = (float4(right_dual_g0[1]) * float4(this.e1, this.e2, this.e3, this.e4)) + (float4(other.e45, other.e45, other.e45, other.e41) * this.group1.xyzx) + float4(float3(0.0), (this.e412 * other.e12) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((right_dual_g8.yzx * this.group0.zxy), this.e423 * other.e23) + float4((other.group4.yzx * this.group2.zxy), this.e431 * other.e31) - float4((float3(this.e321) * other.group5.xyz), 0.0) - float4((right_dual_g8.zxy * this.group0.yzx), 0.0) - float4((other.group4.zxy * this.group2.yzx), 0.0);
        float anti_wedge_g2 = (right_dual_g0[1] * this.e5) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g9_xyz[1] * this.e425) - (right_dual_g9_xyz[2] * this.e435)) + float4((float3(right_dual_g9_w) * this.group1.xyz), 0.0) + float4((right_dual_g9_xyz.zxy * this.group2.yzx), 0.0) - float4((right_dual_g9_xyz.yzx * this.group2.zxy), right_dual_g9_xyz[0] * this.e415);
        float3 anti_wedge_g4 = (float3(right_dual_g10) * this.group1.xyz) + (right_dual_g9_xyz.yzx * this.group0.zxy) - (right_dual_g9_xyz.zxy * this.group0.yzx);
        float3 anti_wedge_g5 = (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9_w) * this.group0.xyz) - (right_dual_g9_xyz * float3(this.e321));
        float4 anti_wedge_g6 = float4(right_dual_g0[1]) * this.group1;
        float3 anti_wedge_g7 = float3(right_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(right_dual_g0[1]) * this.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(right_dual_g1) * this.group1.xyz) + (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) + float4((float3(right_dual_g1) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g0_xyz[0]));
        float4 anti_wedge_g2 = (float4(right_dual_g0_w) * float4(this.group1.xyz, this.e4)) + float4(float3(0.0), (right_dual_g0_xyz[0] * this.e1) + (right_dual_g0_xyz[1] * this.e2) + (right_dual_g0_xyz[2] * this.e3)) + float4((right_dual_g0_xyz.zxy * this.group2.yzx), right_dual_g1 * this.e5) - float4((right_dual_g0_xyz.yzx * this.group2.zxy), 0.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group0.yz), 0.0) - float3((anti_wedge_g2.yz * other.group0.zx), 0.0)), anti_wedge_g2[3] * other.e4), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g2[3]) * float4(other.group0.xyz, other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_project_via_horizon_onto(Scalar other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.scalar * other.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (float3(right_dual_g2_w) * this.group1.xyz) + (right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e425) - (right_dual_g3_xyz[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g3_xyz, right_dual_g3_xyz[0]));
        float4 anti_wedge_g2 = (this.group1 * float4(float3(right_dual_g3_w), other.e321)) + (this.group2.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g3_w)) + float4(float3(0.0), (right_dual_g2_w * this.e5) + (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) - (right_dual_g0_xyz[1] * this.e315) - (right_dual_g0_xyz[2] * this.e125) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412)) - (this.group2.zxyx * float4(right_dual_g3_xyz.yzx, right_dual_g0_xyz[0]));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2[3]) * other.group0) + float4(float3(0.0), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((float3(other.e4) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group3.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (this.e423 * other.e23) + (this.e431 * other.e31) + (this.e412 * other.e12) + (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43) + (this.e4 * other.scalar);
        float4 anti_wedge_g3 = (float4(other.scalar) * this.group3) + (this.group2.zxyy * float4(other.group0.yzx, other.e31)) + (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_dual_g2_xyz[2] * this.e435) * -1.0) + float4((right_dual_g2_xyz.yzx * this.group0.zxy), this.e125 * other.e12) - (this.group1.wwwx * float4(other.group1.xyz, right_dual_g2_xyz[0])) - float4((right_dual_g2_xyz.zxy * this.group0.yzx), right_dual_g2_xyz[1] * this.e425) - float4((this.group2.yzx * other.group0.zxy), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2_w) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g0 * float3(other.scalar)), anti_wedge_g3[1] * other.e4315) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2_xyz[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215 * -1.0) * other.group0.xyz;
        float anti_wedge_g0_w = other.scalar * this.scalar;
        float4 anti_wedge_g1 = float4(this.e3215) * float4((other.group1.xyz * float3(-1.0)), other.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0_w * other.e15) + (anti_wedge_g1[0] * other.scalar), (anti_wedge_g0_w * other.e25) + (anti_wedge_g1[1] * other.scalar), (anti_wedge_g0_w * other.e35) + (anti_wedge_g1[2] * other.scalar), -(anti_wedge_g0_xyz[0] * other.e41) - (anti_wedge_g0_xyz[1] * other.e42) - (anti_wedge_g0_xyz[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.zxyw * float4(other.group0.yzx, other.scalar)) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g0_xyz * float3(other.e45)), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4((other.group3.xyz * float3(-1.0)), other.e4);
        float4 anti_wedge_g1 = float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e321);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) + float3(float2(0.0), anti_wedge_g1[0] * other.e2 * -1.0) - float3((anti_wedge_g1.yz * other.group3.zx), 0.0)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), anti_wedge_g0[3] * other.e4) - (anti_wedge_g1.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1[3]) * float4(other.group0.xyz, other.e321)) + (float4(anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g1[0]) * other.group2.wwwx) + (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + float4(float3(0.0), (anti_wedge_g1[2] * other.e125) - (anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) - float4((anti_wedge_g1.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_via_horizon_onto(AntiDualNum other) {
        float2 anti_wedge_g0 = float2(other.scalar) * this.group0.xy;
        return AntiDualNum(
            /* e3215, scalar */
            float2((anti_wedge_g0[0] * other.scalar) + (anti_wedge_g0[1] * other.e3215), anti_wedge_g0[1] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_via_horizon_onto(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * other.e321 * this.e3215, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public Flector anti_project_via_horizon_onto(AntiFlector other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215 * -1.0) * other.group1.xyz;
        float anti_wedge_g1_w = this.e3215 * other.e321;
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g1_w) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g1_w * other.e321) - (anti_wedge_g0_xyz[0] * other.e1) - (anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_via_horizon_onto(AntiLine other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215 * -1.0) * other.group0.xyz;
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_w = right_dual_g0[3] * this.scalar;
        float4 anti_wedge_g1 = right_dual_g0 * float4(this.e3215);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(anti_wedge_g0_w) * other.group0, 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_w) * other.group1) + float4(float3(0.0), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_via_horizon_onto(AntiPlane other) {
        float3 anti_wedge_g0_xyz = float3(this.e3215 * -1.0) * other.group0.xyz;
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(anti_wedge_g0_xyz[0] * other.e1) - (anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_via_horizon_onto(Circle other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_via_horizon_onto(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_via_horizon_onto(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e3215 * -1.0) * other.group0.xyz;
        float3 anti_wedge_g1 = float3(this.e3215 * -1.0) * other.group1.xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g0 * float3(other.e45)), 0.0) + float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float3 anti_wedge_g1_xyz = float3(this.e3215 * -1.0) * other.group1.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0[3]) * other.group2.xyz), (anti_wedge_g0[3] * other.e1234) - (anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group3.xyz), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g1_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float anti_wedge_g0_x = (right_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234);
        float4 anti_wedge_g1 = float4((float3(this.e3215 * -1.0) * other.group7.xyz), 0.0);
        float anti_wedge_g2 = this.e3215 * other.e321;
        float4 anti_wedge_g3 = float4((float3(this.e3215 * -1.0) * other.group5.xyz), 0.0);
        float3 anti_wedge_g5 = float3(this.e3215 * -1.0) * other.group4.xyz;
        float4 anti_wedge_g6 = float4(float3(0.0), this.e3215 * other.e4);
        float3 anti_wedge_g8 = float3(this.e3215 * -1.0) * other.group1.xyz;
        float4 anti_wedge_g9 = float4(float3(0.0), right_dual_g0[1] * this.e3215);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * float3(other.e4)), anti_wedge_g1[2] * other.e125) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[0] * other.e15) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_via_horizon_onto(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e4 * -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_via_horizon_onto(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_via_horizon_onto(Sphere other) {
        float anti_wedge_g0 = this.e3215 * other.e1234;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e3215) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e321);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) + float3(float2(0.0), anti_wedge_g1[0] * other.e2 * -1.0) - float3((anti_wedge_g1.yz * other.group3.zx), 0.0)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(float3(anti_wedge_g1[3]), anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) - (anti_wedge_g1.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) + float4((float3(other.e4) * anti_wedge_g0.xyz), 0.0) - float4((anti_wedge_g1.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4((float3(this.e3215 * -1.0) * other.group0.xyz), (this.e3215 * other.e1234) + (this.scalar * other.scalar));
        float4 anti_wedge_g1 = float4(this.e3215) * float4((other.group1.xyz * float3(-1.0)), other.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)) + float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + (anti_wedge_g1.zxyw * other.group0.yzxw) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group3.xyz), 0.0) - (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = (this.group0.yzxx * float4(right_dual_g0.zxy, other.e23)) + float4(float3(0.0), (other.e31 * this.e315) + (other.e12 * this.e125)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_dual_g0.yzx * this.group0.zxy), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)) + float4((other.group0.yzx * anti_wedge_g1.zxy), 0.0) - (float4(anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g0[0] * other.e41) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group0.xyz), anti_wedge_g0[3] * other.scalar) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group2.zx), 0.0) - float3((anti_wedge_g1.zx * other.group2.yz), 0.0)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e321 * this.e321) - (other.e412 * this.e125)) - (this.group0.xyzy * float4(float3(other.e4), other.e431)) - (this.group0.wwwx * float4(right_dual_g3_xyz, other.e423));
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.zxy * this.group0.yzx) - (right_dual_g3_xyz.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[0] * other.e4) + (anti_wedge_g0[3] * other.e423), (anti_wedge_g0[1] * other.e4) + (anti_wedge_g0[3] * other.e431), (anti_wedge_g0[2] * other.e4) + (anti_wedge_g0[3] * other.e412), -(anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0[3]) * other.group1) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + float4((anti_wedge_g1_xyz * float3(other.e4)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g0 = float4(this.e321) * float4((right_dual_g1_xyz * float3(-1.0)), other.e321);
        float3 anti_wedge_g1_xyz = (right_dual_g1_xyz.zxy * this.group0.yzx) - (right_dual_g1_xyz.yzx * this.group0.zxy);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group1.yzx), 0.0) - (other.group1.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_dual_g0 * float3(this.e321);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)) + float4((float3(-(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) * other.group0.xyz), 0.0) + float4((anti_wedge_g0_xyz.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g0_xyz.zxy * other.group1.yzx), anti_wedge_g0_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g1 = float4((float3(this.e321 * -1.0) * other.group0.xyz), (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            (other.group0 * float4(float3(anti_wedge_g1[3]), this.e321 * other.scalar)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(other.scalar * other.scalar) * this.group0.xyz), 0.0) + float4((anti_wedge_g1.yzx * other.group1.zxy), 0.0) - (anti_wedge_g1.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 anti_wedge_g0 = right_dual_g0_xyz * float3(this.e321 * -1.0);
        float3 anti_wedge_g1 = (right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0) - (other.group0.zxyx * float4(anti_wedge_g1.yzx, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_project_via_horizon_onto(Circle other) {
        float anti_wedge_g0 = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float anti_wedge_g0 = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_dual_g0.yzx * this.group0.zxy), 0.0);
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_dual_g0.yzx * this.group0.zxy), 0.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g0[3]) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)) - float4((anti_wedge_g0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float anti_wedge_g0_x = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((right_dual_g6_xyz * float3(this.e321)), 0.0) + float4((right_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_dual_g7.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = -(right_dual_g6_xyz[0] * this.e235) - (right_dual_g6_xyz[1] * this.e315) - (right_dual_g6_xyz[2] * this.e125);
        float4 anti_wedge_g3 = float4(((right_dual_g9.zxy * this.group0.yzx) - (right_dual_g9.yzx * this.group0.zxy)), 0.0);
        float3 anti_wedge_g5 = -(float3(this.e321) * right_dual_g9.xyz) - (float3(other.e4) * this.group0.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), right_dual_g0[1] * this.e321);
        float3 anti_wedge_g8 = float3(right_dual_g0[1]) * this.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * float3(other.e4)), anti_wedge_g1[2] * other.e125) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[0] * other.e15) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 anti_wedge_g0 = -(right_dual_g0_xyz * float3(this.e321)) - (float3(other.e4) * this.group0.xyz);
        float3 anti_wedge_g1 = (right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy);
        return Circle(
            /* e423, e431, e412 */
            anti_wedge_g0 * float3(other.e4), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * float3(other.e4)), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)), 
            /* e235, e315, e125 */
            (anti_wedge_g0 * float3(other.e5)) + (anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_via_horizon_onto(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), (this.e321 * other.e321) - (right_dual_g0_xyz[2] * this.e125)) - (this.group0.xyzy * float4(float3(other.e4), right_dual_g0_xyz[1])) - (this.group0.wwwx * float4(right_dual_g3_xyz, right_dual_g0_xyz[0]));
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.zxy * this.group0.yzx) - (right_dual_g3_xyz.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(float3(other.e4), other.e12345)) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0[3]) * other.group1) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + float4((anti_wedge_g1_xyz * float3(other.e4)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = (this.group0.zxyx * float4(other.group0.yzx, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e1234) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)) - (float4(anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g0[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(float3(anti_wedge_g1[3]), anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group2.zx), 0.0) - float3((anti_wedge_g1.zx * other.group2.yz), 0.0)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiFlector> operator /(AntiFlector lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = (float4(other.scalar) * this.group1) + (this.group0.yzxx * float4(right_dual_g0.zxy, other.e23)) + float4(float3(0.0), (other.e31 * this.e315) + (other.e12 * this.e125)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_dual_g0.yzx * this.group0.zxy), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)) + float4((other.group0.yzx * anti_wedge_g1.zxy), 0.0) - (float4(anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g0[0] * other.e41) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group0.xyz), anti_wedge_g0[3] * other.scalar) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group2.zx), 0.0) - float3((anti_wedge_g1.zx * other.group2.yz), 0.0)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) + (other.e321 * this.e321) - (other.e431 * this.e315) - (other.e412 * this.e125)) - (float4(other.e4) * float4(this.group0.xyz, this.e5)) - (this.group0.wwwx * float4(right_dual_g3_xyz, other.e423));
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.zxy * this.group0.yzx) - (right_dual_g3_xyz.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[0] * other.e4) + (anti_wedge_g0[3] * other.e423), (anti_wedge_g0[1] * other.e4) + (anti_wedge_g0[3] * other.e431), (anti_wedge_g0[2] * other.e4) + (anti_wedge_g0[3] * other.e412), -(anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0[3]) * other.group1) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + float4((anti_wedge_g1_xyz * float3(other.e4)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_via_horizon_onto(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g0 = float4((right_dual_g1_xyz * float3(this.e321 * -1.0)), (right_dual_g1_xyz[0] * this.e1) + (right_dual_g1_xyz[1] * this.e2) + (right_dual_g1_xyz[2] * this.e3) + (other.e321 * this.e321));
        float3 anti_wedge_g1_xyz = (right_dual_g1_xyz.zxy * this.group0.yzx) - (right_dual_g1_xyz.yzx * this.group0.zxy);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group1.yzx), 0.0) - (other.group1.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_dual_g0 * float3(this.e321);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)) + float4((float3(-(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) * other.group0.xyz), 0.0) + float4((anti_wedge_g0_xyz.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g0_xyz.zxy * other.group1.yzx), anti_wedge_g0_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = (right_dual_g0 * float4(float3(this.e321), this.e5)) + float4(float3(0.0), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) + float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            (other.group0 * float4(float3(anti_wedge_g1[3]), right_dual_g0[3] * this.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(right_dual_g0[3] * other.scalar) * this.group0.xyz), 0.0) + float4((anti_wedge_g1.yzx * other.group1.zxy), 0.0) - (anti_wedge_g1.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 anti_wedge_g0_xyz = right_dual_g0_xyz * float3(this.e321 * -1.0);
        float3 anti_wedge_g1_xyz = (right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3)) + float4((anti_wedge_g0_xyz * float3(other.e5)), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - (other.group0.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0_xyz[0])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0_xyz[0] * this.e1) + (right_dual_g0_xyz[1] * this.e2) + (right_dual_g0_xyz[2] * this.e3)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public Circle anti_project_via_horizon_onto(Circle other) {
        float anti_wedge_g0 = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float anti_wedge_g0 = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public Circle anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_dual_g0.yzx * this.group0.zxy), 0.0);
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_dual_g0.yzx * this.group0.zxy), 0.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g0[3]) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)) - float4((anti_wedge_g0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g10 * this.e5) + (right_dual_g9_xyz[0] * this.e1) + (right_dual_g9_xyz[1] * this.e2) + (right_dual_g9_xyz[2] * this.e3) + (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((right_dual_g6_xyz * float3(this.e321)), 0.0) + float4((float3(right_dual_g0[1]) * this.group1.xyz), 0.0) + float4((right_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_dual_g7.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = (right_dual_g0[1] * this.e5) - (right_dual_g6_xyz[0] * this.e235) - (right_dual_g6_xyz[1] * this.e315) - (right_dual_g6_xyz[2] * this.e125);
        float4 anti_wedge_g3 = float4(((right_dual_g9_xyz.zxy * this.group0.yzx) - (right_dual_g9_xyz.yzx * this.group0.zxy)), 0.0);
        float3 anti_wedge_g5 = (float3(right_dual_g10) * this.group0.xyz) - (right_dual_g9_xyz * float3(this.e321));
        float4 anti_wedge_g6 = float4(float3(0.0), right_dual_g0[1] * this.e321);
        float3 anti_wedge_g8 = float3(right_dual_g0[1]) * this.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * float3(other.e4)), anti_wedge_g1[2] * other.e125) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[0] * other.e15) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), (right_dual_g0_xyz[0] * this.e1) + (right_dual_g0_xyz[1] * this.e2) + (right_dual_g0_xyz[2] * this.e3)) - (float4(other.e4) * float4(this.group0.xyz, this.e5)) - float4((right_dual_g0_xyz * float3(this.e321)), 0.0);
        float3 anti_wedge_g1_xyz = (right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e4) * anti_wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1_xyz * float3(other.e4)), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group0.yzx) - (anti_wedge_g1_xyz.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e4), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_via_horizon_onto(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) + (this.e321 * other.e321) - (right_dual_g0_xyz[1] * this.e315) - (right_dual_g0_xyz[2] * this.e125)) - (float4(other.e4) * float4(this.group0.xyz, this.e5)) - (this.group0.wwwx * float4(right_dual_g3_xyz, right_dual_g0_xyz[0]));
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.zxy * this.group0.yzx) - (right_dual_g3_xyz.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(float3(other.e4), other.e12345)) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0[3]) * other.group1) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + float4((anti_wedge_g1_xyz * float3(other.e4)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = (float4(other.scalar) * this.group1) + (this.group0.zxyx * float4(other.group0.yzx, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e1234) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)) - (float4(anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g0[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(float3(anti_wedge_g1[3]), anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group2.zx), 0.0) - float3((anti_wedge_g1.zx * other.group2.yz), 0.0)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiLine> operator /(AntiLine lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float anti_wedge_g0_w = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12);
        float3 anti_wedge_g1_xyz = float3(other.scalar) * this.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0_w * other.e15) + (anti_wedge_g1_xyz[0] * other.scalar), (anti_wedge_g0_w * other.e25) + (anti_wedge_g1_xyz[1] * other.scalar), (anti_wedge_g0_w * other.e35) + (anti_wedge_g1_xyz[2] * other.scalar), -(anti_wedge_g0_xyz[0] * other.e41) - (anti_wedge_g0_xyz[1] * other.e42) - (anti_wedge_g0_xyz[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) + float4((anti_wedge_g0_xyz * float3(other.e45)), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), right_dual_g3_xyz[2] * this.e35) + float4((float3(other.e4) * this.group1.xyz), right_dual_g3_xyz[1] * this.e25) + float4((right_dual_g3_xyz.zxy * this.group0.yzx), right_dual_g3_xyz[0] * this.e15) - float4((right_dual_g3_xyz.yzx * this.group0.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group3.yzx) + float3(float2(0.0), anti_wedge_g0[0] * other.e2 * -1.0) - float3((anti_wedge_g0.yz * other.group3.zx), 0.0)), anti_wedge_g0[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g0[3]) * other.group3.xyz), 0.0) - (anti_wedge_g0.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_via_horizon_onto(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public AntiMotor anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), (right_dual_g1_xyz[1] * this.e25) + (right_dual_g1_xyz[2] * this.e35)) + float4((right_dual_g1_xyz.zxy * this.group0.yzx), right_dual_g1_xyz[0] * this.e15) - float4((right_dual_g1_xyz.yzx * this.group0.zxy), 0.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.e1, other.e2, other.e3, other.e321)) + float4(float3(0.0), (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((float3(other.e5) * anti_wedge_g0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_via_horizon_onto(AntiLine other) {
        float anti_wedge_g0 = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public AntiMotor anti_project_via_horizon_onto(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float anti_wedge_g0_w = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12);
        float4 anti_wedge_g1 = float4((float3(other.scalar) * this.group1.xyz), 0.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * other.scalar), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_w) * other.group1) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) + float4((right_dual_g0_xyz.zxy * this.group0.yzx), right_dual_g0_xyz[0] * this.e15) - float4((right_dual_g0_xyz.yzx * this.group0.zxy), 0.0);
        return AntiLine(
            /* e23, e31, e12 */
            (anti_wedge_g0.zxy * other.group0.yzx) + float3(float2(0.0), (anti_wedge_g0[0] * other.e2) * -1.0) - float3((anti_wedge_g0.yz * other.group0.zx), 0.0), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g0[3]) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public Dipole anti_project_via_horizon_onto(Dipole other) {
        float anti_wedge_g0 = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float anti_wedge_g0 = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float anti_wedge_g0_x = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43);
        float4 anti_wedge_g1 = float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((right_dual_g9_xyz.zxy * this.group0.yzx), 0.0) - float4((right_dual_g9_xyz.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35);
        float4 anti_wedge_g3 = float4((float3(right_dual_g0[1]) * this.group1.xyz), 0.0);
        float3 anti_wedge_g5 = float3(right_dual_g0[1]) * this.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g1[2] * other.e125) * float4(other.group4.yzx, 1.0)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[1] * other.e315) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[0] * other.e15) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public Dipole anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), right_dual_g0_xyz[2] * this.e35) + float4((float3(other.e4) * this.group1.xyz), right_dual_g0_xyz[1] * this.e25) + float4((right_dual_g0_xyz.zxy * this.group0.yzx), right_dual_g0_xyz[0] * this.e15) - float4((right_dual_g0_xyz.yzx * this.group0.zxy), 0.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) + float3(float2(0.0), anti_wedge_g0[0] * other.e2 * -1.0) - float3((anti_wedge_g0.yz * other.group0.zx), 0.0)), anti_wedge_g0[3] * other.e4), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g0[3]) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_via_horizon_onto(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), right_dual_g3_xyz[2] * this.e35) + float4((float3(other.e4) * this.group1.xyz), right_dual_g3_xyz[1] * this.e25) + float4((right_dual_g3_xyz.zxy * this.group0.yzx), right_dual_g3_xyz[0] * this.e15) - float4((right_dual_g3_xyz.yzx * this.group0.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group3.yzx) + float3(float2(0.0), anti_wedge_g0[0] * other.e2 * -1.0) - float3((anti_wedge_g0.yz * other.group3.zx), 0.0)), anti_wedge_g0[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g0[3]) * other.group3.xyz), 0.0) - (anti_wedge_g0.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiLine {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4((float3(other.scalar) * this.group0.xyz), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43));
        float3 anti_wedge_g1_xyz = float3(other.scalar) * this.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)) + float4((anti_wedge_g1_xyz * float3(other.scalar)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group3.xyz), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g1_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiMotor> operator /(AntiMotor lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (float4(other.scalar) * this.group0) + float4(float3(0.0), (other.e31 * this.e31) + (other.e12 * this.e12) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35)) + float4((right_dual_g0 * float3(this.e3215)), other.e23 * this.e23);
        float3 anti_wedge_g1_xyz = (float3(other.scalar) * this.group1.xyz) - (float3(this.e3215) * other.group1.xyz);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g1_xyz[0] * other.scalar) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1_xyz[1] * other.scalar) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1_xyz[2] * other.scalar) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(other.e45) * anti_wedge_g0.xyz), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), other.scalar * other.scalar * this.e3215) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g1_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(right_dual_g3_xyz, right_dual_g2_w);
        float4 anti_wedge_g1 = float4(float3(0.0), (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35) + (other.e321 * this.e3215)) + float4((right_dual_g3_xyz.zxy * this.group0.yzx), right_dual_g3_xyz[0] * this.e15) - float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g3_xyz.yzx * this.group0.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) + float3(float2(0.0), anti_wedge_g1[0] * other.e2 * -1.0) - float3((anti_wedge_g1.yz * other.group3.zx), 0.0)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), anti_wedge_g0[3] * other.e4) - (anti_wedge_g1.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1[3]) * float4(other.group0.xyz, other.e321)) + (float4(anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g1[0]) * other.group2.wwwx) + (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + float4(float3(0.0), (anti_wedge_g1[2] * other.e125) - (anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) - float4((anti_wedge_g1.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_via_horizon_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e15, e25, e35, e3215 */
            float4((float2(other.scalar) * anti_wedge_g1.xy), anti_wedge_g1[2] * other.scalar, (anti_wedge_g0[3] * other.e3215) + (anti_wedge_g1[3] * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public AntiDualNum anti_project_via_horizon_onto(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * other.e321 * this.e3215, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 anti_wedge_g0_xyz = right_dual_g1_xyz * float3(this.e3215 * -1.0);
        float4 anti_wedge_g1 = float4(float3(0.0), (right_dual_g1_xyz[1] * this.e25) + (right_dual_g1_xyz[2] * this.e35) + (other.e321 * this.e3215)) + float4((right_dual_g1_xyz.zxy * this.group0.yzx), right_dual_g1_xyz[0] * this.e15) - float4((right_dual_g1_xyz.yzx * this.group0.zxy), 0.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (float4(anti_wedge_g1[3]) * float4(other.e1, other.e2, other.e3, other.e321)) + float4(float3(0.0), (anti_wedge_g1[0] * other.e235) + (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3)) - (float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g0_xyz[0]) * other.group1.wwwx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        float3 anti_wedge_g1_xyz = right_dual_g0 * float3(this.e3215);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_via_horizon_onto(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float anti_wedge_g0_w = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar);
        float4 anti_wedge_g1 = float4(((float3(other.scalar) * this.group1.xyz) - (float3(this.e3215) * other.group0.xyz)), other.scalar * this.e3215);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * other.scalar), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_w) * other.group1) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 anti_wedge_g0_xyz = right_dual_g0_xyz * float3(this.e3215 * -1.0);
        float4 anti_wedge_g1 = float4(float3(0.0), (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) + float4((right_dual_g0_xyz.zxy * this.group0.yzx), right_dual_g0_xyz[0] * this.e15) - float4((right_dual_g0_xyz.yzx * this.group0.zxy), 0.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3)) + float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0) - (float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g0_xyz[0]) * other.group0.wwwx)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public Sphere anti_project_via_horizon_onto(Circle other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public Sphere anti_project_via_horizon_onto(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_dual_g0 * float3(this.e3215);
        float anti_wedge_g0_w = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35);
        float3 anti_wedge_g1_xyz = float3(this.e3215 * -1.0) * other.group1.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0_w) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0_w) * other.group2.xyz), -(anti_wedge_g0_xyz[0] * other.e41) - (anti_wedge_g0_xyz[1] * other.e42) - (anti_wedge_g0_xyz[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) + float4((anti_wedge_g0_xyz * float3(other.e45)), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_dual_g0 * float3(this.e3215)), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e3215 * other.e1234) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35));
        float3 anti_wedge_g1_xyz = float3(this.e3215 * -1.0) * other.group1.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0[3]) * other.group2.xyz), (anti_wedge_g0[3] * other.e1234) - (anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group3.xyz), 0.0) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g1_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234) - (right_dual_g6_xyz[0] * this.e23) - (right_dual_g6_xyz[1] * this.e31) - (right_dual_g6_xyz[2] * this.e12) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35);
        float4 anti_wedge_g1 = float4((right_dual_g9_xyz.zxy * this.group0.yzx), 0.0) - float4((float3(right_dual_g10) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_dual_g9_xyz.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35) + (this.e3215 * other.e321);
        float4 anti_wedge_g3 = float4(((right_dual_g6_xyz * float3(this.e3215)) + (float3(right_dual_g0[1]) * this.group1.xyz)), 0.0);
        float3 anti_wedge_g5 = (right_dual_g7 * float3(this.e3215)) + (float3(right_dual_g0[1]) * this.group0.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), right_dual_g10 * this.e3215 * -1.0);
        float3 anti_wedge_g8 = right_dual_g9_xyz * float3(this.e3215 * -1.0);
        float4 anti_wedge_g9 = float4(float3(0.0), right_dual_g0[1] * this.e3215);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * float3(other.e4)), anti_wedge_g1[2] * other.e125) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[0] * other.e15) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(right_dual_g0_xyz, right_dual_g1);
        float4 anti_wedge_g1 = float4(float3(0.0), (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) + float4((right_dual_g0_xyz.zxy * this.group0.yzx), right_dual_g0_xyz[0] * this.e15) - float4((float3(right_dual_g1) * this.group1.xyz), 0.0) - float4((right_dual_g0_xyz.yzx * this.group0.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group0.yzx) + float3(float2(0.0), anti_wedge_g1[0] * other.e2 * -1.0) - float3((anti_wedge_g1.yz * other.group0.zx), 0.0)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g1.xyz)), anti_wedge_g0[3] * other.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_via_horizon_onto(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public Sphere anti_project_via_horizon_onto(Sphere other) {
        float anti_wedge_g0 = this.e3215 * other.e1234;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(this.e3215 * -1.0) * float4(right_dual_g3_xyz, right_dual_g2_w);
        float4 anti_wedge_g1 = float4(float3(0.0), (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35) + (this.e3215 * other.e321)) + float4((right_dual_g3_xyz.zxy * this.group0.yzx), right_dual_g3_xyz[0] * this.e15) - float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g3_xyz.yzx * this.group0.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) + float3(float2(0.0), anti_wedge_g1[0] * other.e2 * -1.0) - float3((anti_wedge_g1.yz * other.group3.zx), 0.0)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(float3(anti_wedge_g1[3]), anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) - (anti_wedge_g1.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) + float4((float3(other.e4) * anti_wedge_g0.xyz), 0.0) - float4((anti_wedge_g1.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (right_dual_g0 * float4(float3(this.e3215), this.scalar)) + (this.group0.xyzx * float4(float3(right_dual_g0[3]), other.e23)) + float4(float3(0.0), (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e3215 * other.e1234) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35));
        float3 anti_wedge_g1_xyz = (float3(right_dual_g0[3]) * this.group1.xyz) - (float3(this.e3215) * other.group1.xyz);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)) + float4((anti_wedge_g1_xyz * float3(other.scalar)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + (other.group0.yzxw * float4(anti_wedge_g1_xyz.zxy, right_dual_g0[3] * this.e3215)) + float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group3.xyz), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g1_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiPlane> operator /(AntiPlane lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e4 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group2.yzx), 0.0), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float anti_wedge_g0 = (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) - (other.e4 * this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(anti_wedge_g0) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_project_via_horizon_onto(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float anti_wedge_g0 = (right_dual_g1_xyz[0] * this.e1) + (right_dual_g1_xyz[1] * this.e2) + (right_dual_g1_xyz[2] * this.e3);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group1.zxy), 0.0) - (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(right_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float anti_wedge_g0_x = (right_dual_g9_xyz[0] * this.e1) + (right_dual_g9_xyz[1] * this.e2) + (right_dual_g9_xyz[2] * this.e3) - (this.e5 * other.e4);
        float4 anti_wedge_g1 = float4((float3(right_dual_g0[1]) * this.group0.xyz), 0.0);
        float anti_wedge_g2 = right_dual_g0[1] * this.e5;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), 0.0) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float anti_wedge_g0 = (right_dual_g0_xyz[0] * this.e1) + (right_dual_g0_xyz[1] * this.e2) + (right_dual_g0_xyz[2] * this.e3) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group0, 
            /* e5 */
            anti_wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_project_via_horizon_onto(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float anti_wedge_g0 = (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) - (this.e5 * other.e4);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiPlane {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e1234)) - float4((anti_wedge_g0.yzx * other.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (anti_wedge_g0.yzx * other.group2.zxy) + float3(float2(0.0), anti_wedge_g0[1] * other.e15 * -1.0) - float3((anti_wedge_g0.zx * other.group2.yz), 0.0)), anti_wedge_g0[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * anti_wedge_g0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345 * -1.0) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            anti_wedge_g0 * float3(other.scalar), 
            /* e415, e425, e435, e321 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * anti_wedge_g2.xyz), (anti_wedge_g2[3] * other.scalar) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group3.zxy) - (anti_wedge_g0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g2[3])) + (float4(anti_wedge_g2[2], anti_wedge_g2[0], anti_wedge_g2[1], anti_wedge_g3[0]) * other.group3.yzxx) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) - (anti_wedge_g2.yzxx * float4(other.group3.zxy, other.e423))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public DualNum anti_project_via_horizon_onto(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar * this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e12345 * (other.e321 * -1.0) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(AntiFlector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group1.xyz), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g0[3] * other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345 * -1.0) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * anti_wedge_g0.xyz), (anti_wedge_g0[3] * other.scalar) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), (anti_wedge_g2[3] * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345 * -1.0) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g2 = float3(this.e12345 * -1.0) * other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345 * -1.0) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g2[3] * other.e3215) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0]) * other.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345 * other.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * anti_wedge_g1.xyz), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e12345 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 anti_wedge_g0 = float2(this.e12345) * other.group0.yx * float2(-1.0, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float anti_wedge_g2 = this.e12345 * other.e3215;
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group6.xyz;
        float4 anti_wedge_g6 = float4(this.e12345) * float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 anti_wedge_g7 = float3(this.e12345 * -1.0) * other.group4.xyz;
        float3 anti_wedge_g8 = float3(this.e12345 * -1.0) * other.group3.xyz;
        float4 anti_wedge_g9 = float4(this.e12345) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g10 = this.e12345 * other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0[0] * other.scalar, (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g0[0] * other.e12345) + (anti_wedge_g0[1] * other.scalar) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group1), 
            /* e5 */
            (anti_wedge_g2 * other.scalar) + (anti_wedge_g0[0] * other.e5), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g2) * other.group1) + (float4(anti_wedge_g0[0]) * other.group3) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g2) * other.group5.xyz) + (float3(anti_wedge_g0[0]) * other.group8.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (float4(anti_wedge_g0[0]) * other.group9) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g10 * other.scalar) + (anti_wedge_g0[0] * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g0[3] * other.e4) - (this.e12345 * other.e4 * other.e5)
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
            /* e12345 */
            this.e12345 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_via_horizon_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension AntiScalar {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 anti_wedge_g3 = float4(this.e12345) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g2[3] * other.e1234) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Circle> operator /(Circle lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (other.e41 * this.e415) + (other.e42 * this.e425) + (other.e43 * this.e435) + (other.e23 * this.e423) + (other.e31 * this.e431) + (other.e12 * this.e412);
        float4 anti_wedge_g3 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) + (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (other.e12 * this.e125) + (other.e25 * this.e425) + (other.e35 * this.e435)) + float4((other.group0.yzx * this.group2.zxy), other.e31 * this.e315) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g0 * float3(other.scalar)), 0.0) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g0[0] * other.e15) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            (other.group2 * float4(float3(anti_wedge_g2_w), anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) + float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (float3(right_dual_g2_w) * this.group1.xyz) + (right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e425) - (right_dual_g3_xyz[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g3_xyz, right_dual_g3_xyz[0]));
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(right_dual_g3_w) * this.group1.xyz), 0.0) + float4((right_dual_g3_xyz.zxy * this.group2.yzx), 0.0) - float4((right_dual_g3_xyz.yzx * this.group2.zxy), other.e321 * this.e321 * -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group3.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Circle anti_project_via_horizon_onto(AntiDualNum other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiFlatPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            -(float4(other.e321 * this.e321 * -1.0) * other.group0) - (float4(right_dual_g0_xyz[0] * this.e423) * other.group0) - (float4(right_dual_g0_xyz[1] * this.e431) * other.group0) - (float4(right_dual_g0_xyz[2] * this.e412) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g1_xyz.yzx * this.group0.zxy) - (right_dual_g1_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435)) + float4((float3(right_dual_g1_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g1_xyz, right_dual_g1_xyz[0]));
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0_xyz[0] * this.e423) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412)) + float4((float3(right_dual_g1_w) * this.group1.xyz), 0.0) + float4((right_dual_g1_xyz.zxy * this.group2.yzx), 0.0) - float4((right_dual_g1_xyz.yzx * this.group2.zxy), other.e321 * this.e321 * -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)) + float4((anti_wedge_g0.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group1.yz), 0.0) - float3((anti_wedge_g2.yz * other.group1.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Circle anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) + float4((right_dual_g0 * float3(this.e321)), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - float4((other.group1.yzx * this.group0.zxy), right_dual_g0[0] * this.e423);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125 */
            (float3(other.e15 * this.e415) * other.group0.xyz) + (float3(other.e25 * this.e425) * other.group0.xyz) + (float3(other.e35 * this.e435) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g0.yzx) - (float3(right_dual_g0[0] * this.e235) * other.group0.xyz) - (float3(right_dual_g0[1] * this.e315) * other.group0.xyz) - (float3(right_dual_g0[2] * this.e125) * other.group0.xyz) - (other.group1.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_via_horizon_onto(AntiMotor other) {
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float anti_wedge_g2_w = (other.e23 * this.e423) + (other.e31 * this.e431) + (other.e12 * this.e412);
        float4 anti_wedge_g3 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group1.zxyx) + float4(float3(0.0), (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e25 * this.e425) + (other.e35 * this.e435)) - float4((float3(this.e321) * other.group0.xyz), 0.0) - float4((this.group0.zxy * other.group1.yzx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2_w) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g0 * float3(other.scalar)), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g3[3]) * other.group0.xyz) + (float3(other.scalar * other.scalar) * this.group2.xyz) + (anti_wedge_g3.yzx * other.group1.zxy) + float3(float2(0.0), anti_wedge_g3[1] * other.e15 * -1.0) - float3((anti_wedge_g3.zx * other.group1.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Circle anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g0_xyz[0]));
        float3 anti_wedge_g2 = (float3(right_dual_g0_w) * this.group1.xyz) + (right_dual_g0_xyz.zxy * this.group2.yzx) - (right_dual_g0_xyz.yzx * this.group2.zxy);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Circle anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float anti_wedge_g0 = (other.e321 * this.e321) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float anti_wedge_g0 = (this.e321 * other.e321) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Circle anti_project_via_horizon_onto(Dipole other) {
        float4 anti_wedge_g0 = (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((this.group2.zxy * other.group0.yzx), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0);
        float anti_wedge_g1 = (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float4 anti_wedge_g0 = (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((this.group2.zxy * other.group0.yzx), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0);
        float anti_wedge_g1 = (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g1) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215)) - float4((anti_wedge_g0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_project_via_horizon_onto(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * other.e5 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Line anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g0 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0);
        return Line(
            /* e415, e425, e435 */
            float3(other.e45 * -1.0) * anti_wedge_g0.xyz, 
            /* e235, e315, e125 */
            (anti_wedge_g0.yzx * other.group0.zxy) + float3(float2(0.0), (anti_wedge_g0[1] * other.e15) * -1.0) - float3((anti_wedge_g0.zx * other.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * anti_wedge_g0.xyz), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Line anti_project_via_horizon_onto(Line other) {
        float anti_wedge_g0 = -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 anti_wedge_g0_xyz = float3(other.e5 * -1.0) * this.group0.xyz;
        float anti_wedge_g0_w = -(right_dual_g0_xyz[0] * this.e415) - (right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435) - (right_dual_g1_xyz[0] * this.e423) - (right_dual_g1_xyz[1] * this.e431) - (right_dual_g1_xyz[2] * this.e412);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(anti_wedge_g0_w) * other.group0.xy), anti_wedge_g0_w * other.e435, (anti_wedge_g0_w * other.e12345) - (anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e5)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (this.e321 * other.e321) - (right_dual_g5[0] * this.e415) - (right_dual_g5[1] * this.e425) - (right_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = (float4(other.e45, other.e45, other.e45, other.e41) * this.group1.xyzx) + float4(float3(0.0), (this.e412 * other.e12) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((right_dual_g8.yzx * this.group0.zxy), this.e423 * other.e23) + float4((this.group2.zxy * other.group4.yzx), this.e431 * other.e31) - float4((float3(this.e321) * other.group5.xyz), 0.0) - float4((right_dual_g8.zxy * this.group0.yzx), 0.0) - float4((this.group2.yzx * other.group4.zxy), 0.0);
        float anti_wedge_g2 = (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g9_xyz[1] * this.e425) - (right_dual_g9_xyz[2] * this.e435)) + float4((float3(right_dual_g9_w) * this.group1.xyz), 0.0) + float4((right_dual_g9_xyz.zxy * this.group2.yzx), 0.0) - float4((right_dual_g9_xyz.yzx * this.group2.zxy), right_dual_g9_xyz[0] * this.e415);
        float3 anti_wedge_g4 = (float3(right_dual_g10) * this.group1.xyz) + (right_dual_g9_xyz.yzx * this.group0.zxy) - (right_dual_g9_xyz.zxy * this.group0.yzx);
        float3 anti_wedge_g5 = (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9_w) * this.group0.xyz) - (right_dual_g9_xyz * float3(this.e321));
        float4 anti_wedge_g6 = float4(right_dual_g0[1]) * this.group1;
        float3 anti_wedge_g7 = float3(right_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(right_dual_g0[1]) * this.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Circle anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(right_dual_g1) * this.group1.xyz) + (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) + float4((float3(right_dual_g1) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g0_xyz[0]));
        float3 anti_wedge_g2 = (float3(right_dual_g0_w) * this.group1.xyz) + (right_dual_g0_xyz.zxy * this.group2.yzx) - (right_dual_g0_xyz.yzx * this.group2.zxy);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((anti_wedge_g2 * float3(other.e4)), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public Circle anti_project_via_horizon_onto(Scalar other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (float3(right_dual_g2_w) * this.group1.xyz) + (right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e425) - (right_dual_g3_xyz[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_dual_g3_xyz, right_dual_g3_xyz[0]));
        float4 anti_wedge_g2 = (this.group1 * float4(float3(right_dual_g3_w), other.e321)) + float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e315) - (right_dual_g0_xyz[2] * this.e125) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412)) + float4((right_dual_g3_xyz.zxy * this.group2.yzx), 0.0) - float4((right_dual_g3_xyz.yzx * this.group2.zxy), right_dual_g0_xyz[0] * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2[3]) * other.group0) + float4(float3(0.0), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((float3(other.e4) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group3.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (this.e423 * other.e23) + (this.e431 * other.e31) + (this.e412 * other.e12) + (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43);
        float4 anti_wedge_g3 = (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4((right_dual_g2_xyz.yzx * this.group0.zxy), this.e315 * other.e31) + float4((this.group2.zxy * other.group0.yzx), this.e125 * other.e12) - (this.group1.wwwx * float4(other.group1.xyz, right_dual_g2_xyz[0])) - float4((right_dual_g2_xyz.zxy * this.group0.yzx), right_dual_g2_xyz[1] * this.e425) - float4((this.group2.yzx * other.group0.zxy), right_dual_g2_xyz[2] * this.e435);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2_w) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g0 * float3(other.scalar)), anti_wedge_g3[1] * other.e4315) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2_xyz[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<CircleRotor> operator /(CircleRotor lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(((right_dual_g0 * float3(this.e12345)) + (float3(other.scalar) * this.group0.xyz)), other.scalar * this.e12345);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1);
        float anti_wedge_g2_x = (other.scalar * this.e235) - (other.e15 * this.e12345);
        float anti_wedge_g2_y = (other.scalar * this.e315) - (other.e25 * this.e12345);
        float anti_wedge_g2_z = (other.scalar * this.e125) - (other.e35 * this.e12345);
        float anti_wedge_g2_w = (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g0[0] * this.e415) - float4((this.group0.zxy * other.group2.yzx), right_dual_g0[1] * this.e425);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g2_y * other.e42) - (anti_wedge_g2_z * other.e43) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2_x * other.e41) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2_w) * other.group1.xyz) + (float3(other.scalar) * float3(anti_wedge_g2_x, anti_wedge_g2_y, anti_wedge_g2_z)) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2_w * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((this.group0.zxy * right_dual_g3.yzx), 0.0) - (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], right_dual_g1_w * this.e321) * float4(this.group0.yzx, 1.0));
        float4 anti_wedge_g1 = (this.group2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) + float4(float3(0.0), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((right_dual_g1_xyz * float3(this.e12345)), 0.0) + float4((float3(right_dual_g3[3]) * this.group0.xyz), 0.0) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (right_dual_g2_xyz * float3(this.e12345)) + (float3(right_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_dual_g3[1] * this.e235) - (right_dual_g3[0] * this.e315)) + float3((right_dual_g3.zx * this.group2.yz), 0.0) - float3((right_dual_g3.yz * this.group2.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g3[2] * other.e3) + (right_dual_g2_w * other.e5 * this.e12345) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), anti_wedge_g3[1] * other.e2) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_via_horizon_onto(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar * other.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group2 * other.group0.xy.yyyx, 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(AntiFlatPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float anti_wedge_g2_w = -(right_dual_g0_w * this.e321) - (right_dual_g0_xyz[0] * this.e423) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * other.e321), 
            /* e235, e315, e125, e12345 */
            other.group0 * float4(float3(anti_wedge_g2_w), right_dual_g0_w * this.e12345) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0_xyz[0] * this.e423) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412)) + float4((this.group0.zxy * right_dual_g1.yzx), 0.0) - (float4(right_dual_g1[2], right_dual_g1[0], right_dual_g1[1], right_dual_g0_w * this.e321) * float4(this.group0.yzx, 1.0));
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + float4((float3(right_dual_g1[3]) * this.group0.xyz), right_dual_g0_w * this.e12345) - (right_dual_g1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((right_dual_g0_xyz * float3(this.e12345)), 0.0) + float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((right_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_dual_g1.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_dual_g1 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g2[3]) * other.group1.zxyw) + float4(float3(0.0), (anti_wedge_g3[0] * other.e1) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)) - (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group1.yz), 0.0) - float3((anti_wedge_g2.yz * other.group1.zx), 0.0)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = right_dual_g0 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412);
        float4 anti_wedge_g3 = (this.group1.wwwx * float4(right_dual_g0, other.e15)) + float4(float3(0.0), (other.e35 * this.e435) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) + float4((other.group1.zxy * this.group0.yzx), other.e25 * this.e425) - float4((other.group1.yzx * this.group0.zxy), right_dual_g0[0] * this.e235);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) + float4((other.group1.zxy * anti_wedge_g3.yzx), 0.0) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g1_xyz[0] * other.e23) * float4(other.group1.yzx, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * float4(this.group0.xyz, this.e12345);
        float4 anti_wedge_g1 = float4(((float3(other.scalar) * this.group1.xyz) - (float3(this.e12345) * other.group0.xyz)), other.scalar * this.e321);
        float4 anti_wedge_g2 = (this.group2 * float4(float3(other.scalar), other.e3215)) + float4(float3(0.0), (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435)) - float4((float3(this.e12345) * other.group1.xyz), 0.0);
        float4 anti_wedge_g3 = float4(float3(0.0), (other.e31 * this.e431) + (other.e12 * this.e412)) + float4((this.group0.yzx * other.group1.zxy), other.e23 * this.e423) - float4((float3(this.e321) * other.group0.xyz), 0.0) - float4((this.group0.zxy * other.group1.yzx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group1.zx), 0.0) - float3((anti_wedge_g3.zx * other.group1.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(AntiPlane other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float3 anti_wedge_g0 = (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) - (right_dual_g0.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_dual_g0.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_dual_g0 * float4(this.e12345);
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g2[3])) + (float4(anti_wedge_g2[2], anti_wedge_g2[0], anti_wedge_g2[1], anti_wedge_g3[0]) * other.group0.yzxx) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3)) - float4((anti_wedge_g2.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_project_via_horizon_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(Circle other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group2.xyz), -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_dual_g2_xyz * float3(this.e12345);
        float anti_wedge_g2_w = -(right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(anti_wedge_g2_w) * other.group2.xy), anti_wedge_g2_w * other.e125, (anti_wedge_g2_w * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_dual_g2 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) + float4((right_dual_g2.yzx * this.group0.zxy), 0.0) - (this.group2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g2[0] * this.e415);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group2.zxy * anti_wedge_g3.yzx), 0.0) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0] * other.e15) * float4(other.group2.yzx, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345 * -1.0) * other.group2.xyz;
        float anti_wedge_g2_w = (this.e12345 * other.e1234) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = (float4(right_dual_g1[0], right_dual_g1[1], right_dual_g1[2], other.e15) * this.group1.wwwx) + (this.group1.xyzy * float4(float3(right_dual_g1[3]), other.e25)) + (this.group2.yzxw * float4(right_dual_g0.zxy, other.e3215)) + (other.group2.zxyz * float4(this.group0.yzx, this.e435)) - (this.group2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - (this.group2.wwwy * float4(other.group3.xyz, right_dual_g1[1])) - float4((this.group0.zxy * other.group2.yzx), right_dual_g1[2] * this.e125);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g2_w * other.e3215) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0]) * other.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_via_horizon_onto(DualNum other) {
        float4 anti_wedge_g0 = float4((float3(other.e5 * -1.0) * this.group0.xyz), this.e12345 * other.e12345 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g1 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45 * -1.0) * float4(anti_wedge_g1.xyz, this.e12345 * other.e45), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g1 = (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), (this.e435 * other.e35) + (this.e12345 * other.e3215)) - float4((float3(this.e12345) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group0.yzx), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * anti_wedge_g1.xyz), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g0 = (right_dual_g0 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g1_xyz[0] * this.e423) - (right_dual_g1_xyz[1] * this.e431) - (right_dual_g1_xyz[2] * this.e412) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g0[0] * this.e12345) - (right_dual_g5[0] * this.e415) - (right_dual_g5[1] * this.e425) - (right_dual_g5[2] * this.e435) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = (this.group2.yzxw * float4(right_dual_g7.zxy, other.e1234)) + float4(float3(0.0), -(right_dual_g7[2] * this.e435) - (right_dual_g6[0] * this.e423) - (right_dual_g6[1] * this.e431) - (right_dual_g6[2] * this.e412)) + float4((float3(right_dual_g6[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_dual_g6.xyz), 0.0) + float4((right_dual_g8.yzx * this.group0.zxy), 0.0) - float4((float3(this.e12345) * other.group9.xyz), 0.0) - float4((right_dual_g7.yzx * this.group2.zxy), right_dual_g7[0] * this.e415) - float4((right_dual_g8.zxy * this.group0.yzx), right_dual_g7[1] * this.e425);
        float anti_wedge_g2 = (this.e12345 * other.e3215) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = (right_dual_g3 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) + float4((float3(right_dual_g9[3]) * this.group1.xyz), 0.0) + float4((right_dual_g9.zxy * this.group2.yzx), 0.0) - (right_dual_g9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + (this.group0.zxy * right_dual_g9.yzx) - (this.group0.yzx * right_dual_g9.zxy);
        float3 anti_wedge_g5 = (right_dual_g5 * float3(this.e12345)) + (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_dual_g9.xyz);
        float4 anti_wedge_g6 = (right_dual_g6 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group1);
        float3 anti_wedge_g7 = (right_dual_g7 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group0.xyz);
        float3 anti_wedge_g8 = (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group2.xyz);
        float4 anti_wedge_g9 = right_dual_g9 * float4(this.e12345);
        float anti_wedge_g10 = right_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g0[1] * this.e12345 * other.scalar) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(right_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(right_dual_g1) * this.group2.xyz), 0.0) + float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) - (right_dual_g0.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_dual_g0[3]) * this.group1.xyz) + (right_dual_g0.zxy * this.group2.yzx) + float3(float2(0.0), (right_dual_g0[0] * this.e315) * -1.0) - float3((right_dual_g0.yz * this.group2.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g0 * float4(this.e12345);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], right_dual_g1 * this.e12345)) + (other.group0.yzxx * float4(anti_wedge_g2_xyz.zxy, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4)) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_via_horizon_onto(Scalar other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar * other.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_project_via_horizon_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (right_dual_g0 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) + float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0) + float4((this.group0.zxy * right_dual_g3.yzx), 0.0) - (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], right_dual_g1_w * this.e321) * float4(this.group0.yzx, 1.0));
        float4 anti_wedge_g1 = (this.group2 * float4(float3(right_dual_g2_w), right_dual_g1_w)) + float4(float3(0.0), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((right_dual_g1_xyz * float3(this.e12345)), 0.0) + float4((float3(right_dual_g3[3]) * this.group0.xyz), 0.0) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (right_dual_g2_xyz * float3(this.e12345)) + (float3(right_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_dual_g3[1] * this.e235) - (right_dual_g3[0] * this.e315)) + float3((right_dual_g3.zx * this.group2.yz), 0.0) - float3((right_dual_g3.yz * this.group2.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (right_dual_g2_w * this.e12345 * other.e5) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4(((float3(other.scalar) * this.group0.xyz) - (float3(this.e12345) * other.group0.xyz)), this.e12345 * other.scalar);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1);
        float4 anti_wedge_g2 = (this.group2 * float4(float3(other.scalar), other.e3215)) + float4(float3(0.0), -(right_dual_g2_xyz[0] * this.e415) - (right_dual_g2_xyz[1] * this.e425) - (right_dual_g2_xyz[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + float4((right_dual_g2_xyz * float3(this.e12345)), 0.0);
        float4 anti_wedge_g3 = (float4(right_dual_g1[0], right_dual_g1[1], right_dual_g1[2], other.e41) * this.group1.wwwx) + (this.group1.xyzy * float4(float3(right_dual_g1[3]), other.e42)) + (other.group0.yzxz * float4(this.group2.zxy, this.e435)) + float4(float3(0.0), -(right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((right_dual_g2_xyz.yzx * this.group0.zxy), this.e12345 * other.e1234) - float4((float3(this.e12345) * other.group3.xyz), 0.0) - float4((right_dual_g2_xyz.zxy * this.group0.yzx), right_dual_g1[0] * this.e423) - float4((this.group2.yzx * other.group0.zxy), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g2[3] * other.e1234) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Dipole> operator /(Dipole lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2_w) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), 0.0) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g3_w * this.e45) + (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g1) * other.group3.xyz), 0.0) - (anti_wedge_g0 * float4(float3(other.e5), other.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_via_horizon_onto(AntiDualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(other.scalar * other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e42) - (right_dual_g1_xyz[2] * this.e43)) + float4((float3(right_dual_g1_w) * this.group0.xyz), 0.0) + float4((right_dual_g1_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g1_xyz.yzx * this.group1.zxy), right_dual_g1_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g1_w * this.e45) + (right_dual_g1_xyz[0] * this.e15) + (right_dual_g1_xyz[1] * this.e25) + (right_dual_g1_xyz[2] * this.e35);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) + float3(float2(0.0), anti_wedge_g0[0] * other.e2 * -1.0) - float3((anti_wedge_g0.yz * other.group1.zx), 0.0)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e321 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public AntiLine anti_project_via_horizon_onto(AntiLine other) {
        float anti_wedge_g0 = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public VersorOdd anti_project_via_horizon_onto(AntiMotor other) {
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group2.xyz;
        float anti_wedge_g2_w = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(anti_wedge_g0, anti_wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g2_w) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.zxyw * float4(anti_wedge_g0.yzx, anti_wedge_g2_w)) + float4(float3(0.0), -(anti_wedge_g2_xyz[0] * other.e23) - (anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0) - (other.group1.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e41);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) + float3(float2(0.0), anti_wedge_g0[0] * other.e2 * -1.0) - float3((anti_wedge_g0.yz * other.group0.zx), 0.0)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35 */
            (float3(right_dual_g0_w * this.e45) * other.group0.xyz) + (right_dual_g0_xyz * this.group2.xyz * other.group0.xyz) + (right_dual_g0_xyz.yxx * this.group2.yxx * other.group0.xyz) + (right_dual_g0_xyz.zzy * this.group2.zzy * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_via_horizon_onto(Dipole other) {
        float anti_wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float anti_wedge_g0 = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public FlatPoint anti_project_via_horizon_onto(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * other.e5) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public FlatPoint anti_project_via_horizon_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            (float4(this.e41 * other.e15, this.e42 * other.e25, this.e43 * other.e35, this.e41 * other.e15) * other.group0) + (float4(this.e42 * other.e25, this.e41 * other.e15, this.e41 * other.e15, this.e42 * other.e25) * other.group0) + (other.group0 * float4(float2(this.e43 * other.e35), this.e42 * other.e25, this.e43 * other.e35)) - (float4(this.e45 * other.e45) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float anti_wedge_g0 = (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float4 anti_wedge_g0 = float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((anti_wedge_g0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float anti_wedge_g0_x = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (this.e45 * other.e45);
        float4 anti_wedge_g1 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g9_xyz[1] * this.e42) - (right_dual_g9_xyz[2] * this.e43)) + float4((float3(right_dual_g9_w) * this.group0.xyz), 0.0) + float4((right_dual_g9_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g9_xyz.yzx * this.group1.zxy), right_dual_g9_xyz[0] * this.e41);
        float anti_wedge_g2 = (right_dual_g9_w * this.e45) + (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35);
        float4 anti_wedge_g3 = float4(right_dual_g0[1]) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(right_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(right_dual_g0[1]) * this.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g0_w * this.e45) + (right_dual_g0_xyz[0] * this.e15) + (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1) * other.group0.yzxw) - (anti_wedge_g0.yzxw * float4(other.group0.zxy, other.e5)), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g1) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_via_horizon_onto(Scalar other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(other.scalar * other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) + float4((float3(right_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[0] * this.e41);
        float anti_wedge_g1 = (right_dual_g3_w * this.e45) + (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1) * other.group3.yzxw) - (float4(other.e3, other.e1, other.e2, other.e5) * anti_wedge_g0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g1) * other.group3.xyz), 0.0) - (anti_wedge_g0 * float4(float3(other.e5), other.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Dipole {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = float3(other.scalar) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float4 anti_wedge_g2 = float4((float3(other.scalar) * this.group2.xyz), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_dual_g2_xyz[0] * this.e41) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2[3]) * other.group0.xyz)), anti_wedge_g2[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(float3(other.scalar), other.e1234)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)) + float4((float3(anti_wedge_g2[3]) * other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23)) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((float3(right_dual_g2[3]) * this.group0.xyz), other.e41 * this.e15) + float4((other.group0.yzx * this.group3.zxy), other.e42 * this.e25) - (other.group1 * float4(float3(this.e1234), this.e45)) - float4((other.group0.zxy * this.group3.yzx), right_dual_g2[0] * this.e41);
        float4 anti_wedge_g1 = (right_dual_g2 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((float3(right_dual_g2[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0);
        float3 anti_wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group2.xyz) + (right_dual_g2.yzx * this.group3.zxy) + float3(float2(0.0), (right_dual_g2[1] * this.e4235) * -1.0) - (float3(this.e3215) * other.group1.xyz) - float3((right_dual_g2.zx * this.group3.yz), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (other.group2 * float4(float3(anti_wedge_g0[3]), right_dual_g2[3] * this.e1234)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)) + float4((anti_wedge_g2_xyz * float3(other.scalar)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual_g2[3] * other.scalar) * this.group3) + float4(float3(0.0), -(anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g3_xyz * float3(this.e1234)) - (float3(right_dual_g2_w) * this.group3.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g3_xyz.yzx * this.group3.zxy), right_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g2_w));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (right_dual_g1_w * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_dual_g3_xyz[0] * this.e41) - (right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) - float4((right_dual_g3_xyz * float3(this.e3215)), right_dual_g2_w * this.e45);
        float4 anti_wedge_g3 = (float4(right_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_dual_g2_xyz, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_dual_g1_xyz.zxy * this.group3.yzx), right_dual_g3_xyz[1] * this.e25) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), right_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1_w)) - (this.group3.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0])) - float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g2_xyz[1] * this.e4315);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(other.e2, other.e3, other.e1, other.e4) * anti_wedge_g3.zxyw) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[3]) * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (float4(anti_wedge_g2[0], anti_wedge_g2[1], anti_wedge_g2[2], anti_wedge_g3[0]) * other.group2.wwwx) + (anti_wedge_g3.yzxy * float4(other.group1.zxy, other.e315)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1.yzx * other.group3.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group3.yzxx) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_via_horizon_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = other.group0.xy.yyyx * float4(this.group0.xyz, this.e1234);
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar * other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4((float2(other.scalar) * anti_wedge_g3.xy), anti_wedge_g3[2] * other.scalar, (anti_wedge_g0[3] * other.e3215) + (anti_wedge_g3[3] * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_via_horizon_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float4 anti_wedge_g0 = right_dual_g0 * float4(this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz), (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125) - (right_dual_g0[0] * other.e321 * this.e4235) - (right_dual_g0[1] * other.e321 * this.e4315) - (right_dual_g0[2] * other.e321 * this.e4125) - (right_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = right_dual_g1_xyz * float3(this.e1234);
        float3 anti_wedge_g1_xyz = (right_dual_g1_xyz.yzx * this.group3.zxy) - (right_dual_g1_xyz.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4((float2(right_dual_g1_w) * this.group3.xy), right_dual_g1_w * this.e4125, right_dual_g0_w * this.e1234) + float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e42) - (right_dual_g1_xyz[2] * this.e43)) - float4((right_dual_g1_xyz * float3(this.e3215)), right_dual_g1_xyz[0] * this.e41);
        float4 anti_wedge_g3 = (float4(right_dual_g1_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_dual_g0_xyz, right_dual_g1_xyz[0])) + float4(float3(0.0), (right_dual_g1_xyz[2] * this.e35) - (right_dual_g0_xyz[0] * this.e4235) - (right_dual_g0_xyz[1] * this.e4315) - (right_dual_g0_xyz[2] * this.e4125)) + float4((right_dual_g1_xyz.zxy * this.group1.yzx), right_dual_g1_xyz[1] * this.e25) - float4((right_dual_g1_xyz.yzx * this.group1.zxy), right_dual_g0_w * this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group1.yzx) + float3(float2(0.0), anti_wedge_g3[0] * other.e2 * -1.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321)) - float4((float3(other.e5) * anti_wedge_g3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) + (anti_wedge_g3[3] * other.e321) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group1.zxy), anti_wedge_g3[0] * other.e235) - (float4(other.e5) * float4(anti_wedge_g0, right_dual_g1_w * this.e1234)) - (anti_wedge_g2.wwwy * float4(other.group0.xyz, other.e2)) - (other.group1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e1234);
        float3 anti_wedge_g1_xyz = right_dual_g1 * float3(this.e1234);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) + float4((right_dual_g0 * float3(this.e3215)), 0.0) + float4((right_dual_g1.yzx * this.group3.zxy), 0.0) - float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g0[0] * this.e23);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e25) - (anti_wedge_g1_xyz[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(-(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) * other.group0.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g1_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = (float4(this.e41, this.e42, this.e43, this.e23) * other.group0.wwwx) + float4(float3(0.0), (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)) - float4((float3(this.e1234) * other.group0.xyz), 0.0);
        float4 anti_wedge_g1 = (float4(other.scalar) * this.group1) + float4(float3(0.0), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(this.e1234) * other.group1.xyz), 0.0);
        float4 anti_wedge_g2 = float4(((float3(other.scalar) * this.group2.xyz) + float3(float2(0.0), (other.e25 * this.e4235) - (other.e15 * this.e4315)) + float3((other.group1.zx * this.group3.yz), 0.0) - (float3(this.e3215) * other.group0.xyz) - float3((other.group1.yz * this.group3.zx), 0.0)), other.scalar * this.e1234);
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(anti_wedge_g1[3]), anti_wedge_g3[3])) + (anti_wedge_g0.yzxw * other.group1.zxyw) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(other.scalar) * anti_wedge_g3.xyz), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = right_dual_g0_xyz * float3(this.e1234);
        float3 anti_wedge_g1_xyz = (right_dual_g0_xyz.yzx * this.group3.zxy) - (right_dual_g0_xyz.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_dual_g0_xyz * float3(this.e3215)), right_dual_g0_xyz[0] * this.e41);
        float4 anti_wedge_g3 = (float4(right_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), right_dual_g0_xyz[0] * this.e15) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3] * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group0.yzx) + float3(float2(0.0), anti_wedge_g3[0] * other.e2 * -1.0) - float3((anti_wedge_g3.yz * other.group0.zx), 0.0)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group0.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((float3(other.e5) * anti_wedge_g3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, right_dual_g0_w * this.e1234)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float4 anti_wedge_g0 = (float4(this.e1234) * float4(other.group2.xyz, right_dual_g1_w)) + (this.group3.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group3.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(other.e235) * float4(anti_wedge_g0[3], other.e431 * this.e4235, other.e412 * this.e4235, other.e321 * this.e4235)) - (float4(other.e315) * float4(other.e423 * this.e4315, anti_wedge_g0[3], other.e412 * this.e4315, other.e321 * this.e4315)) - (float4(other.e125) * float4(other.e423 * this.e4125, other.e431 * this.e4125, anti_wedge_g0[3], other.e321 * this.e4125)) - (float4(other.e423 * other.e235, other.e431 * other.e315, other.e412 * other.e125, right_dual_g1_w * other.e321) * this.group3) - float4((float3(right_dual_g1_w * this.e3215) * other.group0.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float4 anti_wedge_g0 = (float4(this.e1234) * float4(right_dual_g2_xyz, right_dual_g1_w)) + (this.group3.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group3.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], right_dual_g2_xyz[0] * this.e4235) * other.group1.yzxw) - (float4(right_dual_g1_w * other.e423, right_dual_g1_w * other.e431, right_dual_g1_w * other.e412, right_dual_g2_xyz[1] * other.e321) * this.group3.wwwy) - (float4(right_dual_g2_xyz[0] * other.e423, right_dual_g2_xyz[1] * other.e431, right_dual_g2_xyz[2] * other.e412, right_dual_g1_w * other.e321) * this.group3) - (float4(right_dual_g2_xyz[1] * other.e423, right_dual_g2_xyz[0] * other.e431, right_dual_g2_xyz[0] * other.e412, right_dual_g2_xyz[2] * other.e321) * this.group3.yxxz) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((other.group0.xyz * right_dual_g2_xyz.zzy * this.group3.zzy), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((right_dual_g2 * float3(this.e1234)), other.e23 * this.e4235) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0);
        float4 anti_wedge_g2 = float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((right_dual_g2.yzx * this.group3.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e3215), this.e45)) - float4((right_dual_g2.zxy * this.group3.yzx), right_dual_g2[0] * this.e41);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float3 anti_wedge_g0 = (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx);
        float anti_wedge_g1_x = -(other.e41 * this.e3215) - (other.e45 * this.e4235) - (other.e15 * this.e1234);
        float anti_wedge_g1_y = -(other.e42 * this.e3215) - (other.e45 * this.e4315) - (other.e25 * this.e1234);
        float anti_wedge_g1_z = -(other.e43 * this.e3215) - (other.e45 * this.e4125) - (other.e35 * this.e1234);
        float4 anti_wedge_g2 = (other.group2.zxyx * float4(this.group3.yzx, this.e41)) + float4(float3(0.0), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group1 * float4(float3(this.e3215), this.e45)) - (this.group3.zxyx * float4(other.group2.yzx, other.e4235));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g1_x * other.e41) - (anti_wedge_g1_y * other.e42) - (anti_wedge_g1_z * other.e43) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1_y * other.e25) - (anti_wedge_g1_z * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(other.e45) * float3(anti_wedge_g1_x, anti_wedge_g1_y, anti_wedge_g1_z)), 0.0) + float4((float3((other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) * other.group1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1_x))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_via_horizon_onto(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * other.e5) * this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e1234 * other.e5 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35)) - (other.group0 * float4(float3(this.e1234), this.e45)) - float4((float3(other.e45) * this.group3.xyz), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e1234 * other.e3215) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)) - (float4(other.e45, other.e45, other.e45, other.e4235) * this.group3.xyzx) - (other.group0 * float4(float3(this.e1234), this.e45));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Plane anti_project_via_horizon_onto(Line other) {
        float4 anti_wedge_g0 = float4(float3(0.0), -(this.e4315 * other.e315) - (this.e4125 * other.e125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group3.zxyx);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e435, other.e415, other.e425, other.e235) * anti_wedge_g0.yzxx) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((other.group0.yzx * anti_wedge_g0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((float3(right_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(right_dual_g0_xyz.yzx, right_dual_g1[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (right_dual_g1[3] * this.e1234 * other.e5)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_dual_g6_xyz[0] * this.e23) - (right_dual_g6_xyz[1] * this.e31) - (right_dual_g6_xyz[2] * this.e12) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (this.e45 * other.e45);
        float4 anti_wedge_g1 = (right_dual_g3 * float4(this.e1234)) + (this.group3.yzxx * float4(right_dual_g5.zxy, other.e423)) + float4((float3(right_dual_g9_w) * this.group0.xyz), this.e4315 * other.e431) + float4((right_dual_g9_xyz.zxy * this.group1.yzx), this.e4125 * other.e412) - (float4(right_dual_g10) * float4(this.e15, this.e25, this.e35, this.e45)) - float4((float3(this.e3215) * other.group7.xyz), right_dual_g9_xyz[2] * this.e43) - float4((right_dual_g5.yzx * this.group3.zxy), right_dual_g9_xyz[0] * this.e41) - float4((right_dual_g9_xyz.yzx * this.group1.zxy), right_dual_g9_xyz[1] * this.e42);
        float anti_wedge_g2 = (right_dual_g9_w * this.e45) + (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35) - (right_dual_g3[0] * this.e4235) - (right_dual_g3[1] * this.e4315) - (right_dual_g3[2] * this.e4125) - (right_dual_g3[3] * this.e3215);
        float4 anti_wedge_g3 = (float4(right_dual_g0[1]) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g6_xyz[1] * this.e4315) - (right_dual_g6_xyz[2] * this.e4125)) + float4((right_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_dual_g8.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0]));
        float3 anti_wedge_g4 = (right_dual_g6_xyz * float3(this.e1234)) + (float3(right_dual_g0[1]) * this.group0.xyz) + (right_dual_g7.zxy * this.group3.yzx) - (right_dual_g7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) + (float3(right_dual_g0[1]) * this.group1.xyz) - (float3(other.e45) * this.group3.xyz);
        float4 anti_wedge_g6 = float4((right_dual_g9_xyz.yzx * this.group3.zxy), right_dual_g9_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g9_xyz.zxy, right_dual_g10));
        float3 anti_wedge_g7 = (right_dual_g9_xyz * float3(this.e1234)) - (float3(right_dual_g10) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(right_dual_g9_w) * this.group3.xyz) - (right_dual_g9_xyz * float3(this.e3215));
        float4 anti_wedge_g9 = float4(right_dual_g0[1]) * this.group3;
        float anti_wedge_g10 = right_dual_g0[1] * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e1234 * other.e3215) * other.group0) - (float4(this.e4235 * other.e4235, this.e4315 * other.e4315, this.e4125 * other.e4125, this.e4235 * other.e4235) * other.group0) - (float4(this.e4315 * other.e4315, this.e4235 * other.e4235, this.e4235 * other.e4235, this.e4315 * other.e4315) * other.group0) - (other.group0 * float4(float2(this.e4125 * other.e4125), this.e4315 * other.e4315, this.e4125 * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g0_xyz * float3(this.e1234)) - (float3(right_dual_g1) * this.group3.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g0_xyz.yzx * this.group3.zxy), right_dual_g0_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g0_xyz.zxy, right_dual_g1));
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0_xyz[0] * this.e41) - (right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_dual_g0_xyz * float3(this.e3215)), right_dual_g1 * this.e45);
        float4 anti_wedge_g3 = (float4(right_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), right_dual_g0_xyz[0] * this.e15) - float4((float3(right_dual_g1) * this.group2.xyz), 0.0) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group0.yzxw) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) * -1.0) - float4((anti_wedge_g3.yzx * other.group0.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group0 * float4(float3(anti_wedge_g3[3]), anti_wedge_g1[3])) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((float3(other.e5) * anti_wedge_g3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) + float4((anti_wedge_g1.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(Scalar other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar * other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_via_horizon_onto(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g3_xyz * float3(this.e1234)) - (float3(right_dual_g2_w) * this.group3.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g3_xyz.yzx * this.group3.zxy), right_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g2_w));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(float3(right_dual_g3_w), right_dual_g0_xyz[0])) + float4(float3(0.0), (right_dual_g1_w * this.e1234) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) - (right_dual_g3_xyz[0] * this.e41) - (right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) - float4((right_dual_g3_xyz * float3(this.e3215)), right_dual_g2_w * this.e45);
        float4 anti_wedge_g3 = (float4(right_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_dual_g2_xyz, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_dual_g1_xyz.zxy * this.group3.yzx), right_dual_g3_xyz[1] * this.e25) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), right_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(right_dual_g0_xyz, right_dual_g1_w)) - (this.group3.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0])) - float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g2_xyz[1] * this.e4315);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) * -1.0) - float4((anti_wedge_g3.yzx * other.group3.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(float3(anti_wedge_g3[3]), anti_wedge_g1[3])) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2[2] * other.e3)) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) + float4((anti_wedge_g1.yzx * other.group3.zxy), 0.0) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group3.yzxx) - (other.group2 * float4(float3(anti_wedge_g2[3]), anti_wedge_g1[3])) - float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[1] * other.e2) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (float4(right_dual_g0[2], right_dual_g0[0], right_dual_g0[1], right_dual_g3_xyz[0]) * this.group3.yzxx) + float4(float3(0.0), (right_dual_g3_xyz[2] * this.e4125) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_dual_g2_xyz[0] * this.e41) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35)) + float4((float3(right_dual_g0[3]) * this.group0.xyz), right_dual_g3_xyz[1] * this.e4315) - (other.group1 * float4(float3(this.e1234), this.e45)) - (right_dual_g0.yzxx * float4(this.group3.zxy, this.e15));
        float4 anti_wedge_g1 = (right_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), (this.e4315 * other.e31) + (this.e4125 * other.e12)) + float4((right_dual_g2_xyz * float3(this.e1234)), this.e4235 * other.e23) + float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0);
        float4 anti_wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (right_dual_g2_xyz.yzx * this.group3.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_dual_g2_xyz.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * other.group3) + (float4(right_dual_g0[3] * other.scalar) * this.group3) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<DualNum> operator /(DualNum lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), right_dual_g2[3]);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = right_dual_g2 * float4(float3(this.e12345), this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(other.scalar) * anti_wedge_g0.xyz), (anti_wedge_g0[3] * other.scalar) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        float4 anti_wedge_g0 = float4((float3(this.e12345) * other.group0.xyz), right_dual_g2[3] * this.e5);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = right_dual_g2 * float4(this.e12345);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g2[3]) * other.group3.zxyw) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), anti_wedge_g3[0] * other.e1) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_via_horizon_onto(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.scalar * other.scalar * this.e5) + (other.e3215 * other.scalar * this.e12345), other.scalar * other.scalar * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e12345 * (other.e321 * -1.0) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(AntiFlector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group1.xyz), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g0[3] * other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345 * -1.0) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = right_dual_g0 * float4(this.e12345);
        float anti_wedge_g1_w = (right_dual_g0[3] * this.e5) + (other.e3215 * this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * anti_wedge_g0.xyz), (anti_wedge_g0[3] * other.scalar) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1_w) * other.group0.xyz) - (float3(other.scalar * this.e12345) * other.group1.xyz)), anti_wedge_g1_w * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Circle other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), (anti_wedge_g2[3] * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345 * -1.0) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g2 = float3(this.e12345 * -1.0) * other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345 * -1.0) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g2[3] * other.e3215) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0]) * other.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * this.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * anti_wedge_g1.xyz), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e12345 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345);
        float4 anti_wedge_g1 = float4(this.e12345) * float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float anti_wedge_g2 = (right_dual_g0[1] * this.e5) + (this.e12345 * other.e3215);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group6.xyz;
        float4 anti_wedge_g6 = float4(this.e12345) * float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 anti_wedge_g7 = float3(this.e12345 * -1.0) * other.group4.xyz;
        float3 anti_wedge_g8 = float3(this.e12345 * -1.0) * other.group3.xyz;
        float4 anti_wedge_g9 = float4(this.e12345) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g10 = right_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g0[1] * this.e12345 * other.scalar) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_via_horizon_onto(RoundPoint other) {
        float right_dual_g1 = other.e4 * -1.0;
        float anti_wedge_g0_w = right_dual_g1 * this.e5;
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_wedge_g3[0] * other.e1) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) + (right_dual_g1 * this.e12345 * other.e5)), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), anti_wedge_g0_w * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * other.group0
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
            /* e5, e12345 */
            float2(other.scalar * other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_via_horizon_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        float4 anti_wedge_g0 = float4((float3(this.e12345) * other.group0.xyz), (right_dual_g2[3] * this.e5) - (this.e12345 * other.e12345));
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = right_dual_g2 * float4(this.e12345);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 anti_wedge_g0 = right_dual_g0 * float4(this.e12345);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float2 anti_wedge_g2_xy = float2(this.e12345) * right_dual_g2_xyz.xy;
        float anti_wedge_g2_z = right_dual_g2_xyz[2] * this.e12345;
        float anti_wedge_g2_w = (right_dual_g0[3] * this.e5) + (this.e12345 * other.e3215);
        float4 anti_wedge_g3 = float4(this.e12345) * float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g2_w * other.e1234) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g2_xy[0] * other.e41) - (anti_wedge_g2_xy[1] * other.e42) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2_z) * other.group0.zxyz), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(float3(anti_wedge_g2_w), anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) + float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2_w) * other.group1.xyz) + (float3(other.scalar) * float3(anti_wedge_g2_xy, anti_wedge_g2_z)) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2_w * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<FlatPoint> operator /(FlatPoint lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 anti_wedge_g1 = float4(float3(0.0), other.scalar * this.e45);
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group0.xyz;
        float anti_wedge_g2_w = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g2_w) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2_w) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g2_w) * other.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(other.e4) * this.group0;
        float anti_wedge_g1 = (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35) - (other.e5 * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g1) * other.group3.xyz), 0.0) - (anti_wedge_g0 * float4(float3(other.e5), other.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_via_horizon_onto(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public Flector anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float anti_wedge_g0_x = (right_dual_g1_xyz[0] * this.e15) + (right_dual_g1_xyz[1] * this.e25) + (right_dual_g1_xyz[2] * this.e35) - (other.e5 * this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_wedge_g0_x * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public Flector anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * this.e45 * -1.0) * other.group0.xyz), 0.0) + float4((float3(right_dual_g0_xyz[0] * this.e15) * other.group0.xyz), 0.0) + float4((float3(right_dual_g0_xyz[1] * this.e25) * other.group0.xyz), 0.0) + float4((float3(right_dual_g0_xyz[2] * this.e35) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public Dipole anti_project_via_horizon_onto(Dipole other) {
        float anti_wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float anti_wedge_g0 = (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_via_horizon_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float anti_wedge_g0 = this.e45 * other.e45 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float3 right_dual_g9_xyz = other.group1.xyz;
        float anti_wedge_g0_x = (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45);
        float4 anti_wedge_g1 = float4(other.e4) * this.group0;
        float anti_wedge_g2 = (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35) - (this.e45 * other.e5);
        float4 anti_wedge_g3 = float4(other.scalar) * this.group0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g1[1] * other.e315) * float4(other.group4.yzx, 1.0)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e12) * -1.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), anti_wedge_g1[2] * other.e125) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g3[1] * other.e31) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public Dipole anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 anti_wedge_g0 = float4(other.e4) * this.group0;
        float anti_wedge_g1 = (right_dual_g0_xyz[0] * this.e15) + (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35) - (this.e45 * other.e5);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1) * other.group0.yzxw) - (anti_wedge_g0.yzxw * float4(other.group0.zxy, other.e5)), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g1) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_via_horizon_onto(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float4 anti_wedge_g0 = float4(other.e4) * this.group0;
        float anti_wedge_g1 = (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35) - (this.e45 * other.e5);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1) * other.group3.yzxw) - (float4(other.e3, other.e1, other.e2, other.e5) * anti_wedge_g0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) + float4((float3(anti_wedge_g1) * other.group3.xyz), 0.0) - (anti_wedge_g0 * float4(float3(other.e5), other.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g1 = float4(float3(0.0), this.e45 * other.scalar);
        float4 anti_wedge_g2 = float4((float3(other.scalar) * this.group0.xyz), (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g2[3]) * other.group0, 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(float3(other.scalar), other.e1234)) + float4(float3(0.0), -(anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)) + float4((float3(anti_wedge_g2[3]) * other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Flector> operator /(Flector lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35)) + float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e15) - float4((other.group0.zxy * this.group1.yzx), other.e45 * this.e45);
        float4 anti_wedge_g1 = float4(-(other.e41 * this.e3215) - (other.e45 * this.e4235), -(other.e42 * this.e3215) - (other.e45 * this.e4315), -(other.e43 * this.e3215) - (other.e45 * this.e4125), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125) + (other.scalar * this.e45));
        float4 anti_wedge_g2 = float4((float3(other.scalar) * this.group0.xyz), 0.0) + float4((other.group2.zxy * this.group1.yzx), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((other.group2.yzx * this.group1.zxy), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (other.group2 * float4(float3(anti_wedge_g0[3]), anti_wedge_g2[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)) + float4((float3(other.scalar) * anti_wedge_g2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar * other.scalar) * this.group1) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = float3(right_dual_g2_w * -1.0) * this.group1.xyz;
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.yzx * this.group1.zxy) - (right_dual_g3_xyz.zxy * this.group1.yzx);
        float anti_wedge_g1_w = right_dual_g2_w * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (this.group1.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((right_dual_g3_xyz * float3(this.e3215)), right_dual_g2_w * this.e45);
        float4 anti_wedge_g3 = (this.group1.yzxw * float4(right_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_dual_g3_w * this.e45) + (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35) - (right_dual_g2_xyz[2] * this.e4125)) - (this.group1.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0])) - (this.group1.wwwy * float4(other.group0.xyz, right_dual_g2_xyz[1])) - float4((float3(right_dual_g2_w) * this.group0.xyz), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(other.e2, other.e3, other.e1, other.e4) * anti_wedge_g3.zxyw) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[3]) * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (float4(anti_wedge_g2[0], anti_wedge_g2[1], anti_wedge_g2[2], anti_wedge_g3[0]) * other.group2.wwwx) + (anti_wedge_g3.yzxy * float4(other.group1.zxy, other.e315)) + float4(float3(0.0), (anti_wedge_g2[2] * other.e3) * -1.0) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[2] * other.e125) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public AntiDualNum anti_project_via_horizon_onto(AntiFlatPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiDualNum(
            /* e3215, scalar */
            float2((other.e321 * other.e321 * this.e3215) - (right_dual_g0_xyz[0] * other.e321 * this.e4235) - (right_dual_g0_xyz[1] * other.e321 * this.e4315) - (right_dual_g0_xyz[2] * other.e321 * this.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float3 anti_wedge_g0_xyz = (right_dual_g1_xyz.yzx * this.group1.zxy) - (right_dual_g1_xyz.zxy * this.group1.yzx);
        float4 anti_wedge_g1 = (this.group1 * float4(float3(right_dual_g1_w), other.e321)) + float4(float3(0.0), (right_dual_g1_w * this.e45) + (right_dual_g1_xyz[0] * this.e15) + (right_dual_g1_xyz[1] * this.e25) + (right_dual_g1_xyz[2] * this.e35) - (right_dual_g0_xyz[1] * this.e4315) - (right_dual_g0_xyz[2] * this.e4125)) - (this.group1.wwwx * float4(right_dual_g1_xyz, right_dual_g0_xyz[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0_xyz.yzx * other.group1.zxy), anti_wedge_g1[3] * other.e321) - (other.group1.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Plane anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) + float4((right_dual_g0 * float3(this.e3215)), 0.0) + float4((right_dual_g1.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (right_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) + float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) + float4((other.group1.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(other.group1.yzx, right_dual_g0[0]));
        float4 anti_wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(anti_wedge_g0[3]), anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 anti_wedge_g0_xyz = (right_dual_g0_xyz.yzx * this.group1.zxy) - (right_dual_g0_xyz.zxy * this.group1.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), (right_dual_g0_xyz[0] * this.e15) + (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) - (float4(other.e5) * float4(this.e4235, this.e4315, this.e4125, this.e45)) - float4((right_dual_g0_xyz * float3(this.e3215)), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0_xyz.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Sphere anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float4 anti_wedge_g0 = (this.group1.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group1.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(other.e235) * float4(anti_wedge_g0[3], other.e431 * this.e4235, other.e412 * this.e4235, other.e321 * this.e4235)) - (float4(other.e315) * float4(other.e423 * this.e4315, anti_wedge_g0[3], other.e412 * this.e4315, other.e321 * this.e4315)) - (float4(other.e125) * float4(other.e423 * this.e4125, other.e431 * this.e4125, anti_wedge_g0[3], other.e321 * this.e4125)) - (float4(other.e423 * other.e235, other.e431 * other.e315, other.e412 * other.e125, right_dual_g1_w * other.e321) * this.group1) - float4((float3(right_dual_g1_w * this.e3215) * other.group0.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Sphere anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float4 anti_wedge_g0 = (this.group1.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group1.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], right_dual_g2_xyz[0] * this.e4235) * other.group1.yzxw) - (float4(right_dual_g1_w * other.e423, right_dual_g1_w * other.e431, right_dual_g1_w * other.e412, right_dual_g2_xyz[1] * other.e321) * this.group1.wwwy) - (float4(right_dual_g2_xyz[0] * other.e423, right_dual_g2_xyz[1] * other.e431, right_dual_g2_xyz[2] * other.e412, right_dual_g1_w * other.e321) * this.group1) - (float4(right_dual_g2_xyz[1] * other.e423, right_dual_g2_xyz[0] * other.e431, right_dual_g2_xyz[0] * other.e412, right_dual_g2_xyz[2] * other.e321) * this.group1.yxxz) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((other.group0.xyz * right_dual_g2_xyz.zzy * this.group1.zzy), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float anti_wedge_g1_x = -(other.e41 * this.e3215) - (other.e45 * this.e4235);
        float anti_wedge_g1_y = -(other.e42 * this.e3215) - (other.e45 * this.e4315);
        float anti_wedge_g1_z = -(other.e43 * this.e3215) - (other.e45 * this.e4125);
        float4 anti_wedge_g2 = float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35)) + float4((right_dual_g2.yzx * this.group1.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e3215), this.e45)) - float4((right_dual_g2.zxy * this.group1.yzx), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g1_x * other.e41) - (anti_wedge_g1_y * other.e42) - (anti_wedge_g1_z * other.e43) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1_y * other.e25) - (anti_wedge_g1_z * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(other.e45) * float3(anti_wedge_g1_x, anti_wedge_g1_y, anti_wedge_g1_z)), 0.0) + float4((float3((other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) * other.group1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1_x * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float3 anti_wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float anti_wedge_g1_x = -(other.e41 * this.e3215) - (other.e45 * this.e4235);
        float anti_wedge_g1_y = -(other.e42 * this.e3215) - (other.e45 * this.e4315);
        float anti_wedge_g1_z = -(other.e43 * this.e3215) - (other.e45 * this.e4125);
        float4 anti_wedge_g2 = (other.group2.zxyw * this.group1.yzxw) + float4(float3(0.0), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group1 * float4(float3(this.e3215), this.e45)) - (this.group1.zxyx * float4(other.group2.yzx, other.e4235));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g1_x * other.e41) - (anti_wedge_g1_y * other.e42) - (anti_wedge_g1_z * other.e43) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1_y * other.e25) - (anti_wedge_g1_z * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(other.e45) * float3(anti_wedge_g1_x, anti_wedge_g1_y, anti_wedge_g1_z)), 0.0) + float4((float3((other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) * other.group1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1_x))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g0 = float4(other.e45 * -1.0) * float4(this.group1.xyz, this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = float4((float3(other.e45 * -1.0) * this.group1.xyz), -(other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Plane anti_project_via_horizon_onto(Line other) {
        float4 anti_wedge_g0 = float4(float3(0.0), -(this.e4315 * other.e315) - (this.e4125 * other.e125)) + float4((other.group0.zxy * this.group1.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group1.zxyx);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e435, other.e415, other.e425, other.e235) * anti_wedge_g0.yzxx) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((other.group0.yzx * anti_wedge_g0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e4315) - (right_dual_g1_xyz[2] * this.e4125)) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), this.e45 * other.e5 * -1.0) - (this.group1.zxyx * float4(right_dual_g0_xyz.yzx, right_dual_g1_xyz[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (this.e3215 * other.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (this.e45 * other.e45);
        float4 anti_wedge_g1 = (this.group1.yzxx * float4(right_dual_g5.zxy, other.e423)) + float4(float3(0.0), (this.e4315 * other.e431) + (this.e4125 * other.e412)) - (float4(right_dual_g10) * this.group0) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_dual_g5.yzx * this.group1.zxy), 0.0);
        float anti_wedge_g2 = (right_dual_g9_w * this.e45) + (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35) + (this.e3215 * other.e321) - (this.e4235 * other.e235) - (this.e4315 * other.e315) - (this.e4125 * other.e125);
        float4 anti_wedge_g3 = (float4(right_dual_g0[1]) * this.group0) + float4(float3(0.0), -(right_dual_g6_xyz[1] * this.e4315) - (right_dual_g6_xyz[2] * this.e4125)) + float4((right_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_dual_g8.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0]));
        float3 anti_wedge_g4 = (right_dual_g7.zxy * this.group1.yzx) - (right_dual_g7.yzx * this.group1.zxy);
        float3 anti_wedge_g5 = (right_dual_g7 * float3(this.e3215)) - (float3(other.e45) * this.group1.xyz);
        float4 anti_wedge_g6 = float4(((right_dual_g9_xyz.yzx * this.group1.zxy) - (right_dual_g9_xyz.zxy * this.group1.yzx)), right_dual_g10 * this.e3215 * -1.0);
        float3 anti_wedge_g7 = float3(right_dual_g10 * -1.0) * this.group1.xyz;
        float3 anti_wedge_g8 = (float3(right_dual_g9_w) * this.group1.xyz) - (right_dual_g9_xyz * float3(this.e3215));
        float4 anti_wedge_g9 = float4(right_dual_g0[1]) * this.group1;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
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
            /* e4235, e4315, e4125, e3215 */
            -(float4(this.e4235 * other.e4235, this.e4315 * other.e4315, this.e4125 * other.e4125, this.e4235 * other.e4235) * other.group0) - (float4(this.e4315 * other.e4315, this.e4235 * other.e4235, this.e4235 * other.e4235, this.e4315 * other.e4315) * other.group0) - (other.group0 * float4(float2(this.e4125 * other.e4125), this.e4315 * other.e4315, this.e4125 * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = float3(right_dual_g1 * -1.0) * this.group1.xyz;
        float3 anti_wedge_g1_xyz = (right_dual_g0_xyz.yzx * this.group1.zxy) - (right_dual_g0_xyz.zxy * this.group1.yzx);
        float anti_wedge_g1_w = right_dual_g1 * this.e3215 * -1.0;
        float3 anti_wedge_g2_xyz = (float3(right_dual_g0_w) * this.group1.xyz) - (right_dual_g0_xyz * float3(this.e3215));
        float anti_wedge_g2_w = right_dual_g1 * this.e45 * -1.0;
        float2 anti_wedge_g3_xy = float2(right_dual_g1 * -1.0) * this.group0.xy;
        float anti_wedge_g3_z = right_dual_g1 * this.e35 * -1.0;
        float anti_wedge_g3_w = (right_dual_g0_w * this.e45) + (right_dual_g0_xyz[0] * this.e15) + (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * float3(anti_wedge_g3_xy, anti_wedge_g3_z)) - (float3(anti_wedge_g2_w) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (float4(anti_wedge_g3_z, anti_wedge_g3_xy[0], anti_wedge_g3_xy[1], anti_wedge_g3_w) * other.group0.yzxw) + float4(float3(0.0), (anti_wedge_g2_w * other.e5) * -1.0) - float4((float3(anti_wedge_g3_xy[1], anti_wedge_g3_z, anti_wedge_g3_xy[0]) * other.group0.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group0 * float4(float3(anti_wedge_g3_w), anti_wedge_g1_w)) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((float3(other.e5) * float3(anti_wedge_g3_xy, anti_wedge_g3_z)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2_xyz[1] * other.e2) - (anti_wedge_g2_xyz[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_xyz[0]))
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
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public Sphere anti_project_via_horizon_onto(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = float3(right_dual_g2_w * -1.0) * this.group1.xyz;
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.yzx * this.group1.zxy) - (right_dual_g3_xyz.zxy * this.group1.yzx);
        float anti_wedge_g1_w = right_dual_g2_w * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (this.group1.xyzx * float4(float3(right_dual_g3_w), right_dual_g0_xyz[0])) + float4(float3(0.0), (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125)) - float4((right_dual_g3_xyz * float3(this.e3215)), right_dual_g2_w * this.e45);
        float4 anti_wedge_g3 = (this.group1.yzxw * float4(right_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_dual_g3_w * this.e45) + (right_dual_g3_xyz[0] * this.e15) + (right_dual_g3_xyz[1] * this.e25) + (right_dual_g3_xyz[2] * this.e35) - (right_dual_g2_xyz[2] * this.e4125)) - (this.group1.zxyy * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[1])) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g2_xyz[0])) - float4((float3(right_dual_g2_w) * this.group0.xyz), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) * -1.0) - float4((anti_wedge_g3.yzx * other.group3.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(float3(anti_wedge_g3[3]), anti_wedge_g1_w)) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2[2] * other.e3)) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[1] * other.e315) - (other.group2 * float4(float3(anti_wedge_g2[3]), anti_wedge_g1_w)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[1] * other.e2) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Flector {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (float4(right_dual_g0[2], right_dual_g0[0], right_dual_g0[1], right_dual_g3_xyz[0]) * this.group1.yzxx) + float4(float3(0.0), (right_dual_g3_xyz[1] * this.e4315) + (right_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (this.e45 * other.e45)) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e15));
        float4 anti_wedge_g1 = (right_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), (this.e4235 * other.e23) + (this.e4315 * other.e31) + (this.e4125 * other.e12)) - float4((float3(other.e45) * this.group1.xyz), 0.0);
        float4 anti_wedge_g2 = float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) + float4((right_dual_g2.yzx * this.group1.zxy), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((right_dual_g2.zxy * this.group1.yzx), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * other.group3) + (float4(right_dual_g0[3] * other.scalar) * this.group1) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Line> operator /(Line lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 anti_wedge_g1_xyz = float3(other.scalar) * this.group0.xyz;
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group1.xyz;
        float anti_wedge_g2_w = (other.e41 * this.e415) + (other.e42 * this.e425) + (other.e43 * this.e435);
        float4 anti_wedge_g3 = (float4(this.e415, this.e425, this.e435, this.e235) * other.group1.wwwx) + float4(float3(0.0), (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435)) + float4((other.group0.yzx * this.group1.zxy), other.e31 * this.e315) - float4((other.group0.zxy * this.group1.yzx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43)) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g1_xyz[0] * other.e23) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g1_xyz * float3(other.scalar)), 0.0) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float3 anti_wedge_g0 = float3(right_dual_g2_w) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(right_dual_g2_w) * this.group1.xyz;
        float anti_wedge_g1_w = -(right_dual_g3_xyz[0] * this.e415) - (right_dual_g3_xyz[1] * this.e425) - (right_dual_g3_xyz[2] * this.e435);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((float3(other.e5) * this.group0.xyz), right_dual_g1_xyz[1] * this.e425) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g1_xyz[0] * this.e415);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g1_xyz * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group3.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1_w * other.e321), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e2) - (anti_wedge_g1_xyz[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0) - (other.group3.xyzx * float4(float3(anti_wedge_g1_w), anti_wedge_g1_xyz[0])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e5)) + (float3(anti_wedge_g2[3]) * other.group2.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Motor anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435)) + float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0) + float4((right_dual_g1_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g1_xyz.yzx * this.group1.zxy), right_dual_g1_xyz[0] * this.e415);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_wedge_g0[3] * -1.0) * float4(other.group1.xyz, other.e321), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public AntiFlatPoint anti_project_via_horizon_onto(AntiLine other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e15 * this.e415) * other.group0.xyz), 0.0) + float4((float3(other.e25 * this.e425) * other.group0.xyz), 0.0) + float4((float3(other.e35 * this.e435) * other.group0.xyz), 0.0) + float4((other.group0.xyz * other.group0.xyz * this.group1.xyz), 0.0) + float4((other.group0.xyz * other.group0.yxx * this.group1.yxx), 0.0) + float4((other.group0.xyz * other.group0.zzy * this.group1.zzy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Motor anti_project_via_horizon_onto(AntiMotor other) {
        float3 anti_wedge_g0_xyz = float3(other.scalar) * this.group0.xyz;
        float anti_wedge_g1_w = (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float3(other.scalar)), -(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1_w) * other.group0.xyz) + (float3(other.scalar * other.scalar) * this.group1.xyz)), anti_wedge_g1_w * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435)) + float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e415);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            (anti_wedge_g0.zxy * other.group0.yzx) + float3(float2(0.0), (anti_wedge_g0[0] * other.e2) * -1.0) - float3((anti_wedge_g0.yz * other.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Circle anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float anti_wedge_g0 = -(right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float anti_wedge_g0 = -(right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(anti_wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Circle anti_project_via_horizon_onto(Dipole other) {
        float4 anti_wedge_g0 = float4(float3(0.0), other.e43 * this.e435) + float4((float3(other.e45) * this.group0.xyz), other.e42 * this.e425) + float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e415) - float4((other.group0.zxy * this.group1.yzx), 0.0);
        float anti_wedge_g1 = (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(float3(0.0), other.e43 * this.e435) + float4((float3(other.e45) * this.group0.xyz), other.e42 * this.e425) + float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e415) - float4((other.group0.zxy * this.group1.yzx), 0.0);
        float anti_wedge_g1 = (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g1) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215)) - float4((anti_wedge_g0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(FlatPoint other) {
        float3 anti_wedge_g0_xyz = float3(other.e45) * this.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            anti_wedge_g0_xyz * float3(other.e45 * -1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * other.group0.zxy) - (anti_wedge_g0_xyz.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float3 anti_wedge_g0_xyz = float3(other.e45) * this.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float3(other.e45 * -1.0)), (anti_wedge_g0_xyz[0] * other.e4235) + (anti_wedge_g0_xyz[1] * other.e4315) + (anti_wedge_g0_xyz[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * other.group0.zxy) - (anti_wedge_g0_xyz.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Line anti_project_via_horizon_onto(Line other) {
        float anti_wedge_g0 = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435);
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e235, e315, e125 */
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
        float3 right_dual_g0_xyz = other.group0.xyz;
        float anti_wedge_g0_w = -(right_dual_g0_xyz[0] * this.e415) - (right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_wedge_g0_w) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0_w) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = -(right_dual_g5[0] * this.e415) - (right_dual_g5[1] * this.e425) - (right_dual_g5[2] * this.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4(float3(0.0), this.e435 * other.e43) + float4((float3(other.e45) * this.group0.xyz), this.e415 * other.e41) + float4((this.group1.zxy * other.group4.yzx), this.e425 * other.e42) - float4((this.group1.yzx * other.group4.zxy), 0.0);
        float anti_wedge_g2 = (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435);
        float4 anti_wedge_g3 = float4(float3(0.0), (right_dual_g9_xyz[2] * this.e435) * -1.0) + float4((right_dual_g9_xyz.zxy * this.group1.yzx), 0.0) - (float4(other.e5, other.e5, other.e5, right_dual_g9_xyz[1] * this.e425) * float4(this.group0.xyz, 1.0)) - float4((right_dual_g9_xyz.yzx * this.group1.zxy), right_dual_g9_xyz[0] * this.e415);
        float3 anti_wedge_g4 = float3(right_dual_g10) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(right_dual_g10) * this.group1.xyz;
        float4 anti_wedge_g6 = float4((float3(right_dual_g0[1]) * this.group0.xyz), 0.0);
        float3 anti_wedge_g8 = float3(right_dual_g0[1]) * this.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public Circle anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = float3(right_dual_g1) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(right_dual_g1) * this.group1.xyz;
        float3 anti_wedge_g2 = (right_dual_g0_xyz.zxy * this.group1.yzx) - (float3(other.e5) * this.group0.xyz) - (right_dual_g0_xyz.yzx * this.group1.zxy);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz * float3(other.e4)) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4) + (right_dual_g0_xyz[0] * this.e415 * other.e1) + (right_dual_g0_xyz[1] * this.e425 * other.e1) + (right_dual_g0_xyz[2] * this.e435 * other.e1), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4) + (right_dual_g0_xyz[0] * this.e415 * other.e2) + (right_dual_g0_xyz[1] * this.e425 * other.e2) + (right_dual_g0_xyz[2] * this.e435 * other.e2), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4) + (right_dual_g0_xyz[0] * this.e415 * other.e3) + (right_dual_g0_xyz[1] * this.e425 * other.e3) + (right_dual_g0_xyz[2] * this.e435 * other.e3), -(anti_wedge_g1_xyz[0] * other.e1) - (anti_wedge_g1_xyz[1] * other.e2) - (anti_wedge_g1_xyz[2] * other.e3)), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz * float3(other.e5)) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)
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
            /* e415, e425, e435 */
            float3(other.scalar * other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float3 anti_wedge_g0 = float3(right_dual_g2_w) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(right_dual_g2_w) * this.group1.xyz;
        float anti_wedge_g1_w = -(right_dual_g3_xyz[0] * this.e415) - (right_dual_g3_xyz[1] * this.e425) - (right_dual_g3_xyz[2] * this.e435);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0_xyz[2] * this.e125) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435)) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((float3(other.e5) * this.group0.xyz), right_dual_g0_xyz[1] * this.e315) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2[3]) * other.group0) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g1_xyz * float3(other.e4)), 0.0) + float4((anti_wedge_g0.yzx * other.group3.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1_w * other.e321), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e2) - (anti_wedge_g1_xyz[2] * other.e3)) + float4((anti_wedge_g0 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2[3]) * other.group1.xyz), 0.0) - (other.group3.xyzx * float4(float3(anti_wedge_g1_w), anti_wedge_g1_xyz[0])), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e5)) + (float3(anti_wedge_g2[3]) * other.group2.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group3.yz), 0.0) - float3((anti_wedge_g2.yz * other.group3.zx), 0.0)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 anti_wedge_g1 = float4((float3(other.scalar) * this.group0.xyz), 0.0);
        float3 anti_wedge_g2_xyz = float3(other.scalar) * this.group1.xyz;
        float anti_wedge_g2_w = (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43);
        float4 anti_wedge_g3 = (float4(this.e415, this.e425, this.e435, this.e235) * other.group1.wwwx) + float4(float3(0.0), (this.e125 * other.e12) - (right_dual_g2_xyz[1] * this.e425) - (right_dual_g2_xyz[2] * this.e435)) + float4((this.group1.zxy * other.group0.yzx), this.e315 * other.e31) - float4((this.group1.yzx * other.group0.zxy), right_dual_g2_xyz[0] * this.e415);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g2_w) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2_xyz[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Motor> operator /(Motor lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), right_dual_g2[3]);
        float4 anti_wedge_g1 = float4(((float3(right_dual_g2[3]) * this.group0.xyz) - (float3(this.e12345) * other.group1.xyz)), other.e45 * this.e12345);
        float4 anti_wedge_g2 = (right_dual_g2 * float4(float3(this.e12345), this.e5)) + (this.group1.xyzx * float4(float3(right_dual_g2[3]), other.e23)) + float4(float3(0.0), (other.e31 * this.e315) + (other.e12 * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435));
        float4 anti_wedge_g3 = (this.group0.xyzx * float4(float3(other.e45), other.e41)) + float4(float3(0.0), other.e43 * this.e435) + float4((other.group0.yzx * this.group1.zxy), other.e42 * this.e425) - float4((other.group0.zxy * this.group1.yzx), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g0[0] * other.e15) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (float4(right_dual_g2_w) * float4(this.group0.xyz, this.e5)) + float4(float3(0.0), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(this.e12345) * other.group0.xyz), 0.0);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g3[0] * this.e415) - (right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0);
        float3 anti_wedge_g2_xyz = (float3(right_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz) + (right_dual_g3.zxy * this.group1.yzx) + float3(float2(0.0), (right_dual_g3[0] * this.e315) * -1.0) - float3((right_dual_g3.yz * this.group1.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g3[2] * other.e3) + (right_dual_g2_w * other.e5 * this.e12345) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), anti_wedge_g3[1] * other.e2) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * float3((float2(other.scalar) * this.group1.xy), other.scalar * this.e125)), (other.scalar * other.scalar * this.e5) + (other.e3215 * other.scalar * this.e12345))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e12345 * (other.e321 * -1.0) * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(AntiFlector other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)) + float4(float3(0.0), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + float4((float3(right_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_dual_g1.zxy * this.group1.yzx), 0.0) - (right_dual_g1.yzxx * float4(this.group1.zxy, this.e415));
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group1.xyz), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g0[3] * other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_dual_g0 * float3(this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e15 * this.e415) * other.group0.xyz), 0.0) + float4((float3(other.e25 * this.e425) * other.group0.xyz), 0.0) + float4((float3(other.e35 * this.e435) * other.group0.xyz), 0.0) - float4((float3(right_dual_g0[0] * this.e235) * other.group0.xyz), 0.0) - float4((float3(right_dual_g0[1] * this.e315) * other.group0.xyz), 0.0) - float4((float3(right_dual_g0[2] * this.e125) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = (float3(other.scalar) * this.group0.xyz) - (float3(this.e12345) * other.group0.xyz);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1) + float4(float3(0.0), (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float3(other.scalar)), (other.scalar * other.scalar * this.e12345) - (anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(AntiPlane other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) + float4((right_dual_g0.zxy * this.group1.yzx), 0.0) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e415));
        float4 anti_wedge_g1 = right_dual_g0 * float4(this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_via_horizon_onto(Circle other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group2.xyz), -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(anti_wedge_g2_w) * other.group2.xy), anti_wedge_g2_w * other.e125, (anti_wedge_g2_w * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_dual_g2 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + float4((float3(right_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_dual_g0.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]));
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group2.zxy * anti_wedge_g3.yzx), 0.0) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0] * other.e15) * float4(other.group2.yzx, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345 * -1.0) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e1234 * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435);
        float4 anti_wedge_g3 = (this.group0 * float4(float3(right_dual_g1[3]), other.e3215)) + float4(float3(0.0), (other.e25 * this.e425) + (other.e35 * this.e435) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + float4((right_dual_g0.zxy * this.group1.yzx), other.e15 * this.e415) - (this.group1.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((float3(this.e12345) * other.group3.xyz), 0.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g2_w * other.e3215) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0]) * other.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * this.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g1 = float4((float3(other.e45) * this.group0.xyz), (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45 * -1.0) * float4(anti_wedge_g1.xyz, other.e45 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g1 = (this.group0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435)) - float4((float3(this.e12345) * other.group1.xyz), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * anti_wedge_g1.xyz), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 anti_wedge_g0_xyz = right_dual_g0_xyz * float3(this.e12345);
        float anti_wedge_g0_w = -(right_dual_g0_xyz[0] * this.e415) - (right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435) - (other.e12345 * this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(anti_wedge_g0_w) * other.group0.xy), anti_wedge_g0_w * other.e435, (anti_wedge_g0_w * other.e12345) - (anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e5)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345) - (right_dual_g5[0] * this.e415) - (right_dual_g5[1] * this.e425) - (right_dual_g5[2] * this.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = (this.group0 * float4(float3(other.e45), other.e1234)) + float4(float3(0.0), (right_dual_g7[2] * this.e435) * -1.0) + float4((right_dual_g7.zxy * this.group1.yzx), 0.0) - (this.group0.wwwx * float4(other.group9.xyz, right_dual_g7[0])) - float4((right_dual_g7.yzx * this.group1.zxy), right_dual_g7[1] * this.e425);
        float anti_wedge_g2 = (right_dual_g0[1] * this.e5) + (this.e12345 * other.e3215) - (right_dual_g6_xyz[0] * this.e235) - (right_dual_g6_xyz[1] * this.e315) - (right_dual_g6_xyz[2] * this.e125) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435) - (this.e12345 * other.e321)) + float4((float3(right_dual_g9[3]) * this.group0.xyz), 0.0) + float4((float3(this.e12345) * other.group8.xyz), 0.0) + float4((right_dual_g9.zxy * this.group1.yzx), 0.0) - (right_dual_g9.yzxx * float4(this.group1.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_dual_g10) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz);
        float3 anti_wedge_g5 = (right_dual_g5 * float3(this.e12345)) + (float3(right_dual_g10) * this.group1.xyz);
        float4 anti_wedge_g6 = float4(((right_dual_g6_xyz * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group0.xyz)), this.e12345 * other.e45);
        float3 anti_wedge_g7 = right_dual_g7 * float3(this.e12345);
        float3 anti_wedge_g8 = (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group1.xyz);
        float4 anti_wedge_g9 = right_dual_g9 * float4(this.e12345);
        float anti_wedge_g10 = right_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g0[1] * this.e12345 * other.scalar) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_via_horizon_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = float4(right_dual_g1) * float4(this.group0.xyz, this.e5);
        float2 anti_wedge_g1_xy = float2(right_dual_g1) * this.group1.xy;
        float anti_wedge_g1_z = right_dual_g1 * this.e125;
        float3 anti_wedge_g2_xyz = (float3(right_dual_g0[3]) * this.group0.xyz) + (right_dual_g0.zxy * this.group1.yzx) + float3(float2(0.0), (right_dual_g0[0] * this.e315) * -1.0) - float3((right_dual_g0.yz * this.group1.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g0 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(anti_wedge_g1_xy, anti_wedge_g1_z, anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group0.zxyx) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (right_dual_g1 * this.e12345 * other.e5)) - float4((anti_wedge_g0.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5) + (right_dual_g0[0] * this.e415 * other.e1) + (right_dual_g0[1] * this.e425 * other.e1) + (right_dual_g0[2] * this.e435 * other.e1), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5) + (right_dual_g0[0] * this.e415 * other.e2) + (right_dual_g0[1] * this.e425 * other.e2) + (right_dual_g0[2] * this.e435 * other.e2), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5) + (right_dual_g0[0] * this.e415 * other.e3) + (right_dual_g0[1] * this.e425 * other.e3) + (right_dual_g0[2] * this.e435 * other.e3), -(anti_wedge_g1_z * other.e3) - (anti_wedge_g1_xy[0] * other.e1) - (anti_wedge_g1_xy[1] * other.e2)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * float3(anti_wedge_g1_xy, anti_wedge_g1_z)) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group0
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
            /* e415, e425, e435, e12345 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar * other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_via_horizon_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (this.group0 * float4(float3(right_dual_g2_w), other.e12345 * -1.0)) + float4(float3(0.0), -(right_dual_g0_xyz[0] * this.e235) - (right_dual_g0_xyz[1] * this.e315) - (right_dual_g0_xyz[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + float4((right_dual_g0_xyz * float3(this.e12345)), right_dual_g2_w * this.e5);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g3[0] * this.e415) - (right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0);
        float3 anti_wedge_g2_xyz = (float3(right_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz) + (right_dual_g3.zxy * this.group1.yzx) + float3(float2(0.0), (right_dual_g3[0] * this.e315) * -1.0) - float3((right_dual_g3.yz * this.group1.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (right_dual_g2_w * this.e12345 * other.e5) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 anti_wedge_g0 = right_dual_g0 * float4(this.e12345);
        float4 anti_wedge_g1 = float4(((float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e12345) * other.group1.xyz)), this.e12345 * other.e45);
        float4 anti_wedge_g2 = (right_dual_g2 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group1) + float4(float3(0.0), (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435));
        float4 anti_wedge_g3 = (this.group0 * float4(float3(other.e45), other.e1234)) + float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(this.e12345 * -1.0) * other.group3.xyz), 0.0) + float4((right_dual_g0.zxy * this.group1.yzx), 0.0) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e415));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g2[3] * other.e1234) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<MultiVector> operator /(MultiVector lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        float4 anti_wedge_g1 = (float4(other.scalar) * this.group1) + float4(float3(0.0), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((float3(right_dual_g1[3]) * this.group6.xyz), 0.0) + float4((float3(this.e321) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group8.yzx), 0.0) + float4((this.group7.yzx * other.group2.zxy), 0.0) - float4((right_dual_g0.yzx * this.group8.zxy), right_dual_g0[0] * this.e415) - float4((this.group7.zxy * other.group2.yzx), right_dual_g0[1] * this.e425);
        float anti_wedge_g2 = (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) + (other.scalar * this.e5) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125);
        float4 anti_wedge_g3 = (float4(other.scalar) * this.group3) + float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((other.group2.zxy * this.group9.yzx), 0.0) - (this.group9.zxyx * float4(other.group2.yzx, right_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(other.scalar) * this.group4.xyz) + (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group9.yzx) - (right_dual_g0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (right_dual_g0 * float3(this.e3215)) + (float3(other.scalar) * this.group5.xyz) - (float3(right_dual_g1[3]) * this.group9.xyz) - (float3(this.e1234) * other.group2.xyz);
        float4 anti_wedge_g6 = (right_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group6);
        float3 anti_wedge_g7 = (right_dual_g0 * float3(this.e12345)) + (float3(other.scalar) * this.group7.xyz);
        float3 anti_wedge_g8 = (float3(other.scalar) * this.group8.xyz) - (float3(this.e12345) * other.group2.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (other.scalar * other.scalar * this.e12345) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e5 */
            anti_wedge_g2 * other.scalar, 
            /* e15, e25, e35, e45 */
            (other.group2 * float4(float3(anti_wedge_g0_x), anti_wedge_g3[3])) + float4((float3(other.scalar) * anti_wedge_g3.xyz), anti_wedge_g0_x * other.e45), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            (other.group2 * float4(float3(anti_wedge_g1[3]), anti_wedge_g6[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group0.xyz), 0.0) + float4((float3(other.scalar) * anti_wedge_g6.xyz), 0.0) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g1[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g2) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group2.zx), 0.0) - float3((anti_wedge_g1.zx * other.group2.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar * other.scalar) * this.group9) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group2.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])), 
            /* e1234 */
            (other.scalar * other.scalar * this.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float anti_wedge_g0_x = (right_dual_g2_w * this.e5) + (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g1_w * this.e321) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(right_dual_g2_xyz, right_dual_g1_w)) + (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], other.e431 * this.e4315) * float4(this.group5.yzx, 1.0)) + (this.group9.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4((float3(right_dual_g3[3]) * this.group4.xyz), other.e412 * this.e4125) - (float4(right_dual_g2_w) * this.group3) - (right_dual_g3.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(this.e3215) * other.group0.xyz), right_dual_g3[2] * this.e43) - float4((right_dual_g1_xyz.yzx * this.group9.zxy), right_dual_g3[1] * this.e42);
        float anti_wedge_g2 = (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45) - (right_dual_g1_w * this.e3215) - (right_dual_g2_xyz[0] * this.e4235) - (right_dual_g2_xyz[1] * this.e4315) - (right_dual_g2_xyz[2] * this.e4125);
        float4 anti_wedge_g3 = (float4(this.e12345) * float4(right_dual_g2_xyz, right_dual_g1_w)) + float4(float3(0.0), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((float3(right_dual_g3[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_dual_g3.zxy), 0.0) - (right_dual_g3.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_dual_g2_w) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.zxy * right_dual_g3.yzx) - (this.group7.yzx * right_dual_g3.zxy);
        float3 anti_wedge_g5 = (right_dual_g1_xyz * float3(this.e12345)) + (float3(right_dual_g2_w) * this.group8.xyz) + (float3(right_dual_g3[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g3.xyz);
        float4 anti_wedge_g6 = (right_dual_g3.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], right_dual_g2_w) * this.group9.yzxw);
        float3 anti_wedge_g7 = (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2_w) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(right_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g3.xyz);
        float4 anti_wedge_g9 = right_dual_g3 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g2_w * other.e5 * this.e12345) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * float4(other.group3.xyz, other.e4), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(anti_wedge_g2) * float4(other.e1, other.e2, other.e3, other.e4)) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group3.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group3.yzx) + float3(float2(0.0), (anti_wedge_g1[0] * other.e2) * -1.0) - float3((anti_wedge_g1.yz * other.group3.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group1) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (other.group3.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group0.xyz) + (anti_wedge_g4.yzx * other.group3.zxy) - (anti_wedge_g4.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + float3(float2(0.0), (anti_wedge_g3[1] * other.e1) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g3.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group3.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + (other.group2.wwwx * float4(anti_wedge_g8, anti_wedge_g1[0])) + float4(float3(0.0), (anti_wedge_g1[2] * other.e125) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3)) + float4((anti_wedge_g6.yzx * other.group3.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group3.yzxx) - float4((float3(anti_wedge_g1[3]) * other.group2.xyz), 0.0) - float4((anti_wedge_g1.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiDualNum other) {
        float anti_wedge_g0_x = (other.e3215 * this.e1234) + (other.scalar * this.scalar);
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        float4 anti_wedge_g9 = float4(other.scalar) * this.group9;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g1[3] * other.e3215) + (other.scalar * other.scalar * this.e12345)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e5 */
            (other.scalar * other.scalar * this.e5) + (other.e3215 * other.scalar * this.e12345), 
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(other.scalar * other.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float2(other.scalar) * anti_wedge_g9.xy), anti_wedge_g9[2] * other.scalar, (anti_wedge_g0_x * other.e3215) + (anti_wedge_g9[3] * other.scalar)), 
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float anti_wedge_g0_x = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        float4 anti_wedge_g1 = right_dual_g0 * float4(this.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, right_dual_g0[3] * other.e321 * this.e12345 * -1.0), 
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
            float4(float3(0.0), anti_wedge_g0_x * other.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g1[3] * -1.0) * other.group0.xyz), (anti_wedge_g1[0] * other.e235) + (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (right_dual_g0[0] * other.e321 * this.e4235) - (right_dual_g0[1] * other.e321 * this.e4315) - (right_dual_g0[2] * other.e321 * this.e4125) - (right_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g1[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiFlector other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float anti_wedge_g0_x = (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        float4 anti_wedge_g1 = (right_dual_g0 * float4(this.e1234)) + float4(float3(0.0), -(right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) + float4((float3(right_dual_g1[3]) * this.group4.xyz), 0.0) + float4((this.group5.yzx * right_dual_g1.zxy), 0.0) - (right_dual_g1.yzxx * float4(this.group5.zxy, this.e41));
        float anti_wedge_g2 = (right_dual_g1[0] * this.e15) + (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35) + (right_dual_g1[3] * this.e45) - (right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215);
        float4 anti_wedge_g3 = (right_dual_g0 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + float4((float3(right_dual_g1[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_dual_g1.zxy), 0.0) - (right_dual_g1.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (this.group7.zxy * right_dual_g1.yzx) - (this.group7.yzx * right_dual_g1.zxy);
        float3 anti_wedge_g5 = (float3(right_dual_g1[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g1.xyz);
        float3 anti_wedge_g6_xyz = (right_dual_g1.yzx * this.group9.zxy) + float3(float2(0.0), (right_dual_g1[1] * this.e4235) * -1.0) - float3((right_dual_g1.zx * this.group9.yz), 0.0);
        float3 anti_wedge_g7 = float3(this.e1234) * right_dual_g1.xyz;
        float3 anti_wedge_g8 = (float3(right_dual_g1[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g1.xyz);
        float4 anti_wedge_g9 = right_dual_g1 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(anti_wedge_g2) * other.group1.xyz) - (float3(other.e5) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.e5 * -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g1[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group1.yzx) + float3(float2(0.0), (anti_wedge_g1[0] * other.e2) * -1.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + float4((anti_wedge_g4 * float3(other.e5)), anti_wedge_g0_x * other.e321) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g4.yzx * other.group1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group0.xyz) + float3(float2(0.0), (anti_wedge_g3[1] * other.e1) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g1[0] * other.e235) + (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3)) + float4((anti_wedge_g6_xyz.yzx * other.group1.zxy), anti_wedge_g2 * other.e321) - (float4(other.e5) * float4(anti_wedge_g7, right_dual_g1[3] * this.e1234)) - (other.group1.yzxx * float4(anti_wedge_g6_xyz.zxy, anti_wedge_g8[0])) - float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float anti_wedge_g0_x = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) + float4((right_dual_g0 * float3(this.e321)), 0.0) + float4((right_dual_g1.yzx * this.group7.zxy), 0.0) - float4((right_dual_g1.zxy * this.group7.yzx), right_dual_g0[0] * this.e423);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) + float4((right_dual_g0 * float3(this.e3215)), 0.0) + float4((right_dual_g1.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]));
        float3 anti_wedge_g4 = right_dual_g0 * float3(this.e1234);
        float3 anti_wedge_g5 = right_dual_g1 * float3(this.e1234);
        float3 anti_wedge_g6_xyz = right_dual_g0 * float3(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g6_xyz[0] * other.e23) - (anti_wedge_g6_xyz[1] * other.e31) - (anti_wedge_g6_xyz[2] * other.e12)), 
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
            float4((float3(anti_wedge_g1[3]) * other.group1.xyz), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e423, e431, e412 */
            float3(anti_wedge_g1[3]) * other.group0.xyz, 
            /* e235, e315, e125 */
            (other.group1.zxy * anti_wedge_g1.yzx) - (float3(right_dual_g0[0] * this.e235) * other.group0.xyz) - (float3(right_dual_g0[1] * this.e315) * other.group0.xyz) - (float3(right_dual_g0[2] * this.e125) * other.group0.xyz) - (float3(right_dual_g1[0] * this.e415) * other.group0.xyz) - (float3(right_dual_g1[1] * this.e425) * other.group0.xyz) - (float3(right_dual_g1[2] * this.e435) * other.group0.xyz) - (other.group1.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g4.zxy * other.group1.yzx), anti_wedge_g5[0] * other.e15), 
            /* e1234 */
            -(anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_dual_g0[3] * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        float4 anti_wedge_g1 = (right_dual_g0 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) + float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((this.group7.yzx * other.group1.zxy), 0.0) - float4((this.group7.zxy * other.group1.yzx), right_dual_g0[0] * this.e423);
        float anti_wedge_g2 = (right_dual_g0[3] * this.e5) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) + (other.e3215 * this.e12345) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125);
        float4 anti_wedge_g3 = (right_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) + float4((float3(right_dual_g0[3]) * this.group3.xyz), 0.0) + float4((other.group1.zxy * this.group9.yzx), 0.0) - (this.group9.zxyx * float4(other.group1.yzx, right_dual_g0[0]));
        float3 anti_wedge_g4 = (float3(right_dual_g0[3]) * this.group4.xyz) + (float3(this.e1234) * right_dual_g0.xyz);
        float3 anti_wedge_g5 = (float3(right_dual_g0[3]) * this.group5.xyz) - (float3(this.e1234) * other.group1.xyz);
        float4 anti_wedge_g6 = float4(((float3(right_dual_g0[3]) * this.group6.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e321);
        float3 anti_wedge_g7 = float3(right_dual_g0[3]) * this.group7.xyz;
        float4 anti_wedge_g9 = float4(right_dual_g0[3]) * this.group9;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g1[3] * other.e3215) + (right_dual_g0[3] * other.scalar * this.e12345) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e5 */
            anti_wedge_g2 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(anti_wedge_g0_x) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g3.xyz)), anti_wedge_g3[3] * other.scalar), 
            /* e41, e42, e43 */
            anti_wedge_g4 * float3(other.scalar), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g1[3]) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * other.group0.xyz) + (float3(right_dual_g0[3] * other.scalar) * this.group8.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group1.zx), 0.0) - (float3(other.scalar * this.e12345) * other.group1.xyz) - float3((anti_wedge_g1.zx * other.group1.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(anti_wedge_g3[3]), anti_wedge_g9[3])) + (other.group1.zxyw * float4(anti_wedge_g4.yzx, anti_wedge_g0_x)) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(other.scalar) * anti_wedge_g9.xyz), 0.0) - (other.group1.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])), 
            /* e1234 */
            (right_dual_g0[3] * other.scalar * this.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(AntiPlane other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float anti_wedge_g0_x = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) + float4((float3(right_dual_g0[3]) * this.group4.xyz), 0.0) + float4((this.group5.yzx * right_dual_g0.zxy), 0.0) - (right_dual_g0.yzxx * float4(this.group5.zxy, this.e41));
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(right_dual_g0[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_dual_g0.zxy), 0.0) - (right_dual_g0.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (this.group7.zxy * right_dual_g0.yzx) - (this.group7.yzx * right_dual_g0.zxy);
        float3 anti_wedge_g5 = (float3(right_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g0.xyz);
        float3 anti_wedge_g6_xyz = (right_dual_g0.yzx * this.group9.zxy) + float3(float2(0.0), (right_dual_g0[1] * this.e4235) * -1.0) - float3((right_dual_g0.zx * this.group9.yz), 0.0);
        float3 anti_wedge_g7 = float3(this.e1234) * right_dual_g0.xyz;
        float3 anti_wedge_g8 = (float3(right_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g0.xyz);
        float4 anti_wedge_g9 = right_dual_g0 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3)), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_x) * other.group0.xyz), 0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual_g0[0] * this.e15) * other.group0.xyz) + (float3(right_dual_g0[1] * this.e25) * other.group0.xyz) + (float3(right_dual_g0[2] * this.e35) * other.group0.xyz) + (float3(right_dual_g0[3] * this.e45) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.e5 * -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g1[3] * -1.0) * other.group0.xyz, 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group0.yzx) + float3(float2(0.0), (anti_wedge_g1[0] * other.e2) * -1.0) - float3((anti_wedge_g1.yz * other.group0.zx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) - (other.group0.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g4.yzx * other.group0.zxy) - (anti_wedge_g4.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + float3(float2(0.0), (anti_wedge_g3[1] * other.e1) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g3.zx * other.group0.yz), 0.0) - float3((anti_wedge_g3.yz * other.group0.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3)) + float4((anti_wedge_g6_xyz.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, right_dual_g0[3] * this.e1234)) - (other.group0.yzxx * float4(anti_wedge_g6_xyz.zxy, anti_wedge_g8[0])), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public AntiScalar anti_project_via_horizon_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float anti_wedge_g0_x = -(right_dual_g1_w * this.e321) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(other.group2.xyz, right_dual_g1_w)) + (this.group9.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group9.zxy), 0.0);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group2.xyz, right_dual_g1_w);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = right_dual_g1_xyz * float3(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
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
            float4(anti_wedge_g0_x) * other.group1, 
            /* e423, e431, e412 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - (float4(other.e235) * float4(anti_wedge_g1[3], other.e431 * this.e4235, other.e412 * this.e4235, other.e321 * this.e4235)) - (float4(other.e315) * float4(other.e423 * this.e4315, anti_wedge_g1[3], other.e412 * this.e4315, other.e321 * this.e4315)) - (float4(other.e125) * float4(other.e423 * this.e4125, other.e431 * this.e4125, anti_wedge_g1[3], other.e321 * this.e4125)) - (float4(other.e423 * other.e235, other.e431 * other.e315, other.e412 * other.e125, right_dual_g1_w * other.e321) * this.group9) - float4((float3(right_dual_g1_w * this.e3215) * other.group0.xyz), 0.0) - float4((anti_wedge_g1.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float anti_wedge_g0_x = -(right_dual_g1_w * this.e321) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(right_dual_g2_xyz, right_dual_g1_w)) + (this.group9.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group9.zxy), 0.0);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(right_dual_g2_xyz, right_dual_g1_w);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = right_dual_g1_xyz * float3(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.e12345) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
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
            float4(anti_wedge_g0_x) * other.group1, 
            /* e423, e431, e412 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], right_dual_g2_xyz[0] * this.e4235) * other.group1.yzxw) - (float4(right_dual_g1_w * other.e423, right_dual_g1_w * other.e431, right_dual_g1_w * other.e412, right_dual_g2_xyz[1] * other.e321) * this.group9.wwwy) - (float4(right_dual_g2_xyz[0] * other.e423, right_dual_g2_xyz[1] * other.e431, right_dual_g2_xyz[2] * other.e412, right_dual_g1_w * other.e321) * this.group9) - (float4(right_dual_g2_xyz[1] * other.e423, right_dual_g2_xyz[0] * other.e431, right_dual_g2_xyz[0] * other.e412, right_dual_g2_xyz[2] * other.e321) * this.group9.yxxz) - float4((float3(anti_wedge_g1[3]) * other.group2.xyz), 0.0) - float4((other.group0.xyz * right_dual_g2_xyz.zzy * this.group9.zzy), 0.0), 
            /* e1234 */
            -(anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float anti_wedge_g0_x = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((float3(right_dual_g1[3]) * this.group6.xyz), 0.0) + float4((float3(this.e321) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group8.yzx), 0.0) + float4((right_dual_g2.yzx * this.group7.zxy), 0.0) - float4((right_dual_g0.yzx * this.group8.zxy), right_dual_g0[0] * this.e415) - float4((right_dual_g2.zxy * this.group7.yzx), right_dual_g0[1] * this.e425);
        float anti_wedge_g2 = -(right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((right_dual_g2.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group9.yzx) - (right_dual_g0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (right_dual_g0 * float3(this.e3215)) + (right_dual_g2 * float3(this.e1234)) - (float3(right_dual_g1[3]) * this.group9.xyz);
        float4 anti_wedge_g6 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g7 = right_dual_g0 * float3(this.e12345);
        float3 anti_wedge_g8 = right_dual_g2 * float3(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0_x) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0_x) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group2.xyz), 0.0) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g1[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g1.yzx) - (other.group2.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - float4((anti_wedge_g4.zxy * other.group2.yzx), anti_wedge_g5[0] * other.e15), 
            /* e1234 */
            -(anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
        float4 anti_wedge_g1 = (other.group2.zxyw * float4(this.group7.yzx, this.e12345)) + float4(float3(0.0), -(right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((float3(right_dual_g1[3]) * this.group6.xyz), 0.0) + float4((float3(this.e321) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group8.yzx), 0.0) - float4((float2(this.e12345) * other.group3.xy), other.e4125 * this.e12345, right_dual_g0[2] * this.e435) - float4((right_dual_g0.yzx * this.group8.zxy), right_dual_g0[0] * this.e415) - float4((this.group7.zxy * other.group2.yzx), right_dual_g0[1] * this.e425);
        float anti_wedge_g2 = (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) + (other.e3215 * this.e12345) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((other.group2.zxy * this.group9.yzx), 0.0) - (this.group9.zxyx * float4(other.group2.yzx, right_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group9.yzx) - (right_dual_g0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (right_dual_g0 * float3(this.e3215)) - (float3(right_dual_g1[3]) * this.group9.xyz) - (float3(this.e1234) * other.group2.xyz);
        float4 anti_wedge_g6 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g7 = right_dual_g0 * float3(this.e12345);
        float3 anti_wedge_g8 = float3(this.e12345 * -1.0) * other.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0_x) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(anti_wedge_g0_x) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0_x) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group2.xyz), 0.0) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g1[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group2.zx), 0.0) - float3((anti_wedge_g1.zx * other.group2.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group3) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group2.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(DualNum other) {
        float anti_wedge_g0_x = -(other.e5 * this.e4) - (other.e12345 * this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, anti_wedge_g0_x * other.e12345), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * other.e5) * this.group4.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e5 * other.e5 * -1.0) * this.group7.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e5 * other.e5 * this.e1234), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g1 = float4((float3(other.e45) * this.group6.xyz), 0.0) + float4((this.group7.yzx * other.group0.zxy), 0.0) - float4((this.group7.zxy * other.group0.yzx), 0.0);
        float3 anti_wedge_g5 = -(float3(other.e45) * this.group9.xyz) - (float3(this.e1234) * other.group0.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, other.e45 * other.e45 * this.e12345 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            (float4(other.e15 * this.e41, other.e25 * this.e42, other.e35 * this.e43, other.e15 * this.e41) * other.group0) + (float4(other.e25 * this.e42, other.e15 * this.e41, other.e15 * this.e41, other.e25 * this.e42) * other.group0) + (other.group0 * float4(float2(other.e35 * this.e43), other.e25 * this.e42, other.e35 * this.e43)) - (float4(other.e45 * this.e45) * other.group0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) - (float3(other.e45) * anti_wedge_g1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1.yzx * other.group0.zxy) + float3(float2(0.0), (anti_wedge_g1[1] * other.e15) * -1.0) - float3((anti_wedge_g1.zx * other.group0.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g5 * float3(other.e45)), -(anti_wedge_g5[0] * other.e15) - (anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Flector other) {
        float anti_wedge_g0_x = (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125);
        float4 anti_wedge_g1 = float4((float3(other.e45) * this.group6.xyz), 0.0) + float4((this.group7.yzx * other.group0.zxy), 0.0) - float4((float3(this.e12345) * other.group1.xyz), 0.0) - float4((this.group7.zxy * other.group0.yzx), 0.0);
        float3 anti_wedge_g5 = -(float3(other.e45) * this.group9.xyz) - (float3(this.e1234) * other.group0.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (other.e45 * other.e45 * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0_x) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) - (float3(other.e45) * anti_wedge_g1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1.yzx * other.group0.zxy) + float3(float2(0.0), (anti_wedge_g1[1] * other.e15) * -1.0) - float3((anti_wedge_g1.zx * other.group0.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group1) + float4(float3(0.0), -(anti_wedge_g5[0] * other.e15) - (anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35)) + float4((anti_wedge_g5 * float3(other.e45)), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Line other) {
        float anti_wedge_g0_x = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float4 anti_wedge_g1 = float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0);
        float3 anti_wedge_g5 = float3(this.e12345) * other.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435)), 
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
            float4((float3(anti_wedge_g0_x) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(anti_wedge_g0_x) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e435, other.e415, other.e425, other.e235) * anti_wedge_g1.yzxx) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) - float4((other.group0.yzx * anti_wedge_g1.zxy), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float anti_wedge_g0_x = (right_dual_g1_w * this.e4) - (right_dual_g0_xyz[0] * this.e415) - (right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435) - (right_dual_g1_xyz[0] * this.e423) - (right_dual_g1_xyz[1] * this.e431) - (right_dual_g1_xyz[2] * this.e412) - (other.e12345 * this.e12345);
        float4 anti_wedge_g1 = float4((right_dual_g1_xyz * float3(this.e1234)), 0.0) + float4((float3(right_dual_g1_w) * this.group4.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group9.yzx), 0.0) - float4((right_dual_g0_xyz.yzx * this.group9.zxy), 0.0);
        float3 anti_wedge_g5 = (right_dual_g0_xyz * float3(this.e12345)) + (float3(right_dual_g1_w) * this.group7.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.e12345) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0_x) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - (other.group1 * float4(float3(anti_wedge_g1[3]), right_dual_g1_w * this.e1234)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345) + (right_dual_g0[1] * this.scalar) + (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) + (other.e3215 * this.e1234) - (right_dual_g5[0] * this.e415) - (right_dual_g5[1] * this.e425) - (right_dual_g5[2] * this.e435) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + (right_dual_g3 * float4(this.e1234)) + (float4(right_dual_g0[1]) * this.group1) + (this.group9.yzxx * float4(right_dual_g5.zxy, other.e423)) + float4(float3(0.0), -(right_dual_g6[1] * this.e431) - (right_dual_g6[2] * this.e412) - (right_dual_g9[1] * this.e42) - (right_dual_g9[2] * this.e43)) + float4((float3(right_dual_g6[3]) * this.group6.xyz), 0.0) + float4((float3(right_dual_g9[3]) * this.group4.xyz), 0.0) + float4((float3(this.e321) * right_dual_g6.xyz), 0.0) + float4((right_dual_g7.zxy * this.group8.yzx), other.e431 * this.e4315) + float4((right_dual_g8.yzx * this.group7.zxy), other.e412 * this.e4125) + float4((this.group5.yzx * right_dual_g9.zxy), 0.0) - (float4(right_dual_g10) * this.group3) - (right_dual_g9.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(this.e3215) * other.group7.xyz), right_dual_g6[0] * this.e423) - float4((right_dual_g5.yzx * this.group9.zxy), right_dual_g7[0] * this.e415) - float4((right_dual_g7.yzx * this.group8.zxy), right_dual_g7[1] * this.e425) - float4((right_dual_g8.zxy * this.group7.yzx), right_dual_g7[2] * this.e435);
        float anti_wedge_g2 = (right_dual_g0[1] * this.e5) + (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45) + (other.e3215 * this.e12345) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g3[0] * this.e4235) - (right_dual_g3[1] * this.e4315) - (right_dual_g3[2] * this.e4125) - (right_dual_g3[3] * this.e3215) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = (right_dual_g3 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group3) + float4(float3(0.0), -(right_dual_g6[1] * this.e4315) - (right_dual_g6[2] * this.e4125) - (right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) + float4((float3(right_dual_g9[3]) * this.group6.xyz), 0.0) + float4((float3(this.e3215) * right_dual_g6.xyz), 0.0) + float4((right_dual_g8.yzx * this.group9.zxy), 0.0) + float4((this.group8.yzx * right_dual_g9.zxy), 0.0) - (right_dual_g9.yzxx * float4(this.group8.zxy, this.e415)) - (this.group9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0]));
        float3 anti_wedge_g4 = (float3(right_dual_g10) * this.group6.xyz) + (float3(right_dual_g0[1]) * this.group4.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * right_dual_g6.xyz) + (right_dual_g7.zxy * this.group9.yzx) + (this.group7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * this.group9.zxy) - (this.group7.yzx * right_dual_g9.zxy);
        float3 anti_wedge_g5 = (right_dual_g5 * float3(this.e12345)) + (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) + (float3(right_dual_g10) * this.group8.xyz) + (float3(right_dual_g0[1]) * this.group5.xyz) + (float3(right_dual_g9[3]) * this.group7.xyz) - (float3(right_dual_g6[3]) * this.group9.xyz) - (float3(this.e321) * right_dual_g9.xyz);
        float4 anti_wedge_g6 = (right_dual_g6 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group6) + (right_dual_g9.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_dual_g9[2], right_dual_g9[0], right_dual_g9[1], right_dual_g10) * this.group9.yzxw);
        float3 anti_wedge_g7 = (right_dual_g7 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group7.xyz) + (float3(this.e1234) * right_dual_g9.xyz) - (float3(right_dual_g10) * this.group9.xyz);
        float3 anti_wedge_g8 = (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group8.xyz) + (float3(right_dual_g9[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g9.xyz);
        float4 anti_wedge_g9 = (right_dual_g9 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group9);
        float anti_wedge_g10 = (right_dual_g10 * this.e12345) + (right_dual_g0[1] * this.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g0[1] * other.scalar * this.e12345) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Plane other) {
        float3 anti_wedge_g1_xyz = float3(this.e12345 * -1.0) * other.group0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1_xyz[0] * other.e4235) + (anti_wedge_g1_xyz[1] * other.e4315) + (anti_wedge_g1_xyz[2] * other.e4125)), 
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
            (float4(this.e1234 * other.e3215) * other.group0) - (float4(this.e4235 * other.e4235, this.e4315 * other.e4315, this.e4125 * other.e4125, this.e4235 * other.e4235) * other.group0) - (float4(this.e4315 * other.e4315, this.e4235 * other.e4235, this.e4235 * other.e4235, this.e4315 * other.e4315) * other.group0) - (other.group0 * float4(float2(this.e4125 * other.e4125), this.e4315 * other.e4315, this.e4125 * other.e4125)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g1 * this.e5) + (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) + float4((float3(right_dual_g0[3]) * this.group4.xyz), 0.0) + float4((this.group5.yzx * right_dual_g0.zxy), 0.0) - (float4(right_dual_g1) * this.group3) - (right_dual_g0.yzxx * float4(this.group5.zxy, this.e41));
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(right_dual_g0[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_dual_g0.zxy), 0.0) - (right_dual_g0.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_dual_g1) * this.group6.xyz) + (this.group7.zxy * right_dual_g0.yzx) - (this.group7.yzx * right_dual_g0.zxy);
        float3 anti_wedge_g5 = (float3(right_dual_g1) * this.group8.xyz) + (float3(right_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g0.xyz);
        float4 anti_wedge_g6 = (right_dual_g0.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_dual_g0[2], right_dual_g0[0], right_dual_g0[1], right_dual_g1) * this.group9.yzxw);
        float3 anti_wedge_g7 = (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g1) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(right_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g0.xyz);
        float4 anti_wedge_g9 = right_dual_g0 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g1 * this.e12345 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * other.group0, 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_dual_g0[0] * this.e15) * other.group0) + (float4(right_dual_g0[1] * this.e25) * other.group0) + (float4(right_dual_g0[2] * this.e35) * other.group0) + (float4(right_dual_g0[3] * this.e45) * other.group0) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group0.yzx) + float3(float2(0.0), (anti_wedge_g1[0] * other.e2) * -1.0) - float3((anti_wedge_g1.yz * other.group0.zx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (other.group0.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g4.yzx * other.group0.zxy) - (anti_wedge_g4.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + float3(float2(0.0), (anti_wedge_g3[1] * other.e1) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g3.zx * other.group0.yz), 0.0) - float3((anti_wedge_g3.yz * other.group0.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3)) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((anti_wedge_g6.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group0.yzxx), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4)
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
            /* scalar, e12345 */
            float2(other.scalar * other.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e5 */
            this.e5 * other.scalar * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.scalar * other.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(other.scalar * other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar * other.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(other.scalar * other.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar * other.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group9, 
            /* e1234 */
            this.e1234 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(Sphere other) {
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float anti_wedge_g0_x = (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215);
        float4 anti_wedge_g1 = right_dual_g0 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)), 
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
            float4(anti_wedge_g0_x) * other.group0, 
            /* e1234 */
            anti_wedge_g0_x * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float anti_wedge_g0_x = (right_dual_g2_w * this.e5) + (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g1_w * this.e321) - (right_dual_g0_xyz[0] * this.e235) - (right_dual_g0_xyz[1] * this.e315) - (right_dual_g0_xyz[2] * this.e125) - (right_dual_g1_xyz[0] * this.e415) - (right_dual_g1_xyz[1] * this.e425) - (right_dual_g1_xyz[2] * this.e435) - (right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (this.e12345 * other.e12345);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(right_dual_g2_xyz, right_dual_g1_w)) + (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], right_dual_g0_xyz[1] * this.e4315) * float4(this.group5.yzx, 1.0)) + (this.group9.yzxx * float4(right_dual_g1_xyz.zxy, right_dual_g0_xyz[0])) + float4((float3(right_dual_g3[3]) * this.group4.xyz), right_dual_g0_xyz[2] * this.e4125) - (float4(right_dual_g2_w) * this.group3) - (right_dual_g3.yzxx * float4(this.group5.zxy, this.e41)) - float4((right_dual_g0_xyz * float3(this.e3215)), right_dual_g3[1] * this.e42) - float4((right_dual_g1_xyz.yzx * this.group9.zxy), right_dual_g3[2] * this.e43);
        float anti_wedge_g2 = (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45) - (right_dual_g1_w * this.e3215) - (right_dual_g2_xyz[0] * this.e4235) - (right_dual_g2_xyz[1] * this.e4315) - (right_dual_g2_xyz[2] * this.e4125);
        float4 anti_wedge_g3 = (float4(this.e12345) * float4(right_dual_g2_xyz, right_dual_g1_w)) + float4(float3(0.0), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((float3(right_dual_g3[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_dual_g3.zxy), 0.0) - (right_dual_g3.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (right_dual_g0_xyz * float3(this.e12345)) + (float3(right_dual_g2_w) * this.group6.xyz) + (this.group7.zxy * right_dual_g3.yzx) - (this.group7.yzx * right_dual_g3.zxy);
        float3 anti_wedge_g5 = (right_dual_g1_xyz * float3(this.e12345)) + (float3(right_dual_g2_w) * this.group8.xyz) + (float3(right_dual_g3[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g3.xyz);
        float4 anti_wedge_g6 = (right_dual_g3.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], right_dual_g2_w) * this.group9.yzxw);
        float3 anti_wedge_g7 = (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2_w) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(right_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g3.xyz);
        float4 anti_wedge_g9 = right_dual_g3 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g2_w * this.e12345 * other.e5) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * other.group3, 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(anti_wedge_g2) * other.group3) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group3.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group3.yzx) + float3(float2(0.0), (anti_wedge_g1[0] * other.e2) * -1.0) - float3((anti_wedge_g1.yz * other.group3.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group1) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (other.group3.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group0.xyz) + (anti_wedge_g4.yzx * other.group3.zxy) - (anti_wedge_g4.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + float3(float2(0.0), (anti_wedge_g3[1] * other.e1) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g3.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group3.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[2] * other.e125) - (anti_wedge_g8[2] * other.e3)) + float4((anti_wedge_g8 * float3(other.e4)), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g6.yzx * other.group3.zxy), 0.0) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group3.yzxx) - (other.group2 * float4(float3(anti_wedge_g1[3]), anti_wedge_g6[3])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g8[1] * other.e2) - float4((anti_wedge_g1.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        float anti_wedge_g0_x = (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) - (right_dual_g2_xyz[0] * this.e41) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        float4 anti_wedge_g1 = (right_dual_g3 * float4(this.e12345)) + (float4(other.scalar) * this.group1) + (float4(right_dual_g1[0], right_dual_g1[1], right_dual_g1[2], other.e41) * this.group6.wwwx) + (this.group6.xyzy * float4(float3(right_dual_g1[3]), other.e42)) + (other.group0.yzxz * float4(this.group8.zxy, this.e435)) + float4(float3(0.0), (right_dual_g1[2] * this.e412) * -1.0) + float4((right_dual_g2_xyz.yzx * this.group7.zxy), 0.0) - float4((right_dual_g2_xyz.zxy * this.group7.yzx), right_dual_g1[0] * this.e423) - float4((this.group8.yzx * other.group0.zxy), right_dual_g1[1] * this.e431);
        float anti_wedge_g2 = (this.e12345 * other.e3215) + (this.e5 * other.scalar) - (right_dual_g2_xyz[0] * this.e415) - (right_dual_g2_xyz[1] * this.e425) - (right_dual_g2_xyz[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125);
        float4 anti_wedge_g3 = (float4(other.scalar) * this.group3) + float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((right_dual_g2_xyz.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(right_dual_g2_xyz.zxy, right_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(this.e1234) * right_dual_g1.xyz) + (float3(other.scalar) * this.group4.xyz) + float3(float2(0.0), (this.e4315 * other.e41) - (this.e4235 * other.e42)) + float3((this.group9.zx * other.group0.yz), 0.0) - float3((this.group9.yz * other.group0.zx), 0.0);
        float3 anti_wedge_g5 = (right_dual_g2_xyz * float3(this.e1234)) + (float3(other.scalar) * this.group5.xyz) - (float3(right_dual_g1[3]) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz);
        float4 anti_wedge_g6 = (right_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group6);
        float3 anti_wedge_g7 = (float3(other.scalar) * this.group7.xyz) - (float3(this.e12345) * other.group0.xyz);
        float3 anti_wedge_g8 = (right_dual_g2_xyz * float3(this.e12345)) + (float3(other.scalar) * this.group8.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (this.e12345 * other.scalar * other.scalar) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e5 */
            anti_wedge_g2 * other.scalar, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * float4(other.e15, other.e25, other.e35, other.e45)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(float3(anti_wedge_g2), anti_wedge_g6[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group2.xyz), 0.0) + float4((float3(other.scalar) * anti_wedge_g6.xyz), 0.0) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g1[3]) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e41) - (anti_wedge_g1[0] * other.e42)) + float3((anti_wedge_g1.zx * other.group0.yz), 0.0) - float3((anti_wedge_g1.yz * other.group0.zx), 0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g2) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group2.zx), 0.0) - float3((anti_wedge_g1.zx * other.group2.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group3) + (float4(other.scalar * other.scalar) * this.group9) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g3.zxy * other.group0.yzx), 0.0) - (anti_wedge_g3.yzxx * float4(other.group0.zxy, other.e23)) - (other.group2.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (other.scalar * other.scalar * this.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Plane> operator /(Plane lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (this.group0.wwwx * float4(right_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0);
        float4 anti_wedge_g2 = float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(other.scalar), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * anti_wedge_g2.xyz), (anti_wedge_g2[3] * other.scalar) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar * other.scalar) * this.group0) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float3 anti_wedge_g0 = float3(right_dual_g2_w * -1.0) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx);
        float anti_wedge_g1_w = right_dual_g2_w * this.e3215 * -1.0;
        float anti_wedge_g2_x = -(right_dual_g3_xyz[0] * this.e3215) - (other.e5 * this.e4235);
        float anti_wedge_g2_y = -(right_dual_g3_xyz[1] * this.e3215) - (other.e5 * this.e4315);
        float anti_wedge_g2_z = -(right_dual_g3_xyz[2] * this.e3215) - (other.e5 * this.e4125);
        float anti_wedge_g2_w = (other.e423 * this.e4235) + (other.e431 * this.e4315) + (other.e412 * this.e4125);
        float4 anti_wedge_g3 = (this.group0.yzxw * float4(right_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_dual_g2_xyz[2] * this.e4125) * -1.0) - (this.group0.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0])) - (this.group0.wwwy * float4(other.group0.xyz, right_dual_g2_xyz[1]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2_w) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(other.e2, other.e3, other.e1, other.e4) * anti_wedge_g3.zxyw) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2_w) * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2_w * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (float4(anti_wedge_g2_x, anti_wedge_g2_y, anti_wedge_g2_z, anti_wedge_g3[0]) * other.group2.wwwx) + (anti_wedge_g3.yzxy * float4(other.group1.zxy, other.e315)) + float4(float3(0.0), -(anti_wedge_g2_y * other.e2) - (anti_wedge_g2_z * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[2] * other.e125) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_x)) - float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public AntiDualNum anti_project_via_horizon_onto(AntiFlatPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiDualNum(
            /* e3215, scalar */
            float2((other.e321 * other.e321 * this.e3215) - (right_dual_g0_xyz[0] * other.e321 * this.e4235) - (right_dual_g0_xyz[1] * other.e321 * this.e4315) - (right_dual_g0_xyz[2] * other.e321 * this.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 anti_wedge_g0_xyz = (right_dual_g1_xyz.yzx * this.group0.zxy) - (right_dual_g1_xyz.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), (other.e321 * this.e3215) - (right_dual_g0_xyz[2] * this.e4125)) - (this.group0.xyzy * float4(float3(other.e5), right_dual_g0_xyz[1])) - (this.group0.wwwx * float4(right_dual_g1_xyz, right_dual_g0_xyz[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0_xyz.yzx * other.group1.zxy), anti_wedge_g1[3] * other.e321) - (other.group1.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) + float4((right_dual_g0 * float3(this.e3215)), 0.0) + float4((right_dual_g1.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(other.group1.zxy, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group0.zxy), 0.0);
        float4 anti_wedge_g1 = float4(other.scalar) * this.group0;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(anti_wedge_g0[3]), anti_wedge_g1[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(other.scalar) * anti_wedge_g1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 anti_wedge_g0 = (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx);
        float3 anti_wedge_g1 = -(right_dual_g0_xyz * float3(this.e3215)) - (float3(other.e5) * this.group0.xyz);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group0.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(other.e235) * float4(anti_wedge_g0[3], other.e431 * this.e4235, other.e412 * this.e4235, other.e321 * this.e4235)) - (float4(other.e315) * float4(other.e423 * this.e4315, anti_wedge_g0[3], other.e412 * this.e4315, other.e321 * this.e4315)) - (float4(other.e125) * float4(other.e423 * this.e4125, other.e431 * this.e4125, anti_wedge_g0[3], other.e321 * this.e4125)) - (float4(other.e423 * other.e235, other.e431 * other.e315, other.e412 * other.e125, right_dual_g1_w * other.e321) * this.group0) - float4((float3(right_dual_g1_w * this.e3215) * other.group0.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group0.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], right_dual_g2_xyz[0] * this.e4235) * other.group1.yzxw) - (float4(right_dual_g1_w * other.e423, right_dual_g1_w * other.e431, right_dual_g1_w * other.e412, right_dual_g2_xyz[1] * other.e321) * this.group0.wwwy) - (float4(right_dual_g2_xyz[0] * other.e423, right_dual_g2_xyz[1] * other.e431, right_dual_g2_xyz[2] * other.e412, right_dual_g1_w * other.e321) * this.group0) - (float4(right_dual_g2_xyz[1] * other.e423, right_dual_g2_xyz[0] * other.e431, right_dual_g2_xyz[0] * other.e412, right_dual_g2_xyz[2] * other.e321) * this.group0.yxxz) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((other.group0.xyz * right_dual_g2_xyz.zzy * this.group0.zzy), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (this.group0.wwwx * float4(right_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0);
        float3 anti_wedge_g2 = (right_dual_g2.yzx * this.group0.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_dual_g2.zxy * this.group0.yzx);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2[0] * other.e23) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g2[1] * other.e31), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (this.group0.wwwx * float4(right_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0);
        float4 anti_wedge_g2 = (other.group2.zxyw * this.group0.yzxw) + float4(float3(0.0), (other.e4125 * this.e4125) * -1.0) - (this.group0.zxyy * float4(other.group2.yzx, other.e4315)) - (this.group0.wwwx * float4(other.group1.xyz, other.e4235));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(FlatPoint other) {
        float3 anti_wedge_g0 = float3(other.e45 * -1.0) * this.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float3(other.e45)), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = float4((float3(other.e45 * -1.0) * this.group0.xyz), -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Plane anti_project_via_horizon_onto(Line other) {
        float4 anti_wedge_g0 = float4(float3(0.0), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group0.zxyx);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e435, other.e415, other.e425, other.e235) * anti_wedge_g0.yzxx) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((other.group0.yzx * anti_wedge_g0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e4315) - (right_dual_g1_xyz[2] * this.e4125)) + float4((right_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(right_dual_g0_xyz.yzx, right_dual_g1_xyz[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (other.e1234 * this.e3215);
        float4 anti_wedge_g1 = (this.group0.yzxx * float4(right_dual_g5.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_dual_g5.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = (other.e321 * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g6_xyz[1] * this.e4315) - (right_dual_g6_xyz[2] * this.e4125)) + float4((right_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_dual_g8.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0]));
        float3 anti_wedge_g4 = (right_dual_g7.zxy * this.group0.yzx) - (right_dual_g7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (right_dual_g7 * float3(this.e3215)) - (float3(other.e45) * this.group0.xyz);
        float4 anti_wedge_g6 = float4(((right_dual_g9_xyz.yzx * this.group0.zxy) - (right_dual_g9_xyz.zxy * this.group0.yzx)), right_dual_g10 * this.e3215 * -1.0);
        float3 anti_wedge_g7 = float3(right_dual_g10 * -1.0) * this.group0.xyz;
        float3 anti_wedge_g8 = -(right_dual_g9_xyz * float3(this.e3215)) - (float3(other.e5) * this.group0.xyz);
        float4 anti_wedge_g9 = float4(other.scalar) * this.group0;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
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
            /* e4235, e4315, e4125, e3215 */
            -(float4(other.e4235 * this.e4235, other.e4315 * this.e4315, other.e4125 * this.e4125, other.e4235 * this.e4235) * other.group0) - (float4(other.e4315 * this.e4315, other.e4235 * this.e4235, other.e4235 * this.e4235, other.e4315 * this.e4315) * other.group0) - (other.group0 * float4(float2(other.e4125 * this.e4125), other.e4315 * this.e4315, other.e4125 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = float3(right_dual_g1 * -1.0) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx);
        float anti_wedge_g1_w = right_dual_g1 * this.e3215 * -1.0;
        float3 anti_wedge_g2 = -(right_dual_g0_xyz * float3(this.e3215)) - (float3(other.e5) * this.group0.xyz);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g2 * float3(other.e4)), 0.0) + float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])), 
            /* e1234 */
            (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
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
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_via_horizon_onto(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float3 anti_wedge_g0 = float3(right_dual_g2_w * -1.0) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = (right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx);
        float anti_wedge_g1_w = right_dual_g2_w * this.e3215 * -1.0;
        float anti_wedge_g2_x = -(right_dual_g3_xyz[0] * this.e3215) - (this.e4235 * other.e5);
        float anti_wedge_g2_y = -(right_dual_g3_xyz[1] * this.e3215) - (this.e4315 * other.e5);
        float anti_wedge_g2_z = -(right_dual_g3_xyz[2] * this.e3215) - (this.e4125 * other.e5);
        float anti_wedge_g2_w = (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125);
        float4 anti_wedge_g3 = (this.group0.yzxw * float4(right_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_dual_g2_xyz[2] * this.e4125) * -1.0) - (this.group0.zxyy * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[1])) - (this.group0.wwwx * float4(right_dual_g0_xyz, right_dual_g2_xyz[0]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2_w) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) + float4(float3(0.0), (anti_wedge_g2_w * other.e5) * -1.0) - float4((anti_wedge_g3.yzx * other.group3.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(float3(anti_wedge_g3[3]), anti_wedge_g1_w)) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2_w * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2_z * other.e3)) + float4((float3(other.e4) * float3(anti_wedge_g2_x, anti_wedge_g2_y, anti_wedge_g2_z)), 0.0) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[1] * other.e315) - (other.group2 * float4(float3(anti_wedge_g2_w), anti_wedge_g1_w)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_x)) - float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2_y * other.e2) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Plane {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (this.group0.zxyx * float4(other.group0.yzx, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g3_xyz[1] * this.e4315) + (right_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234)) - float4((this.group0.yzx * other.group0.zxy), 0.0);
        float4 anti_wedge_g1 = float4(-(this.e4235 * other.e45) - (this.e3215 * other.e41), -(this.e4315 * other.e45) - (this.e3215 * other.e42), -(this.e4125 * other.e45) - (this.e3215 * other.e43), (this.e4235 * other.e23) + (this.e4315 * other.e31) + (this.e4125 * other.e12));
        float4 anti_wedge_g2 = float4((right_dual_g2.yzx * this.group0.zxy), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((right_dual_g2.zxy * this.group0.yzx), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * other.group3) + (float4(other.scalar * other.scalar) * this.group0) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<RoundPoint> operator /(RoundPoint lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float anti_wedge_g1 = other.scalar * this.e5;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e4 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g0[0] * other.e25) - (anti_wedge_g0[1] * other.e15)) + float3((anti_wedge_g0.yz * other.group2.zx), 0.0) - float3((anti_wedge_g0.zx * other.group2.yz), 0.0)), anti_wedge_g0[3] * other.scalar), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * float4(anti_wedge_g0.xyz, anti_wedge_g1)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float anti_wedge_g0 = (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(anti_wedge_g0) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_via_horizon_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.e3215), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.scalar * other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public AntiFlector anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float anti_wedge_g0 = (right_dual_g1_xyz[0] * this.e1) + (right_dual_g1_xyz[1] * this.e2) + (right_dual_g1_xyz[2] * this.e3) - (other.e5 * this.e4);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float anti_wedge_g1 = other.scalar * this.e5;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1) * other.group0.xyz) + float3(float2(0.0), (anti_wedge_g0[0] * other.e25) - (anti_wedge_g0[1] * other.e15)) + float3((anti_wedge_g0.yz * other.group1.zx), 0.0) - float3((anti_wedge_g0.zx * other.group1.yz), 0.0)), anti_wedge_g1 * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(other.e5 * this.e4 * -1.0) * other.group0) + (float4(right_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public DualNum anti_project_via_horizon_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * this.e4 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float anti_wedge_g0 = other.e5 * this.e4 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_wedge_g0) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float anti_wedge_g0_x = (right_dual_g9_xyz[0] * this.e1) + (right_dual_g9_xyz[1] * this.e2) + (right_dual_g9_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4);
        float4 anti_wedge_g1 = float4(right_dual_g0[1]) * this.group0;
        float anti_wedge_g2 = right_dual_g0[1] * this.e5;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) - (anti_wedge_g1[1] * other.e15)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), 0.0) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float anti_wedge_g0 = (right_dual_g0_xyz[0] * this.e1) + (right_dual_g0_xyz[1] * this.e2) + (right_dual_g0_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group0, 
            /* e5 */
            anti_wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_project_via_horizon_onto(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g3_xyz = other.group3.xyz;
        float anti_wedge_g0 = (right_dual_g3_xyz[0] * this.e1) + (right_dual_g3_xyz[1] * this.e2) + (right_dual_g3_xyz[2] * this.e3) - (this.e4 * other.e5) - (this.e5 * other.e4);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(anti_wedge_g0) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float anti_wedge_g1 = this.e5 * other.scalar;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g0.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g1 * other.e1234) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)) - float4((anti_wedge_g0.yzx * other.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g1) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + float3(float2(0.0), (anti_wedge_g0[0] * other.e25) - (anti_wedge_g0[1] * other.e15)) + float3((anti_wedge_g0.yz * other.group2.zx), 0.0) - float3((anti_wedge_g0.zx * other.group2.yz), 0.0)), anti_wedge_g1 * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Scalar> operator /(Scalar lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_project_via_horizon_onto(AntiCircleRotor other) {
        float anti_wedge_g0 = other.scalar * this.scalar;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(anti_wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public AntiDualNum anti_project_via_horizon_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public AntiMotor anti_project_via_horizon_onto(AntiMotor other) {
        float anti_wedge_g0 = other.scalar * this.scalar;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(anti_wedge_g0) * other.group0, 
            /* e15, e25, e35, e3215 */
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
            /* scalar, e12345 */
            float2(anti_wedge_g0) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group1, 
            /* e5 */
            anti_wedge_g0 * other.e5, 
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group3, 
            /* e41, e42, e43 */
            float3(anti_wedge_g0) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g0) * other.group6, 
            /* e423, e431, e412 */
            float3(anti_wedge_g0) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(anti_wedge_g0) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group9, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
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
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Scalar {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float anti_wedge_g0 = this.scalar * other.scalar;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(anti_wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<Sphere> operator /(Sphere lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_dual_g0.zxy * this.group0.yzx) - (float3(this.e1234) * other.group1.xyz) - (right_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (this.group0.wwwx * float4(right_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0) - float4((float3(this.e1234) * other.group2.xyz), 0.0);
        float3 anti_wedge_g2_xyz = (other.group2.zxy * this.group0.yzx) + float3(float2(0.0), (other.e15 * this.e4315) * -1.0) - (float3(this.e3215) * other.group1.xyz) - float3((other.group2.yz * this.group0.zx), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(other.scalar), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_xyz * float3(other.scalar)), (other.scalar * other.scalar * this.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar * other.scalar) * this.group0) + float4(float3(0.0), -(anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), 0.0) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g3_xyz * float3(this.e1234)) - (float3(right_dual_g2_w) * this.group0.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g3_xyz.yzx * this.group0.zxy), right_dual_g3_w * this.e1234) - (this.group0.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g2_w));
        float4 anti_wedge_g2 = (this.group0.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (right_dual_g1_w * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((right_dual_g3_xyz * float3(this.e3215)), 0.0);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g2_xyz[1] * this.e4315) - (right_dual_g2_xyz[2] * this.e4125)) + float4((right_dual_g2_xyz * float3(this.e1234)), 0.0) + float4((right_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1_w)) - (this.group0.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(other.e2, other.e3, other.e1, other.e4) * anti_wedge_g3.zxyw) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[3]) * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (float4(anti_wedge_g2[0], anti_wedge_g2[1], anti_wedge_g2[2], anti_wedge_g3[0]) * other.group2.wwwx) + (anti_wedge_g3.yzxy * float4(other.group1.zxy, other.e315)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1.yzx * other.group3.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group3.yzxx) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public VersorOdd anti_project_via_horizon_onto(AntiDualNum other) {
        float anti_wedge_g0_w = other.e3215 * this.e1234;
        float4 anti_wedge_g3 = float4(other.scalar) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), anti_wedge_g0_w * other.scalar), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.scalar * other.scalar * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4((float2(other.scalar) * anti_wedge_g3.xy), anti_wedge_g3[2] * other.scalar, (anti_wedge_g0_w * other.e3215) + (anti_wedge_g3[3] * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float4 anti_wedge_g0 = right_dual_g0 * float4(this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz), (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125) - (right_dual_g0[0] * other.e321 * this.e4235) - (right_dual_g0[1] * other.e321 * this.e4315) - (right_dual_g0[2] * other.e321 * this.e4125) - (right_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = right_dual_g1_xyz * float3(this.e1234);
        float3 anti_wedge_g1_xyz = (right_dual_g1_xyz.yzx * this.group0.zxy) - (right_dual_g1_xyz.zxy * this.group0.yzx);
        float3 anti_wedge_g2_xyz = (float3(right_dual_g1_w) * this.group0.xyz) - (right_dual_g1_xyz * float3(this.e3215));
        float anti_wedge_g2_w = right_dual_g0_w * this.e1234;
        float4 anti_wedge_g3 = float4(this.e1234, this.e1234, this.e1234, -(right_dual_g0_w * this.e3215) - (right_dual_g0_xyz[0] * this.e4235) - (right_dual_g0_xyz[1] * this.e4315) - (right_dual_g0_xyz[2] * this.e4125)) * float4(right_dual_g0_xyz, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2_w * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group1.yzx) + float3(float2(0.0), anti_wedge_g3[0] * other.e2 * -1.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0)), anti_wedge_g2_w * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2_w * other.e321)) - float4((float3(other.e5) * anti_wedge_g3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) + (anti_wedge_g3[3] * other.e321) - (anti_wedge_g2_xyz[1] * other.e2) - (anti_wedge_g2_xyz[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group1.zxy), anti_wedge_g3[0] * other.e235) - (float4(other.e5) * float4(anti_wedge_g0, right_dual_g1_w * this.e1234)) - (other.group1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_xyz[0])) - float4((float3(anti_wedge_g2_w) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e1234);
        float4 anti_wedge_g1 = float4(this.e1234, this.e1234, this.e1234, -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) * float4(right_dual_g1, 1.0);
        float3 anti_wedge_g2 = (right_dual_g0 * float3(this.e3215)) + (right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group0.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e23), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public VersorOdd anti_project_via_horizon_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(this.e1234) * float4((other.group0.xyz * float3(-1.0)), other.e3215);
        float4 anti_wedge_g1 = float4(this.e1234, this.e1234, this.e1234, (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) * float4((other.group1.xyz * float3(-1.0)), 1.0);
        float4 anti_wedge_g2 = float4(((other.group1.zxy * this.group0.yzx) + float3(float2(0.0), other.e15 * this.e4315 * -1.0) - (float3(this.e3215) * other.group0.xyz) - float3((other.group1.yz * this.group0.zx), 0.0)), other.scalar * this.e1234);
        float4 anti_wedge_g3 = float4(other.scalar) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(anti_wedge_g1[3]), anti_wedge_g3[3])) + (anti_wedge_g0.yzxw * other.group1.zxyw) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(other.scalar) * anti_wedge_g3.xyz), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = right_dual_g0_xyz * float3(this.e1234);
        float3 anti_wedge_g1_xyz = (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx);
        float3 anti_wedge_g2 = (float3(right_dual_g0_w) * this.group0.xyz) - (right_dual_g0_xyz * float3(this.e3215));
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, right_dual_g0_w * this.e1234)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])), 
            /* e1234 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float4 anti_wedge_g0 = (float4(this.e1234) * float4(other.group2.xyz, right_dual_g1_w)) + (this.group0.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group0.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(other.e235) * float4(anti_wedge_g0[3], other.e431 * this.e4235, other.e412 * this.e4235, other.e321 * this.e4235)) - (float4(other.e315) * float4(other.e423 * this.e4315, anti_wedge_g0[3], other.e412 * this.e4315, other.e321 * this.e4315)) - (float4(other.e125) * float4(other.e423 * this.e4125, other.e431 * this.e4125, anti_wedge_g0[3], other.e321 * this.e4125)) - (float4(other.e423 * other.e235, other.e431 * other.e315, other.e412 * other.e125, right_dual_g1_w * other.e321) * this.group0) - float4((float3(right_dual_g1_w * this.e3215) * other.group0.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float4 anti_wedge_g0 = (float4(this.e1234) * float4(right_dual_g2_xyz, right_dual_g1_w)) + (this.group0.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group0.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], right_dual_g2_xyz[0] * this.e4235) * other.group1.yzxw) - (float4(right_dual_g1_w * other.e423, right_dual_g1_w * other.e431, right_dual_g1_w * other.e412, right_dual_g2_xyz[1] * other.e321) * this.group0.wwwy) - (float4(right_dual_g2_xyz[0] * other.e423, right_dual_g2_xyz[1] * other.e431, right_dual_g2_xyz[2] * other.e412, right_dual_g1_w * other.e321) * this.group0) - (float4(right_dual_g2_xyz[1] * other.e423, right_dual_g2_xyz[0] * other.e431, right_dual_g2_xyz[0] * other.e412, right_dual_g2_xyz[2] * other.e321) * this.group0.yxxz) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((other.group0.xyz * right_dual_g2_xyz.zzy * this.group0.zzy), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_dual_g0.zxy * this.group0.yzx) - (float3(this.e1234) * other.group1.xyz) - (right_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (float4(this.e1234, this.e1234, this.e1234, other.e31 * this.e4315) * float4(right_dual_g2, 1.0)) + (this.group0.wwwx * float4(right_dual_g0, other.e23)) + float4(float3(0.0), other.e12 * this.e4125) - float4((float3(other.e45) * this.group0.xyz), 0.0);
        float3 anti_wedge_g2 = (right_dual_g2.yzx * this.group0.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_dual_g2.zxy * this.group0.yzx);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2[0] * other.e23) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g2[1] * other.e31), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_dual_g0.zxy * this.group0.yzx) - (float3(this.e1234) * other.group1.xyz) - (right_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (this.group0.wwwx * float4(right_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0) - float4((float3(this.e1234) * other.group2.xyz), 0.0);
        float4 anti_wedge_g2 = (other.group2.zxyw * this.group0.yzxw) + float4(float3(0.0), (other.e3215 * this.e1234) - (other.e4125 * this.e4125)) - (this.group0.zxyy * float4(other.group2.yzx, other.e4315)) - (this.group0.wwwx * float4(other.group1.xyz, other.e4235));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public AntiDualNum anti_project_via_horizon_onto(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e5 * other.e5 * this.e1234, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Plane anti_project_via_horizon_onto(FlatPoint other) {
        float3 anti_wedge_g0 = -(float3(other.e45) * this.group0.xyz) - (float3(this.e1234) * other.group0.xyz);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float3(other.e45)), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (float4(other.e45, other.e45, other.e45, other.e4235) * this.group0.xyzx) - float4((float3(this.e1234) * other.group0.xyz), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Plane anti_project_via_horizon_onto(Line other) {
        float4 anti_wedge_g0 = float4(float3(0.0), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group0.zxyx);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e435, other.e415, other.e425, other.e235) * anti_wedge_g0.yzxx) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((other.group0.yzx * anti_wedge_g0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e4315) - (right_dual_g1_xyz[2] * this.e4125)) + float4((right_dual_g1_xyz * float3(this.e1234)), 0.0) + float4((right_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(right_dual_g0_xyz.yzx, right_dual_g1_xyz[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) + (other.e5 * other.e5 * this.e1234)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215);
        float4 anti_wedge_g1 = (right_dual_g3 * float4(this.e1234)) + (this.group0.yzxx * float4(right_dual_g5.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_dual_g5.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = -(right_dual_g3[0] * this.e4235) - (right_dual_g3[1] * this.e4315) - (right_dual_g3[2] * this.e4125) - (right_dual_g3[3] * this.e3215);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g6_xyz[1] * this.e4315) - (right_dual_g6_xyz[2] * this.e4125)) + float4((right_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_dual_g8.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0]));
        float3 anti_wedge_g4 = (right_dual_g6_xyz * float3(this.e1234)) + (right_dual_g7.zxy * this.group0.yzx) - (right_dual_g7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) - (float3(other.e45) * this.group0.xyz);
        float4 anti_wedge_g6 = float4((right_dual_g9_xyz.yzx * this.group0.zxy), right_dual_g9_w * this.e1234) - (this.group0.yzxw * float4(right_dual_g9_xyz.zxy, right_dual_g10));
        float3 anti_wedge_g7 = (right_dual_g9_xyz * float3(this.e1234)) - (float3(right_dual_g10) * this.group0.xyz);
        float3 anti_wedge_g8 = (float3(right_dual_g9_w) * this.group0.xyz) - (right_dual_g9_xyz * float3(this.e3215));
        float4 anti_wedge_g9 = float4(right_dual_g0[1]) * this.group0;
        float anti_wedge_g10 = right_dual_g0[1] * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215 * this.e1234) * other.group0) - (float4(other.e4235 * this.e4235, other.e4315 * this.e4315, other.e4125 * this.e4125, other.e4235 * this.e4235) * other.group0) - (float4(other.e4315 * this.e4315, other.e4235 * this.e4235, other.e4235 * this.e4235, other.e4315 * this.e4315) * other.group0) - (other.group0 * float4(float2(other.e4125 * this.e4125), other.e4315 * this.e4315, other.e4125 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g0_xyz * float3(this.e1234)) - (float3(right_dual_g1) * this.group0.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g0_xyz.yzx * this.group0.zxy), right_dual_g0_w * this.e1234) - (this.group0.yzxw * float4(right_dual_g0_xyz.zxy, right_dual_g1));
        float3 anti_wedge_g2 = (float3(right_dual_g0_w) * this.group0.xyz) - (right_dual_g0_xyz * float3(this.e3215));
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g2 * float3(other.e4)), 0.0) + float4((anti_wedge_g1.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group0.yzxx), 
            /* e1234 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_via_horizon_onto(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g3_xyz * float3(this.e1234)) - (float3(right_dual_g2_w) * this.group0.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g3_xyz.yzx * this.group0.zxy), right_dual_g3_w * this.e1234) - (this.group0.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g2_w));
        float4 anti_wedge_g2 = (this.group0.xyzx * float4(float3(right_dual_g3_w), right_dual_g0_xyz[0])) + float4(float3(0.0), (right_dual_g1_w * this.e1234) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125)) - float4((right_dual_g3_xyz * float3(this.e3215)), 0.0);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g2_xyz[1] * this.e4315) - (right_dual_g2_xyz[2] * this.e4125)) + float4((right_dual_g2_xyz * float3(this.e1234)), 0.0) + float4((right_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - (float4(this.e3215) * float4(right_dual_g0_xyz, right_dual_g1_w)) - (this.group0.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) * -1.0) - float4((anti_wedge_g3.yzx * other.group3.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(float3(anti_wedge_g3[3]), anti_wedge_g1[3])) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2[2] * other.e3)) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) + float4((anti_wedge_g1.yzx * other.group3.zxy), 0.0) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group3.yzxx) - (other.group2 * float4(float3(anti_wedge_g2[3]), anti_wedge_g1[3])) - float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[1] * other.e2) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension Sphere {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (this.group0.zxyx * float4(other.group0.yzx, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g3_xyz[1] * this.e4315) + (right_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) + (this.e1234 * other.e3215)) - float4((float3(this.e1234) * other.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0);
        float4 anti_wedge_g1 = (float4(this.e1234, this.e1234, this.e1234, this.e4235 * other.e23) * float4(right_dual_g2_xyz, 1.0)) + float4(float3(0.0), (this.e4315 * other.e31) + (this.e4125 * other.e12)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((float3(other.e45) * this.group0.xyz), 0.0);
        float4 anti_wedge_g2 = float4(((right_dual_g2_xyz.yzx * this.group0.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_dual_g2_xyz.zxy * this.group0.yzx)), this.e1234 * other.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * other.group3) + (float4(other.scalar * other.scalar) * this.group0) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<VersorEven> operator /(VersorEven lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(((right_dual_g0 * float3(this.e12345)) + (float3(right_dual_g2[3]) * this.group0.xyz)), right_dual_g2[3] * this.e12345);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g2[3]) * this.group1);
        float4 anti_wedge_g2 = (right_dual_g2 * float4(float3(this.e12345), this.e5)) + float4(float3(0.0), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) + float4((float3(right_dual_g2[3]) * this.group2.xyz), 0.0);
        float4 anti_wedge_g3 = (float4(right_dual_g2[3]) * this.group3) + float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) + float4((right_dual_g2.yzx * this.group0.zxy), 0.0) - (float4(right_dual_g2[2], right_dual_g2[0], right_dual_g2[1], right_dual_g1[0]) * this.group0.yzxx) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g0[0] * this.e415);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group0.yzx * anti_wedge_g3.zxy), 0.0) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g0[0] * other.e15) * float4(other.group0.zxy, 1.0)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (float4(right_dual_g2_w) * float4(this.group1.xyz, this.e5)) + (right_dual_g3.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(this.e12345) * other.group0.xyz), right_dual_g3[1] * this.e2) - (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], right_dual_g2_xyz[0]) * this.group0.yzxx);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_dual_g3[3]) * this.group0.xyz), 0.0) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (right_dual_g2_xyz * float3(this.e12345)) + (float3(right_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_dual_g3[1] * this.e235) - (right_dual_g3[0] * this.e315)) + float3((right_dual_g3.zx * this.group2.yz), 0.0) - float3((right_dual_g3.yz * this.group2.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g3[2] * other.e3) + (right_dual_g2_w * other.e5 * this.e12345) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((float3(anti_wedge_g0[3]) * other.group0.xyz), anti_wedge_g3[1] * other.e2) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float2(other.scalar) * anti_wedge_g0.xy), anti_wedge_g0[2] * other.scalar, (anti_wedge_g0[3] * other.scalar) + (anti_wedge_g3[3] * other.e3215)), 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * float3((float2(other.scalar) * this.group2.xy), other.scalar * this.e125)), (other.scalar * other.scalar * this.e5) + (other.e3215 * other.scalar * this.e12345)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_via_horizon_onto(AntiFlatPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float anti_wedge_g2_w = -(right_dual_g0_w * this.e321) - (right_dual_g0_xyz[0] * this.e423) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * other.e321), 
            /* e235, e315, e125, e12345 */
            other.group0 * float4(float3(anti_wedge_g2_w), right_dual_g0_w * this.e12345) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (right_dual_g1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g0_w * this.e321) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412)) - (float4(right_dual_g1[2], right_dual_g1[0], right_dual_g1[1], right_dual_g0_xyz[0]) * this.group0.yzxx);
        float4 anti_wedge_g1 = (this.group0 * float4(float3(right_dual_g1[3]), right_dual_g0_w)) + float4(float3(0.0), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - (right_dual_g1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((right_dual_g0_xyz * float3(this.e12345)), 0.0) + float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((right_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_dual_g1.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_dual_g1 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g2[3]) * other.group1.zxyw) + float4(float3(0.0), (anti_wedge_g3[0] * other.e1) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)) - (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group1.yz), 0.0) - float3((anti_wedge_g2.yz * other.group1.zx), 0.0)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = right_dual_g0 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412);
        float4 anti_wedge_g3 = (this.group1.wwwx * float4(right_dual_g0, other.e15)) + float4(float3(0.0), (other.e35 * this.e435) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) + float4((other.group1.zxy * this.group0.yzx), other.e25 * this.e425) - float4((other.group1.yzx * this.group0.zxy), right_dual_g0[0] * this.e235);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) + float4((other.group1.zxy * anti_wedge_g3.yzx), 0.0) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g1_xyz[0] * other.e23) * float4(other.group1.yzx, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_dual_g0[3]) * this.group0;
        float4 anti_wedge_g1 = float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e321);
        float4 anti_wedge_g2 = (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435));
        float4 anti_wedge_g3 = (right_dual_g0 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) + float4((float3(right_dual_g0[3]) * this.group3.xyz), 0.0) + float4((right_dual_g1.yzx * this.group0.zxy), 0.0) - (float4(right_dual_g1[2], right_dual_g1[0], right_dual_g1[1], right_dual_g0[0]) * this.group0.yzxx);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group1.zx), 0.0) - float3((anti_wedge_g3.zx * other.group1.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(AntiPlane other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (right_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)) - float4((right_dual_g0.zxy * this.group0.yzx), 0.0);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) - (right_dual_g0.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_dual_g0.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_dual_g0 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g2[3]) * other.group0.zxyw) + float4(float3(0.0), (anti_wedge_g3[0] * other.e1) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3)) - float4((anti_wedge_g0.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((float3(other.e5) * anti_wedge_g0.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * anti_wedge_g1.xyz) + float3(float2(0.0), (anti_wedge_g2[1] * other.e1) - (anti_wedge_g2[0] * other.e2)) + float3((anti_wedge_g2.zx * other.group0.yz), 0.0) - float3((anti_wedge_g2.yz * other.group0.zx), 0.0)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public AntiScalar anti_project_via_horizon_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_via_horizon_onto(Circle other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group2.xyz), -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_via_horizon_onto(CircleRotor other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_dual_g2_xyz * float3(this.e12345);
        float anti_wedge_g2_w = -(right_dual_g2_xyz[0] * this.e423) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(anti_wedge_g2_w) * other.group2.xy), anti_wedge_g2_w * other.e125, (anti_wedge_g2_w * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_dual_g2 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4(float3(0.0), -(right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) + float4((right_dual_g2.yzx * this.group0.zxy), 0.0) - (this.group2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g2[0] * this.e415);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((float3(anti_wedge_g3[3]) * other.group1.xyz), 0.0) + float4((other.group2.zxy * anti_wedge_g3.yzx), 0.0) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0] * other.e15) * float4(other.group2.yzx, 1.0))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_via_horizon_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345 * -1.0) * other.group2.xyz;
        float anti_wedge_g2_w = (other.e1234 * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = (float4(right_dual_g1[0], right_dual_g1[1], right_dual_g1[2], other.e15) * this.group1.wwwx) + (other.group2.zxyz * float4(this.group0.yzx, this.e435)) + (this.group1.xyzy * float4(float3(right_dual_g1[3]), other.e25)) + float4(float3(0.0), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + float4((right_dual_g0.zxy * this.group2.yzx), other.e3215 * this.e12345) - (this.group2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((float3(this.e12345) * other.group3.xyz), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group2.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group0.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (anti_wedge_g2_w * other.e3215) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - (float4(anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g3[1], anti_wedge_g0[0]) * other.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_via_horizon_onto(DualNum other) {
        float4 anti_wedge_g0 = float4((float3(other.e5 * -1.0) * this.group0.xyz), -(other.e5 * this.e4) - (other.e12345 * this.e12345));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g1 = (other.group0.zxyx * float4(this.group0.yzx, this.e415)) + (other.group0.wwwy * this.group1.xyzy) + float4(float3(0.0), other.e35 * this.e435) - float4((other.group0.yzx * this.group0.zxy), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45 * -1.0) * float4(anti_wedge_g1.xyz, other.e45 * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g1 = (other.group0.zxyx * float4(this.group0.yzx, this.e415)) + (other.group0.wwwy * this.group1.xyzy) + float4(float3(0.0), (other.e35 * this.e435) + (other.e3215 * this.e12345)) - float4((float3(this.e12345) * other.group1.xyz), 0.0) - float4((other.group0.yzx * this.group0.zxy), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * anti_wedge_g1.xyz), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (other.e45 * other.e45 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_via_horizon_onto(Line other) {
        float3 anti_wedge_g0_xyz = float3(this.e12345) * other.group0.xyz;
        float anti_wedge_g0_w = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float4 anti_wedge_g0 = (this.group0 * float4(float3(right_dual_g1_w), other.e12345 * -1.0)) + float4(float3(0.0), -(right_dual_g0_xyz[0] * this.e415) - (right_dual_g0_xyz[1] * this.e425) - (right_dual_g0_xyz[2] * this.e435) - (right_dual_g1_xyz[0] * this.e423) - (right_dual_g1_xyz[1] * this.e431) - (right_dual_g1_xyz[2] * this.e412)) + float4((right_dual_g0_xyz * float3(this.e12345)), right_dual_g1_w * this.e4);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float4 right_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) - (right_dual_g5[0] * this.e415) - (right_dual_g5[1] * this.e425) - (right_dual_g5[2] * this.e435) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = (float4(right_dual_g0[1]) * this.group3) + (this.group0.zxyw * float4(right_dual_g8.yzx, other.e1234)) + float4(float3(0.0), -(right_dual_g7[1] * this.e425) - (right_dual_g7[2] * this.e435) - (right_dual_g6[2] * this.e412)) + float4((float3(right_dual_g6[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_dual_g6.xyz), 0.0) + float4((right_dual_g7.zxy * this.group2.yzx), 0.0) - (this.group0.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])) - (this.group0.wwwy * float4(other.group9.xyz, right_dual_g6[1])) - float4((right_dual_g7.yzx * this.group2.zxy), right_dual_g7[0] * this.e415);
        float anti_wedge_g2 = (right_dual_g0[1] * this.e5) + (other.e3215 * this.e12345) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = (right_dual_g3 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) + float4((float3(right_dual_g9[3]) * this.group1.xyz), 0.0) + float4((right_dual_g9.zxy * this.group2.yzx), 0.0) - (right_dual_g9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + float3(float2(0.0), (right_dual_g9[0] * this.e431) - (right_dual_g9[1] * this.e423)) + float3((right_dual_g9.yz * this.group0.zx), 0.0) - float3((right_dual_g9.zx * this.group0.yz), 0.0);
        float3 anti_wedge_g5 = (right_dual_g5 * float3(this.e12345)) + (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_dual_g9.xyz);
        float4 anti_wedge_g6 = (right_dual_g6 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group1);
        float3 anti_wedge_g7 = (right_dual_g7 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group0.xyz);
        float3 anti_wedge_g8 = (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group2.xyz);
        float4 anti_wedge_g9 = right_dual_g9 * float4(this.e12345);
        float anti_wedge_g10 = right_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_dual_g0[1] * other.scalar * this.e12345) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public AntiScalar anti_project_via_horizon_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = (float4(right_dual_g1) * float4(this.group1.xyz, this.e5)) + (right_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)) - float4((right_dual_g0.zxy * this.group0.yzx), 0.0);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((float3(right_dual_g1) * this.group2.xyz), 0.0) + float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) - (right_dual_g0.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_dual_g0[3]) * this.group1.xyz) + (right_dual_g0.zxy * this.group2.yzx) + float3(float2(0.0), (right_dual_g0[0] * this.e315) * -1.0) - float3((right_dual_g0.yz * this.group2.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g0 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group0.zxyx) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (right_dual_g1 * other.e5 * this.e12345)) - float4((anti_wedge_g0.zxy * other.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((float3(other.e5) * anti_wedge_g0.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(Scalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar * other.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(other.scalar * other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public AntiScalar anti_project_via_horizon_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (other.e3215 * other.e1234 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        float4 anti_wedge_g0 = (right_dual_g0 * float4(this.e12345)) + (float4(right_dual_g2_w) * float4(this.group1.xyz, this.e5)) + (right_dual_g3.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g2_xyz[1] * this.e431) - (right_dual_g2_xyz[2] * this.e412) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321)) - (float4(right_dual_g3[2], right_dual_g3[0], right_dual_g3[1], right_dual_g2_xyz[0]) * this.group0.yzxx);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_dual_g3[3]) * this.group0.xyz), 0.0) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (right_dual_g2_xyz * float3(this.e12345)) + (float3(right_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_dual_g3[1] * this.e235) - (right_dual_g3[0] * this.e315)) + float3((right_dual_g3.zx * this.group2.yz), 0.0) - float3((right_dual_g3.yz * this.group2.zx), 0.0);
        float4 anti_wedge_g3 = right_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (float4(other.e4) * float4(anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g1[2], anti_wedge_g3[3])) + (float4(anti_wedge_g0[1], anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g3[0]) * other.group3.zxyx) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (right_dual_g2_w * other.e5 * this.e12345) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) - (anti_wedge_g0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g2_xyz * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectViaHorizonOntoInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float4 anti_wedge_g0 = float4(((float3(other.scalar) * this.group0.xyz) - (float3(this.e12345) * other.group0.xyz)), this.e12345 * other.scalar);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1);
        float4 anti_wedge_g2 = (right_dual_g2 * float4(this.e12345)) + (float4(other.scalar) * this.group2) + float4(float3(0.0), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435));
        float4 anti_wedge_g3 = (float4(other.scalar) * this.group3) + (float4(right_dual_g1[0], right_dual_g1[1], right_dual_g1[2], other.e41) * this.group1.wwwx) + (float4(right_dual_g2[1], right_dual_g2[2], right_dual_g2[0], other.e1234) * this.group0.zxyw) + (this.group1.xyzy * float4(float3(right_dual_g1[3]), other.e42)) + (other.group0.yzxz * float4(this.group2.zxy, this.e435)) + float4(float3(0.0), (right_dual_g1[2] * this.e412) * -1.0) - (float4(right_dual_g2[2], right_dual_g2[0], right_dual_g2[1], right_dual_g1[0]) * this.group0.yzxx) - (this.group0.wwwy * float4(other.group3.xyz, right_dual_g1[1])) - float4((this.group2.yzx * other.group0.zxy), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (float4(anti_wedge_g3[3]) * float4(other.group1.xyz, other.e3215)) + (anti_wedge_g3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g2[3] * other.e1234) + (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[0]) * other.group0.zxyx), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group2.xyz), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + float3(float2(0.0), (anti_wedge_g3[0] * other.e25) - (anti_wedge_g3[1] * other.e15)) + float3((anti_wedge_g3.yz * other.group2.zx), 0.0) - float3((anti_wedge_g3.zx * other.group2.yz), 0.0)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectViaHorizonOntoInfixPartial<VersorOdd> operator /(VersorOdd lhs, AntiProjectViaHorizonOntoInfix rhs) {
    return AntiProjectViaHorizonOntoInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_via_horizon_onto(AntiCircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (float4(right_dual_g2[3]) * this.group0) + float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((other.group0.yzx * this.group3.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e1234), this.e45)) - float4((other.group0.zxy * this.group3.yzx), right_dual_g2[0] * this.e41);
        float4 anti_wedge_g1 = (right_dual_g2 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((float3(right_dual_g2[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0);
        float3 anti_wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group2.xyz) + (right_dual_g2.yzx * this.group3.zxy) + float3(float2(0.0), (right_dual_g2[1] * this.e4235) * -1.0) - (float3(this.e3215) * other.group1.xyz) - float3((right_dual_g2.zx * this.group3.yz), 0.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (other.group2 * float4(float3(anti_wedge_g0[3]), right_dual_g2[3] * this.e1234)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)) + float4((anti_wedge_g2_xyz * float3(other.scalar)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_dual_g2[3] * other.scalar) * this.group3) + float4(float3(0.0), -(anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_xyz[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(AntiDipoleInversion other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g3_xyz * float3(this.e1234)) - (float3(right_dual_g2_w) * this.group3.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g3_xyz.yzx * this.group3.zxy), right_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g2_w));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(float3(right_dual_g3_w), other.e423)) + float4(float3(0.0), (right_dual_g1_w * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_dual_g3_xyz[0] * this.e41) - (right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) - float4((right_dual_g3_xyz * float3(this.e3215)), right_dual_g2_w * this.e45);
        float4 anti_wedge_g3 = (float4(right_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_dual_g2_xyz, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_dual_g1_xyz.zxy * this.group3.yzx), right_dual_g3_xyz[1] * this.e25) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), right_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1_w)) - (this.group3.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0])) - float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g2_xyz[1] * this.e4315);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(other.e2, other.e3, other.e1, other.e4) * anti_wedge_g3.zxyw) - (float4(anti_wedge_g3[1], anti_wedge_g3[2], anti_wedge_g3[0], anti_wedge_g2[3]) * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (float4(anti_wedge_g2[0], anti_wedge_g2[1], anti_wedge_g2[2], anti_wedge_g3[0]) * other.group2.wwwx) + (anti_wedge_g3.yzxy * float4(other.group1.zxy, other.e315)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1.yzx * other.group3.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group3.yzxx) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_via_horizon_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4((float3(other.scalar) * this.group0.xyz), (other.e3215 * this.e1234) + (other.scalar * this.scalar));
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar * other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4((float2(other.scalar) * anti_wedge_g3.xy), anti_wedge_g3[2] * other.scalar, (anti_wedge_g0[3] * other.e3215) + (anti_wedge_g3[3] * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_via_horizon_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float4 anti_wedge_g0 = right_dual_g0 * float4(this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3] * -1.0) * other.group0.xyz), (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125) - (right_dual_g0[0] * other.e321 * this.e4235) - (right_dual_g0[1] * other.e321 * this.e4315) - (right_dual_g0[2] * other.e321 * this.e4125) - (right_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(AntiFlector other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e321 * -1.0;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = right_dual_g1_xyz * float3(this.e1234);
        float3 anti_wedge_g1_xyz = (right_dual_g1_xyz.yzx * this.group3.zxy) - (right_dual_g1_xyz.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4((float2(right_dual_g1_w) * this.group3.xy), right_dual_g1_w * this.e4125, right_dual_g0_w * this.e1234) + float4(float3(0.0), -(right_dual_g1_xyz[1] * this.e42) - (right_dual_g1_xyz[2] * this.e43)) - float4((right_dual_g1_xyz * float3(this.e3215)), right_dual_g1_xyz[0] * this.e41);
        float4 anti_wedge_g3 = (float4(right_dual_g1_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_dual_g0_xyz, right_dual_g1_xyz[0])) + float4(float3(0.0), (right_dual_g1_xyz[2] * this.e35) - (right_dual_g0_xyz[0] * this.e4235) - (right_dual_g0_xyz[1] * this.e4315) - (right_dual_g0_xyz[2] * this.e4125)) + float4((right_dual_g1_xyz.zxy * this.group1.yzx), right_dual_g1_xyz[1] * this.e25) - float4((right_dual_g1_xyz.yzx * this.group1.zxy), right_dual_g0_w * this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3] * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group1.yzx) + float3(float2(0.0), anti_wedge_g3[0] * other.e2 * -1.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321)) - float4((float3(other.e5) * anti_wedge_g3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) + (anti_wedge_g3[3] * other.e321) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group1.zxy), anti_wedge_g3[0] * other.e235) - (float4(other.e5) * float4(anti_wedge_g0, right_dual_g1_w * this.e1234)) - (anti_wedge_g2.wwwy * float4(other.group0.xyz, other.e2)) - (other.group1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_dual_g0 * float3(this.e1234);
        float3 anti_wedge_g1_xyz = right_dual_g1 * float3(this.e1234);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) + float4((right_dual_g0 * float3(this.e3215)), 0.0) + float4((right_dual_g1.yzx * this.group3.zxy), 0.0) - float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g0[0] * this.e23);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(anti_wedge_g2[3]) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1_xyz[1] * other.e25) - (anti_wedge_g1_xyz[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(-(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) * other.group0.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group1.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g1_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_via_horizon_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (right_dual_g0 * float4(float3(this.e1234), this.scalar)) + (this.group0.xyzx * float4(float3(right_dual_g0[3]), other.e15)) + float4(float3(0.0), (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12));
        float4 anti_wedge_g1 = (float4(right_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) - float4((float3(this.e1234) * other.group1.xyz), 0.0);
        float4 anti_wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_dual_g0.xyz) + float3(float2(0.0), (other.e25 * this.e4235) - (other.e15 * this.e4315)) + float3((other.group1.zx * this.group3.yz), 0.0) - float3((other.group1.yz * this.group3.zx), 0.0)), right_dual_g0[3] * this.e1234);
        float4 anti_wedge_g3 = float4(right_dual_g0[3]) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(anti_wedge_g1[3]), anti_wedge_g3[3])) + (anti_wedge_g0.yzxw * other.group1.zxyw) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(other.scalar) * anti_wedge_g3.xyz), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(AntiPlane other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = right_dual_g0_xyz * float3(this.e1234);
        float3 anti_wedge_g1_xyz = (right_dual_g0_xyz.yzx * this.group3.zxy) - (right_dual_g0_xyz.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_dual_g0_xyz * float3(this.e3215)), right_dual_g0_xyz[0] * this.e41);
        float4 anti_wedge_g3 = (float4(right_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), right_dual_g0_xyz[0] * this.e15) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3] * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group0.yzx) + float3(float2(0.0), anti_wedge_g3[0] * other.e2 * -1.0) - float3((anti_wedge_g3.yz * other.group0.zx), 0.0)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group0.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((float3(other.e5) * anti_wedge_g3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, right_dual_g0_w * this.e1234)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_via_horizon_onto(Circle other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float4 anti_wedge_g0 = (float4(this.e1234) * float4(other.group2.xyz, right_dual_g1_w)) + (this.group3.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group3.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(other.e235) * float4(anti_wedge_g0[3], other.e431 * this.e4235, other.e412 * this.e4235, other.e321 * this.e4235)) - (float4(other.e315) * float4(other.e423 * this.e4315, anti_wedge_g0[3], other.e412 * this.e4315, other.e321 * this.e4315)) - (float4(other.e125) * float4(other.e423 * this.e4125, other.e431 * this.e4125, anti_wedge_g0[3], other.e321 * this.e4125)) - (float4(other.e423 * other.e235, other.e431 * other.e315, other.e412 * other.e125, right_dual_g1_w * other.e321) * this.group3) - float4((float3(right_dual_g1_w * this.e3215) * other.group0.xyz), 0.0) - float4((anti_wedge_g0.zxy * other.group1.yzx), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_via_horizon_onto(CircleRotor other) {
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float4 anti_wedge_g0 = (float4(this.e1234) * float4(right_dual_g2_xyz, right_dual_g1_w)) + (this.group3.yzxx * float4(right_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_dual_g1_xyz.yzx * this.group3.zxy), 0.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], right_dual_g2_xyz[0] * this.e4235) * other.group1.yzxw) - (float4(right_dual_g1_w * other.e423, right_dual_g1_w * other.e431, right_dual_g1_w * other.e412, right_dual_g2_xyz[1] * other.e321) * this.group3.wwwy) - (float4(right_dual_g2_xyz[0] * other.e423, right_dual_g2_xyz[1] * other.e431, right_dual_g2_xyz[2] * other.e412, right_dual_g1_w * other.e321) * this.group3) - (float4(right_dual_g2_xyz[1] * other.e423, right_dual_g2_xyz[0] * other.e431, right_dual_g2_xyz[0] * other.e412, right_dual_g2_xyz[2] * other.e321) * this.group3.yxxz) - float4((float3(anti_wedge_g0[3]) * other.group2.xyz), 0.0) - float4((other.group0.xyz * right_dual_g2_xyz.zzy * this.group3.zzy), 0.0), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(Dipole other) {
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx);
        float4 anti_wedge_g1 = float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((right_dual_g2 * float3(this.e1234)), other.e23 * this.e4235) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0);
        float4 anti_wedge_g2 = float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((right_dual_g2.yzx * this.group3.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e3215), this.e45)) - float4((right_dual_g2.zxy * this.group3.yzx), right_dual_g2[0] * this.e41);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(DipoleInversion other) {
        float3 anti_wedge_g0 = (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx);
        float anti_wedge_g1_x = -(other.e41 * this.e3215) - (other.e45 * this.e4235) - (other.e15 * this.e1234);
        float anti_wedge_g1_y = -(other.e42 * this.e3215) - (other.e45 * this.e4315) - (other.e25 * this.e1234);
        float anti_wedge_g1_z = -(other.e43 * this.e3215) - (other.e45 * this.e4125) - (other.e35 * this.e1234);
        float4 anti_wedge_g2 = (other.group2.zxyx * float4(this.group3.yzx, this.e41)) + float4(float3(0.0), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group1 * float4(float3(this.e3215), this.e45)) - (this.group3.zxyx * float4(other.group2.yzx, other.e4235));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g1_x * other.e41) - (anti_wedge_g1_y * other.e42) - (anti_wedge_g1_z * other.e43) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2[3]) * other.group3) + float4(float3(0.0), -(anti_wedge_g1_y * other.e25) - (anti_wedge_g1_z * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(other.e45) * float3(anti_wedge_g1_x, anti_wedge_g1_y, anti_wedge_g1_z)), 0.0) + float4((float3((other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) * other.group1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * anti_wedge_g2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g2.yzxx) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1_x))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_via_horizon_onto(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * other.e5) * this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e5 * other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_via_horizon_onto(FlatPoint other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)) - (other.group0 * float4(float3(this.e1234), this.e45)) - float4((float3(other.e45) * this.group3.xyz), 0.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_via_horizon_onto(Flector other) {
        float4 anti_wedge_g0 = float4(float3(0.0), (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (float4(other.e45, other.e45, other.e45, other.e4235) * this.group3.xyzx) - (other.group0 * float4(float3(this.e1234), this.e45));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Plane anti_project_via_horizon_onto(Line other) {
        float4 anti_wedge_g0 = float4(float3(0.0), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group3.zxyx);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e435, other.e415, other.e425, other.e235) * anti_wedge_g0.yzxx) + float4(float3(0.0), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) - float4((other.group0.yzx * anti_wedge_g0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_via_horizon_onto(Motor other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        float4 anti_wedge_g1 = (right_dual_g1 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((float3(right_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_dual_g0_xyz.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(right_dual_g0_xyz.yzx, right_dual_g1[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * anti_wedge_g1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (right_dual_g1[3] * other.e5 * this.e1234)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public MultiVector anti_project_via_horizon_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g5 = other.group6.xyz;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_dual_g9_xyz = other.group1.xyz;
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_dual_g0[1] * this.scalar) + (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (right_dual_g6_xyz[0] * this.e23) - (right_dual_g6_xyz[1] * this.e31) - (right_dual_g6_xyz[2] * this.e12) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (other.e45 * this.e45);
        float4 anti_wedge_g1 = (right_dual_g3 * float4(this.e1234)) + (this.group3.yzxx * float4(right_dual_g5.zxy, other.e423)) + float4(float3(0.0), other.e412 * this.e4125) + float4((float3(right_dual_g9_w) * this.group0.xyz), 0.0) + float4((right_dual_g9_xyz.zxy * this.group1.yzx), other.e431 * this.e4315) - (float4(right_dual_g10) * float4(this.e15, this.e25, this.e35, this.e45)) - float4((float3(this.e3215) * other.group7.xyz), right_dual_g9_xyz[2] * this.e43) - float4((right_dual_g5.yzx * this.group3.zxy), right_dual_g9_xyz[0] * this.e41) - float4((right_dual_g9_xyz.yzx * this.group1.zxy), right_dual_g9_xyz[1] * this.e42);
        float anti_wedge_g2 = (right_dual_g9_w * this.e45) + (right_dual_g9_xyz[0] * this.e15) + (right_dual_g9_xyz[1] * this.e25) + (right_dual_g9_xyz[2] * this.e35) - (right_dual_g3[0] * this.e4235) - (right_dual_g3[1] * this.e4315) - (right_dual_g3[2] * this.e4125) - (right_dual_g3[3] * this.e3215);
        float4 anti_wedge_g3 = (float4(right_dual_g0[1]) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_dual_g6_xyz[1] * this.e4315) - (right_dual_g6_xyz[2] * this.e4125)) + float4((right_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_dual_g8.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(right_dual_g8.zxy, right_dual_g6_xyz[0]));
        float3 anti_wedge_g4 = (right_dual_g6_xyz * float3(this.e1234)) + (float3(right_dual_g0[1]) * this.group0.xyz) + (right_dual_g7.zxy * this.group3.yzx) - (right_dual_g7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) + (float3(right_dual_g0[1]) * this.group1.xyz) - (float3(other.e45) * this.group3.xyz);
        float4 anti_wedge_g6 = float4((right_dual_g9_xyz.yzx * this.group3.zxy), right_dual_g9_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g9_xyz.zxy, right_dual_g10));
        float3 anti_wedge_g7 = (right_dual_g9_xyz * float3(this.e1234)) - (float3(right_dual_g10) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(right_dual_g9_w) * this.group3.xyz) - (right_dual_g9_xyz * float3(this.e3215));
        float4 anti_wedge_g9 = float4(right_dual_g0[1]) * this.group3;
        float anti_wedge_g10 = right_dual_g0[1] * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[1] * other.e1) - (anti_wedge_g1[0] * other.e2)) + float3((anti_wedge_g1.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.yz * other.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) + float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g4 * float3(other.e5)), 0.0) + float4((float3(anti_wedge_g2) * other.group4.xyz), 0.0) + float4((float3(anti_wedge_g1[3]) * other.group3.xyz), 0.0) + float4((float3(other.e4) * anti_wedge_g3.xyz), 0.0) - (anti_wedge_g1.xyzx * float4(float3(other.e45), other.e23)) - (other.group1.xyzx * float4(float3(anti_wedge_g3[3]), anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + float3(float2(0.0), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g3[1] * other.e1) - (anti_wedge_g1[1] * other.e15) - (anti_wedge_g3[0] * other.e2)) + float3((anti_wedge_g1.yz * other.group3.zx), 0.0) + float3((anti_wedge_g3.zx * other.group1.yz), 0.0) - float3((anti_wedge_g1.zx * other.group3.yz), 0.0) - float3((anti_wedge_g3.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + (float4(anti_wedge_g2) * float4(other.e423, other.e431, other.e412, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group6.zxy, other.e235)) + float4(float3(0.0), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g8[1] * other.e2) - (anti_wedge_g8[2] * other.e3) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((anti_wedge_g5 * float3(other.e45)), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g8 * float3(other.e4)), 0.0) + float4((float3(anti_wedge_g3[3]) * other.group5.xyz), 0.0) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), 0.0) + float4((anti_wedge_g6.yzx * other.group1.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g7, anti_wedge_g6[3])) - (float4(anti_wedge_g6[2], anti_wedge_g6[0], anti_wedge_g6[1], anti_wedge_g8[0]) * other.group1.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * anti_wedge_g3.yzxx) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((float3(anti_wedge_g1[3]) * other.group8.xyz), anti_wedge_g5[1] * other.e25) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Plane anti_project_via_horizon_onto(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215 * this.e1234) * other.group0) - (float4(other.e4235 * this.e4235, other.e4315 * this.e4315, other.e4125 * this.e4125, other.e4235 * this.e4235) * other.group0) - (float4(other.e4315 * this.e4315, other.e4235 * this.e4235, other.e4235 * this.e4235, other.e4315 * this.e4315) * other.group0) - (other.group0 * float4(float2(other.e4125 * this.e4125), other.e4315 * this.e4315, other.e4125 * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(RoundPoint other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float right_dual_g0_w = other.e5 * -1.0;
        float right_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g0_xyz * float3(this.e1234)) - (float3(right_dual_g1) * this.group3.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g0_xyz.yzx * this.group3.zxy), right_dual_g0_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g0_xyz.zxy, right_dual_g1));
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_dual_g0_xyz[0] * this.e41) - (right_dual_g0_xyz[1] * this.e42) - (right_dual_g0_xyz[2] * this.e43)) + float4((float3(right_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_dual_g0_xyz * float3(this.e3215)), right_dual_g1 * this.e45);
        float4 anti_wedge_g3 = (float4(right_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_dual_g0_xyz[1] * this.e25) + (right_dual_g0_xyz[2] * this.e35)) + float4((right_dual_g0_xyz.zxy * this.group1.yzx), right_dual_g0_xyz[0] * this.e15) - float4((float3(right_dual_g1) * this.group2.xyz), 0.0) - float4((right_dual_g0_xyz.yzx * this.group1.zxy), 0.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group0.yzxw) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) * -1.0) - float4((anti_wedge_g3.yzx * other.group0.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group0 * float4(float3(anti_wedge_g3[3]), anti_wedge_g1[3])) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((float3(other.e5) * anti_wedge_g3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) + float4((anti_wedge_g1.yzx * other.group0.zxy), 0.0) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_via_horizon_onto(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar * other.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.scalar * other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar * other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar * other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_via_horizon_onto(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_via_horizon_onto(VersorEven other) {
        float3 right_dual_g0_xyz = other.group0.xyz;
        float3 right_dual_g1_xyz = other.group1.xyz;
        float right_dual_g1_w = other.e321 * -1.0;
        float3 right_dual_g2_xyz = other.group2.xyz;
        float right_dual_g2_w = other.e4 * -1.0;
        float3 right_dual_g3_xyz = other.group3.xyz;
        float right_dual_g3_w = other.e5 * -1.0;
        float3 anti_wedge_g0 = (right_dual_g3_xyz * float3(this.e1234)) - (float3(right_dual_g2_w) * this.group3.xyz);
        float4 anti_wedge_g1 = float4((right_dual_g3_xyz.yzx * this.group3.zxy), right_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_dual_g3_xyz.zxy, right_dual_g2_w));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(float3(right_dual_g3_w), right_dual_g0_xyz[0])) + float4(float3(0.0), (right_dual_g1_w * this.e1234) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) - (right_dual_g3_xyz[0] * this.e41) - (right_dual_g3_xyz[1] * this.e42) - (right_dual_g3_xyz[2] * this.e43)) - float4((right_dual_g3_xyz * float3(this.e3215)), right_dual_g2_w * this.e45);
        float4 anti_wedge_g3 = (float4(right_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_dual_g2_xyz, right_dual_g3_xyz[0])) + float4(float3(0.0), (right_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_dual_g1_xyz.zxy * this.group3.yzx), right_dual_g3_xyz[1] * this.e25) + float4((right_dual_g3_xyz.zxy * this.group1.yzx), right_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(right_dual_g0_xyz, right_dual_g1_w)) - (this.group3.zxyx * float4(right_dual_g1_xyz.yzx, right_dual_g2_xyz[0])) - float4((float3(right_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g2_xyz[1] * this.e4315);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) * -1.0) - float4((anti_wedge_g3.yzx * other.group3.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(float3(anti_wedge_g3[3]), anti_wedge_g1[3])) + float4(float3(0.0), (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - (anti_wedge_g3.xyzx * float4(float3(other.e5), other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxx * float4(other.group1.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) - (anti_wedge_g2[2] * other.e3)) + float4((float3(other.e4) * anti_wedge_g2.xyz), 0.0) + float4((anti_wedge_g1.yzx * other.group3.zxy), 0.0) - (float4(anti_wedge_g1[2], anti_wedge_g1[0], anti_wedge_g1[1], anti_wedge_g2[0]) * other.group3.yzxx) - (other.group2 * float4(float3(anti_wedge_g2[3]), anti_wedge_g1[3])) - float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[1] * other.e2) - float4((anti_wedge_g3.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectViaHorizonOntoInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_via_horizon_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_via_horizon_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        float4 anti_wedge_g0 = (float4(right_dual_g0[3]) * this.group0) + (float4(right_dual_g0[2], right_dual_g0[0], right_dual_g0[1], right_dual_g3_xyz[0]) * this.group3.yzxx) + float4(float3(0.0), (right_dual_g3_xyz[1] * this.e4315) + (right_dual_g3_xyz[2] * this.e4125) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (right_dual_g2_xyz[0] * this.e41) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35)) - (other.group1 * float4(float3(this.e1234), this.e45)) - (right_dual_g0.yzxx * float4(this.group3.zxy, this.e15));
        float4 anti_wedge_g1 = (right_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((right_dual_g2_xyz * float3(this.e1234)), other.e23 * this.e4235) + float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0);
        float4 anti_wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (right_dual_g2_xyz.yzx * this.group3.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_dual_g2_xyz.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * other.group3) + (float4(right_dual_g0[3] * other.scalar) * this.group3) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((float3(anti_wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e45) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - (float4(anti_wedge_g0[2], anti_wedge_g0[0], anti_wedge_g0[1], anti_wedge_g1[0]) * other.group2.yzxx) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
