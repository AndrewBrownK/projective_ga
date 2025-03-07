implementing cga3d;
using traits;
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g2[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g2[3]) * this.group2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2_w) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g2_xyz[2] * other.e42) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g2_xyz[0] * other.e43) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g2_xyz[1] * other.e41) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g2_xyz[0] * other.e23) - (anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[2] * other.e35)) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g3[2] * this.e31) + (right_anti_dual_g3[3] * this.e41), (right_anti_dual_g3[0] * this.e12) + (right_anti_dual_g3[3] * this.e42), (right_anti_dual_g3[1] * this.e23) + (right_anti_dual_g3[3] * this.e43), -(right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g3[2] * this.e43)) - (right_anti_dual_g3.yzxx * float4(this.group1.zxy, this.e41)) - float4((this.group2.xyz * right_anti_dual_g2.www), right_anti_dual_g3[1] * this.e42);
        float anti_wedge_g1 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g1) * other.group3.xyz), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)) - (anti_wedge_g0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_orthogonally_onto(AntiDualNum other) {
        float4 anti_wedge_g2 = float4(other.scalar) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4((float3(other.scalar) * this.group0.xyz), anti_wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2.xyz * float3(other.group0.yy, other.scalar)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_wedge_g2[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g1[2] * this.e31) + (right_anti_dual_g1[3] * this.e41), (right_anti_dual_g1[0] * this.e12) + (right_anti_dual_g1[3] * this.e42), (right_anti_dual_g1[1] * this.e23) + (right_anti_dual_g1[3] * this.e43), -(right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) - (right_anti_dual_g1.yzxx * float4(this.group1.zxy, this.e41));
        float anti_wedge_g1 = (right_anti_dual_g1[0] * this.e15) + (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35) + (right_anti_dual_g1[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e321 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0)), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiLine anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g0[3]) * this.group2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g0[3] * this.scalar) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(anti_wedge_g0, anti_wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g2_w) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.zxyw * float4(anti_wedge_g0.yzx, anti_wedge_g2_w)) + float4((other.group0.xyz * anti_wedge_g1.www), -(anti_wedge_g2_xyz[0] * other.e23) - (anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) - (other.group1.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Dipole anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e31) + (right_anti_dual_g0[3] * this.e41), (right_anti_dual_g0[0] * this.e12) + (right_anti_dual_g0[3] * this.e42), (right_anti_dual_g0[1] * this.e23) + (right_anti_dual_g0[3] * this.e43), -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e41));
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35 */
            (float3((right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45)) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Dipole anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
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
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
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
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint anti_project_orthogonally_onto(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * this.group0.xyz * float3(other.group0.xx, other.e5) * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public FlatPoint anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Flector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Flector anti_project_orthogonally_onto(Motor other) {
        float4 anti_wedge_g0 = float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g0.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4((anti_wedge_g0.zxy * other.group0.yzx * float3(-1.0)), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g0[1] * this.scalar) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45);
        float4 anti_wedge_g1 = float4((right_anti_dual_g9[2] * this.e31) + (right_anti_dual_g9[3] * this.e41), (right_anti_dual_g9[0] * this.e12) + (right_anti_dual_g9[3] * this.e42), (right_anti_dual_g9[1] * this.e23) + (right_anti_dual_g9[3] * this.e43), -(right_anti_dual_g10 * this.e45) - (right_anti_dual_g9[2] * this.e43)) - (right_anti_dual_g9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_anti_dual_g10) * this.group2.xyz), right_anti_dual_g9[1] * this.e42);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[1]) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(right_anti_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(right_anti_dual_g0[1]) * this.group1.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), 0.0) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public Dipole anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e31) + (right_anti_dual_g0[3] * this.e41), (right_anti_dual_g0[0] * this.e12) + (right_anti_dual_g0[3] * this.e42), (right_anti_dual_g0[1] * this.e23) + (right_anti_dual_g0[3] * this.e43), -(right_anti_dual_g1 * this.e45) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_anti_dual_g1) * this.group2.xyz), right_anti_dual_g0[1] * this.e42);
        float anti_wedge_g1 = (right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group0.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * float4(other.group0.zxy, other.e5)), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g1) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_project_orthogonally_onto(Scalar other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, scalar */
            pow(float4(other.scalar), 2) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g3[2] * this.e31) + (right_anti_dual_g3[3] * this.e41), (right_anti_dual_g3[0] * this.e12) + (right_anti_dual_g3[3] * this.e42), (right_anti_dual_g3[1] * this.e23) + (right_anti_dual_g3[3] * this.e43), -(right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g3[2] * this.e43)) - (right_anti_dual_g3.yzxx * float4(this.group1.zxy, this.e41)) - float4((this.group2.xyz * right_anti_dual_g2.www), right_anti_dual_g3[1] * this.e42);
        float anti_wedge_g1 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * float4(other.group3.zxy, other.e5)), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g1) * other.group3.xyz), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)) - (anti_wedge_g0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g0[3]) * this.group2.xyz), (right_anti_dual_g0[3] * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2[3]) * other.group0.xyz)), anti_wedge_g2[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.group0.www, other.e1234)) + float4((other.group2.xyz * anti_wedge_g2.www), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g0.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g2[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g2[3]) * this.group2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.e4) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[3] * this.e1), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[3] * this.e2), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[3] * this.e3), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[2] * this.e435)) + (right_anti_dual_g2.yzxw * float4(this.group0.zxy, this.e5)) - (right_anti_dual_g2.zxyx * float4(this.group0.yzx, this.e415)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g2[1] * this.e425);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g2_w * other.e23) + (anti_wedge_g0[0] * other.scalar) + (anti_wedge_g3[2] * other.e42), (anti_wedge_g2_w * other.e31) + (anti_wedge_g0[1] * other.scalar) + (anti_wedge_g3[0] * other.e43), (anti_wedge_g2_w * other.e12) + (anti_wedge_g0[2] * other.scalar) + (anti_wedge_g3[1] * other.e41), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_xyz[0] * other.e41), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g1[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g1[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g1[2] * other.scalar), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g1[3] * other.scalar) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(right_anti_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g3.yzx) - (this.group0.yzx * right_anti_dual_g3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g2[3] * this.e235) + (right_anti_dual_g3[3] * this.e423), (right_anti_dual_g2[3] * this.e315) + (right_anti_dual_g3[3] * this.e431), (right_anti_dual_g2[3] * this.e125) + (right_anti_dual_g3[3] * this.e412), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = (float4(right_anti_dual_g3[3]) * float4(this.group1.xyz, this.e4)) + (right_anti_dual_g3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (right_anti_dual_g2[3] * this.e5) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((right_anti_dual_g3.yzx * this.group2.zxy), right_anti_dual_g2[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[1] * other.e3) + (anti_wedge_g1[0] * other.e4) + (anti_wedge_g2[3] * other.e423), (anti_wedge_g0[2] * other.e1) + (anti_wedge_g1[1] * other.e4) + (anti_wedge_g2[3] * other.e431), (anti_wedge_g0[0] * other.e2) + (anti_wedge_g1[2] * other.e4) + (anti_wedge_g2[3] * other.e412), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * other.e4) + (anti_wedge_g2[3] * other.e415), (anti_wedge_g2[1] * other.e4) + (anti_wedge_g2[3] * other.e425), (anti_wedge_g2[2] * other.e4) + (anti_wedge_g2[3] * other.e435), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * other.group3.www), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_orthogonally_onto(AntiDualNum other) {
        float4 anti_wedge_g2 = float4(other.scalar) * this.group2;
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.yy, other.scalar, other.e3215) * float4((float3(other.scalar) * this.group0.xyz), anti_wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * float4(anti_wedge_g2.xyz, anti_wedge_g3[3]), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (this.group0.zxy * right_anti_dual_g1.yzx) - (this.group0.yzx * right_anti_dual_g1.zxy);
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g1.www), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - (right_anti_dual_g1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = (float4(right_anti_dual_g1[3]) * float4(this.group1.xyz, this.e4)) + (right_anti_dual_g1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321)) - float4((right_anti_dual_g1.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0.yzx * other.group1.zxy) - (anti_wedge_g0.zxy * other.group1.yzx)), -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * other.group1.www), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group1.yzx) - (anti_wedge_g2.yzx * other.group1.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * anti_wedge_g2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Circle anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) - float4((right_anti_dual_g1.zxy * this.group0.yzx), right_anti_dual_g0[0] * this.e423);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125 */
            (float3(-(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g0.yzx) - (other.group1.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float anti_wedge_g2_w = (right_anti_dual_g0[3] * this.e4) - (right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g0[3] * this.e1), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g0[3] * this.e2), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g0[3] * this.e3), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((this.group0.zxy * right_anti_dual_g1.yzx), right_anti_dual_g0[3] * this.e5) - (right_anti_dual_g1.zxyx * float4(this.group0.yzx, this.e415));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0 * other.group0.www) + (float3(anti_wedge_g2_w) * other.group0.xyz)), (anti_wedge_g2_w * other.e3215) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g3[3]) * other.group0.xyz) + (anti_wedge_g3.yzx * other.group1.zxy) + (float3(right_anti_dual_g0[3]) * float3(other.scalar) * this.group2.xyz) - (anti_wedge_g3.zxy * other.group1.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy);
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = (float4(right_anti_dual_g0[3]) * float4(this.group1.xyz, this.e4)) + (right_anti_dual_g0.zxyx * float4(this.group2.yzx, this.e1)) + float4((right_anti_dual_g0.yzx * this.group2.zxy * float3(-1.0)), (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g0, -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3))) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e5) * anti_wedge_g1.xyz), 0.0) + float4((anti_wedge_g2.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g2.yzx * other.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g2[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Circle anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
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
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Circle anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[2] * this.e412)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g1[1] * this.e431) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g1[0] * this.e423);
        float anti_wedge_g1 = -(right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * this.e425);
        float anti_wedge_g1 = -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g1) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4((anti_wedge_g0.zxy * other.group2.yzx * float3(-1.0)), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Motor anti_project_orthogonally_onto(DualNum other) {
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
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Line anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[1] * this.e412) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[2] * this.e423) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[0] * this.e431) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e415));
        return Line(
            /* e415, e425, e435 */
            float3(other.e45) * anti_wedge_g0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[1] * this.e412) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[2] * this.e423) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[0] * this.e431) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e415));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * anti_wedge_g0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Line anti_project_orthogonally_onto(Line other) {
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
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = float3(right_anti_dual_g1[3]) * this.group0.xyz;
        float anti_wedge_g0_w = (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), (anti_wedge_g0_w * other.e12345) - (anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e5)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g10 * this.e5) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) + (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((right_anti_dual_g7[2] * this.e315) + (right_anti_dual_g8[1] * this.e412) + (right_anti_dual_g6[0] * this.e321) + (right_anti_dual_g6[3] * this.e415), (right_anti_dual_g7[0] * this.e125) + (right_anti_dual_g8[2] * this.e423) + (right_anti_dual_g6[1] * this.e321) + (right_anti_dual_g6[3] * this.e425), (right_anti_dual_g7[1] * this.e235) + (right_anti_dual_g8[0] * this.e431) + (right_anti_dual_g6[2] * this.e321) + (right_anti_dual_g6[3] * this.e435), -(right_anti_dual_g7[0] * this.e415) - (right_anti_dual_g7[1] * this.e425) - (right_anti_dual_g7[2] * this.e435) - (right_anti_dual_g6[2] * this.e412)) + (float4(right_anti_dual_g0[1]) * float4(this.group3.xyz, this.e4)) - float4((right_anti_dual_g7.yzx * this.group2.zxy), right_anti_dual_g6[1] * this.e431) - float4((right_anti_dual_g8.zxy * this.group0.yzx), right_anti_dual_g6[0] * this.e423);
        float anti_wedge_g2 = (right_anti_dual_g0[1] * this.e5) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g9[2] * this.e315) + (right_anti_dual_g9[3] * this.e415), (right_anti_dual_g9[0] * this.e125) + (right_anti_dual_g9[3] * this.e425), (right_anti_dual_g9[1] * this.e235) + (right_anti_dual_g9[3] * this.e435), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) - (right_anti_dual_g9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g10) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g9.yzx) - (this.group0.yzx * right_anti_dual_g9.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g6 = float4(right_anti_dual_g0[1]) * this.group1;
        float3 anti_wedge_g7 = float3(right_anti_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(right_anti_dual_g0[1]) * this.group2.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(right_anti_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1 * this.e235) + (right_anti_dual_g0[3] * this.e423), (right_anti_dual_g1 * this.e315) + (right_anti_dual_g0[3] * this.e431), (right_anti_dual_g1 * this.e125) + (right_anti_dual_g0[3] * this.e412), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = (float4(right_anti_dual_g0[3]) * float4(this.group1.xyz, this.e4)) + (right_anti_dual_g0.zxyx * float4(this.group2.yzx, this.e1)) + float4((right_anti_dual_g0.yzx * this.group2.zxy * float3(-1.0)), (right_anti_dual_g1 * this.e5) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)), anti_wedge_g2[3] * other.e4), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g2[3]) * float4(other.group0.xyz, other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_project_orthogonally_onto(Scalar other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e4 */
            pow(float4(other.scalar), 2) * this.group2, 
            /* e1, e2, e3, e5 */
            pow(float4(other.scalar), 2) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(right_anti_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g3.yzx) - (this.group0.yzx * right_anti_dual_g3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g2[3] * this.e235) + (right_anti_dual_g3[3] * this.e423), (right_anti_dual_g2[3] * this.e315) + (right_anti_dual_g3[3] * this.e431), (right_anti_dual_g2[3] * this.e125) + (right_anti_dual_g3[3] * this.e412), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = (float4(right_anti_dual_g3[3]) * float4(this.group1.xyz, this.e4)) + (right_anti_dual_g3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (right_anti_dual_g2[3] * this.e5) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412)) - float4((right_anti_dual_g3.yzx * this.group2.zxy), right_anti_dual_g2[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g1[0] * other.e4) + (anti_wedge_g2[3] * other.e423), (anti_wedge_g1[1] * other.e4) + (anti_wedge_g2[3] * other.e431), (anti_wedge_g1[2] * other.e4) + (anti_wedge_g2[3] * other.e412), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g0.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e12345) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * other.e4) + (anti_wedge_g2[3] * other.e415), (anti_wedge_g2[1] * other.e4) + (anti_wedge_g2[3] * other.e425), (anti_wedge_g2[2] * other.e4) + (anti_wedge_g2[3] * other.e435), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * other.group2.www), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g0[3]) * this.group2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g0[3] * this.e4) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g0[3] * this.e1) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g0[3] * this.e2) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g0[3] * this.e3) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[2] * this.e435)) + float4((this.group0.zxy * right_anti_dual_g2.yzx), right_anti_dual_g0[3] * this.e5) - (right_anti_dual_g2.zxyx * float4(this.group0.yzx, this.e415)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g2[1] * this.e425);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g3.zxyy * float4(other.group0.yzx, other.e4315)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g0 * other.group0.www), anti_wedge_g2_w * other.e3215) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), anti_wedge_g3[0] * other.e4235) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(this.group0.xx, this.e3215, this.scalar) * float4((other.group0.xyz * float3(-1.0)), right_anti_dual_g2[3]);
        float4 anti_wedge_g1 = float4(this.e3215) * float4((other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, right_anti_dual_g2[3]);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0[3] * other.e15) + (anti_wedge_g1[0] * other.scalar), (anti_wedge_g0[3] * other.e25) + (anti_wedge_g1[1] * other.scalar), (anti_wedge_g0[3] * other.e35) + (anti_wedge_g1[2] * other.scalar), -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e45, other.e45, other.e45, 1.0) * float4(anti_wedge_g0.xyz, -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12))) + (anti_wedge_g1.zxyw * float4(other.group0.yzx, other.scalar)) - (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4((other.group3 * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e4 * -1.0) * float4(-1.0);
        float4 anti_wedge_g1 = float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g1.yzx * other.group3.zxy)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) + float4((other.group3.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * other.e4) - (anti_wedge_g1.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)) + (other.group2.wwwy * float4(anti_wedge_g0.xyz, anti_wedge_g1[1])) + float4(float3(0.0), (anti_wedge_g1[2] * other.e125) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) - float4((anti_wedge_g1.zxy * other.group1.yzx), anti_wedge_g0[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_orthogonally_onto(AntiDualNum other) {
        float2 anti_wedge_g0 = float2(other.scalar) * this.group0.xy;
        return AntiDualNum(
            /* e3215, scalar */
            float2((anti_wedge_g0[0] * other.scalar) + (anti_wedge_g0[1] * other.e3215), anti_wedge_g0[1] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_orthogonally_onto(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * other.e321 * this.e3215, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Flector anti_project_orthogonally_onto(AntiFlector other) {
        float3 anti_wedge_g0_xyz = (other.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(this.group0.xx, this.e3215) * float3(-1.0);
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
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_orthogonally_onto(AntiLine other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_w = right_anti_dual_g0[3] * this.scalar;
        float4 anti_wedge_g1 = right_anti_dual_g0 * float4(this.e3215);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), anti_wedge_g0_w * other.scalar), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_w) * other.group1) + float4(float3(0.0), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_orthogonally_onto(AntiPlane other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(anti_wedge_g0_xyz[0] * other.e1) - (anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_orthogonally_onto(Circle other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.wwwx * float4(other.group0.xyz, other.e235)) + float4((anti_wedge_g0.zxy * other.group1.yzx * float3(-1.0)), (anti_wedge_g0[2] * other.e125) + (anti_wedge_g0[3] * other.e321)) + float4((anti_wedge_g0.yzx * other.group1.zxy), anti_wedge_g0[1] * other.e315), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_orthogonally_onto(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxy * float4(other.group1.zxy, other.e315)) + float4((anti_wedge_g0.zxy * other.group1.yzx * float3(-1.0)), (anti_wedge_g0[2] * other.e125) + (anti_wedge_g0[3] * other.e321)) + float4((other.group0.xyz * anti_wedge_g0.www), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_orthogonally_onto(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e3215) * other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1 = float3(this.e3215) * (other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[0] * other.e45) + (anti_wedge_g1[2] * other.e42), (anti_wedge_g0[1] * other.e45) + (anti_wedge_g1[0] * other.e43), (anti_wedge_g0[2] * other.e45) + (anti_wedge_g1[1] * other.e41), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - float4((anti_wedge_g1.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e1234);
        float3 anti_wedge_g1_xyz = (other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz * float3(this.group0.xx, this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * other.e1234) - (anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[0] * other.e45) + (anti_wedge_g0[3] * other.e4235), (anti_wedge_g0[1] * other.e45) + (anti_wedge_g0[3] * other.e4315), (anti_wedge_g0[2] * other.e45) + (anti_wedge_g0[3] * other.e4125), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), anti_wedge_g0[3] * other.e3215) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234);
        float4 anti_wedge_g1 = float4((other.group7.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0)), 0.0);
        float anti_wedge_g2 = this.e3215 * other.e321;
        float4 anti_wedge_g3 = float4(((float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0)).xyz * float3(this.group0.xx, this.e3215)), 0.0);
        float3 anti_wedge_g5 = float3(this.e3215) * other.group4.xyz * float3(-1.0);
        float4 anti_wedge_g6 = float4(float3(0.0), this.e3215 * other.e4);
        float3 anti_wedge_g8 = float3(this.e3215) * (float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(-1.0);
        float4 anti_wedge_g9 = float4(float3(0.0), right_anti_dual_g0[1] * this.e3215);
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_orthogonally_onto(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4((float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e4 * -1.0) * float4(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_project_orthogonally_onto(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            Simd32x2::pow(float2(other.scalar), 2) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public Sphere anti_project_orthogonally_onto(Sphere other) {
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
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4((float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e4 * -1.0) * float4(-1.0);
        float4 anti_wedge_g1 = float4(this.e3215) * float4((other.group0 * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e321 * -1.0) * float4(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g1.yzx * other.group3.zxy)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(anti_wedge_g1.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) - (anti_wedge_g1.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + (anti_wedge_g1.wwwz * float4(other.group0.xyz, other.e125)) + float4(float3(0.0), (anti_wedge_g1[3] * other.e321) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) + float4((anti_wedge_g0.xyz * other.group3.www), anti_wedge_g1[0] * other.e235) - float4((anti_wedge_g1.zxy * other.group1.yzx), anti_wedge_g0[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(this.group0.xx, this.e3215, (right_anti_dual_g0[3] * this.scalar) + (this.e3215 * other.e1234)) * float4(right_anti_dual_g0.xyz, 1.0);
        float4 anti_wedge_g1 = float4(this.e3215) * float4((other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, right_anti_dual_g0[3]);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            (float4(anti_wedge_g0[3]) * other.group2) + (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g1.xyz, -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43))), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group1.www, other.e3215)) + float4((anti_wedge_g1.zxy * other.group0.yzx), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((other.group3.xyz * anti_wedge_g0.www), anti_wedge_g1[3] * other.scalar) - float4((anti_wedge_g1.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - float4((right_anti_dual_g0.yzx * this.group0.zxy), right_anti_dual_g1[0] * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((other.group0.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * other.scalar) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * other.group2.zxy) - (anti_wedge_g1.zxy * other.group2.yzx)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * other.group2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = (this.group0.xyz * (other.group2 * float4(1.0, 1.0, 1.0, -1.0)).www) - (right_anti_dual_g3.xyz * this.group0.www);
        float anti_wedge_g0_w = (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.zxy * this.group0.yzx) - (right_anti_dual_g3.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g0_xyz, -(anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435) - (anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412))) + float4((float3(anti_wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0_w) * other.group1) + (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g1_xyz, -(anti_wedge_g0_xyz[0] * other.e1) - (anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3))), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e5)) + (float3(anti_wedge_g0_w) * other.group2.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0_w * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(this.e321) * float4(right_anti_dual_g1.xyz, other.e321 * -1.0) * float4(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g1.zxy * this.group0.yzx) - (right_anti_dual_g1.yzx * this.group0.zxy);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g1_xyz[2] * other.e2) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g1_xyz[0] * other.e3) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g1_xyz[1] * other.e1) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group0) - (other.group1.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g0 * float3(this.e321);
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_w * other.e23) + (anti_wedge_g0_xyz[1] * other.e35), (anti_wedge_g0_w * other.e31) + (anti_wedge_g0_xyz[2] * other.e15), (anti_wedge_g0_w * other.e12) + (anti_wedge_g0_xyz[0] * other.e25), -(anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)) - float4((anti_wedge_g0_xyz.zxy * other.group1.yzx), anti_wedge_g0_w * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0.xyz * this.group0.www), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g1[1] * other.e35) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g1[2] * other.e15) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(right_anti_dual_g0[3]) * float4(other.scalar) * this.group0) - (anti_wedge_g1.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e321) * right_anti_dual_g0.xyz * float3(-1.0);
        float3 anti_wedge_g1 = (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g1[2] * other.e2), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g1[0] * other.e3), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g1[1] * other.e1), -(anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3)) - (other.group0.zxyx * float4(anti_wedge_g1.yzx, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_project_orthogonally_onto(Circle other) {
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
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
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - float4((right_anti_dual_g0.yzx * this.group0.zxy), right_anti_dual_g1[0] * this.e235);
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - (this.group0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + (anti_wedge_g0.wwwy * float4(other.group1.xyz, other.e4315)) + float4((anti_wedge_g0.zxy * other.group2.yzx * float3(-1.0)), (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e1234))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((float3(this.e321) * right_anti_dual_g6.xyz), 0.0) + float4((right_anti_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g7.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = -(right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4(((right_anti_dual_g9.zxy * this.group0.yzx) - (right_anti_dual_g9.yzx * this.group0.zxy)), 0.0);
        float3 anti_wedge_g5 = (float3(other.e4 * -1.0) * this.group0.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), right_anti_dual_g0[1] * this.e321);
        float3 anti_wedge_g8 = float3(right_anti_dual_g0[1]) * this.group0.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(other.e4 * -1.0) * this.group0.xyz) - (float3(this.e321) * right_anti_dual_g0.xyz);
        float3 anti_wedge_g1 = (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
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
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_project_orthogonally_onto(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((this.group0.xyz * (float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0)).www), (this.e321 * other.e321) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)) - (this.group0.wwwx * float4(right_anti_dual_g3.xyz, right_anti_dual_g0[0]));
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.zxy * this.group0.yzx) - (right_anti_dual_g3.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.group3.www, other.e12345)) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)) + float4((other.group0.xyz * anti_wedge_g0.www), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0[3]) * other.group1) + (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g1_xyz, -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3))), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0[3]) * this.group0;
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - (this.group0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e1234) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)) - (other.group0.zxyx * float4(anti_wedge_g1.yzx, anti_wedge_g0[0])), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(anti_wedge_g1.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * other.group2.zxy) - (anti_wedge_g1.zxy * other.group2.yzx)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * other.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> operator /(AntiFlector lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g2[3]) * this.group0;
        float4 anti_wedge_g1 = float4((right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g2[3] * this.e1), (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g2[3] * this.e2), (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g2[3] * this.e3), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + float4((right_anti_dual_g0.zxy * this.group0.yzx), right_anti_dual_g2[3] * this.e5) - float4((right_anti_dual_g0.yzx * this.group0.zxy), right_anti_dual_g1[0] * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx)), -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((other.group0.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * other.scalar) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * other.group2.zxy) - (anti_wedge_g1.zxy * other.group2.yzx)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * other.group2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), (right_anti_dual_g2[3] * this.e5) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (other.e321 * this.e321) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((this.group0.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * this.e1) - float4((right_anti_dual_g3.xyz * this.group0.www), other.e423 * this.e235);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.zxy * this.group0.yzx) - (right_anti_dual_g3.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g0.xyz, -(anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435))) + float4((other.group0.xyz * anti_wedge_g0.www), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0[3]) * other.group1) + (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g1_xyz, -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3))), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_orthogonally_onto(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e1, e2, e3, e5 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g1.xyz * this.group0.www * float3(-1.0);
        float anti_wedge_g0_w = (right_anti_dual_g1[0] * this.e1) + (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) + (other.e321 * this.e321);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g1.zxy * this.group0.yzx) - (right_anti_dual_g1.yzx * this.group0.zxy);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_xyz[0] * other.e5) + (anti_wedge_g1_xyz[2] * other.e2), (anti_wedge_g0_xyz[1] * other.e5) + (anti_wedge_g1_xyz[0] * other.e3), (anti_wedge_g0_xyz[2] * other.e5) + (anti_wedge_g1_xyz[1] * other.e1), -(anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3)) + (float4(anti_wedge_g0_w) * other.group0) - (other.group1.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0_xyz[0])), 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0_w) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g0 * float3(this.e321);
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_w * other.e23) + (anti_wedge_g0_xyz[1] * other.e35), (anti_wedge_g0_w * other.e31) + (anti_wedge_g0_xyz[2] * other.e15), (anti_wedge_g0_w * other.e12) + (anti_wedge_g0_xyz[0] * other.e25), -(anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)) - float4((anti_wedge_g0_xyz.zxy * other.group1.yzx), anti_wedge_g0_w * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = (right_anti_dual_g0 * float4(this.group0.www, this.e5)) + float4((this.group1.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g1[1] * other.e35) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g1[2] * other.e15) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g1[0] * other.e25) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(right_anti_dual_g0[3]) * float4(other.scalar) * this.group0) - (anti_wedge_g1.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            anti_wedge_g1 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g0.xyz * this.group0.www * float3(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0_xyz[0] * other.e5) + (anti_wedge_g1_xyz[2] * other.e2), (anti_wedge_g0_xyz[1] * other.e5) + (anti_wedge_g1_xyz[0] * other.e3), (anti_wedge_g0_xyz[2] * other.e5) + (anti_wedge_g1_xyz[1] * other.e1), -(anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3)) - (other.group0.zxyx * float4(anti_wedge_g1_xyz.yzx, anti_wedge_g0_xyz[0])), 
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public Circle anti_project_orthogonally_onto(Circle other) {
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
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
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public Circle anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - float4((right_anti_dual_g0.yzx * this.group0.zxy), right_anti_dual_g1[0] * this.e235);
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - (this.group0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + (anti_wedge_g0.wwwy * float4(other.group1.xyz, other.e4315)) + float4((anti_wedge_g0.zxy * other.group2.yzx * float3(-1.0)), (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e1234))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g10 * this.e5) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((float3(right_anti_dual_g0[1]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g6.xyz), 0.0) + float4((right_anti_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g7.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = (right_anti_dual_g0[1] * this.e5) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4(((right_anti_dual_g9.zxy * this.group0.yzx) - (right_anti_dual_g9.yzx * this.group0.zxy)), 0.0);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g10) * this.group0.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), right_anti_dual_g0[1] * this.e321);
        float3 anti_wedge_g8 = float3(right_anti_dual_g0[1]) * this.group0.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_anti_dual_g0.xyz, (right_anti_dual_g1 * this.e5) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + float4((float3(right_anti_dual_g1) * this.group0.xyz), right_anti_dual_g0[0] * this.e1);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
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
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_project_orthogonally_onto(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e1, e2, e3, e5 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), (right_anti_dual_g2[3] * this.e5) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (this.e321 * other.e321) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)) + float4((this.group0.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * this.e1) - (this.group0.wwwx * float4(right_anti_dual_g3.xyz, right_anti_dual_g0[0]));
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.zxy * this.group0.yzx) - (right_anti_dual_g3.yzx * this.group0.zxy);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.group3.www, other.e12345)) + float4(float3(0.0), -(anti_wedge_g1_xyz[0] * other.e423) - (anti_wedge_g1_xyz[1] * other.e431) - (anti_wedge_g1_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)) + float4((other.group0.xyz * anti_wedge_g0.www), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(anti_wedge_g0[3]) * other.group1) + (float4(other.e4, other.e4, other.e4, 1.0) * float4(anti_wedge_g1_xyz, -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3))), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g0.xyz) + (anti_wedge_g1_xyz.zxy * other.group3.yzx) - (anti_wedge_g1_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiFlector {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0[3]) * this.group0;
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[3] * this.e1) + (right_anti_dual_g1[0] * this.e321), (right_anti_dual_g0[3] * this.e2) + (right_anti_dual_g1[1] * this.e321), (right_anti_dual_g0[3] * this.e3) + (right_anti_dual_g1[2] * this.e321), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + (right_anti_dual_g0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e1234) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43) - (anti_wedge_g0[3] * other.e45)) - (other.group0.zxyx * float4(anti_wedge_g1.yzx, anti_wedge_g0[0])), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(anti_wedge_g1.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz) + (anti_wedge_g1.yzx * other.group2.zxy) - (anti_wedge_g1.zxy * other.group2.yzx)), anti_wedge_g1[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g1.xyz * other.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiLine> operator /(AntiLine lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = float3(right_anti_dual_g2[3]) * this.group0.xyz;
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
        float3 anti_wedge_g1_xyz = this.group1.xyz * right_anti_dual_g2.www;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0_w * other.e15) + (anti_wedge_g1_xyz[0] * other.scalar), (anti_wedge_g0_w * other.e25) + (anti_wedge_g1_xyz[1] * other.scalar), (anti_wedge_g0_w * other.e35) + (anti_wedge_g1_xyz[2] * other.scalar), -(anti_wedge_g0_xyz[0] * other.e41) - (anti_wedge_g0_xyz[1] * other.e42) - (anti_wedge_g0_xyz[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e45, other.e45, other.e45, 1.0) * float4(anti_wedge_g0_xyz, -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12))) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g2[3] * this.e15) - (right_anti_dual_g3[1] * this.e12), -(right_anti_dual_g2[3] * this.e25) - (right_anti_dual_g3[2] * this.e23), -(right_anti_dual_g2[3] * this.e35) - (right_anti_dual_g3[0] * this.e31), (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35)) + (right_anti_dual_g3.zxyx * float4(this.group0.yzx, this.e15));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group3.yzx) - (anti_wedge_g0.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4((other.group3.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) - (anti_wedge_g0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4((anti_wedge_g0.zxy * other.group1.yzx * float3(-1.0)), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_orthogonally_onto(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e15, e25, e35 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiMotor anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g1.zxyx * float4(this.group0.yzx, this.e15)) + float4((this.group0.zxy * right_anti_dual_g1.yzx * float3(-1.0)), (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g0.xyz, (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125) + (anti_wedge_g0[3] * other.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (anti_wedge_g0.wwwx * float4(other.group1.xyz, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiMotor anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        float4 anti_wedge_g1 = float4((this.group1.xyz * right_anti_dual_g0.www), 0.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * other.scalar), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_w) * other.group1) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e15)) + float4((this.group0.zxy * right_anti_dual_g0.yzx * float3(-1.0)), (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35));
        return AntiLine(
            /* e23, e31, e12 */
            (anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g0[3]) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public Dipole anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
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
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
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
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = -(right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12);
        float4 anti_wedge_g1 = float4((this.group0.yzx * right_anti_dual_g9.zxy), 0.0) - float4((float3(other.e4 * -1.0) * this.group1.xyz), 0.0) - float4((this.group0.zxy * right_anti_dual_g9.yzx), 0.0);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35);
        float4 anti_wedge_g3 = float4((this.group1.xyz * float3(right_anti_dual_g0.yy, right_anti_dual_g0[1])), 0.0);
        float3 anti_wedge_g5 = float3(right_anti_dual_g0[1]) * this.group0.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4(float3(0.0), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), 0.0) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public Dipole anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1 * this.e15) - (right_anti_dual_g0[1] * this.e12), -(right_anti_dual_g1 * this.e25) - (right_anti_dual_g0[2] * this.e23), -(right_anti_dual_g1 * this.e35) - (right_anti_dual_g0[0] * this.e31), (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35)) + (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e15));
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e4), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g0[3]) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public AntiLine anti_project_orthogonally_onto(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e15, e25, e35 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g2[3] * this.e15) - (right_anti_dual_g3[1] * this.e12), -(right_anti_dual_g2[3] * this.e25) - (right_anti_dual_g3[2] * this.e23), -(right_anti_dual_g2[3] * this.e35) - (right_anti_dual_g3[0] * this.e31), (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35)) + (right_anti_dual_g3.zxyx * float4(this.group0.yzx, this.e15));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group3.yzx) - (anti_wedge_g0.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4((other.group3.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)) - (anti_wedge_g0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + (anti_wedge_g0.wwwy * float4(other.group0.xyz, other.e315)) + float4((anti_wedge_g0.zxy * other.group1.yzx * float3(-1.0)), (anti_wedge_g0[2] * other.e125) + (anti_wedge_g0[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiLine {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12));
        float3 anti_wedge_g1_xyz = this.group1.xyz * right_anti_dual_g0.www;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            (float4(anti_wedge_g0[3]) * other.group2) + (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g1_xyz, -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43))), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group1.www, other.e3215)) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((other.group3.xyz * anti_wedge_g0.www), 0.0) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> operator /(AntiMotor lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = (right_anti_dual_g0 * this.group1.www) + (this.group0.xyz * right_anti_dual_g2.www);
        float anti_wedge_g0_w = (right_anti_dual_g2[3] * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
        float3 anti_wedge_g1_xyz = (float3(right_anti_dual_g2[3]) * this.group1.xyz) + (float3(this.e3215) * right_anti_dual_g1.xyz);
        float anti_wedge_g1_w = right_anti_dual_g2[3] * this.e3215;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0_w) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e45), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g0_w * other.e15) + (anti_wedge_g1_xyz[0] * other.scalar), (anti_wedge_g0_w * other.e25) + (anti_wedge_g1_xyz[1] * other.scalar), (anti_wedge_g0_w * other.e35) + (anti_wedge_g1_xyz[2] * other.scalar), -(anti_wedge_g0_xyz[0] * other.e41) - (anti_wedge_g0_xyz[1] * other.e42) - (anti_wedge_g0_xyz[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e45, other.e45, other.e45, 1.0) * float4(anti_wedge_g0_xyz, -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12))) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), anti_wedge_g1_w * other.scalar) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g1_w * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(this.e3215) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3]) * float4(-1.0);
        float4 anti_wedge_g1 = float4(-(right_anti_dual_g2[3] * this.e15) - (right_anti_dual_g3[1] * this.e12), -(right_anti_dual_g2[3] * this.e25) - (right_anti_dual_g3[2] * this.e23), -(right_anti_dual_g2[3] * this.e35) - (right_anti_dual_g3[0] * this.e31), (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35)) + (right_anti_dual_g3.zxyx * float4(this.group0.yzx, this.e15)) - float4((other.group0.xyz * this.group1.www), other.e321 * this.e3215 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g1.yzx * other.group3.zxy)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) + float4((other.group3.xyz * anti_wedge_g1.www), anti_wedge_g0[3] * other.e4) - (anti_wedge_g1.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g1[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)) + (other.group2.wwwy * float4(anti_wedge_g0.xyz, anti_wedge_g1[1])) + float4(float3(0.0), (anti_wedge_g1[2] * other.e125) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) - float4((anti_wedge_g1.zxy * other.group1.yzx), anti_wedge_g0[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_orthogonally_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g1 = float4(other.scalar) * this.group1;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e15, e25, e35, e3215 */
            float4(other.group0.yy, other.scalar, (anti_wedge_g0[3] * other.e3215) + (anti_wedge_g1[3] * other.scalar)) * float4(anti_wedge_g1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiDualNum anti_project_orthogonally_onto(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * other.e321 * this.e3215, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g1.xyz * this.group1.www * float3(-1.0);
        float4 anti_wedge_g1 = (right_anti_dual_g1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35)) - float4((right_anti_dual_g1.yzx * this.group0.zxy), other.e321 * this.e3215 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1.wwwx * float4(other.group1.xyz, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) + (anti_wedge_g1[3] * other.e321) - (anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3)) - (other.group1.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        float3 anti_wedge_g1_xyz = right_anti_dual_g0 * this.group1.www;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float anti_wedge_g0_w = (right_anti_dual_g0[3] * this.scalar) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        float4 anti_wedge_g1 = float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.e3215) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e3215);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * other.scalar), 
            /* e15, e25, e35, e3215 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_w) * other.group1) + float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g0.xyz * this.group1.www * float3(-1.0);
        float4 anti_wedge_g1 = (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e15)) + float4((right_anti_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((other.group0.xyz * anti_wedge_g1.www), -(anti_wedge_g0_xyz[1] * other.e2) - (anti_wedge_g0_xyz[2] * other.e3)) - (other.group0.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public Sphere anti_project_orthogonally_onto(Circle other) {
        float3 anti_wedge_g0_xyz = other.group0.xyz * this.group1.www * float3(-1.0);
        float anti_wedge_g0_w = this.e3215 * other.e321;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_xyz, anti_wedge_g0_w).wwwx * float4(other.group0.xyz, other.e235)) + float4((anti_wedge_g0_xyz.zxy * other.group1.yzx * float3(-1.0)), (anti_wedge_g0_w * other.e321) + (anti_wedge_g0_xyz[2] * other.e125)) + float4((anti_wedge_g0_xyz.yzx * other.group1.zxy), anti_wedge_g0_xyz[1] * other.e315), 
            /* e1234 */
            -(anti_wedge_g0_xyz[0] * other.e423) - (anti_wedge_g0_xyz[1] * other.e431) - (anti_wedge_g0_xyz[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public Sphere anti_project_orthogonally_onto(CircleRotor other) {
        float4 anti_wedge_g0 = float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxy * float4(other.group1.zxy, other.e315)) + float4((anti_wedge_g0.zxy * other.group1.yzx * float3(-1.0)), (anti_wedge_g0[2] * other.e125) + (anti_wedge_g0[3] * other.e321)) + float4((other.group0.xyz * anti_wedge_g0.www), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g0 * float3(this.e3215);
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
        float3 anti_wedge_g1_xyz = right_anti_dual_g1.xyz * this.group1.www;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0_w) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0_w) * other.group2.xyz), -(anti_wedge_g0_xyz[0] * other.e41) - (anti_wedge_g0_xyz[1] * other.e42) - (anti_wedge_g0_xyz[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz[0] * other.e45) + (anti_wedge_g1_xyz[2] * other.e42), (anti_wedge_g0_xyz[1] * other.e45) + (anti_wedge_g1_xyz[0] * other.e43), (anti_wedge_g0_xyz[2] * other.e45) + (anti_wedge_g1_xyz[1] * other.e41), -(anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g0 * float3(this.e3215);
        float anti_wedge_g0_w = (this.e3215 * other.e1234) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12);
        float3 anti_wedge_g1_xyz = right_anti_dual_g1.xyz * this.group1.www;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g0_w) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g0_w) * other.group2.xyz), (anti_wedge_g0_w * other.e1234) - (anti_wedge_g0_xyz[0] * other.e41) - (anti_wedge_g0_xyz[1] * other.e42) - (anti_wedge_g0_xyz[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_w * other.e4235) + (anti_wedge_g0_xyz[0] * other.e45), (anti_wedge_g0_w * other.e4315) + (anti_wedge_g0_xyz[1] * other.e45), (anti_wedge_g0_w * other.e4125) + (anti_wedge_g0_xyz[2] * other.e45), -(anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), anti_wedge_g0_w * other.e3215) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0_xyz[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12);
        float4 anti_wedge_g1 = float4((right_anti_dual_g9.zxy * this.group0.yzx), 0.0) - float4((float3(right_anti_dual_g10) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_anti_dual_g9.yzx * this.group0.zxy), 0.0);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (this.e3215 * other.e321);
        float4 anti_wedge_g3 = float4(((float3(right_anti_dual_g0[1]) * this.group1.xyz) + (float3(this.e3215) * right_anti_dual_g6.xyz)), 0.0);
        float3 anti_wedge_g5 = (right_anti_dual_g7 * float3(this.e3215)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz);
        float4 anti_wedge_g6 = float4(float3(0.0), right_anti_dual_g10 * this.e3215) * float4(0.0, 0.0, 0.0, -1.0);
        float3 anti_wedge_g8 = float3(this.e3215) * right_anti_dual_g9.xyz * float3(-1.0);
        float4 anti_wedge_g9 = float4(float3(0.0), right_anti_dual_g0[1] * this.e3215);
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(float3(0.0), anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = float4(this.e3215) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1) * float4(-1.0);
        float4 anti_wedge_g1 = float4(-(right_anti_dual_g1 * this.e15) - (right_anti_dual_g0[1] * this.e12), -(right_anti_dual_g1 * this.e25) - (right_anti_dual_g0[2] * this.e23), -(right_anti_dual_g1 * this.e35) - (right_anti_dual_g0[0] * this.e31), (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35)) + (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e15));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g1.xyz)), anti_wedge_g0[3] * other.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e1) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_project_orthogonally_onto(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e15, e25, e35, e3215 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public Sphere anti_project_orthogonally_onto(Sphere other) {
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
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(this.e3215) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3]) * float4(-1.0);
        float4 anti_wedge_g1 = float4(-(right_anti_dual_g0[0] * this.e3215) - (right_anti_dual_g2[3] * this.e15), -(right_anti_dual_g0[1] * this.e3215) - (right_anti_dual_g2[3] * this.e25), -(right_anti_dual_g0[2] * this.e3215) - (right_anti_dual_g2[3] * this.e35), (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35)) + (right_anti_dual_g3.zxyx * float4(this.group0.yzx, this.e15)) - float4((right_anti_dual_g3.yzx * this.group0.zxy), this.e3215 * other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4) * anti_wedge_g1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g1.yzx * other.group3.zxy)), anti_wedge_g1[3] * other.e4), 
            /* e15, e25, e35, e1234 */
            (other.group3 * float4(anti_wedge_g1.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412)) - (anti_wedge_g1.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + (anti_wedge_g1.wwwz * float4(other.group0.xyz, other.e125)) + float4(float3(0.0), (anti_wedge_g1[3] * other.e321) - (anti_wedge_g0[1] * other.e2) - (anti_wedge_g0[2] * other.e3) - (anti_wedge_g0[3] * other.e5)) + float4((anti_wedge_g0.xyz * other.group3.www), anti_wedge_g1[0] * other.e235) - float4((anti_wedge_g1.zxy * other.group1.yzx), anti_wedge_g0[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (float4(right_anti_dual_g0[3]) * this.group0) + (float4(this.e3215) * float4(right_anti_dual_g0.xyz, other.e1234)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12));
        float3 anti_wedge_g1_xyz = (float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.e3215) * right_anti_dual_g1.xyz);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e45), 
            /* e15, e25, e35, e1234 */
            (float4(anti_wedge_g0[3]) * other.group2) + (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g1_xyz, -(anti_wedge_g0[0] * other.e41) - (anti_wedge_g0[1] * other.e42) - (anti_wedge_g0[2] * other.e43))), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group1.www, other.e3215)) + float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35)) + float4((other.group3.xyz * anti_wedge_g0.www), right_anti_dual_g0[3] * this.e3215 * other.scalar) - float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> operator /(AntiPlane lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_project_orthogonally_onto(AntiCircleRotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e4 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), 0.0) + float4((anti_wedge_g0.yzx * other.group2.zxy), 0.0) - float4((anti_wedge_g0.zxy * other.group2.yzx), 0.0), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) - (other.e4 * this.e5);
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
public AntiPlane operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_project_orthogonally_onto(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g1[0] * this.e1) + (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector anti_project_orthogonally_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g0[3] * other.e23), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g0[3] * other.e31), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g0[3] * other.e12), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) - (this.e5 * other.e4);
        float4 anti_wedge_g1 = float4((this.group0.xyz * float3(right_anti_dual_g0.yy, right_anti_dual_g0[1])), 0.0);
        float anti_wedge_g2 = right_anti_dual_g0[1] * this.e5;
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
            (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group6) - float4((anti_wedge_g1.xyz * other.group3.www), 0.0), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) - (anti_wedge_g1.zxy * other.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g2 * other.e321) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4(float3(0.0), anti_wedge_g1[1] * other.e315) + float4(float3(0.0), anti_wedge_g1[2] * other.e125) + float4(((float3(anti_wedge_g2) * other.group7.xyz) + (anti_wedge_g1.yzx * other.group6.zxy)), 0.0) - float4((other.group8.xyz * anti_wedge_g1.www), 0.0) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group0, 
            /* e5 */
            anti_wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_project_orthogonally_onto(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) - (this.e5 * other.e4);
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
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiPlane {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.zxyx * float4(other.group0.yzx, other.e4235)) + float4((anti_wedge_g0.yzx * other.group0.zxy * float3(-1.0)), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e1234)), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * anti_wedge_g0.www), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (anti_wedge_g0.yzx * other.group2.zxy) - (anti_wedge_g0.zxy * other.group2.yzx)), anti_wedge_g0[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((anti_wedge_g0.xyz * other.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            anti_wedge_g0 * float3(other.scalar), 
            /* e415, e425, e435, e321 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g2.xyz * other.group2.www), (anti_wedge_g2[3] * other.scalar) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group3.zxy) - (anti_wedge_g0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (other.group3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g2.zxy * other.group3.yzx), anti_wedge_g3[3] * other.e4) - (anti_wedge_g2.yzxx * float4(other.group3.zxy, other.e423))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public DualNum anti_project_orthogonally_onto(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * float2(this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * other.e321 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_orthogonally_onto(AntiFlector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g0[3] * other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_orthogonally_onto(AntiMotor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0.xyz * other.group0.www), (anti_wedge_g0[3] * other.scalar) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Circle other) {
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2[3] * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2[3] * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((anti_wedge_g3.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public DualNum anti_project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345 * other.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * other.e45 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (this.e12345 * pow(other.e45, 2))), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 anti_wedge_g0 = float2(this.e12345) * other.group0.yx * float2(-1.0, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g2 = this.e12345 * other.e3215;
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group8.xyz, other.e321 * -1.0);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group6.xyz;
        float4 anti_wedge_g6 = float4(this.e12345) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g7 = float3(this.e12345) * other.group4.xyz * float3(-1.0);
        float3 anti_wedge_g8 = float3(this.e12345) * other.group3.xyz * float3(-1.0);
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0[0]) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g2) * other.group5.xyz) + (float3(anti_wedge_g0[0]) * other.group8.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0[0]) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g10 * other.scalar) + (anti_wedge_g0[0] * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(RoundPoint other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g0[3] * other.e4) - (this.e12345 * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * other.scalar * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_project_orthogonally_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (other.group3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g0.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e5) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension AntiScalar {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e3215) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((anti_wedge_g3.zxy * other.group0.yzx), anti_wedge_g2[3] * other.e1234) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * other.e41) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g2[3] * other.e42) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g2[3] * other.e43) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<Circle> operator /(Circle lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g2[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g2[3]) * this.group2.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431), -(right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g1[1] * this.e315) - float4((this.group0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[0] * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g2_w * other.e23) + (anti_wedge_g0[0] * other.scalar) + (anti_wedge_g3[2] * other.e42), (anti_wedge_g2_w * other.e31) + (anti_wedge_g0[1] * other.scalar) + (anti_wedge_g3[0] * other.e43), (anti_wedge_g2_w * other.e12) + (anti_wedge_g0[2] * other.scalar) + (anti_wedge_g3[1] * other.e41), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_xyz[0] * other.e41), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g1[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g1[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g1[2] * other.scalar), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g1[3] * other.scalar) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(right_anti_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g3.yzx) - (this.group0.yzx * right_anti_dual_g3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g2[3] * this.e235) + (right_anti_dual_g3[3] * this.e423), (right_anti_dual_g2[3] * this.e315) + (right_anti_dual_g3[3] * this.e431), (right_anti_dual_g2[3] * this.e125) + (right_anti_dual_g3[3] * this.e412), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((right_anti_dual_g3[2] * this.e315) + (right_anti_dual_g3[3] * this.e415), (right_anti_dual_g3[0] * this.e125) + (right_anti_dual_g3[3] * this.e425), (right_anti_dual_g3[1] * this.e235) + (right_anti_dual_g3[3] * this.e435), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((this.group2.zxy * right_anti_dual_g3.yzx), right_anti_dual_g2[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[1] * other.e3) + (anti_wedge_g1[0] * other.e4) + (anti_wedge_g2[3] * other.e423), (anti_wedge_g0[2] * other.e1) + (anti_wedge_g1[1] * other.e4) + (anti_wedge_g2[3] * other.e431), (anti_wedge_g0[0] * other.e2) + (anti_wedge_g1[2] * other.e4) + (anti_wedge_g2[3] * other.e412), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * other.e4) + (anti_wedge_g2[3] * other.e415), (anti_wedge_g2[1] * other.e4) + (anti_wedge_g2[3] * other.e425), (anti_wedge_g2[2] * other.e4) + (anti_wedge_g2[3] * other.e435), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * other.group3.www), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle anti_project_orthogonally_onto(AntiDualNum other) {
        return Circle(
            /* e423, e431, e412 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (this.group0.zxy * right_anti_dual_g1.yzx) - (this.group0.yzx * right_anti_dual_g1.zxy);
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g1.www), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - (right_anti_dual_g1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((right_anti_dual_g1[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g1[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g1[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321)) - float4((this.group2.zxy * right_anti_dual_g1.yzx), right_anti_dual_g0[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0.yzx * other.group1.zxy) - (anti_wedge_g0.zxy * other.group1.yzx)), -(anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * other.group1.www), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group1.yzx) - (anti_wedge_g2.yzx * other.group1.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * anti_wedge_g2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) - float4((right_anti_dual_g1.zxy * this.group0.yzx), right_anti_dual_g0[0] * this.e423);
        return Circle(
            /* e423, e431, e412 */
            float3(anti_wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125 */
            (float3(-(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g0.yzx) - (other.group1.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - float4((this.group0.yzx * right_anti_dual_g1.zxy), right_anti_dual_g0[0] * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((anti_wedge_g0 * other.group0.www) + (float3(anti_wedge_g2_w) * other.group0.xyz)), (anti_wedge_g2_w * other.e3215) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g3[3]) * other.group0.xyz) + (anti_wedge_g3.yzx * other.group1.zxy) + (float3(right_anti_dual_g0[3]) * float3(other.scalar) * this.group2.xyz) - (anti_wedge_g3.zxy * other.group1.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy);
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2 = (float3(right_anti_dual_g0[3]) * this.group1.xyz) + (this.group2.yzx * right_anti_dual_g0.zxy) - (this.group2.zxy * right_anti_dual_g0.yzx);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g0, -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3))) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
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
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e415) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g0[1] * this.e425);
        float anti_wedge_g1 = -(right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * this.e425);
        float anti_wedge_g1 = -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g1) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4((anti_wedge_g0.zxy * other.group2.yzx * float3(-1.0)), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint anti_project_orthogonally_onto(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * this.group0.xyz * float3(other.group0.xx, other.e5)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Line anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[1] * this.e412) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[2] * this.e423) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[0] * this.e431) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - float4((this.group0.yzx * right_anti_dual_g0.zxy), right_anti_dual_g0[0] * this.e415);
        return Line(
            /* e415, e425, e435 */
            float3(other.e45) * anti_wedge_g0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[1] * this.e412) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[2] * this.e423) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[0] * this.e431) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - float4((this.group0.yzx * right_anti_dual_g0.zxy), right_anti_dual_g0[0] * this.e415);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * anti_wedge_g0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Line anti_project_orthogonally_onto(Line other) {
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
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = float3(right_anti_dual_g1[3]) * this.group0.xyz;
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), (anti_wedge_g0_w * other.e12345) - (anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e5)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((right_anti_dual_g7[2] * this.e315) + (right_anti_dual_g8[1] * this.e412) + (right_anti_dual_g6[0] * this.e321) + (right_anti_dual_g6[3] * this.e415), (right_anti_dual_g7[0] * this.e125) + (right_anti_dual_g8[2] * this.e423) + (right_anti_dual_g6[1] * this.e321) + (right_anti_dual_g6[3] * this.e425), (right_anti_dual_g7[1] * this.e235) + (right_anti_dual_g8[0] * this.e431) + (right_anti_dual_g6[2] * this.e321) + (right_anti_dual_g6[3] * this.e435), -(right_anti_dual_g7[0] * this.e415) - (right_anti_dual_g7[1] * this.e425) - (right_anti_dual_g7[2] * this.e435) - (right_anti_dual_g6[2] * this.e412)) - float4((right_anti_dual_g7.yzx * this.group2.zxy), right_anti_dual_g6[1] * this.e431) - float4((right_anti_dual_g8.zxy * this.group0.yzx), right_anti_dual_g6[0] * this.e423);
        float anti_wedge_g2 = -(right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g9[2] * this.e315) + (right_anti_dual_g9[3] * this.e415), (right_anti_dual_g9[0] * this.e125) + (right_anti_dual_g9[3] * this.e425), (right_anti_dual_g9[1] * this.e235) + (right_anti_dual_g9[3] * this.e435), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) - (right_anti_dual_g9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g10) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g9.yzx) - (this.group0.yzx * right_anti_dual_g9.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g6 = float4(right_anti_dual_g0[1]) * this.group1;
        float3 anti_wedge_g7 = float3(right_anti_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g8 = float3(right_anti_dual_g0[1]) * this.group2.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(right_anti_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1 * this.e235) + (right_anti_dual_g0[3] * this.e423), (right_anti_dual_g1 * this.e315) + (right_anti_dual_g0[3] * this.e431), (right_anti_dual_g1 * this.e125) + (right_anti_dual_g0[3] * this.e412), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2 = (float3(right_anti_dual_g0[3]) * this.group1.xyz) + (this.group2.yzx * right_anti_dual_g0.zxy) - (this.group2.zxy * right_anti_dual_g0.yzx);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public Circle anti_project_orthogonally_onto(Scalar other) {
        return Circle(
            /* e423, e431, e412 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(right_anti_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g3.yzx) - (this.group0.yzx * right_anti_dual_g3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g2[3] * this.e235) + (right_anti_dual_g3[3] * this.e423), (right_anti_dual_g2[3] * this.e315) + (right_anti_dual_g3[3] * this.e431), (right_anti_dual_g2[3] * this.e125) + (right_anti_dual_g3[3] * this.e412), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((right_anti_dual_g3[2] * this.e315) + (right_anti_dual_g3[3] * this.e415), (right_anti_dual_g3[0] * this.e125) + (right_anti_dual_g3[3] * this.e425), (right_anti_dual_g3[1] * this.e235) + (right_anti_dual_g3[3] * this.e435), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412)) - float4((this.group2.zxy * right_anti_dual_g3.yzx), right_anti_dual_g2[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g1[0] * other.e4) + (anti_wedge_g2[3] * other.e423), (anti_wedge_g1[1] * other.e4) + (anti_wedge_g2[3] * other.e431), (anti_wedge_g1[2] * other.e4) + (anti_wedge_g2[3] * other.e412), -(anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g0.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e12345) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * other.e4) + (anti_wedge_g2[3] * other.e415), (anti_wedge_g2[1] * other.e4) + (anti_wedge_g2[3] * other.e425), (anti_wedge_g2[2] * other.e4) + (anti_wedge_g2[3] * other.e435), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0 * other.group2.www), anti_wedge_g2[3] * other.e321) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Circle {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g0[3]) * this.group2.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431), -(right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) - float4((this.group0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[0] * this.e235) - float4((this.group2.zxy * right_anti_dual_g0.yzx), right_anti_dual_g1[1] * this.e315);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g3.zxyy * float4(other.group0.yzx, other.e4315)) + float4(float3(0.0), (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g0 * other.group0.www), anti_wedge_g2_w * other.e3215) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), anti_wedge_g3[0] * other.e4235) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> operator /(CircleRotor lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = (right_anti_dual_g0 * float3(this.e12345)) + (float3(right_anti_dual_g2[3]) * this.group0.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g2[3]) * this.group1);
        float anti_wedge_g2_x = (right_anti_dual_g2[0] * this.e12345) + (right_anti_dual_g2[3] * this.e235);
        float anti_wedge_g2_y = (right_anti_dual_g2[1] * this.e12345) + (right_anti_dual_g2[3] * this.e315);
        float anti_wedge_g2_z = (right_anti_dual_g2[2] * this.e12345) + (right_anti_dual_g2[3] * this.e125);
        float anti_wedge_g2_w = -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * this.e425);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0_xyz[0] * other.scalar) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g0_xyz[1] * other.scalar) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g0_xyz[2] * other.scalar) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g2_y * other.e42) - (anti_wedge_g2_z * other.e43) - (anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((other.group0.yzx * anti_wedge_g3.zxy), right_anti_dual_g2[3] * other.scalar * this.e12345) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_x * other.e41), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1[0] * other.scalar) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g1[1] * other.scalar) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g1[2] * other.scalar) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((float3(anti_wedge_g2_w) * other.group0.xyz), anti_wedge_g1[3] * other.scalar) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2_w) * other.group1.xyz) + (float3(other.scalar) * float3(anti_wedge_g2_x, anti_wedge_g2_y, anti_wedge_g2_z)) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2_w * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g2[3] * this.e415) + (right_anti_dual_g3[1] * this.e412) + (other.e423 * this.e12345), (right_anti_dual_g2[3] * this.e425) + (right_anti_dual_g3[2] * this.e423) + (other.e431 * this.e12345), (right_anti_dual_g2[3] * this.e435) + (right_anti_dual_g3[0] * this.e431) + (other.e412 * this.e12345), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((this.group0.yzx * right_anti_dual_g3.zxy), right_anti_dual_g2[0] * this.e423);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1[0] * this.e12345) + (right_anti_dual_g2[3] * this.e235), (right_anti_dual_g1[1] * this.e12345) + (right_anti_dual_g2[3] * this.e315), (right_anti_dual_g1[2] * this.e12345) + (right_anti_dual_g2[3] * this.e125), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((this.group0.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * this.e12345) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * this.group2.yzx) - (right_anti_dual_g3.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group3.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * other.e3) + (right_anti_dual_g2[3] * other.e5 * this.e12345) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((other.group0.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.e4) + float4((anti_wedge_g1.xyz * other.group2.www), anti_wedge_g3[0] * other.e1) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_orthogonally_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.scalar) * float4(this.group0.xyz, this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.yy, other.scalar, anti_wedge_g0[3] * other.scalar) * float4(anti_wedge_g0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group2 * float4(other.group0.yy, other.scalar, other.e3215), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * other.e321), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group0.xyz), right_anti_dual_g0[3] * other.e321 * this.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(((this.group0.zxy * right_anti_dual_g1.yzx) - (this.group0.yzx * right_anti_dual_g1.zxy)), -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321));
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((this.group0.xyz * right_anti_dual_g1.www), right_anti_dual_g0[3] * this.e12345) - (right_anti_dual_g1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * right_anti_dual_g0.xyz), 0.0) + float4((right_anti_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g1.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_anti_dual_g1 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group1.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)) - (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group1.yzx) - (anti_wedge_g2.yzx * other.group1.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * anti_wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = right_anti_dual_g0 * this.group2.www;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - float4((right_anti_dual_g1.zxy * this.group0.yzx), right_anti_dual_g0[0] * this.e235);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e35) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g3[2] * other.e15) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g3[0] * other.e25) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) - float4((other.group1.yzx * anti_wedge_g3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0[3]) * float4(this.group0.xyz, this.e12345);
        float4 anti_wedge_g1 = float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e321);
        float4 anti_wedge_g2 = (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_anti_dual_g1.xyz, -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435))) + (this.group2 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3]));
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) - float4((this.group0.yzx * right_anti_dual_g1.zxy), right_anti_dual_g0[0] * this.e423);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0 * float4(anti_wedge_g3.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g0.xyz * other.group0.www), anti_wedge_g3[3] * other.e3215), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4((other.group1.xyz * anti_wedge_g3.www), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group1.zxy) - (anti_wedge_g3.zxy * other.group1.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy);
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g0.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_anti_dual_g0 * float4(this.e12345);
        return CircleRotor(
            /* e423, e431, e412 */
            (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g0, -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3))) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            (other.group0.yzxx * float4(anti_wedge_g2.zxy, anti_wedge_g3[0])) + (other.group0.wwwy * float4(anti_wedge_g1.xyz, anti_wedge_g3[1])) + float4((anti_wedge_g2.yzx * other.group0.zxy * float3(-1.0)), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * right_anti_dual_g2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group2.xyz), (anti_wedge_g2_w * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_anti_dual_g2 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g2[0] * this.e415) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g2[1] * this.e425);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e35) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g3[2] * other.e15) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g3[0] * other.e25) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - float4((other.group2.yzx * anti_wedge_g3.zxy), anti_wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * right_anti_dual_g2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g3[0] * this.e12345), (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g3[1] * this.e12345), (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g3[2] * this.e12345), -(right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) + (this.group2.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[3])) - (this.group2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((this.group0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[1] * this.e315);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g3.yzx * other.group2.zxy), anti_wedge_g2_w * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_orthogonally_onto(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0.xx, right_anti_dual_g0[0], right_anti_dual_g0[1]) * float4(this.group0.xyz, this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[1] * this.e412) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[2] * this.e423) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[0] * this.e431) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - float4((this.group0.yzx * right_anti_dual_g0.zxy), right_anti_dual_g0[0] * this.e415);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45) * float4(anti_wedge_g1.xyz, right_anti_dual_g0[3] * this.e12345) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[3] * this.e415) + (right_anti_dual_g1[0] * this.e12345), (right_anti_dual_g0[3] * this.e425) + (right_anti_dual_g1[1] * this.e12345), (right_anti_dual_g0[3] * this.e435) + (right_anti_dual_g1[2] * this.e12345), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((this.group0.zxy * right_anti_dual_g0.yzx), right_anti_dual_g1[3] * this.e12345) - float4((this.group0.yzx * right_anti_dual_g0.zxy), right_anti_dual_g0[0] * this.e415);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (right_anti_dual_g0[3] * this.e12345 * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_orthogonally_onto(Line other) {
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
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_anti_dual_g0.xyz, -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412))) + float4((this.group0.xyz * right_anti_dual_g1.www), right_anti_dual_g0[3] * this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g0[0] * this.e12345) - (right_anti_dual_g3_w * this.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((right_anti_dual_g7[2] * this.e315) + (right_anti_dual_g1[0] * this.e12345) + (right_anti_dual_g6[0] * this.e321) + (right_anti_dual_g6[3] * this.e415), (right_anti_dual_g7[0] * this.e125) + (right_anti_dual_g1[1] * this.e12345) + (right_anti_dual_g6[1] * this.e321) + (right_anti_dual_g6[3] * this.e425), (right_anti_dual_g7[1] * this.e235) + (right_anti_dual_g1[2] * this.e12345) + (right_anti_dual_g6[2] * this.e321) + (right_anti_dual_g6[3] * this.e435), -(right_anti_dual_g7[0] * this.e415) - (right_anti_dual_g7[1] * this.e425) - (right_anti_dual_g7[2] * this.e435) - (right_anti_dual_g6[2] * this.e412)) + float4((right_anti_dual_g8.yzx * this.group0.zxy), right_anti_dual_g1[3] * this.e12345) - float4((right_anti_dual_g7.yzx * this.group2.zxy), right_anti_dual_g6[1] * this.e431) - float4((right_anti_dual_g8.zxy * this.group0.yzx), right_anti_dual_g6[0] * this.e423);
        float anti_wedge_g2 = (this.e12345 * other.e3215) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g9[2] * this.e315) + (this.e12345 * other.e235), (right_anti_dual_g9[0] * this.e125) + (this.e12345 * other.e315), (right_anti_dual_g9[1] * this.e235) + (this.e12345 * other.e125), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) + float4((this.group1.xyz * right_anti_dual_g9.www), right_anti_dual_g3_w * this.e12345) - (right_anti_dual_g9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + (this.group0.zxy * right_anti_dual_g9.yzx) - (this.group0.yzx * right_anti_dual_g9.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9[3]) * this.group0.xyz) + (float3(this.e12345) * other.group6.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g6 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group1);
        float3 anti_wedge_g7 = (right_anti_dual_g7 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz);
        float3 anti_wedge_g8 = (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group2.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g9 * float4(this.e12345);
        float anti_wedge_g10 = right_anti_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g0[1] * this.e12345 * other.scalar) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(right_anti_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1 * this.e235) + (right_anti_dual_g0[3] * this.e423), (right_anti_dual_g1 * this.e315) + (right_anti_dual_g0[3] * this.e431), (right_anti_dual_g1 * this.e125) + (right_anti_dual_g0[3] * this.e412), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g0[3]) * this.group1.xyz) + (right_anti_dual_g0.zxy * this.group2.yzx) - (right_anti_dual_g0.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g0 * float4(this.e12345);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * anti_wedge_g1.xyz) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2_xyz[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2_xyz[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2_xyz[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(anti_wedge_g1.xyz, right_anti_dual_g1 * this.e12345)) + (other.group0.yzxx * float4(anti_wedge_g2_xyz.zxy, anti_wedge_g3[0])) + float4((anti_wedge_g2_xyz.yzx * other.group0.zxy * float3(-1.0)), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_project_orthogonally_onto(Scalar other) {
        return CircleRotor(
            /* e423, e431, e412 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e12345 */
            pow(float4(other.scalar), 2) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public AntiScalar anti_project_orthogonally_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g2[3] * this.e415), (right_anti_dual_g0[1] * this.e12345) + (right_anti_dual_g2[3] * this.e425), (right_anti_dual_g0[2] * this.e12345) + (right_anti_dual_g2[3] * this.e435), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412)) + float4((this.group0.zxy * right_anti_dual_g3.yzx), right_anti_dual_g0[3] * this.e12345) - float4((this.group0.yzx * right_anti_dual_g3.zxy), right_anti_dual_g2[0] * this.e423);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1[0] * this.e12345) + (right_anti_dual_g2[3] * this.e235), (right_anti_dual_g1[1] * this.e12345) + (right_anti_dual_g2[3] * this.e315), (right_anti_dual_g1[2] * this.e12345) + (right_anti_dual_g2[3] * this.e125), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((this.group0.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * this.e12345) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * this.group2.yzx) - (right_anti_dual_g3.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (other.group3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((anti_wedge_g0.yzx * other.group3.zxy), right_anti_dual_g2[3] * this.e12345 * other.e5) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension CircleRotor {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e12345);
        float4 anti_wedge_g1 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group1);
        float4 anti_wedge_g2 = (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_anti_dual_g2.xyz, -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435))) + (this.group2 * float4(right_anti_dual_g0.www, right_anti_dual_g2[3]));
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g3[0] * this.e12345), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g3[1] * this.e12345), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g3[2] * this.e12345), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[2] * this.e412)) + float4((this.group0.zxy * right_anti_dual_g2.yzx), right_anti_dual_g3[3] * this.e12345) - float4((this.group0.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[0] * this.e423) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g1[1] * this.e431);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e3215) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((anti_wedge_g3.zxy * other.group0.yzx), anti_wedge_g2[3] * other.e1234) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * other.e41) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g2[3] * other.e42) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g2[3] * other.e43) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<Dipole> operator /(Dipole lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g2[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g2[3]) * this.group2.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2_w) * other.group0.xyz)), anti_wedge_g2_w * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2_w) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g2_xyz[2] * other.e42) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g2_xyz[0] * other.e43) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g2_xyz[1] * other.e41) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g2_xyz[0] * other.e23) - (anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[2] * other.e35)) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g3[2] * this.e31) + (right_anti_dual_g3[3] * this.e41), (right_anti_dual_g3[0] * this.e12) + (right_anti_dual_g3[3] * this.e42), (right_anti_dual_g3[1] * this.e23) + (right_anti_dual_g3[3] * this.e43), -(right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g3[2] * this.e43)) - (right_anti_dual_g3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * this.e41);
        float anti_wedge_g1 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g1) * other.group3.xyz), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)) - (anti_wedge_g0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_orthogonally_onto(AntiDualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35 */
            pow(float3(other.scalar), 2) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g1[2] * this.e31) + (right_anti_dual_g1[3] * this.e41), (right_anti_dual_g1[0] * this.e12) + (right_anti_dual_g1[3] * this.e42), (right_anti_dual_g1[1] * this.e23) + (right_anti_dual_g1[3] * this.e43), -(right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) - (right_anti_dual_g1.yzxx * float4(this.group1.zxy, this.e41));
        float anti_wedge_g1 = (right_anti_dual_g1[0] * this.e15) + (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35) + (right_anti_dual_g1[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e321 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0)), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public AntiLine anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        return AntiLine(
            /* e23, e31, e12 */
            float3(anti_wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(anti_wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g0[3]) * this.group2.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(anti_wedge_g0, anti_wedge_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g2_w) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g2_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g2_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g2_xyz[2] * other.scalar), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.zxyw * float4(anti_wedge_g0.yzx, anti_wedge_g2_w)) + float4((other.group0.xyz * anti_wedge_g1.www), -(anti_wedge_g2_xyz[0] * other.e23) - (anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) - (other.group1.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e31) + (right_anti_dual_g0[3] * this.e41), (right_anti_dual_g0[0] * this.e12) + (right_anti_dual_g0[3] * this.e42), (right_anti_dual_g0[1] * this.e23) + (right_anti_dual_g0[3] * this.e43), -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e41));
        return Dipole(
            /* e41, e42, e43 */
            float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e5 * -1.0), 
            /* e15, e25, e35 */
            (float3((right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45)) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
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
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
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
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public FlatPoint anti_project_orthogonally_onto(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * this.group0.xyz * float3(other.group0.xx, other.e5) * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public FlatPoint anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Flector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Flector anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = this.group0.xyz * right_anti_dual_g1.www;
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g0_xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((anti_wedge_g0_xyz.yzx * other.group0.zxy) - (anti_wedge_g0_xyz.zxy * other.group0.yzx)), (anti_wedge_g0_xyz[1] * other.e315) + (anti_wedge_g0_xyz[2] * other.e125) + (right_anti_dual_g1[3] * this.e45 * other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = -(right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45);
        float4 anti_wedge_g1 = float4((right_anti_dual_g9[2] * this.e31) + (right_anti_dual_g9[3] * this.e41), (right_anti_dual_g9[0] * this.e12) + (right_anti_dual_g9[3] * this.e42), (right_anti_dual_g9[1] * this.e23) + (right_anti_dual_g9[3] * this.e43), -(right_anti_dual_g10 * this.e45) - (right_anti_dual_g9[2] * this.e43)) - (right_anti_dual_g9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_anti_dual_g10) * this.group2.xyz), right_anti_dual_g9[0] * this.e41);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[1]) * float4(this.group2.xyz, this.e45);
        float3 anti_wedge_g4 = float3(right_anti_dual_g0[1]) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(right_anti_dual_g0[1]) * this.group1.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), 0.0) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e31) + (right_anti_dual_g0[3] * this.e41), (right_anti_dual_g0[0] * this.e12) + (right_anti_dual_g0[3] * this.e42), (right_anti_dual_g0[1] * this.e23) + (right_anti_dual_g0[3] * this.e43), -(right_anti_dual_g1 * this.e45) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_anti_dual_g1) * this.group2.xyz), right_anti_dual_g0[0] * this.e41);
        float anti_wedge_g1 = (right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group0.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * float4(other.group0.zxy, other.e5)), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g1) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public Dipole anti_project_orthogonally_onto(Scalar other) {
        return Dipole(
            /* e41, e42, e43 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35 */
            pow(float3(other.scalar), 2) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g3[2] * this.e31) + (right_anti_dual_g3[3] * this.e41), (right_anti_dual_g3[0] * this.e12) + (right_anti_dual_g3[3] * this.e42), (right_anti_dual_g3[1] * this.e23) + (right_anti_dual_g3[3] * this.e43), -(right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g3[2] * this.e43)) - (right_anti_dual_g3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * this.e41);
        float anti_wedge_g1 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * float4(other.group3.zxy, other.e5)), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g1) * other.group3.xyz), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)) - (anti_wedge_g0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Dipole {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g0[3]) * this.group0.xyz;
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g0[3]) * this.group2.xyz), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0 * float3(other.scalar)) + (float3(anti_wedge_g2[3]) * other.group0.xyz)), anti_wedge_g2[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.group0.www, other.e1234)) + float4((other.group2.xyz * anti_wedge_g2.www), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g0.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e4315) + (right_anti_dual_g1[0] * this.e1234) + (right_anti_dual_g2[3] * this.e41), (right_anti_dual_g0[0] * this.e4125) + (right_anti_dual_g1[1] * this.e1234) + (right_anti_dual_g2[3] * this.e42), (right_anti_dual_g0[1] * this.e4235) + (right_anti_dual_g1[2] * this.e1234) + (right_anti_dual_g2[3] * this.e43), -(right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) - float4((right_anti_dual_g0.yzx * this.group3.zxy), right_anti_dual_g0[0] * this.e15);
        float4 anti_wedge_g1 = float4((right_anti_dual_g2[0] * this.e1234) + (right_anti_dual_g2[3] * this.e23), (right_anti_dual_g2[1] * this.e1234) + (right_anti_dual_g2[3] * this.e31), (right_anti_dual_g2[2] * this.e1234) + (right_anti_dual_g2[3] * this.e12), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((right_anti_dual_g0 * this.group3.www), right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g2[3]) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group3.zxy) - (right_anti_dual_g2.zxy * this.group3.yzx);
        float anti_wedge_g2_w = right_anti_dual_g2[3] * this.e1234;
        float4 anti_wedge_g3 = float4(right_anti_dual_g2[3]) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g2_xyz, -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43))) + (other.group2 * float4(anti_wedge_g0.www, anti_wedge_g2_w)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), anti_wedge_g3[3] * other.scalar) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_w * other.e23) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2_xyz[1] * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group3.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (right_anti_dual_g1[3] * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g3[1] * this.e42) - (right_anti_dual_g3[2] * this.e43)) - float4((right_anti_dual_g3.xyz * this.group3.www), right_anti_dual_g3[0] * this.e41);
        float4 anti_wedge_g3 = (right_anti_dual_g3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[2])) + float4((this.group0.xyz * right_anti_dual_g3.www), right_anti_dual_g3[0] * this.e15) + float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g3[1] * this.e25) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((right_anti_dual_g3.yzx * this.group1.zxy), right_anti_dual_g2[2] * this.e4125);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * float4(other.group3.yzx, other.e4)) - (other.group3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group3.www), anti_wedge_g3[0] * other.e423), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxw * other.group1.zxyw) + (other.group2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g1.yzx * other.group3.zxy), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_orthogonally_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.group0.yy, other.scalar, other.e3215) * float4(this.group0.xyz, this.e1234);
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            pow(float4(other.scalar), 2) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.yy, other.scalar, (anti_wedge_g0[3] * other.e3215) + (anti_wedge_g3[3] * other.scalar)) * float4(anti_wedge_g3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = right_anti_dual_g0 * float4(this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125) - (right_anti_dual_g0[0] * other.e321 * this.e4235) - (right_anti_dual_g0[1] * other.e321 * this.e4315) - (right_anti_dual_g0[2] * other.e321 * this.e4125) - (right_anti_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e1234) * right_anti_dual_g1.xyz;
        float3 anti_wedge_g1_xyz = (right_anti_dual_g1.yzx * this.group3.zxy) - (right_anti_dual_g1.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) + float4((this.group3.xyz * right_anti_dual_g1.www), right_anti_dual_g0[3] * this.e1234) - (right_anti_dual_g1.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (float4(right_anti_dual_g1[3]) * float4(this.group0.xyz, this.e45)) + (right_anti_dual_g1.zxyx * float4(this.group1.yzx, this.e15)) + (this.group2.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[1])) + float4(float3(0.0), (right_anti_dual_g1[2] * this.e35) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215)) - float4((right_anti_dual_g1.yzx * this.group1.zxy), right_anti_dual_g0[0] * this.e4235);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g3.yzx * other.group1.zxy)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group1.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((anti_wedge_g3.xyz * other.group1.www), anti_wedge_g2[3] * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) + (anti_wedge_g3[3] * other.e321) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group1.zxy), anti_wedge_g3[0] * other.e235) - (float4(other.e5) * float4(anti_wedge_g0, right_anti_dual_g1[3] * this.e1234)) - (anti_wedge_g2.wwwy * float4(other.group0.xyz, other.e2)) - (other.group1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e1234);
        float3 anti_wedge_g1_xyz = right_anti_dual_g1 * float3(this.e1234);
        float anti_wedge_g1_w = -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125);
        float4 anti_wedge_g2 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) - float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g1[0] * this.e41);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * anti_wedge_g2.www), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_w * other.e23) + (anti_wedge_g0[1] * other.e35), (anti_wedge_g1_w * other.e31) + (anti_wedge_g0[2] * other.e15), (anti_wedge_g1_w * other.e12) + (anti_wedge_g0[0] * other.e25), -(anti_wedge_g1_xyz[0] * other.e15) - (anti_wedge_g1_xyz[1] * other.e25) - (anti_wedge_g1_xyz[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_anti_dual_g0.xyz, -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43))) + float4((this.group0.xyz * right_anti_dual_g0.www), right_anti_dual_g1[3] * this.e1234);
        float4 anti_wedge_g1 = (float4(right_anti_dual_g0[3]) * this.group1) + (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_anti_dual_g1.xyz, -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)));
        float4 anti_wedge_g2 = float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * this.group3.zxy) - (right_anti_dual_g1.zxy * this.group3.yzx)), right_anti_dual_g0[3] * this.e1234);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(anti_wedge_g1.www, anti_wedge_g3[3])) + float4((anti_wedge_g0.yzx * other.group1.zxy), -(anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g3.xyz * other.group0.www), anti_wedge_g0[3] * other.e3215) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e1234) * right_anti_dual_g0.xyz;
        float3 anti_wedge_g1_xyz = (right_anti_dual_g0.yzx * this.group3.zxy) - (right_anti_dual_g0.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4((this.group3.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (float4(right_anti_dual_g0[3]) * float4(this.group0.xyz, this.e45)) + (right_anti_dual_g0.zxyx * float4(this.group1.yzx, this.e15)) + float4((right_anti_dual_g0.yzx * this.group1.zxy * float3(-1.0)), (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group0.yzx) - (anti_wedge_g3.yzx * other.group0.zxy)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g3.xyz, (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) - (float4(other.e5) * float4(anti_wedge_g0, right_anti_dual_g0[3] * this.e1234)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((other.group2.xyz * this.group2.www), other.e423 * this.e4235);
        float anti_wedge_g1 = -(right_anti_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxy * float4(other.group1.zxy, other.e315)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((right_anti_dual_g2.xyz * this.group2.www), other.e431 * this.e4315);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(-(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group3.yzx) - (right_anti_dual_g0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = float4((right_anti_dual_g2[1] * this.e4125) + (right_anti_dual_g1[0] * this.e3215), (right_anti_dual_g2[2] * this.e4235) + (right_anti_dual_g1[1] * this.e3215), (right_anti_dual_g2[0] * this.e4315) + (right_anti_dual_g1[2] * this.e3215), -(right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45)) - float4((right_anti_dual_g2.zxy * this.group3.yzx), right_anti_dual_g0[0] * this.e15);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e15) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group3.yzx) - (right_anti_dual_g0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (this.group3.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e1234) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) - float4((right_anti_dual_g2.zxy * this.group3.yzx), right_anti_dual_g0[0] * this.e15);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g0.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_orthogonally_onto(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(((float4(right_anti_dual_g0[0]) * float4(this.group0.xyz, this.e45)).xyz * float3(other.group0.xx, other.e5) * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g0[0] * this.e1234 * other.e5) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_anti_dual_g0.xyz, -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45))) - float4((this.group3.xyz * right_anti_dual_g0.www), right_anti_dual_g0[0] * this.e41);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e1234) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45)) + float4((right_anti_dual_g0.xyz * this.group2.www), right_anti_dual_g1[0] * this.e4235) - float4((this.group3.xyz * right_anti_dual_g0.www), right_anti_dual_g0[0] * this.e41);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group0.www, other.e3215)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Plane anti_project_orthogonally_onto(Line other) {
        float4 anti_wedge_g0 = float4((this.e1234 * other.e235) + (this.e4315 * other.e435), (this.e1234 * other.e315) + (this.e4125 * other.e415), (this.e1234 * other.e125) + (this.e4235 * other.e425), -(this.e4315 * other.e315) - (this.e4125 * other.e125)) - (this.group3.zxyx * float4(other.group0.yzx, other.e235));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4((other.group0.yzx * anti_wedge_g0.zxy * float3(-1.0)), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Flector anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e4315) + (right_anti_dual_g1[0] * this.e1234), (right_anti_dual_g0[0] * this.e4125) + (right_anti_dual_g1[1] * this.e1234), (right_anti_dual_g0[1] * this.e4235) + (right_anti_dual_g1[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((this.group0.xyz * right_anti_dual_g1.www), right_anti_dual_g1[3] * this.e45) - (this.group3.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g1.zxy * other.group0.yzx), right_anti_dual_g1[3] * this.e1234 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (this.e1234 * other.e3215) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45);
        float4 anti_wedge_g1 = (right_anti_dual_g3 * float4(this.e1234)) + (this.group3.yzxy * float4(other.group6.zxy, other.e431)) + float4((this.group0.xyz * right_anti_dual_g9.www), this.e4235 * other.e423) + float4((right_anti_dual_g9.zxy * this.group1.yzx), this.e4125 * other.e412) - (float4(right_anti_dual_g10) * float4(this.group2.xyz, this.e45)) - (right_anti_dual_g9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group7.xyz * this.group3.www), right_anti_dual_g9[0] * this.e41) - float4((this.group3.zxy * other.group6.yzx), right_anti_dual_g9[1] * this.e42);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45) - (right_anti_dual_g3[0] * this.e4235) - (right_anti_dual_g3[1] * this.e4315) - (right_anti_dual_g3[2] * this.e4125) - (right_anti_dual_g3[3] * this.e3215);
        float4 anti_wedge_g3 = float4((right_anti_dual_g8[1] * this.e4125) + (right_anti_dual_g6[0] * this.e3215), (right_anti_dual_g8[2] * this.e4235) + (right_anti_dual_g6[1] * this.e3215), (right_anti_dual_g8[0] * this.e4315) + (right_anti_dual_g6[2] * this.e3215), -(right_anti_dual_g6[1] * this.e4315) - (right_anti_dual_g6[2] * this.e4125)) + (float4(right_anti_dual_g0[1]) * float4(this.group2.xyz, this.e45)) - float4((right_anti_dual_g8.zxy * this.group3.yzx), right_anti_dual_g6[0] * this.e4235);
        float3 anti_wedge_g4 = (float3(right_anti_dual_g0[1]) * this.group0.xyz) + (float3(this.e1234) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * this.group3.yzx) - (right_anti_dual_g7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) + (float3(right_anti_dual_g0[1]) * this.group1.xyz) - (float3(right_anti_dual_g6[3]) * this.group3.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual_g9.zxy, right_anti_dual_g10));
        float3 anti_wedge_g7 = (float3(this.e1234) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g9[3]) * this.group3.xyz) - (float3(this.e3215) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g9 = float4(right_anti_dual_g0[1]) * this.group3;
        float anti_wedge_g10 = right_anti_dual_g0[1] * this.e1234;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Plane anti_project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e1234)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * this.group3.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g1));
        float4 anti_wedge_g2 = float4((this.group3.xyz * right_anti_dual_g0.www), -(right_anti_dual_g1 * this.e45) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = float4(-(right_anti_dual_g1 * this.e15) - (right_anti_dual_g0[1] * this.e12), -(right_anti_dual_g1 * this.e25) - (right_anti_dual_g0[2] * this.e23), -(right_anti_dual_g1 * this.e35) - (right_anti_dual_g0[0] * this.e31), (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35)) + (float4(right_anti_dual_g0[3]) * float4(this.group0.xyz, this.e45)) + (right_anti_dual_g0.zxyx * float4(this.group1.yzx, this.e15));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group0.yzxw) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g2[3] * other.e5), 
            /* e15, e25, e35, e1234 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g3.xyz, (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[1] * other.e3) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g1[2] * other.e1) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g1[0] * other.e2) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (other.group0.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(Scalar other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            pow(float4(other.scalar), 2) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public Sphere anti_project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group3.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float4 anti_wedge_g2 = float4(float3(0.0), (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g3[1] * this.e42) - (right_anti_dual_g3[2] * this.e43)) + float4((this.group3.xyz * right_anti_dual_g3.www), right_anti_dual_g1[3] * this.e1234) - (right_anti_dual_g3.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (float4(right_anti_dual_g3[3]) * float4(this.group0.xyz, this.e45)) + (right_anti_dual_g3.zxyx * float4(this.group1.yzx, this.e15)) + (this.group2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g3[2] * this.e35) - (float4(this.e3215) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyz * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[2])) - float4((right_anti_dual_g3.yzx * this.group1.zxy), right_anti_dual_g2[0] * this.e4235);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) - float4((anti_wedge_g3.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e5), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group2.www), anti_wedge_g2[3] * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e125)) + float4((anti_wedge_g1.yzx * other.group3.zxy), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g2.xyz * other.group3.www), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (this.group3.yzxy * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[1])) + float4(float3(0.0), (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) + float4((this.group0.xyz * right_anti_dual_g0.www), right_anti_dual_g2[3] * this.e1234) + float4((right_anti_dual_g1.xyz * this.group2.www), right_anti_dual_g3[0] * this.e4235) - float4((right_anti_dual_g0.yzx * this.group3.zxy), right_anti_dual_g2[0] * this.e41);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + (float4(right_anti_dual_g0[3]) * this.group1) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group3.zxy) - (right_anti_dual_g2.zxy * this.group3.yzx)), right_anti_dual_g0[3] * this.e1234);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + (anti_wedge_g0.yzxw * float4(other.group2.zxy, other.e3215)) + float4((other.group3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.scalar) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<DualNum> operator /(DualNum lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), right_anti_dual_g2[3]);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = right_anti_dual_g2 * float4(this.group0.yy, this.e12345, this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g0.xyz, -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43))) + float4(float3(0.0), anti_wedge_g0[3] * other.scalar) - float4(float3(0.0), anti_wedge_g2[0] * other.e41), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.scalar) * anti_wedge_g1.xyz) + (other.group0.xyz * anti_wedge_g2.www)), anti_wedge_g1[3] * other.scalar), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(this.group0.yy, this.e12345, this.e5) * float4(other.group0.xyz, right_anti_dual_g2[3]);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = right_anti_dual_g2 * float4(this.e12345);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group3.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((other.group0.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.e4) + float4((anti_wedge_g1.xyz * other.group2.www), anti_wedge_g3[0] * other.e1) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_orthogonally_onto(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * float2((other.e3215 * this.e12345) + (other.scalar * this.e5), other.scalar * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * other.e321 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_orthogonally_onto(AntiFlector other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g0[3] * other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(AntiLine other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = right_anti_dual_g0 * float4(this.e12345);
        float4 anti_wedge_g1 = float4(this.group0.yy, this.e12345, (right_anti_dual_g0[3] * this.e5) + (right_anti_dual_g1[3] * this.e12345)) * float4(right_anti_dual_g1.xyz, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0.xyz * other.group0.www), (anti_wedge_g0[3] * other.scalar) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(AntiPlane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Circle other) {
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Dipole other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g2 = float3(this.e12345) * other.group2.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz * float3(-1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2[3] * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2[3] * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((anti_wedge_g3.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * this.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * other.e45 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (this.e12345 * pow(other.e45, 2))), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Line other) {
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            -(anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345);
        float4 anti_wedge_g1 = float4(this.e12345) * float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g2 = (right_anti_dual_g0[1] * this.e5) + (this.e12345 * other.e3215);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group8.xyz, other.e321 * -1.0);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group7.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * other.group6.xyz;
        float4 anti_wedge_g6 = float4(this.e12345) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g7 = float3(this.e12345) * other.group4.xyz * float3(-1.0);
        float3 anti_wedge_g8 = float3(this.e12345) * other.group3.xyz * float3(-1.0);
        float4 anti_wedge_g9 = float4(this.e12345) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g10 = right_anti_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g0[1] * this.e12345 * other.scalar) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_orthogonally_onto(RoundPoint other) {
        float right_anti_dual_g1 = other.e4 * -1.0;
        float anti_wedge_g0_w = right_anti_dual_g1 * this.e5;
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), (anti_wedge_g3[0] * other.e1) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) + (right_anti_dual_g1 * this.e12345 * other.e5)), 
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
public DualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum anti_project_orthogonally_onto(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            Simd32x2::pow(float2(other.scalar), 2) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public AntiScalar anti_project_orthogonally_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(this.group0.yy, this.e12345, (right_anti_dual_g0[3] * this.e12345) + (right_anti_dual_g2[3] * this.e5)) * float4(right_anti_dual_g0.xyz, 1.0);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g2 = right_anti_dual_g2 * float4(this.e12345);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (other.group3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g0.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e5) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension DualNum {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = right_anti_dual_g0 * float4(this.e12345);
        float4 anti_wedge_g1 = float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g2 = float4(this.group0.yy, this.e12345, (right_anti_dual_g0[3] * this.e5) + (right_anti_dual_g2[3] * this.e12345)) * float4(right_anti_dual_g2.xyz, 1.0);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e3215) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((anti_wedge_g3.zxy * other.group0.yzx), anti_wedge_g2[3] * other.e1234) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * other.e41) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g2[3] * other.e42) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g2[3] * other.e43) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> operator /(FlatPoint lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4(float3(0.0), right_anti_dual_g2[3] * this.e45);
        float3 anti_wedge_g2_xyz = this.group0.xyz * right_anti_dual_g2.www;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (other.e45 * this.e45);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(anti_wedge_g2_w) * other.group0.xyz), anti_wedge_g2_w * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2_w) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g2_w) * other.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2_xyz[2] * other.e42) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g2_xyz[0] * other.e43) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g2_xyz[1] * other.e41) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g2_xyz[0] * other.e23) - (anti_wedge_g2_xyz[1] * other.e31) - (anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[2] * other.e35)) - (other.group2.yzxx * float4(float3(0.0), anti_wedge_g1[0])) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(other.e4 * -1.0) * this.group0 * float4(-1.0);
        float anti_wedge_g1 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * other.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g1) * other.group3.xyz), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)) - (anti_wedge_g0.xyzx * float4(other.group3.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(anti_wedge_g1) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_orthogonally_onto(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Flector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g1[0] * this.e15) + (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35) + (right_anti_dual_g1[3] * this.e45);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(anti_wedge_g0_x) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), anti_wedge_g0_x * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Flector anti_project_orthogonally_onto(AntiMotor other) {
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
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3((right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Dipole anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (other.e45 * this.e45);
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
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float anti_wedge_g0 = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (other.e45 * this.e45);
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
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_orthogonally_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Flector anti_project_orthogonally_onto(Flector other) {
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
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = -(right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (this.e45 * other.e45);
        float4 anti_wedge_g1 = float4(other.e4 * -1.0) * this.group0 * float4(-1.0);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45);
        float4 anti_wedge_g3 = float4(other.group0.yx[1]) * this.group0;
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
            (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, 0.0)) - float4((anti_wedge_g1.xyz * other.group3.www), 0.0), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4(float3(0.0), anti_wedge_g1[1] * other.e315) + float4(float3(0.0), anti_wedge_g1[2] * other.e125) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - float4((other.group8.xyz * anti_wedge_g1.www), 0.0) - float4((other.group4.zxy * anti_wedge_g3.yzx), 0.0) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public Dipole anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(other.e4 * -1.0) * this.group0 * float4(-1.0);
        float anti_wedge_g1 = (right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group0.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * float4(other.group0.zxy, other.e5)), 
            /* e15, e25, e35 */
            (float3(anti_wedge_g1) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_project_orthogonally_onto(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(other.e4 * -1.0) * this.group0 * float4(-1.0);
        float anti_wedge_g1 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g0.xyz) - (float3(anti_wedge_g0[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g1 * other.e4) - (anti_wedge_g0.yzxw * float4(other.group3.zxy, other.e5)), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g1) * other.group3.xyz), -(anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)) - (anti_wedge_g0.xyzx * float4(other.group2.www, other.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4(float3(0.0), right_anti_dual_g0[3] * this.e45);
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (this.e45 * other.e45));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(anti_wedge_g2[3]) * other.group0.xyz), anti_wedge_g2[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g2[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.group0.www, other.e1234)) + float4((other.group2.xyz * anti_wedge_g2.www), -(anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4(float3(0.0), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(float3(0.0), anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<Flector> operator /(Flector lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = (right_anti_dual_g0.zxy * this.group1.yzx) - (right_anti_dual_g0.yzx * this.group1.zxy);
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[3] * this.e45);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((right_anti_dual_g0 * this.group1.www), right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g2[3]) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * this.group1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * this.group1.yzx), 0.0);
        float4 anti_wedge_g3 = float4(right_anti_dual_g2[3]) * this.group1;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((anti_wedge_g0_xyz * float3(other.scalar)) + (float3(anti_wedge_g0_w) * other.group0.xyz)), anti_wedge_g0_w * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_w) * other.group1), 
            /* e15, e25, e35, e1234 */
            (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g2.xyz, -(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43))) + (other.group2 * float4(float3(anti_wedge_g0_w), anti_wedge_g2[3])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g0_xyz[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g0_xyz[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[2] * other.e12)) + float4((other.group0.yzx * anti_wedge_g2.zxy), anti_wedge_g3[3] * other.scalar) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23)) - float4((anti_wedge_g0_xyz.zxy * other.group2.yzx), anti_wedge_g2[1] * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.yzx * this.group1.zxy) - (right_anti_dual_g3.zxy * this.group1.yzx);
        float anti_wedge_g1_w = right_anti_dual_g2[3] * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (this.group1.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((right_anti_dual_g3.xyz * this.group1.www), right_anti_dual_g2[3] * this.e45);
        float4 anti_wedge_g3 = float4(float3(0.0), (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45) - (right_anti_dual_g2[2] * this.e4125)) + float4((right_anti_dual_g1.zxy * this.group1.yzx), right_anti_dual_g3[0] * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * float4(other.group3.yzx, other.e4)) - (other.group3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group3.www), anti_wedge_g3[0] * other.e423), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxw * other.group1.zxyw) + (other.group2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public AntiDualNum anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDualNum(
            /* e3215, scalar */
            (float2(-(right_anti_dual_g0[0] * other.e321 * this.e4235) - (right_anti_dual_g0[1] * other.e321 * this.e4315) - (right_anti_dual_g0[2] * other.e321 * this.e4125) - (right_anti_dual_g0[3] * other.e321 * this.e3215), 0.0) + float2(0.0, 1.0)) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = (right_anti_dual_g1.yzx * this.group1.zxy) - (right_anti_dual_g1.zxy * this.group1.yzx);
        float4 anti_wedge_g1 = (right_anti_dual_g1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35) + (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215)) - (this.group1.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g0[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * anti_wedge_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0_xyz.yzx * other.group1.zxy), anti_wedge_g1[3] * other.e321) - (other.group1.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Plane anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) - (this.group1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) + (float4(right_anti_dual_g0[3]) * this.group0) - (this.group1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group1;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g1.xyz, -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12))) + (other.group0 * float4(anti_wedge_g0.www, anti_wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = (right_anti_dual_g0.yzx * this.group1.zxy) - (right_anti_dual_g0.zxy * this.group1.yzx);
        float4 anti_wedge_g1 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g0.xyz, (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_anti_dual_g0.wwwx * float4(this.group1.xyz, this.e15));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.group0.xyz * anti_wedge_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0_xyz.yzx * other.group0.zxy), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (other.group0.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Sphere anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group1.yzxx * float4(right_anti_dual_g1.zxy, other.e423));
        float anti_wedge_g1 = -(right_anti_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxy * float4(other.group1.zxy, other.e315)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Sphere anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group1.yzxx * float4(right_anti_dual_g1.zxy, other.e423));
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(-(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_anti_dual_g0.zxy * this.group1.yzx) - (right_anti_dual_g0.yzx * this.group1.zxy);
        float4 anti_wedge_g1 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g0, -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125))) - (right_anti_dual_g1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = float4((right_anti_dual_g2[1] * this.e4125) + (right_anti_dual_g1[0] * this.e3215), (right_anti_dual_g2[2] * this.e4235) + (right_anti_dual_g1[1] * this.e3215), (right_anti_dual_g2[0] * this.e4315) + (right_anti_dual_g1[2] * this.e3215), -(right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[3] * this.e45)) - float4((right_anti_dual_g2.zxy * this.group1.yzx), right_anti_dual_g0[0] * this.e15);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e15) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (right_anti_dual_g0.zxy * this.group1.yzx) - (right_anti_dual_g0.yzx * this.group1.zxy);
        float4 anti_wedge_g1 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g0, -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125))) - (right_anti_dual_g1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (this.group1.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[3] * this.e45)) - float4((right_anti_dual_g2.zxy * this.group1.yzx), right_anti_dual_g0[0] * this.e15);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g0.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(FlatPoint other) {
        float4 anti_wedge_g0 = float4(other.e45) * float4(this.group1.xyz, this.e45) * float4(-1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((this.group1.xyz * right_anti_dual_g0.www * float3(-1.0)), (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) - (right_anti_dual_g0[3] * this.e45));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group0.www, other.e3215)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Plane anti_project_orthogonally_onto(Line other) {
        float4 anti_wedge_g0 = float4(this.e4315 * other.e435, this.e4125 * other.e415, this.e4235 * other.e425, -(this.e4315 * other.e315) - (this.e4125 * other.e125)) - (this.group1.zxyx * float4(other.group0.yzx, other.e235));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4((other.group0.yzx * anti_wedge_g0.zxy * float3(-1.0)), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((right_anti_dual_g0.zxy * this.group1.yzx), right_anti_dual_g1[3] * this.e45) - (this.group1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g6[3] * this.e45);
        float4 anti_wedge_g1 = float4(-(right_anti_dual_g10 * this.e15) - (this.e4125 * other.e425), -(right_anti_dual_g10 * this.e25) - (this.e4235 * other.e435), -(right_anti_dual_g10 * this.e35) - (this.e4315 * other.e415), (this.e4315 * other.e431) + (this.e4125 * other.e412)) + (this.group1.yzxx * float4(other.group6.zxy, other.e423)) - float4((other.group7.xyz * this.group1.www), right_anti_dual_g10 * this.e45);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45) + (this.e3215 * other.e321) - (this.e4235 * other.e235) - (this.e4315 * other.e315) - (this.e4125 * other.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g8[1] * this.e4125) + (right_anti_dual_g6[0] * this.e3215), (right_anti_dual_g8[2] * this.e4235) + (right_anti_dual_g6[1] * this.e3215), (right_anti_dual_g8[0] * this.e4315) + (right_anti_dual_g6[2] * this.e3215), -(right_anti_dual_g6[1] * this.e4315) - (right_anti_dual_g6[2] * this.e4125)) + (float4(right_anti_dual_g0[1]) * this.group0) - float4((right_anti_dual_g8.zxy * this.group1.yzx), right_anti_dual_g6[0] * this.e4235);
        float3 anti_wedge_g4 = (right_anti_dual_g7.zxy * this.group1.yzx) - (right_anti_dual_g7.yzx * this.group1.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g7 * float3(this.e3215)) - (float3(right_anti_dual_g6[3]) * this.group1.xyz);
        float4 anti_wedge_g6 = float4(((right_anti_dual_g9.yzx * this.group1.zxy) - (right_anti_dual_g9.zxy * this.group1.yzx)), right_anti_dual_g10 * this.e3215 * -1.0);
        float3 anti_wedge_g7 = float3(right_anti_dual_g10) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g9[3]) * this.group1.xyz) - (float3(this.e3215) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g9 = float4(right_anti_dual_g0[1]) * this.group1;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Plane anti_project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = float3(right_anti_dual_g1) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g0.yzx * this.group1.zxy) - (right_anti_dual_g0.zxy * this.group1.yzx);
        float anti_wedge_g1_w = right_anti_dual_g1 * this.e3215 * -1.0;
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g0[3]) * this.group1.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz);
        float anti_wedge_g2_w = right_anti_dual_g1 * this.e45 * -1.0;
        float3 anti_wedge_g3_xyz = float3(right_anti_dual_g1) * this.group0.xyz * float3(-1.0);
        float anti_wedge_g3_w = (right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45);
        return DipoleInversion(
            /* e41, e42, e43 */
            (anti_wedge_g3_xyz * float3(other.e4)) - (float3(anti_wedge_g2_w) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (other.group0.yzxw * float4(anti_wedge_g3_xyz.zxy, anti_wedge_g3_w)) - float4((anti_wedge_g3_xyz.yzx * other.group0.zxy), anti_wedge_g2_w * other.e5), 
            /* e15, e25, e35, e1234 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g3_xyz, (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(float3(anti_wedge_g3_w), anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_xyz[1] * other.e3) + (anti_wedge_g2_xyz[0] * other.e4), (anti_wedge_g1_xyz[2] * other.e1) + (anti_wedge_g2_xyz[1] * other.e4), (anti_wedge_g1_xyz[0] * other.e2) + (anti_wedge_g2_xyz[2] * other.e4), -(anti_wedge_g2_xyz[1] * other.e2) - (anti_wedge_g2_xyz[2] * other.e3)) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector anti_project_orthogonally_onto(Scalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public Sphere anti_project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.yzx * this.group1.zxy) - (right_anti_dual_g3.zxy * this.group1.yzx);
        float anti_wedge_g1_w = right_anti_dual_g2[3] * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (this.group1.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125)) - float4((right_anti_dual_g3.xyz * this.group1.www), right_anti_dual_g2[3] * this.e45);
        float4 anti_wedge_g3 = float4(float3(0.0), (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45) - (right_anti_dual_g1[3] * this.e3215)) + float4((right_anti_dual_g1.zxy * this.group1.yzx), right_anti_dual_g3[0] * this.e15) - (right_anti_dual_g2.wwwx * float4(this.group0.xyz, this.e4235)) - (this.group1.zxyy * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[1])) - (this.group1.wwwz * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[2]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) - float4((anti_wedge_g3.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e5), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group2.www), anti_wedge_g2[3] * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e125)) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g2.xyz * other.group3.www), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Flector {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (this.group1.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[3] * this.e45)) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e15));
        float4 anti_wedge_g1 = (right_anti_dual_g0 * float4(this.group1.www, this.e45)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group1.xyzx);
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * this.group1.zxy), 0.0) - float4((right_anti_dual_g2.zxy * this.group1.yzx), 0.0);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group1;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + (anti_wedge_g0.yzxw * float4(other.group2.zxy, other.e3215)) + float4((other.group3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.scalar) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<Line> operator /(Line lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g1_xyz = this.group0.xyz * right_anti_dual_g2.www;
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g2[3]) * this.group1.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) - float4((right_anti_dual_g0.yzx * this.group1.zxy), right_anti_dual_g2[0] * this.e415);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g2_w * other.e23) + (anti_wedge_g3[2] * other.e42), (anti_wedge_g2_w * other.e31) + (anti_wedge_g3[0] * other.e43), (anti_wedge_g2_w * other.e12) + (anti_wedge_g3[1] * other.e41), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43)) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2_xyz[0] * other.e41), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g1_xyz[0] * other.scalar), (anti_wedge_g2_w * other.e25) + (anti_wedge_g1_xyz[1] * other.scalar), (anti_wedge_g2_w * other.e35) + (anti_wedge_g1_xyz[2] * other.scalar), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((other.group0.xyz * anti_wedge_g3.www), 0.0) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(right_anti_dual_g2[3]) * this.group1.xyz;
        float anti_wedge_g1_w = -(right_anti_dual_g3[0] * this.e415) - (right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435);
        float4 anti_wedge_g2 = float4((right_anti_dual_g3[2] * this.e315) + (right_anti_dual_g3[3] * this.e415), (right_anti_dual_g3[0] * this.e125) + (right_anti_dual_g3[3] * this.e425), (right_anti_dual_g3[1] * this.e235) + (right_anti_dual_g3[3] * this.e435), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((this.group1.zxy * right_anti_dual_g3.yzx), other.e423 * this.e235);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[1] * other.e3) + (anti_wedge_g1_xyz[0] * other.e4) + (anti_wedge_g2[3] * other.e423), (anti_wedge_g0[2] * other.e1) + (anti_wedge_g1_xyz[1] * other.e4) + (anti_wedge_g2[3] * other.e431), (anti_wedge_g0[0] * other.e2) + (anti_wedge_g1_xyz[2] * other.e4) + (anti_wedge_g2[3] * other.e412), -(anti_wedge_g1_w * other.e321) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * other.e4) + (anti_wedge_g2[3] * other.e415), (anti_wedge_g2[1] * other.e4) + (anti_wedge_g2[3] * other.e425), (anti_wedge_g2[2] * other.e4) + (anti_wedge_g2[3] * other.e435), -(anti_wedge_g1_xyz[1] * other.e2) - (anti_wedge_g1_xyz[2] * other.e3)) + float4((anti_wedge_g0 * other.group3.www), anti_wedge_g2[3] * other.e321) - (other.group3.xyzx * float4(anti_wedge_g1_xyz, anti_wedge_g1_w).wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e5)) + (float3(anti_wedge_g2[3]) * other.group2.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Line anti_project_orthogonally_onto(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g1[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g1[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g1[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - (right_anti_dual_g1.yzxx * float4(this.group1.zxy, this.e415));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(anti_wedge_g0[3]) * float4(other.group1.xyz, other.e321) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public AntiFlatPoint anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(-(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = this.group0.xyz * right_anti_dual_g0.www;
        float anti_wedge_g1_w = -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * other.group0.www), -(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1_w) * other.group0.xyz) + (float3(right_anti_dual_g0[3]) * float3(other.scalar) * this.group1.xyz)), anti_wedge_g1_w * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Line anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e415));
        return Line(
            /* e415, e425, e435 */
            float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Circle anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
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
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Circle anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - float4((right_anti_dual_g0.yzx * this.group1.zxy), right_anti_dual_g0[0] * this.e415);
        float anti_wedge_g1 = -(right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125);
        return Circle(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g1) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g0.yzx) - (other.group2.yzx * anti_wedge_g0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - float4((right_anti_dual_g0.yzx * this.group1.zxy), right_anti_dual_g0[0] * this.e415);
        float anti_wedge_g1 = -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(anti_wedge_g1) * float4(other.group1.xyz, other.e1234)) + (anti_wedge_g0.yzxx * float4(other.group2.zxy, other.e4235)) + float4((anti_wedge_g0.zxy * other.group2.yzx * float3(-1.0)), (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Line anti_project_orthogonally_onto(FlatPoint other) {
        float3 anti_wedge_g0_xyz = float3(other.e45) * this.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            anti_wedge_g0_xyz * float3(other.e45) * float3(-1.0), 
            /* e235, e315, e125 */
            (anti_wedge_g0_xyz.yzx * other.group0.zxy) - (anti_wedge_g0_xyz.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float3 anti_wedge_g0_xyz = float3(other.e45) * this.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * float3(other.e45) * float3(-1.0)), (anti_wedge_g0_xyz[0] * other.e4235) + (anti_wedge_g0_xyz[1] * other.e4315) + (anti_wedge_g0_xyz[2] * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz.yzx * other.group0.zxy) - (anti_wedge_g0_xyz.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Line anti_project_orthogonally_onto(Line other) {
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
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), anti_wedge_g0_w * other.e12345), 
            /* e235, e315, e125, e5 */
            float4((float3(anti_wedge_g0_w) * other.group1.xyz), anti_wedge_g0_w * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((right_anti_dual_g7[2] * this.e315) + (right_anti_dual_g6[3] * this.e415), (right_anti_dual_g7[0] * this.e125) + (right_anti_dual_g6[3] * this.e425), (right_anti_dual_g7[1] * this.e235) + (right_anti_dual_g6[3] * this.e435), -(right_anti_dual_g7[1] * this.e425) - (right_anti_dual_g7[2] * this.e435)) - float4((right_anti_dual_g7.yzx * this.group1.zxy), right_anti_dual_g7[0] * this.e415);
        float anti_wedge_g2 = -(right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g9[2] * this.e315) + (right_anti_dual_g9[3] * this.e415), (right_anti_dual_g9[0] * this.e125) + (right_anti_dual_g9[3] * this.e425), (right_anti_dual_g9[1] * this.e235) + (right_anti_dual_g9[3] * this.e435), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) - (right_anti_dual_g9.yzxx * float4(this.group1.zxy, this.e415));
        float3 anti_wedge_g4 = float3(right_anti_dual_g10) * this.group0.xyz;
        float3 anti_wedge_g5 = float3(right_anti_dual_g10) * this.group1.xyz;
        float4 anti_wedge_g6 = float4((this.group0.xyz * float3(right_anti_dual_g0.yy, right_anti_dual_g0[1])), 0.0);
        float3 anti_wedge_g8 = float3(right_anti_dual_g0[1]) * this.group1.xyz;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4(float3(0.0), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Circle anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = float3(right_anti_dual_g1) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(right_anti_dual_g1) * this.group1.xyz;
        float3 anti_wedge_g2 = (float3(right_anti_dual_g0[3]) * this.group0.xyz) + (this.group1.yzx * right_anti_dual_g0.zxy) - (this.group1.zxy * right_anti_dual_g0.yzx);
        return Circle(
            /* e423, e431, e412 */
            (anti_wedge_g1_xyz * float3(other.e4)) + (anti_wedge_g0.yzx * other.group0.zxy) - (anti_wedge_g0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0[0] * other.e5) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g0[1] * other.e5) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g0[2] * other.e5) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1_xyz[1] * other.e2) - (anti_wedge_g1_xyz[2] * other.e3)) - (other.group0.xyzx * float4(anti_wedge_g1_xyz, -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)).wwwx), 
            /* e235, e315, e125 */
            (anti_wedge_g1_xyz * float3(other.e5)) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public Line anti_project_orthogonally_onto(Scalar other) {
        return Line(
            /* e415, e425, e435 */
            pow(float3(other.scalar), 2) * this.group0.xyz, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz;
        float3 anti_wedge_g1_xyz = float3(right_anti_dual_g2[3]) * this.group1.xyz;
        float anti_wedge_g1_w = -(right_anti_dual_g3[0] * this.e415) - (right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435);
        float4 anti_wedge_g2 = float4((right_anti_dual_g3[2] * this.e315) + (right_anti_dual_g3[3] * this.e415), (right_anti_dual_g3[0] * this.e125) + (right_anti_dual_g3[3] * this.e425), (right_anti_dual_g3[1] * this.e235) + (right_anti_dual_g3[3] * this.e435), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - float4((this.group1.zxy * right_anti_dual_g3.yzx), right_anti_dual_g1[0] * this.e415);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g1_xyz[0] * other.e4) + (anti_wedge_g2[3] * other.e423), (anti_wedge_g1_xyz[1] * other.e4) + (anti_wedge_g2[3] * other.e431), (anti_wedge_g1_xyz[2] * other.e4) + (anti_wedge_g2[3] * other.e412), -(anti_wedge_g1_w * other.e321) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1_xyz[0] * other.e415) - (anti_wedge_g1_xyz[1] * other.e425) - (anti_wedge_g1_xyz[2] * other.e435) - (anti_wedge_g2[0] * other.e423) - (anti_wedge_g2[1] * other.e431) - (anti_wedge_g2[2] * other.e412)) + float4((anti_wedge_g0.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e12345) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g0[0] * other.e235), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[0] * other.e4) + (anti_wedge_g2[3] * other.e415), (anti_wedge_g2[1] * other.e4) + (anti_wedge_g2[3] * other.e425), (anti_wedge_g2[2] * other.e4) + (anti_wedge_g2[3] * other.e435), -(anti_wedge_g1_xyz[1] * other.e2) - (anti_wedge_g1_xyz[2] * other.e3)) + float4((anti_wedge_g0 * other.group2.www), anti_wedge_g2[3] * other.e321) - (other.group3.xyzx * float4(anti_wedge_g1_xyz, anti_wedge_g1_w).wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e5)) + (float3(anti_wedge_g2[3]) * other.group2.xyz) + (anti_wedge_g2.zxy * other.group3.yzx) - (anti_wedge_g2.yzx * other.group3.zxy)), anti_wedge_g2[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Line {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g0.www), 0.0);
        float3 anti_wedge_g2_xyz = float3(right_anti_dual_g0[3]) * this.group1.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) - float4((this.group1.zxy * right_anti_dual_g0.yzx), right_anti_dual_g2[0] * this.e415);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g3.zxyy * float4(other.group0.yzx, other.e4315)) + float4(float3(0.0), anti_wedge_g2_w * other.e3215) + float4(float3(0.0), (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((float3(anti_wedge_g2_w) * other.group1.xyz), anti_wedge_g3[0] * other.e4235) - float4((anti_wedge_g3.yzx * other.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g2_xyz * float3(other.scalar)) + (float3(anti_wedge_g3[3]) * other.group1.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g3[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(anti_wedge_g3.xyz, anti_wedge_g2_w)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<Motor> operator /(Motor lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(this.e12345) * float4(right_anti_dual_g0, right_anti_dual_g2[3]);
        float3 anti_wedge_g1_xyz = (float3(right_anti_dual_g2[3]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g1.xyz);
        float anti_wedge_g1_w = right_anti_dual_g1[3] * this.e12345;
        float4 anti_wedge_g2 = (right_anti_dual_g2 * float4(this.group0.www, this.e5)) + float4((this.group1.xyz * right_anti_dual_g2.www), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435));
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - float4((right_anti_dual_g0.yzx * this.group1.zxy), right_anti_dual_g0[0] * this.e415);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0[0] * other.scalar) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g0[1] * other.scalar) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g0[2] * other.scalar) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g1_w * other.e45) - (anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12) - (anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((other.group0.yzx * anti_wedge_g3.zxy), anti_wedge_g0[3] * other.scalar) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2[0] * other.e41), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1_xyz[0] * other.scalar) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g1_xyz[1] * other.scalar) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g1_xyz[2] * other.scalar) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((other.group0.xyz * anti_wedge_g2.www), anti_wedge_g1_w * other.scalar) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = (other.group0.xyz * this.group0.www) + (this.group0.xyz * right_anti_dual_g2.www);
        float anti_wedge_g0_w = (right_anti_dual_g2[3] * this.e5) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = (right_anti_dual_g1 * float4(this.e12345)) + float4((this.group1.xyz * right_anti_dual_g2.www), -(right_anti_dual_g3[0] * this.e415) - (right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435));
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * this.group1.yzx) - (right_anti_dual_g3.yzx * this.group1.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group3.zxyy * float4(anti_wedge_g0_xyz.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * other.e3) + (right_anti_dual_g2[3] * other.e5 * this.e12345) - (anti_wedge_g0_xyz[0] * other.e235) - (anti_wedge_g0_xyz[1] * other.e315) - (anti_wedge_g0_xyz[2] * other.e125) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((float3(anti_wedge_g0_w) * other.group0.xyz), anti_wedge_g3[3] * other.e4) + float4((anti_wedge_g1.xyz * other.group2.www), anti_wedge_g3[0] * other.e1) - float4((anti_wedge_g0_xyz.zxy * other.group3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g0_xyz[0] * other.e5) + (anti_wedge_g2_xyz[0] * other.e4), (anti_wedge_g0_xyz[1] * other.e5) + (anti_wedge_g2_xyz[1] * other.e4), (anti_wedge_g0_xyz[2] * other.e5) + (anti_wedge_g2_xyz[2] * other.e4), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0_w) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0_w) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0_w * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_w) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * float4(other.group0.yy, other.scalar, (other.e3215 * this.e12345) + (other.scalar * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_orthogonally_onto(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * other.e321 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g1[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g1[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g1[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + (float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)) - (right_anti_dual_g1.yzxx * float4(this.group1.zxy, this.e415));
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3) - (anti_wedge_g0[3] * other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group1.yzx) - (anti_wedge_g0.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0_xyz = right_anti_dual_g0 * this.group0.www;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4((float3(-(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = (float3(right_anti_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((anti_wedge_g0_xyz * other.group0.www), (right_anti_dual_g0[3] * other.scalar * this.e12345) - (anti_wedge_g0_xyz[0] * other.e23) - (anti_wedge_g0_xyz[1] * other.e31) - (anti_wedge_g0_xyz[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e415));
        float4 anti_wedge_g1 = right_anti_dual_g0 * float4(this.e12345);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e1) + (anti_wedge_g1[1] * other.e2) + (anti_wedge_g1[2] * other.e3)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0.zxy * other.group0.yzx) - (anti_wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * right_anti_dual_g2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group2.xyz), (anti_wedge_g2_w * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_anti_dual_g2 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - float4((right_anti_dual_g0.yzx * this.group1.zxy), right_anti_dual_g2[0] * this.e415);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e35) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g3[2] * other.e15) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g3[0] * other.e25) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - float4((other.group2.yzx * anti_wedge_g3.zxy), anti_wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * right_anti_dual_g2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g3[0] * this.e12345), (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g3[1] * this.e12345), (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g3[2] * this.e12345), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) + float4((right_anti_dual_g0.zxy * this.group1.yzx), right_anti_dual_g3[3] * this.e12345) - (this.group1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g3.yzx * other.group2.zxy), anti_wedge_g2_w * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public DualNum anti_project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * this.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g1_xyz = this.group0.xyz * right_anti_dual_g0.www;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45) * float4(anti_wedge_g1_xyz, right_anti_dual_g0[3] * this.e12345) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz.yzx * other.group0.zxy) - (anti_wedge_g1_xyz.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_anti_dual_g1.xyz, -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435))) + (this.group0 * float4(right_anti_dual_g0.www, right_anti_dual_g1[3]));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (right_anti_dual_g0[3] * other.e45 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Line other) {
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
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = float3(this.e12345) * right_anti_dual_g0.xyz;
        float anti_wedge_g0_w = (right_anti_dual_g0[3] * this.e12345) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(anti_wedge_g0_w) * other.group0.xyz), (anti_wedge_g0_w * other.e12345) - (anti_wedge_g0_xyz[0] * other.e415) - (anti_wedge_g0_xyz[1] * other.e425) - (anti_wedge_g0_xyz[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g0_xyz * float3(other.e5)) + (float3(anti_wedge_g0_w) * other.group1.xyz)), anti_wedge_g0_w * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1[0] * this.e12345) + (right_anti_dual_g6[3] * this.e415), (right_anti_dual_g1[1] * this.e12345) + (right_anti_dual_g6[3] * this.e425), (right_anti_dual_g1[2] * this.e12345) + (right_anti_dual_g6[3] * this.e435), -(right_anti_dual_g7[1] * this.e425) - (right_anti_dual_g7[2] * this.e435)) + float4((right_anti_dual_g7.zxy * this.group1.yzx), right_anti_dual_g1[3] * this.e12345) - float4((right_anti_dual_g7.yzx * this.group1.zxy), right_anti_dual_g7[0] * this.e415);
        float anti_wedge_g2 = (right_anti_dual_g0[1] * this.e5) + (this.e12345 * other.e3215) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g9[2] * this.e315) + (this.e12345 * other.e235), (right_anti_dual_g9[0] * this.e125) + (this.e12345 * other.e315), (right_anti_dual_g9[1] * this.e235) + (this.e12345 * other.e125), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) + (this.group0 * float4(right_anti_dual_g9.www, other.e321 * -1.0)) - (right_anti_dual_g9.yzxx * float4(this.group1.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g10) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group6.xyz);
        float4 anti_wedge_g6 = float4(((float3(right_anti_dual_g0[1]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g6.xyz)), right_anti_dual_g6[3] * this.e12345);
        float3 anti_wedge_g7 = right_anti_dual_g7 * float3(this.e12345);
        float3 anti_wedge_g8 = (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group1.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g9 * float4(this.e12345);
        float anti_wedge_g10 = right_anti_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g0[1] * this.e12345 * other.scalar) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = float4(right_anti_dual_g1) * float4(this.group0.xyz, this.e5);
        float3 anti_wedge_g1_xyz = float3(right_anti_dual_g1) * this.group1.xyz;
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g0[3]) * this.group0.xyz) + (right_anti_dual_g0.zxy * this.group1.yzx) - (right_anti_dual_g0.yzx * this.group1.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g0 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + (other.group0.wwwy * float4(anti_wedge_g1_xyz, anti_wedge_g3[1])) + float4((anti_wedge_g0.zxy * other.group0.yzx * float3(-1.0)), (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) + (right_anti_dual_g1 * this.e12345 * other.e5)), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g1_xyz[1] * other.e2) - (anti_wedge_g1_xyz[2] * other.e3)) - (other.group0.xyzx * float4(anti_wedge_g1_xyz, -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)).wwwx), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1_xyz * float3(other.e5)) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor anti_project_orthogonally_onto(Scalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e235, e315, e125, e5 */
            pow(float4(other.scalar), 2) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public AntiScalar anti_project_orthogonally_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (this.e12345 * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (this.group0 * float4(right_anti_dual_g2.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((right_anti_dual_g0.xyz * this.group0.www), right_anti_dual_g2[3] * this.e5);
        float4 anti_wedge_g1 = (right_anti_dual_g1 * float4(this.e12345)) + float4((this.group1.xyz * right_anti_dual_g2.www), -(right_anti_dual_g3[0] * this.e415) - (right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435));
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * this.group1.yzx) - (right_anti_dual_g3.yzx * this.group1.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (other.group3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((anti_wedge_g0.yzx * other.group3.zxy), right_anti_dual_g2[3] * this.e12345 * other.e5) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Motor {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = right_anti_dual_g0 * float4(this.e12345);
        float4 anti_wedge_g1 = float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g1.xyz)), right_anti_dual_g1[3] * this.e12345);
        float4 anti_wedge_g2 = (right_anti_dual_g2 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435));
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g3[0] * this.e12345), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g3[1] * this.e12345), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g3[2] * this.e12345), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + (this.group0 * float4(right_anti_dual_g1.www, right_anti_dual_g3[3])) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e415));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e3215) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((anti_wedge_g3.zxy * other.group0.yzx), anti_wedge_g2[3] * other.e1234) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * other.e41) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g2[3] * other.e42) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g2[3] * other.e43) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<MultiVector> operator /(MultiVector lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_anti_dual_g2[3] * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g2[3] * this.e1), (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g2[3] * this.e2), (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g2[3] * this.e3), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((right_anti_dual_g0.zxy * this.group8.yzx), right_anti_dual_g2[3] * this.e4) - float4((right_anti_dual_g0.yzx * this.group8.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group7.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * this.e425);
        float anti_wedge_g2 = (right_anti_dual_g2[3] * this.e5) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g2[1] * this.e4125) + (right_anti_dual_g2[3] * this.e15), (right_anti_dual_g2[2] * this.e4235) + (right_anti_dual_g2[3] * this.e25), (right_anti_dual_g2[0] * this.e4315) + (right_anti_dual_g2[3] * this.e35), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((right_anti_dual_g1.xyz * this.group9.www), right_anti_dual_g2[3] * this.e45) - (this.group9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g2[3]) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group9.yzx) - (right_anti_dual_g0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g0 * float3(this.e3215)) + (float3(right_anti_dual_g2[3]) * this.group5.xyz) + (float3(this.e1234) * right_anti_dual_g2.xyz) - (float3(right_anti_dual_g1[3]) * this.group9.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g2[3]) * this.group6);
        float3 anti_wedge_g7 = (right_anti_dual_g0 * float3(this.e12345)) + (float3(right_anti_dual_g2[3]) * this.group7.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g2[3]) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz);
        float4 anti_wedge_g9 = float4(right_anti_dual_g2[3]) * this.group9;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (right_anti_dual_g2[3] * other.scalar * this.e12345) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e5 */
            anti_wedge_g2 * other.scalar, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * float4(other.group2.xyz, other.e45)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1[3] * other.e15) + (anti_wedge_g6[0] * other.scalar), (anti_wedge_g1[3] * other.e25) + (anti_wedge_g6[1] * other.scalar), (anti_wedge_g1[3] * other.e35) + (anti_wedge_g6[2] * other.scalar), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((float3(anti_wedge_g2) * other.group0.xyz), anti_wedge_g6[3] * other.scalar) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g1[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g2) * other.group1.xyz) + (anti_wedge_g1.yzx * other.group2.zxy) - (anti_wedge_g1.zxy * other.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * other.e35) + (anti_wedge_g5[0] * other.e45) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g9[0] * other.scalar), (anti_wedge_g4[2] * other.e15) + (anti_wedge_g5[1] * other.e45) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g9[1] * other.scalar), (anti_wedge_g4[0] * other.e25) + (anti_wedge_g5[2] * other.e45) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g9[2] * other.scalar), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((other.group0.yzx * anti_wedge_g3.zxy), anti_wedge_g9[3] * other.scalar) - (other.group2.yzxy * float4(anti_wedge_g4.zxy, anti_wedge_g5[1])) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g5[0] * other.e15), 
            /* e1234 */
            (right_anti_dual_g2[3] * other.scalar * this.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g2[3] * this.e5) + (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) + (this.group9.yzxz * float4(right_anti_dual_g1.zxy, other.e412)) + float4((this.group4.xyz * right_anti_dual_g3.www), other.e423 * this.e4235) + float4((this.group5.yzx * right_anti_dual_g3.zxy), other.e431 * this.e4315) - (float4(right_anti_dual_g2[3]) * this.group3) - (right_anti_dual_g3.yzxy * float4(this.group5.zxy, this.e42)) - float4((other.group0.xyz * this.group9.www), right_anti_dual_g3[0] * this.e41) - float4((right_anti_dual_g1.yzx * this.group9.zxy), right_anti_dual_g3[2] * this.e43);
        float anti_wedge_g2 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45) - (right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g3[2] * this.e315) + (right_anti_dual_g3[3] * this.e415), (right_anti_dual_g3[0] * this.e125) + (right_anti_dual_g3[3] * this.e425), (right_anti_dual_g3[1] * this.e235) + (right_anti_dual_g3[3] * this.e435), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + (float4(this.e12345) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) - float4((this.group8.zxy * right_anti_dual_g3.yzx), right_anti_dual_g3[0] * this.e415);
        float3 anti_wedge_g4 = (float3(right_anti_dual_g2[3]) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.zxy * right_anti_dual_g3.yzx) - (this.group7.yzx * right_anti_dual_g3.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g2[3]) * this.group8.xyz) + (float3(right_anti_dual_g3[3]) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual_g1.xyz) - (float3(this.e321) * right_anti_dual_g3.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float3 anti_wedge_g7 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g3.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g3 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g2[3] * other.e5 * this.e12345) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * float4(other.group3.xyz, other.e4), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(anti_wedge_g2) * float4(other.group3.xyz, other.e4)) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group3.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g1.yzx * other.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g4[0] * other.e5) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g4[1] * other.e5) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g4[2] * other.e5) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + (float4(anti_wedge_g0_x) * other.group1) - float4((other.group3.xyz * anti_wedge_g3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group0.xyz) + (anti_wedge_g4.yzx * other.group3.zxy) - (anti_wedge_g4.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + (anti_wedge_g3.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + float4((anti_wedge_g8 * other.group2.www), anti_wedge_g1[0] * other.e235) + float4((anti_wedge_g6.yzx * other.group3.zxy), anti_wedge_g1[2] * other.e125) - (anti_wedge_g6.zxyw * other.group3.yzxw) - (other.group3.wwwx * float4(anti_wedge_g7, anti_wedge_g8[0])) - float4((anti_wedge_g1.zxy * other.group1.yzx), anti_wedge_g8[1] * other.e2) - float4((other.group2.xyz * anti_wedge_g1.www), anti_wedge_g8[2] * other.e3), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiDualNum other) {
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
            pow(float4(other.scalar), 2) * this.group3, 
            /* e41, e42, e43 */
            pow(float3(other.scalar), 2) * this.group4.xyz, 
            /* e23, e31, e12 */
            pow(float3(other.scalar), 2) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group6, 
            /* e423, e431, e412 */
            pow(float3(other.scalar), 2) * this.group7.xyz, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.yy, other.scalar, (anti_wedge_g0_x * other.e3215) + (anti_wedge_g9[3] * other.scalar)) * float4(anti_wedge_g9.xyz, 1.0), 
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321);
        float4 anti_wedge_g1 = right_anti_dual_g0 * float4(this.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0, right_anti_dual_g0[3] * other.e321 * this.e12345) * float2(0.0, -1.0), 
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
            float4((float3(anti_wedge_g1[3]) * other.group0.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e235) + (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (right_anti_dual_g0[0] * other.e321 * this.e4235) - (right_anti_dual_g0[1] * other.e321 * this.e4315) - (right_anti_dual_g0[2] * other.e321 * this.e4125) - (right_anti_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g1[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g1[0] * this.e1) + (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) + (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e1234) + (right_anti_dual_g1[2] * this.e31), (right_anti_dual_g0[1] * this.e1234) + (right_anti_dual_g1[0] * this.e12), (right_anti_dual_g0[2] * this.e1234) + (right_anti_dual_g1[1] * this.e23), -(right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) + float4((this.group4.xyz * right_anti_dual_g1.www), right_anti_dual_g0[3] * this.e1234) - (right_anti_dual_g1.yzxx * float4(this.group5.zxy, this.e41));
        float anti_wedge_g2 = (right_anti_dual_g1[0] * this.e15) + (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35) + (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215);
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[2] * this.e315) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g1[0] * this.e125) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g1[1] * this.e235) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + (right_anti_dual_g0 * float4(this.e12345)) - float4((this.group8.zxy * right_anti_dual_g1.yzx), right_anti_dual_g1[0] * this.e415);
        float3 anti_wedge_g4 = (this.group7.zxy * right_anti_dual_g1.yzx) - (this.group7.yzx * right_anti_dual_g1.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g1[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g1.xyz);
        float3 anti_wedge_g6_xyz = (right_anti_dual_g1.yzx * this.group9.zxy) - (right_anti_dual_g1.zxy * this.group9.yzx);
        float3 anti_wedge_g7 = float3(this.e1234) * right_anti_dual_g1.xyz;
        float3 anti_wedge_g8 = (float3(right_anti_dual_g1[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g1.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g1 * float4(this.e12345);
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
            float3(anti_wedge_g1[3]) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + float4((anti_wedge_g4 * other.group1.www), anti_wedge_g0_x * other.e321) - float4((other.group1.xyz * anti_wedge_g3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g4.yzx * other.group1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group0.xyz) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g2 * other.e321) + (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125) - (right_anti_dual_g1[3] * other.e5 * this.e1234)) + float4((anti_wedge_g6_xyz.yzx * other.group1.zxy), anti_wedge_g1[0] * other.e235) - (other.group1.wwwx * float4(anti_wedge_g7, anti_wedge_g8[0])) - float4((anti_wedge_g6_xyz.zxy * other.group1.yzx), anti_wedge_g8[2] * other.e3) - float4((other.group0.xyz * anti_wedge_g1.www), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float anti_wedge_g0_x = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) - float4((right_anti_dual_g1.zxy * this.group7.yzx), right_anti_dual_g0[0] * this.e423);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        float3 anti_wedge_g4 = right_anti_dual_g0 * float3(this.e1234);
        float3 anti_wedge_g5 = right_anti_dual_g1 * float3(this.e1234);
        float3 anti_wedge_g6_xyz = right_anti_dual_g0 * float3(this.group0.yy, this.e12345);
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
            (float3(-(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) * other.group0.xyz) + (other.group1.zxy * anti_wedge_g1.yzx) - (other.group1.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * other.e35) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g4[2] * other.e15) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g4[0] * other.e25) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g5[0] * other.e15) - (anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - float4((anti_wedge_g4.zxy * other.group1.yzx), anti_wedge_g3[0] * other.e23), 
            /* e1234 */
            -(anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[3] * this.scalar) + (right_anti_dual_g1[3] * this.e1234) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g0[3] * this.e1), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g0[3] * this.e2), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g0[3] * this.e3), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((this.group7.zxy * right_anti_dual_g1.yzx), right_anti_dual_g0[3] * this.e4) - float4((this.group7.yzx * right_anti_dual_g1.zxy), right_anti_dual_g0[0] * this.e423);
        float anti_wedge_g2 = (right_anti_dual_g0[3] * this.e5) + (right_anti_dual_g1[3] * this.e12345) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[3] * this.e15) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[3] * this.e25) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[3] * this.e35) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) + (right_anti_dual_g0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g0[3]) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual_g0.xyz);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g0[3]) * this.group5.xyz) + (float3(this.e1234) * right_anti_dual_g1.xyz);
        float4 anti_wedge_g6 = float4(((float3(right_anti_dual_g0[3]) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e321);
        float3 anti_wedge_g7 = float3(right_anti_dual_g0[3]) * this.group7.xyz;
        float4 anti_wedge_g9 = float4(right_anti_dual_g0[3]) * this.group9;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g1[3] * other.e3215) + (right_anti_dual_g0[3] * other.scalar * this.e12345) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12)), 
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
            (anti_wedge_g6 * float4(other.scalar)) + float4((other.group1.xyz * anti_wedge_g1.www), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g1[3]) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * other.group0.xyz) + (float3(other.scalar) * ((float3(right_anti_dual_g0[3]) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual_g1.xyz))) + (anti_wedge_g1.yzx * other.group1.zxy) - (anti_wedge_g1.zxy * other.group1.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g9.xyz, -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12))) + (other.group0 * float4(anti_wedge_g3.www, anti_wedge_g9[3])) + (other.group1.zxyw * float4(anti_wedge_g4.yzx, anti_wedge_g0_x)) - (other.group1.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])), 
            /* e1234 */
            (right_anti_dual_g0[3] * other.scalar * this.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e31) + (right_anti_dual_g0[3] * this.e41), (right_anti_dual_g0[0] * this.e12) + (right_anti_dual_g0[3] * this.e42), (right_anti_dual_g0[1] * this.e23) + (right_anti_dual_g0[3] * this.e43), -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.yzxx * float4(this.group5.zxy, this.e41));
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (this.group7.zxy * right_anti_dual_g0.yzx) - (this.group7.yzx * right_anti_dual_g0.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g0.xyz);
        float3 anti_wedge_g6_xyz = (right_anti_dual_g0.yzx * this.group9.zxy) - (right_anti_dual_g0.zxy * this.group9.yzx);
        float3 anti_wedge_g7 = float3(this.e1234) * right_anti_dual_g0.xyz;
        float3 anti_wedge_g8 = (float3(right_anti_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g0 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3)), 
            /* e1, e2, e3, e4 */
            float4((float3(anti_wedge_g0_x) * other.group0.xyz), 0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3((right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45)) * other.group0.xyz) - (float3(other.e5) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.e5 * -1.0), 
            /* e41, e42, e43 */
            float3(anti_wedge_g1[3]) * other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g4, -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3))) - float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g4.yzx * other.group0.zxy) - (anti_wedge_g4.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g3.zxy * other.group0.yzx) - (anti_wedge_g3.yzx * other.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g6_xyz.yzx * other.group0.zxy), -(anti_wedge_g8[2] * other.e3) - (right_anti_dual_g0[3] * other.e5 * this.e1234)) - (other.group0.wwwx * float4(anti_wedge_g7, anti_wedge_g8[0])) - float4((anti_wedge_g6_xyz.zxy * other.group0.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public AntiScalar anti_project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float4 anti_wedge_g1 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group9.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(this.e1234) * other.group2.xyz), other.e423 * this.e4235);
        float anti_wedge_g2 = -(right_anti_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(other.group2.xyz, right_anti_dual_g1[3]);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * right_anti_dual_g1.xyz;
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
            float4(-(anti_wedge_g1[2] * other.e425) - (anti_wedge_g1[3] * other.e235), -(anti_wedge_g1[0] * other.e435) - (anti_wedge_g1[3] * other.e315), -(anti_wedge_g1[1] * other.e415) - (anti_wedge_g1[3] * other.e125), (anti_wedge_g2 * other.e321) + (anti_wedge_g1[2] * other.e125)) + (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + float4((float3(anti_wedge_g2) * other.group0.xyz), anti_wedge_g1[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group9.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(this.e1234) * right_anti_dual_g2.xyz), other.e431 * this.e4315);
        float4 anti_wedge_g3 = float4(this.e12345) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3]);
        float3 anti_wedge_g4 = float3(this.e12345) * other.group0.xyz;
        float3 anti_wedge_g5 = float3(this.e12345) * right_anti_dual_g1.xyz;
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
            float4(-(anti_wedge_g1[2] * other.e425) - (anti_wedge_g1[3] * other.e235), -(anti_wedge_g1[0] * other.e435) - (anti_wedge_g1[3] * other.e315), -(anti_wedge_g1[1] * other.e415) - (anti_wedge_g1[3] * other.e125), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) + (float4(-(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            -(anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float anti_wedge_g0_x = -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) - float4((right_anti_dual_g0.yzx * this.group8.zxy), right_anti_dual_g0[0] * this.e415) - float4((right_anti_dual_g2.zxy * this.group7.yzx), right_anti_dual_g0[1] * this.e425);
        float anti_wedge_g2 = -(right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g2[1] * this.e4125) + (right_anti_dual_g1[0] * this.e3215), (right_anti_dual_g2[2] * this.e4235) + (right_anti_dual_g1[1] * this.e3215), (right_anti_dual_g2[0] * this.e4315) + (right_anti_dual_g1[2] * this.e3215), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group9.yzx) - (right_anti_dual_g0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g0 * float3(this.e3215)) + (right_anti_dual_g2 * float3(this.e1234)) - (float3(right_anti_dual_g1[3]) * this.group9.xyz);
        float4 anti_wedge_g6 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g7 = right_anti_dual_g0 * float3(this.e12345);
        float3 anti_wedge_g8 = right_anti_dual_g2 * float3(this.e12345);
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
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g1[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * other.group1.xyz) + (other.group2.zxy * anti_wedge_g1.yzx) - (other.group2.yzx * anti_wedge_g1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * other.e35) + (anti_wedge_g5[0] * other.e45) + (anti_wedge_g3[2] * other.e42) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g4[2] * other.e15) + (anti_wedge_g5[1] * other.e45) + (anti_wedge_g3[0] * other.e43) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g4[0] * other.e25) + (anti_wedge_g5[2] * other.e45) + (anti_wedge_g3[1] * other.e41) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - float4((anti_wedge_g4.zxy * other.group2.yzx), anti_wedge_g5[1] * other.e25) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g5[0] * other.e15), 
            /* e1234 */
            -(anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_anti_dual_g2[3] * this.e3215) + (right_anti_dual_g3[0] * this.e4235) + (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e1234) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + (float4(this.e12345) * float4(right_anti_dual_g3.xyz, right_anti_dual_g2[3])) - float4((right_anti_dual_g0.yzx * this.group8.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group7.yzx * right_anti_dual_g2.zxy), right_anti_dual_g0[1] * this.e425);
        float anti_wedge_g2 = (right_anti_dual_g3[3] * this.e12345) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[0] * this.e3215) + (right_anti_dual_g2[1] * this.e4125), (right_anti_dual_g1[1] * this.e3215) + (right_anti_dual_g2[2] * this.e4235), (right_anti_dual_g1[2] * this.e3215) + (right_anti_dual_g2[0] * this.e4315), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (this.group9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group9.yzx) - (right_anti_dual_g0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g0 * float3(this.e3215)) + (float3(this.e1234) * right_anti_dual_g2.xyz) - (float3(right_anti_dual_g1[3]) * this.group9.xyz);
        float4 anti_wedge_g6 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g7 = right_anti_dual_g0 * float3(this.e12345);
        float3 anti_wedge_g8 = float3(this.e12345) * right_anti_dual_g2.xyz;
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
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g1[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g1.zxy) - (other.group0.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g2) * other.group1.xyz) + (anti_wedge_g1.yzx * other.group2.zxy) - (anti_wedge_g1.zxy * other.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g4[1] * other.e35) + (anti_wedge_g5[0] * other.e45) + (anti_wedge_g3[2] * other.e42) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g4[2] * other.e15) + (anti_wedge_g5[1] * other.e45) + (anti_wedge_g3[0] * other.e43) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g4[0] * other.e25) + (anti_wedge_g5[2] * other.e45) + (anti_wedge_g3[1] * other.e41) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group3) - (other.group2.yzxy * float4(anti_wedge_g4.zxy, anti_wedge_g5[1])) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g5[0] * other.e15), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[0] * this.e4) + (right_anti_dual_g0[1] * this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0, anti_wedge_g0_x * other.e12345) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            float4((this.group4.xyz * float3(right_anti_dual_g0.xx, right_anti_dual_g0[0]) * float3(other.group0.xx, other.e5) * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[0]) * float3(other.e5) * this.group7.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g0[0] * other.e5 * this.e1234) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4((float3(right_anti_dual_g0[3]) * this.group6.xyz), 0.0) + float4((this.group7.zxy * right_anti_dual_g0.yzx), 0.0) - float4((this.group7.yzx * right_anti_dual_g0.zxy), 0.0);
        float3 anti_wedge_g5 = (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group9.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(1.0, right_anti_dual_g0[3] * other.e45 * this.e12345) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(-(right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45)) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(anti_wedge_g1[3]) * other.group0.xyz) - (float3(other.e45) * anti_wedge_g1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e45, other.e45, other.e45, 1.0) * float4(anti_wedge_g5, -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35))) - (other.group0.yzxx * float4(float3(0.0), anti_wedge_g5[0])), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e1234) - (right_anti_dual_g0[0] * this.e41) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45);
        float4 anti_wedge_g1 = float4((float3(right_anti_dual_g0[3]) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * right_anti_dual_g1.xyz), 0.0) + float4((this.group7.zxy * right_anti_dual_g0.yzx), 0.0) - float4((this.group7.yzx * right_anti_dual_g0.zxy), 0.0);
        float3 anti_wedge_g5 = (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group9.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) - (right_anti_dual_g0[3] * other.e45 * this.e12345)), 
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
            (anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group1) + (float4(other.e45, other.e45, other.e45, 1.0) * float4(anti_wedge_g5, -(anti_wedge_g5[1] * other.e25) - (anti_wedge_g5[2] * other.e35))) - (other.group0.yzxx * float4(float3(0.0), anti_wedge_g5[0])), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Line other) {
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
            float4(-(anti_wedge_g1[2] * other.e425) - (anti_wedge_g1[3] * other.e235), -(anti_wedge_g1[0] * other.e435) - (anti_wedge_g1[3] * other.e315), -(anti_wedge_g1[1] * other.e415) - (anti_wedge_g1[3] * other.e125), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) + (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[3] * this.e12345) + (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g1 = float4((float3(right_anti_dual_g1[3]) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group9.yzx), 0.0) - float4((right_anti_dual_g0.yzx * this.group9.zxy), 0.0);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g1[3]) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.e12345) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
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
            float4(-(anti_wedge_g1[2] * other.e425) - (anti_wedge_g1[3] * other.e235), -(anti_wedge_g1[0] * other.e435) - (anti_wedge_g1[3] * other.e315), -(anti_wedge_g1[1] * other.e415) - (anti_wedge_g1[3] * other.e125), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) + (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) - float4(float3(0.0), right_anti_dual_g1[3] * other.e5 * this.e1234), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) + (other.e3215 * this.e1234) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (right_anti_dual_g3[3] * this.e321) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = float4((right_anti_dual_g3[0] * this.e1234) + (right_anti_dual_g6[0] * this.e321) + (right_anti_dual_g6[3] * this.e415) + (right_anti_dual_g9[2] * this.e31), (right_anti_dual_g3[1] * this.e1234) + (right_anti_dual_g6[1] * this.e321) + (right_anti_dual_g6[3] * this.e425) + (right_anti_dual_g9[0] * this.e12), (right_anti_dual_g3[2] * this.e1234) + (right_anti_dual_g6[2] * this.e321) + (right_anti_dual_g6[3] * this.e435) + (right_anti_dual_g9[1] * this.e23), -(right_anti_dual_g10 * this.e45) - (right_anti_dual_g6[0] * this.e423) - (right_anti_dual_g6[1] * this.e431) - (right_anti_dual_g6[2] * this.e412)) + (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group1) + (this.group9.yzxx * float4(other.group6.zxy, other.e423)) + float4((this.group4.xyz * right_anti_dual_g9.www), right_anti_dual_g3[3] * this.e1234) + float4((right_anti_dual_g7.zxy * this.group8.yzx), other.e431 * this.e4315) + float4((right_anti_dual_g8.yzx * this.group7.zxy), other.e412 * this.e4125) - (right_anti_dual_g9.yzxy * float4(this.group5.zxy, this.e42)) - float4((float3(right_anti_dual_g10) * this.group3.xyz), right_anti_dual_g9[2] * this.e43) - float4((other.group7.xyz * this.group9.www), right_anti_dual_g7[0] * this.e415) - float4((right_anti_dual_g7.yzx * this.group8.zxy), right_anti_dual_g7[2] * this.e435) - float4((right_anti_dual_g8.zxy * this.group7.yzx), right_anti_dual_g9[0] * this.e41) - float4((other.group6.yzx * this.group9.zxy), right_anti_dual_g7[1] * this.e425);
        float anti_wedge_g2 = (right_anti_dual_g0[1] * this.e5) + (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45) + (other.e3215 * this.e12345) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g3[0] * this.e4235) - (right_anti_dual_g3[1] * this.e4315) - (right_anti_dual_g3[2] * this.e4125) - (right_anti_dual_g3[3] * this.e3215) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g8[1] * this.e4125) + (right_anti_dual_g6[0] * this.e3215) + (right_anti_dual_g9[2] * this.e315) + (right_anti_dual_g9[3] * this.e415), (right_anti_dual_g8[2] * this.e4235) + (right_anti_dual_g6[1] * this.e3215) + (right_anti_dual_g9[0] * this.e125) + (right_anti_dual_g9[3] * this.e425), (right_anti_dual_g8[0] * this.e4315) + (right_anti_dual_g6[2] * this.e3215) + (right_anti_dual_g9[1] * this.e235) + (right_anti_dual_g9[3] * this.e435), -(right_anti_dual_g6[2] * this.e4125) - (right_anti_dual_g9[0] * this.e415) - (right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) + (right_anti_dual_g3 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group3) - (this.group9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - float4((this.group8.zxy * right_anti_dual_g9.yzx), right_anti_dual_g6[1] * this.e4315);
        float3 anti_wedge_g4 = (float3(right_anti_dual_g10) * this.group6.xyz) + (float3(right_anti_dual_g0[1]) * this.group4.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * this.group9.yzx) + (this.group7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * this.group9.zxy) - (this.group7.yzx * right_anti_dual_g9.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) + (float3(right_anti_dual_g10) * this.group8.xyz) + (float3(right_anti_dual_g0[1]) * this.group5.xyz) + (float3(right_anti_dual_g9[3]) * this.group7.xyz) + (float3(this.e12345) * other.group6.xyz) - (float3(right_anti_dual_g6[3]) * this.group9.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g6 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group6) + (right_anti_dual_g9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((right_anti_dual_g9.zxy * this.group9.yzx), right_anti_dual_g10 * this.e3215);
        float3 anti_wedge_g7 = (right_anti_dual_g7 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group7.xyz) + (float3(this.e1234) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * this.group9.xyz);
        float3 anti_wedge_g8 = (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group8.xyz) + (float3(right_anti_dual_g9[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g9 = (right_anti_dual_g9 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group9);
        float anti_wedge_g10 = (right_anti_dual_g10 * this.e12345) + (right_anti_dual_g0[1] * this.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g0[1] * other.scalar * this.e12345) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g1_xyz = right_anti_dual_g0.xyz * float3(this.group0.yy, this.e12345);
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
            float4((right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e1234)) * other.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g1 * this.e5) + (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e31) + (right_anti_dual_g0[3] * this.e41), (right_anti_dual_g0[0] * this.e12) + (right_anti_dual_g0[3] * this.e42), (right_anti_dual_g0[1] * this.e23) + (right_anti_dual_g0[3] * this.e43), -(right_anti_dual_g1 * this.e45) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(right_anti_dual_g1) * this.group3.xyz), right_anti_dual_g0[1] * this.e42);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g1) * this.group6.xyz) + (this.group7.zxy * right_anti_dual_g0.yzx) - (this.group7.yzx * right_anti_dual_g0.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g1) * this.group8.xyz) + (float3(right_anti_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g0.xyz);
        float4 anti_wedge_g6 = float4((right_anti_dual_g0.yzx * this.group9.zxy), right_anti_dual_g0[3] * this.e1234) - (this.group9.yzxw * float4(right_anti_dual_g0.zxy, right_anti_dual_g1));
        float3 anti_wedge_g7 = (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g0 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g1 * this.e12345 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * other.group0, 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            (float4((right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45)) * other.group0) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g4[0] * other.e5) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g4[1] * other.e5) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g4[2] * other.e5) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) - (other.group0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g4.yzx * other.group0.zxy) - (anti_wedge_g4.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g3.zxy * other.group0.yzx) - (anti_wedge_g3.yzx * other.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g8[0] * other.e4) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g8[1] * other.e4) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g8[2] * other.e4) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g8[2] * other.e3) - (anti_wedge_g6[3] * other.e5)) - (other.group0.yzxy * float4(anti_wedge_g6.zxy, anti_wedge_g8[1])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g8[0] * other.e1), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            Simd32x2::pow(float2(other.scalar), 2) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e5 */
            this.e5 * pow(other.scalar, 2), 
            /* e15, e25, e35, e45 */
            pow(float4(other.scalar), 2) * this.group3, 
            /* e41, e42, e43 */
            pow(float3(other.scalar), 2) * this.group4.xyz, 
            /* e23, e31, e12 */
            pow(float3(other.scalar), 2) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group6, 
            /* e423, e431, e412 */
            pow(float3(other.scalar), 2) * this.group7.xyz, 
            /* e235, e315, e125 */
            pow(float3(other.scalar), 2) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group9, 
            /* e1234 */
            this.e1234 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215);
        float4 anti_wedge_g1 = right_anti_dual_g0 * float4(this.e12345);
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
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[3] * this.e12345) + (right_anti_dual_g2[3] * this.e5) + (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412);
        float4 anti_wedge_g1 = (float4(this.e1234) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) + (this.group9.yzxz * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[2])) + float4((this.group4.xyz * right_anti_dual_g3.www), right_anti_dual_g0[0] * this.e4235) + float4((this.group5.yzx * right_anti_dual_g3.zxy), right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g3.yzxx * float4(this.group5.zxy, this.e41)) - float4((right_anti_dual_g1.yzx * this.group9.zxy), right_anti_dual_g3[2] * this.e43) - float4((right_anti_dual_g0.xyz * this.group9.www), right_anti_dual_g2[3] * this.e45) - float4((this.group3.xyz * right_anti_dual_g2.www), right_anti_dual_g3[1] * this.e42);
        float anti_wedge_g2 = (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45) - (right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g3[2] * this.e315) + (right_anti_dual_g3[3] * this.e415), (right_anti_dual_g3[0] * this.e125) + (right_anti_dual_g3[3] * this.e425), (right_anti_dual_g3[1] * this.e235) + (right_anti_dual_g3[3] * this.e435), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + (float4(this.e12345) * float4(right_anti_dual_g2.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g3.yzxx * float4(this.group8.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g2[3]) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz) + (this.group7.zxy * right_anti_dual_g3.yzx) - (this.group7.yzx * right_anti_dual_g3.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g2[3]) * this.group8.xyz) + (float3(right_anti_dual_g3[3]) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual_g1.xyz) - (float3(this.e321) * right_anti_dual_g3.xyz);
        float4 anti_wedge_g6 = float4((right_anti_dual_g3.yzx * this.group9.zxy), right_anti_dual_g3[3] * this.e1234) - (this.group9.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float3 anti_wedge_g7 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group9.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g3.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g3 * float4(this.e12345);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g2[3] * this.e12345 * other.e5) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0_x) * other.group3, 
            /* e5 */
            anti_wedge_g0_x * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(anti_wedge_g2) * other.group3) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group3.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g1.yzx * other.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g4[0] * other.e5) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g4[1] * other.e5) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g4[2] * other.e5) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[1] * other.e2) - (anti_wedge_g5[2] * other.e3)) + (float4(anti_wedge_g0_x) * other.group1) - (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g5[0])), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (float3(anti_wedge_g0_x) * other.group0.xyz) + (anti_wedge_g4.yzx * other.group3.zxy) - (anti_wedge_g4.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (float3(anti_wedge_g0_x) * other.group2.xyz) + (anti_wedge_g3.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g2) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g1.yzxy * float4(other.group1.zxy, other.e315)) + float4((anti_wedge_g8 * other.group3.www), anti_wedge_g1[0] * other.e235) + float4((anti_wedge_g6.yzx * other.group3.zxy), anti_wedge_g1[2] * other.e125) - (anti_wedge_g6.zxyw * float4(other.group3.yzx, other.e5)) - float4((anti_wedge_g7 * other.group2.www), anti_wedge_g8[0] * other.e1) - float4((anti_wedge_g1.zxy * other.group1.yzx), anti_wedge_g8[1] * other.e2) - float4((other.group2.xyz * anti_wedge_g1.www), anti_wedge_g8[2] * other.e3), 
            /* e1234 */
            (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0_x = (right_anti_dual_g0[3] * this.scalar) + (right_anti_dual_g2[3] * this.e1234) + (right_anti_dual_g3[0] * this.e4235) + (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g0[3] * this.e1) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g0[3] * this.e2) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g0[3] * this.e3) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[2] * this.e412)) + (right_anti_dual_g3 * float4(this.e12345)) + float4((this.group7.zxy * right_anti_dual_g2.yzx), right_anti_dual_g0[3] * this.e4) - float4((this.group7.yzx * right_anti_dual_g2.zxy), right_anti_dual_g1[0] * this.e423) - float4((this.group8.zxy * right_anti_dual_g0.yzx), right_anti_dual_g1[1] * this.e431);
        float anti_wedge_g2 = (right_anti_dual_g0[3] * this.e5) + (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435);
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[0] * this.e3215) + (right_anti_dual_g2[1] * this.e4125), (right_anti_dual_g1[1] * this.e3215) + (right_anti_dual_g2[2] * this.e4235), (right_anti_dual_g1[2] * this.e3215) + (right_anti_dual_g2[0] * this.e4315), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + (float4(right_anti_dual_g0[3]) * this.group3) - (this.group9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0]));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g0[3]) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group9.yzx) - (right_anti_dual_g0.yzx * this.group9.zxy);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g0[3]) * this.group5.xyz) + (float3(this.e3215) * right_anti_dual_g0.xyz) + (float3(this.e1234) * right_anti_dual_g2.xyz) - (float3(right_anti_dual_g1[3]) * this.group9.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group6);
        float3 anti_wedge_g7 = (float3(right_anti_dual_g0[3]) * this.group7.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g0[3]) * this.group8.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz);
        float4 anti_wedge_g9 = float4(right_anti_dual_g0[3]) * this.group9;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (right_anti_dual_g0[3] * this.e12345 * other.scalar) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e5 */
            anti_wedge_g2 * other.scalar, 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * float4(other.group2.xyz, other.e45)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g6[0] * other.scalar), (anti_wedge_g2 * other.e42) + (anti_wedge_g6[1] * other.scalar), (anti_wedge_g2 * other.e43) + (anti_wedge_g6[2] * other.scalar), -(anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((other.group2.xyz * anti_wedge_g1.www), anti_wedge_g6[3] * other.scalar) - (anti_wedge_g1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g1[3]) * other.group1.xyz) + (anti_wedge_g1.zxy * other.group0.yzx) - (anti_wedge_g1.yzx * other.group0.zxy), 
            /* e235, e315, e125 */
            (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g2) * other.group1.xyz) + (anti_wedge_g1.yzx * other.group2.zxy) - (anti_wedge_g1.zxy * other.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g5[0] * other.e45) + (anti_wedge_g3[2] * other.e42) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g9[0] * other.scalar), (anti_wedge_g5[1] * other.e45) + (anti_wedge_g3[0] * other.e43) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g9[1] * other.scalar), (anti_wedge_g5[2] * other.e45) + (anti_wedge_g3[1] * other.e41) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g9[2] * other.scalar), -(anti_wedge_g5[2] * other.e35) - (anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group3) + float4((anti_wedge_g4.yzx * other.group2.zxy), anti_wedge_g9[3] * other.scalar) - (other.group2.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g5[1] * other.e25), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (right_anti_dual_g0[3] * this.e1234 * other.scalar) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<Plane> operator /(Plane lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g0, -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125))) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual_g2.zxy * this.group0.yzx), 0.0);
        float4 anti_wedge_g3 = float4(right_anti_dual_g2[3]) * this.group0;
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(other.scalar), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2.xyz * other.group2.www), (anti_wedge_g2[3] * other.scalar) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[2] * other.e12)) + float4((other.group0.yzx * anti_wedge_g2.zxy), anti_wedge_g3[3] * other.scalar) - (anti_wedge_g2.yzxx * float4(other.group0.zxy, other.e23)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2[1] * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.yzx * this.group0.zxy) - (right_anti_dual_g3.zxy * this.group0.yzx);
        float anti_wedge_g1_w = right_anti_dual_g2[3] * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g3.xyz, (other.e431 * this.e4315) + (other.e412 * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_anti_dual_g3.www, other.e423));
        float4 anti_wedge_g3 = float4((right_anti_dual_g1.zxy * this.group0.yzx), -(right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (this.group0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * float4(other.group3.yzx, other.e4)) - (other.group3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group3.www), anti_wedge_g3[0] * other.e423), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxw * other.group1.zxyw) + (other.group2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public AntiDualNum anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDualNum(
            /* e3215, scalar */
            (float2(-(right_anti_dual_g0[0] * other.e321 * this.e4235) - (right_anti_dual_g0[1] * other.e321 * this.e4315) - (right_anti_dual_g0[2] * other.e321 * this.e4125) - (right_anti_dual_g0[3] * other.e321 * this.e3215), 0.0) + float2(0.0, 1.0)) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0_xyz = (right_anti_dual_g1.yzx * this.group0.zxy) - (right_anti_dual_g1.zxy * this.group0.yzx);
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g1.www), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215)) - (this.group0.wwwx * float4(right_anti_dual_g1.xyz, right_anti_dual_g0[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * anti_wedge_g1.www), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + float4((anti_wedge_g0_xyz.yzx * other.group1.zxy), anti_wedge_g1[3] * other.e321) - (other.group1.yzxx * float4(anti_wedge_g0_xyz.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[3]) * this.group0;
        return Flector(
            /* e15, e25, e35, e45 */
            anti_wedge_g0 * float4(other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g1.xyz, -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12))) + (other.group0 * float4(anti_wedge_g0.www, anti_wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (right_anti_dual_g0.yzx * this.group0.zxy) - (right_anti_dual_g0.zxy * this.group0.yzx);
        float3 anti_wedge_g1 = (float3(right_anti_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0.yzx * other.group0.zxy), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (other.group0.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual_g1.zxy, other.e423));
        float anti_wedge_g1 = -(right_anti_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxy * float4(other.group1.zxy, other.e315)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual_g1.zxy, other.e423));
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(-(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g0, -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125))) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float3 anti_wedge_g2 = (float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group0.zxy) - (right_anti_dual_g2.zxy * this.group0.yzx);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g2[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2[0] * other.e23) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g2[1] * other.e31), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g0, -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125))) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (this.group0.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4((right_anti_dual_g2.zxy * this.group0.yzx * float3(-1.0)), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g0.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(FlatPoint other) {
        float3 anti_wedge_g0 = float3(other.e45) * this.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float3(other.e45)), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((this.group0.xyz * (other.group0 * float4(-1.0, -1.0, -1.0, 1.0)).www * float3(-1.0)), (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group0.www, other.e3215)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(Line other) {
        float4 anti_wedge_g0 = float4(other.e435 * this.e4315, other.e415 * this.e4125, other.e425 * this.e4235, -(other.e315 * this.e4315) - (other.e125 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e235));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4((other.group0.yzx * anti_wedge_g0.zxy * float3(-1.0)), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0.zxy * this.group0.yzx), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215);
        float4 anti_wedge_g1 = float4(-(other.e425 * this.e4125) - (other.e423 * this.e3215), -(other.e435 * this.e4235) - (other.e431 * this.e3215), -(other.e415 * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(other.group6.zxy, other.e423));
        float anti_wedge_g2 = (other.e321 * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g8[1] * this.e4125) + (right_anti_dual_g6[0] * this.e3215), (right_anti_dual_g8[2] * this.e4235) + (right_anti_dual_g6[1] * this.e3215), (right_anti_dual_g8[0] * this.e4315) + (right_anti_dual_g6[2] * this.e3215), -(right_anti_dual_g6[1] * this.e4315) - (right_anti_dual_g6[2] * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0]));
        float3 anti_wedge_g4 = (right_anti_dual_g7.zxy * this.group0.yzx) - (right_anti_dual_g7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g7 * float3(this.e3215)) - (float3(right_anti_dual_g6[3]) * this.group0.xyz);
        float4 anti_wedge_g6 = float4(((right_anti_dual_g9.yzx * this.group0.zxy) - (right_anti_dual_g9.zxy * this.group0.yzx)), right_anti_dual_g10 * this.e3215 * -1.0);
        float3 anti_wedge_g7 = float3(right_anti_dual_g10) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g9[3]) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g9 = float4(other.group0.yx[1]) * this.group0;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = float3(right_anti_dual_g1) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g0.yzx * this.group0.zxy) - (right_anti_dual_g0.zxy * this.group0.yzx);
        float anti_wedge_g1_w = right_anti_dual_g1 * this.e3215 * -1.0;
        float3 anti_wedge_g2 = (float3(right_anti_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_xyz[1] * other.e3) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g1_xyz[2] * other.e1) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g1_xyz[0] * other.e2) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g1_w * other.e5) - (anti_wedge_g2[2] * other.e3)) - (other.group0.yzxy * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[1])) - float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[0] * other.e1), 
            /* e1234 */
            (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane anti_project_orthogonally_onto(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public Sphere anti_project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(right_anti_dual_g2[3]) * this.group0.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = (right_anti_dual_g3.yzx * this.group0.zxy) - (right_anti_dual_g3.zxy * this.group0.yzx);
        float anti_wedge_g1_w = right_anti_dual_g2[3] * this.e3215 * -1.0;
        float4 anti_wedge_g2 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g3.xyz, (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0]));
        float4 anti_wedge_g3 = float4((right_anti_dual_g1.zxy * this.group0.yzx), -(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[2] * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - (this.group0.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[1]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) - float4((anti_wedge_g3.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e5), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g1_w * other.e4) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group2.www), anti_wedge_g2[3] * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e125)) + float4((anti_wedge_g1_xyz.yzx * other.group3.zxy), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g2.xyz * other.group3.www), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1_w)) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Plane {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[0])) + float4((right_anti_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215));
        float4 anti_wedge_g1 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g0.xyz, -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125))) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual_g2.zxy * this.group0.yzx), 0.0);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + (anti_wedge_g0.yzxw * float4(other.group2.zxy, other.e3215)) + float4((other.group3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.scalar) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> operator /(RoundPoint lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_project_orthogonally_onto(AntiCircleRotor other) {
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g2[3]) * this.group0;
        float anti_wedge_g1 = right_anti_dual_g2[3] * this.e5;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(anti_wedge_g0[3]) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g0.zxy) - (other.group0.zxy * anti_wedge_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e4 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0.yzx * other.group2.zxy) - (anti_wedge_g0.zxy * other.group2.yzx)), anti_wedge_g0[3] * other.scalar), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * float4(anti_wedge_g0.xyz, anti_wedge_g1)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (other.e4 * this.e5);
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
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_orthogonally_onto(AntiDualNum other) {
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
public AntiFlector operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiFlector anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g1[0] * this.e1) + (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) + (right_anti_dual_g1[3] * this.e4);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(anti_wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0[3]) * this.group0;
        float anti_wedge_g1 = right_anti_dual_g0[3] * this.e5;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(anti_wedge_g0[3]) * float4(other.group0.xyz, other.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g0[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1) * other.group0.xyz) + (anti_wedge_g0.yzx * other.group1.zxy) - (anti_wedge_g0.zxy * other.group1.yzx)), anti_wedge_g1 * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public DualNum anti_project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * this.e4 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public Motor anti_project_orthogonally_onto(Motor other) {
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
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0_x = (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) - (other.e4 * this.e5);
        float4 anti_wedge_g1 = float4(right_anti_dual_g0[1]) * this.group0;
        float anti_wedge_g2 = right_anti_dual_g0[1] * this.e5;
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
            (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g1[3] * other.e15), (anti_wedge_g2 * other.e42) + (anti_wedge_g1[3] * other.e25), (anti_wedge_g2 * other.e43) + (anti_wedge_g1[3] * other.e35), -(anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (float4(anti_wedge_g0_x) * other.group6) - float4((anti_wedge_g1.xyz * other.group3.www), 0.0), 
            /* e423, e431, e412 */
            (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (other.group4.yzx * anti_wedge_g1.zxy) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) - (anti_wedge_g1.zxy * other.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g0_x) * other.group9) + float4(float3(0.0), anti_wedge_g2 * other.e321) + float4(float3(0.0), anti_wedge_g1[0] * other.e235) + float4(float3(0.0), anti_wedge_g1[1] * other.e315) + float4(float3(0.0), anti_wedge_g1[2] * other.e125) + float4(((float3(anti_wedge_g2) * other.group7.xyz) + (anti_wedge_g1.yzx * other.group6.zxy)), 0.0) - float4((other.group8.xyz * anti_wedge_g1.www), 0.0) - float4((anti_wedge_g1.zxy * other.group6.yzx), 0.0), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4) - (other.e4 * this.e5);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0) * other.group0, 
            /* e5 */
            anti_wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_project_orthogonally_onto(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e5 */
            this.e5 * pow(other.scalar, 2)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g0 = (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (this.e5 * other.e4);
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
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension RoundPoint {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0[3]) * this.group0;
        float anti_wedge_g1 = right_anti_dual_g0[3] * this.e5;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0.zxyx * float4(other.group0.yzx, other.e4235)) + (anti_wedge_g0.wwwy * float4(other.group1.xyz, other.e4315)) + float4((anti_wedge_g0.yzx * other.group0.zxy * float3(-1.0)), (anti_wedge_g1 * other.e1234) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215)), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1 * other.e41) + (anti_wedge_g0[3] * other.e15), (anti_wedge_g1 * other.e42) + (anti_wedge_g0[3] * other.e25), (anti_wedge_g1 * other.e43) + (anti_wedge_g0[3] * other.e35), -(anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)) - (anti_wedge_g0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g1) * other.group1.xyz) + (anti_wedge_g0.yzx * other.group2.zxy) - (anti_wedge_g0.zxy * other.group2.yzx)), anti_wedge_g1 * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g0 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<Scalar> operator /(Scalar lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor anti_project_orthogonally_onto(AntiCircleRotor other) {
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
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiDualNum anti_project_orthogonally_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiProjectOrthogonallyOntoInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Scalar {
    public AntiMotor anti_project_orthogonally_onto(AntiMotor other) {
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
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Scalar {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float anti_wedge_g0 = other.group0.yx[1] * this.scalar;
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
public Scalar operator /(AntiProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar anti_project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Scalar {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
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
public AntiProjectOrthogonallyOntoInfixPartial<Sphere> operator /(Sphere lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float3 anti_wedge_g2_xyz = (float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group0.zxy) - (right_anti_dual_g2.zxy * this.group0.yzx);
        float anti_wedge_g2_w = right_anti_dual_g2[3] * this.e1234;
        float4 anti_wedge_g3 = float4(right_anti_dual_g2[3]) * this.group0;
        return DipoleInversion(
            /* e41, e42, e43 */
            anti_wedge_g0 * float3(other.scalar), 
            /* e23, e31, e12, e45 */
            anti_wedge_g1 * float4(other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((anti_wedge_g2_xyz * other.group2.www), (anti_wedge_g2_w * other.scalar) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), anti_wedge_g3[3] * other.scalar) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2_xyz[1] * other.e31) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_w * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group0.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float4 anti_wedge_g2 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g3.xyz, (right_anti_dual_g1[3] * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_anti_dual_g3.www, other.e423));
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[2] * this.e4315) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g1[0] * this.e4125) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g1[1] * this.e4235) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (this.group0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * float4(other.group3.yzx, other.e4)) - (other.group3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group3.www), anti_wedge_g3[0] * other.e423), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxw * other.group1.zxyw) + (other.group2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g1.yzx * other.group3.zxy), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public VersorOdd anti_project_orthogonally_onto(AntiDualNum other) {
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
            float4(other.group0.yy, other.scalar, (anti_wedge_g0_w * other.e3215) + (anti_wedge_g3[3] * other.scalar)) * float4(anti_wedge_g3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = right_anti_dual_g0 * float4(this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125) - (right_anti_dual_g0[0] * other.e321 * this.e4235) - (right_anti_dual_g0[1] * other.e321 * this.e4315) - (right_anti_dual_g0[2] * other.e321 * this.e4125) - (right_anti_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e1234) * right_anti_dual_g1.xyz;
        float3 anti_wedge_g1_xyz = (right_anti_dual_g1.yzx * this.group0.zxy) - (right_anti_dual_g1.zxy * this.group0.yzx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g1[3]) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual_g1.xyz);
        float anti_wedge_g2_w = right_anti_dual_g0[3] * this.e1234;
        float3 anti_wedge_g3_xyz = float3(this.e1234) * right_anti_dual_g0.xyz;
        float anti_wedge_g3_w = -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2_w) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3_xyz.zxy * other.group1.yzx) - (anti_wedge_g3_xyz.yzx * other.group1.zxy)), anti_wedge_g2_w * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group1.xyzx * float4(float3(anti_wedge_g3_w), anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((anti_wedge_g3_xyz * other.group1.www), anti_wedge_g2_w * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g3_w * other.e321) + (anti_wedge_g3_xyz[1] * other.e315) + (anti_wedge_g3_xyz[2] * other.e125) - (anti_wedge_g2_xyz[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group1.zxy), anti_wedge_g3_xyz[0] * other.e235) - (float4(other.e5) * float4(anti_wedge_g0, right_anti_dual_g1[3] * this.e1234)) - (other.group1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2_xyz[0])) - (float4(anti_wedge_g2_xyz, anti_wedge_g2_w).wwwy * float4(other.group0.xyz, other.e2))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e1234);
        float3 anti_wedge_g1_xyz = right_anti_dual_g1 * float3(this.e1234);
        float anti_wedge_g1_w = -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125);
        float3 anti_wedge_g2 = (right_anti_dual_g0 * float3(this.e3215)) + (right_anti_dual_g1.yzx * this.group0.zxy) - (right_anti_dual_g1.zxy * this.group0.yzx);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_w * other.e23) + (anti_wedge_g0[1] * other.e35), (anti_wedge_g1_w * other.e31) + (anti_wedge_g0[2] * other.e15), (anti_wedge_g1_w * other.e12) + (anti_wedge_g0[0] * other.e25), -(anti_wedge_g1_xyz[0] * other.e15) - (anti_wedge_g1_xyz[1] * other.e25) - (anti_wedge_g1_xyz[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e23), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public VersorOdd anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(this.e1234) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3]);
        float3 anti_wedge_g1_xyz = float3(this.e1234) * right_anti_dual_g1.xyz;
        float anti_wedge_g1_w = -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125);
        float4 anti_wedge_g2 = float4(((float3(this.e3215) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * this.group0.zxy) - (right_anti_dual_g1.zxy * this.group0.yzx)), right_anti_dual_g0[3] * this.e1234);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g1_xyz * float3(other.scalar)) + (float3(anti_wedge_g0[3]) * other.group0.xyz)), anti_wedge_g1_w * other.scalar), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(anti_wedge_g1_w), anti_wedge_g3[3])) + float4((anti_wedge_g0.yzx * other.group1.zxy), -(anti_wedge_g1_xyz[0] * other.e15) - (anti_wedge_g1_xyz[1] * other.e25) - (anti_wedge_g1_xyz[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g3.xyz * other.group0.www), anti_wedge_g0[3] * other.e3215) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e1234) * right_anti_dual_g0.xyz;
        float3 anti_wedge_g1_xyz = (right_anti_dual_g0.yzx * this.group0.zxy) - (right_anti_dual_g0.zxy * this.group0.yzx);
        float3 anti_wedge_g2 = (float3(right_anti_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), -(anti_wedge_g2[2] * other.e3) - (right_anti_dual_g0[3] * other.e5 * this.e1234)) - (other.group0.wwwx * float4(anti_wedge_g0, anti_wedge_g2[0])) - float4((anti_wedge_g1_xyz.zxy * other.group0.yzx), anti_wedge_g2[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group0.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((float3(this.e1234) * other.group2.xyz), other.e423 * this.e4235);
        float anti_wedge_g1 = -(right_anti_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxy * float4(other.group1.zxy, other.e315)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((float3(this.e1234) * right_anti_dual_g2.xyz), other.e431 * this.e4315);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(-(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float3 anti_wedge_g2 = (float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group0.zxy) - (right_anti_dual_g2.zxy * this.group0.yzx);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12), -(anti_wedge_g2[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2[0] * other.e23) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g2[1] * other.e31), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (this.group0.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4((right_anti_dual_g2.zxy * this.group0.yzx * float3(-1.0)), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e1234));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g0.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public AntiDualNum anti_project_orthogonally_onto(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e5 * other.e5 * this.e1234, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Plane anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group0.xyz);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0 * float3(other.e45)), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Flector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(((float3(this.e1234) * right_anti_dual_g0.xyz) - (this.group0.xyz * right_anti_dual_g0.www)), (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e1234));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group0.www, other.e3215)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Plane anti_project_orthogonally_onto(Line other) {
        float4 anti_wedge_g0 = float4((other.e435 * this.e4315) + (other.e235 * this.e1234), (other.e415 * this.e4125) + (other.e315 * this.e1234), (other.e425 * this.e4235) + (other.e125 * this.e1234), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e235));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4((other.group0.yzx * anti_wedge_g0.zxy * float3(-1.0)), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Flector anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[2] * this.e4315) + (right_anti_dual_g1[0] * this.e1234), (right_anti_dual_g0[0] * this.e4125) + (right_anti_dual_g1[1] * this.e1234), (right_anti_dual_g0[1] * this.e4235) + (right_anti_dual_g1[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g1.zxy * other.group0.yzx), right_anti_dual_g1[3] * other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234);
        float4 anti_wedge_g1 = float4(-(other.e425 * this.e4125) - (other.e423 * this.e3215), -(other.e435 * this.e4235) - (other.e431 * this.e3215), -(other.e415 * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g3_w * this.e1234) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(other.group6.zxy, other.e423)) + float4((float3(this.e1234) * other.group8.xyz), other.e431 * this.e4315);
        float anti_wedge_g2 = -(right_anti_dual_g3_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g8[1] * this.e4125) + (right_anti_dual_g6[0] * this.e3215), (right_anti_dual_g8[2] * this.e4235) + (right_anti_dual_g6[1] * this.e3215), (right_anti_dual_g8[0] * this.e4315) + (right_anti_dual_g6[2] * this.e3215), -(right_anti_dual_g6[1] * this.e4315) - (right_anti_dual_g6[2] * this.e4125)) - (this.group0.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0]));
        float3 anti_wedge_g4 = (float3(this.e1234) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * this.group0.yzx) - (right_anti_dual_g7.yzx * this.group0.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) - (float3(right_anti_dual_g6[3]) * this.group0.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_anti_dual_g9.zxy * this.group0.yzx), right_anti_dual_g10 * this.e3215);
        float3 anti_wedge_g7 = (float3(this.e1234) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * this.group0.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g9[3]) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g9 = float4(right_anti_dual_g0[1]) * this.group0;
        float anti_wedge_g10 = right_anti_dual_g0[1] * this.e1234;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Plane anti_project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e1234)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * this.group0.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_anti_dual_g0.zxy * this.group0.yzx), right_anti_dual_g1 * this.e3215);
        float3 anti_wedge_g2 = (float3(right_anti_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2[0] * other.e4) + (anti_wedge_g1[1] * other.e3), (anti_wedge_g2[1] * other.e4) + (anti_wedge_g1[2] * other.e1), (anti_wedge_g2[2] * other.e4) + (anti_wedge_g1[0] * other.e2), -(anti_wedge_g2[2] * other.e3) - (anti_wedge_g1[3] * other.e5)) - (other.group0.yzxy * float4(anti_wedge_g1.zxy, anti_wedge_g2[1])) - float4((anti_wedge_g0 * float3(other.e5)), anti_wedge_g2[0] * other.e1), 
            /* e1234 */
            (anti_wedge_g0[0] * other.e1) + (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e1234 */
            other.scalar * other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public Sphere anti_project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group0.xyz);
        float4 anti_wedge_g1 = float4((right_anti_dual_g3.yzx * this.group0.zxy), right_anti_dual_g3[3] * this.e1234) - (this.group0.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float4 anti_wedge_g2 = (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_anti_dual_g3.xyz, (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g1[3] * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0]));
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[2] * this.e4315) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g1[0] * this.e4125) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g1[1] * this.e4235) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[2] * this.e4125)) - (this.group0.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - (this.group0.wwwy * float4(right_anti_dual_g0.xyz, right_anti_dual_g2[1]));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) - float4((anti_wedge_g3.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e5), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group2.www), anti_wedge_g2[3] * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e125)) + float4((anti_wedge_g1.yzx * other.group3.zxy), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g2.xyz * other.group3.www), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension Sphere {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (this.group0.yzxx * float4(right_anti_dual_g0.zxy, right_anti_dual_g3[0])) + float4((right_anti_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_anti_dual_g2[3] * this.e1234) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215)) + float4((float3(this.e1234) * right_anti_dual_g1.xyz), right_anti_dual_g3[1] * this.e4315);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group0.xyzx);
        float4 anti_wedge_g2 = float4(((float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group0.zxy) - (right_anti_dual_g2.zxy * this.group0.yzx)), right_anti_dual_g0[3] * this.e1234);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + (anti_wedge_g0.yzxw * float4(other.group2.zxy, other.e3215)) + float4((other.group3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.scalar) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<VersorEven> operator /(VersorEven lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0_xyz = (right_anti_dual_g0 * float3(this.e12345)) + (float3(right_anti_dual_g2[3]) * this.group0.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g2[3]) * this.group1);
        float4 anti_wedge_g2 = (right_anti_dual_g2 * float4(this.group0.www, this.e5)) + float4((this.group2.xyz * right_anti_dual_g2.www), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435));
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g2[3] * this.e1), (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g2[3] * this.e2), (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g2[3] * this.e3), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((right_anti_dual_g0.zxy * this.group2.yzx), right_anti_dual_g2[3] * this.e4) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e415) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g0[1] * this.e425);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((anti_wedge_g0_xyz[0] * other.scalar) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g0_xyz[1] * other.scalar) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g0_xyz[2] * other.scalar) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g0_xyz[0] * other.e15) - (anti_wedge_g0_xyz[1] * other.e25) - (anti_wedge_g0_xyz[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((other.group0.yzx * anti_wedge_g3.zxy), right_anti_dual_g2[3] * other.scalar * this.e12345) - float4((other.group0.zxy * anti_wedge_g3.yzx), anti_wedge_g2[0] * other.e41), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g1[0] * other.scalar) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g1[1] * other.scalar) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g1[2] * other.scalar) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + float4((other.group0.xyz * anti_wedge_g2.www), anti_wedge_g1[3] * other.scalar) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g3.yzxy * float4(this.group0.zxy, this.e2)) + float4(float3(0.0), (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((other.group0.xyz * this.group0.www), right_anti_dual_g2[3] * this.e5) + float4((this.group1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * this.e1) - float4((right_anti_dual_g3.zxy * this.group0.yzx), other.e423 * this.e235);
        float4 anti_wedge_g1 = float4((right_anti_dual_g2[3] * this.e235) + (right_anti_dual_g3[3] * this.e423), (right_anti_dual_g2[3] * this.e315) + (right_anti_dual_g3[3] * this.e431), (right_anti_dual_g2[3] * this.e125) + (right_anti_dual_g3[3] * this.e412), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + (right_anti_dual_g1 * float4(this.e12345)) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * this.group2.yzx) - (right_anti_dual_g3.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group3.zxyy * float4(anti_wedge_g0.yzx, anti_wedge_g3[1])) + float4(float3(0.0), (anti_wedge_g3[2] * other.e3) + (right_anti_dual_g2[3] * other.e5 * this.e12345) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((other.group0.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.e4) + float4((anti_wedge_g1.xyz * other.group2.www), anti_wedge_g3[0] * other.e1) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.scalar) * this.group0;
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.yy, other.scalar, (anti_wedge_g0[3] * other.scalar) + (anti_wedge_g3[3] * other.e3215)) * float4(anti_wedge_g0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * float4(other.group0.yy, other.scalar, (other.e3215 * this.e12345) + (other.scalar * this.e5)) * float4(this.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), anti_wedge_g2_w * other.e321), 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group0.xyz), right_anti_dual_g0[3] * other.e321 * this.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) + (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321)) - (this.group0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        float4 anti_wedge_g1 = (this.group0 * float4(right_anti_dual_g1.www, right_anti_dual_g0[3])) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - (right_anti_dual_g1.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * right_anti_dual_g0.xyz), 0.0) + float4((right_anti_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g1.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_anti_dual_g1 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group1.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[3] * other.e321)) - (anti_wedge_g0.zxyx * float4(other.group1.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g0 * float4(other.group1.www, other.e321)) + float4(float3(0.0), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group1.yzx) - (anti_wedge_g2.yzx * other.group1.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group1.xyz * anti_wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g1_xyz = right_anti_dual_g0 * this.group0.www;
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[0] * this.e321) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[1] * this.e321) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[2] * this.e321) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - float4((right_anti_dual_g1.zxy * this.group0.yzx), right_anti_dual_g0[0] * this.e235);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(anti_wedge_g2_w) * other.group1.xyz), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e35) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g3[2] * other.e15) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g3[0] * other.e25) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g1_xyz[0] * other.e23) - (anti_wedge_g1_xyz[1] * other.e31) - (anti_wedge_g1_xyz[2] * other.e12)) - float4((other.group1.yzx * anti_wedge_g3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0[3]) * this.group0;
        float4 anti_wedge_g1 = float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e321);
        float4 anti_wedge_g2 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435));
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[3] * this.e1) + (right_anti_dual_g1[1] * this.e412), (right_anti_dual_g0[3] * this.e2) + (right_anti_dual_g1[2] * this.e423), (right_anti_dual_g0[3] * this.e3) + (right_anti_dual_g1[0] * this.e431), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + (right_anti_dual_g0 * float4(this.group1.www, this.e4)) - (this.group0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0 * float4(anti_wedge_g3.www, anti_wedge_g0[3])) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + float4((anti_wedge_g0.xyz * other.group0.www), anti_wedge_g3[3] * other.e3215), 
            /* e415, e425, e435, e321 */
            (anti_wedge_g1 * float4(other.scalar)) + float4((other.group1.xyz * anti_wedge_g3.www), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group1.zxy) - (anti_wedge_g3.zxy * other.group1.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4((right_anti_dual_g0.zxy * this.group0.yzx * float3(-1.0)), (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4));
        float4 anti_wedge_g1 = float4((this.group0.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float4 anti_wedge_g2 = float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g0.yzx * this.group2.zxy), 0.0);
        float4 anti_wedge_g3 = right_anti_dual_g0 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + float4((anti_wedge_g0.zxy * other.group0.yzx * float3(-1.0)), (anti_wedge_g2[3] * other.e5) + (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3)), 
            /* e415, e425, e435, e321 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g0.xyz, -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3))) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2.zxy * other.group0.yzx) - (anti_wedge_g2.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4((other.group0.xyz * anti_wedge_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiScalar anti_project_orthogonally_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * other.group2.xyz;
        float anti_wedge_g2_w = -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
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
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e12345) * other.group0.xyz;
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * right_anti_dual_g2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[0] * this.e423) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(anti_wedge_g2_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(anti_wedge_g2_w) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(anti_wedge_g2_w) * other.group2.xyz), (anti_wedge_g2_w * other.e12345) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g2_xyz[0] * other.e423) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = right_anti_dual_g2 * float3(this.e12345);
        float anti_wedge_g2_w = -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g0[2] * this.e315) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415), (right_anti_dual_g0[0] * this.e125) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425), (right_anti_dual_g0[1] * this.e235) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435), -(right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g2[0] * this.e415) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g2[1] * this.e425);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4((anti_wedge_g3[1] * other.e35) + (anti_wedge_g3[3] * other.e23), (anti_wedge_g3[2] * other.e15) + (anti_wedge_g3[3] * other.e31), (anti_wedge_g3[0] * other.e25) + (anti_wedge_g3[3] * other.e12), -(anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) - float4((other.group2.yzx * anti_wedge_g3.zxy), anti_wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public CircleRotor anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e12345);
        float4 anti_wedge_g1 = right_anti_dual_g1 * float4(this.e12345);
        float3 anti_wedge_g2_xyz = float3(this.e12345) * right_anti_dual_g2.xyz;
        float anti_wedge_g2_w = (right_anti_dual_g2[3] * this.e12345) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412);
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g3[0] * this.e12345), (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g3[1] * this.e12345), (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g3[2] * this.e12345), -(right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) + float4((right_anti_dual_g0.zxy * this.group2.yzx), right_anti_dual_g3[3] * this.e12345) - (this.group2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g1[1] * this.e315);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(anti_wedge_g2_w) * other.group1.xyz) + (other.group0.yzx * anti_wedge_g3.zxy) - (other.group0.zxy * anti_wedge_g3.yzx), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_w * other.e15) + (anti_wedge_g3[3] * other.e41), (anti_wedge_g2_w * other.e25) + (anti_wedge_g3[3] * other.e42), (anti_wedge_g2_w * other.e35) + (anti_wedge_g3[3] * other.e43), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e1234) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g2_xyz[0] * other.e41) - (anti_wedge_g2_xyz[1] * other.e42) - (anti_wedge_g2_xyz[2] * other.e43) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45)) + float4((anti_wedge_g3.yzx * other.group2.zxy), anti_wedge_g2_w * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g3.zxy, anti_wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_orthogonally_onto(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        float4 anti_wedge_g0 = float4(right_anti_dual_g0.xx, right_anti_dual_g0[0], (right_anti_dual_g0[0] * this.e4) + (right_anti_dual_g0[1] * this.e12345)) * float4(this.group0.xyz, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), anti_wedge_g0[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            anti_wedge_g0 * float4(other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[1] * this.e412) + (right_anti_dual_g0[3] * this.e415), (right_anti_dual_g0[2] * this.e423) + (right_anti_dual_g0[3] * this.e425), (right_anti_dual_g0[0] * this.e431) + (right_anti_dual_g0[3] * this.e435), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e415));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45) * float4(anti_wedge_g1.xyz, right_anti_dual_g0[3] * this.e12345) * float4(-1.0), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[3] * this.e415) + (right_anti_dual_g1[0] * this.e12345), (right_anti_dual_g0[3] * this.e425) + (right_anti_dual_g1[1] * this.e12345), (right_anti_dual_g0[3] * this.e435) + (right_anti_dual_g1[2] * this.e12345), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + (this.group0.zxyw * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[3])) - (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e415));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45) * anti_wedge_g1.xyz * float3(-1.0)), (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) - (right_anti_dual_g0[3] * other.e45 * this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((anti_wedge_g1.yzx * other.group0.zxy) - (anti_wedge_g1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_orthogonally_onto(Line other) {
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
public Motor operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public Motor anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g0 * float4(this.e12345)) + (float4(right_anti_dual_g1[3]) * float4(this.group0.xyz, this.e4)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * anti_wedge_g0.www), (anti_wedge_g0[3] * other.e12345) - (anti_wedge_g0[0] * other.e415) - (anti_wedge_g0[1] * other.e425) - (anti_wedge_g0[2] * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group1.xyz) + (float3(other.e5) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) - (right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (right_anti_dual_g3[3] * this.e321) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 anti_wedge_g1 = float4((right_anti_dual_g8[1] * this.e412) + (right_anti_dual_g1[0] * this.e12345) + (right_anti_dual_g6[0] * this.e321) + (right_anti_dual_g6[3] * this.e415), (right_anti_dual_g8[2] * this.e423) + (right_anti_dual_g1[1] * this.e12345) + (right_anti_dual_g6[1] * this.e321) + (right_anti_dual_g6[3] * this.e425), (right_anti_dual_g8[0] * this.e431) + (right_anti_dual_g1[2] * this.e12345) + (right_anti_dual_g6[2] * this.e321) + (right_anti_dual_g6[3] * this.e435), -(right_anti_dual_g7[2] * this.e435) - (right_anti_dual_g6[0] * this.e423) - (right_anti_dual_g6[1] * this.e431) - (right_anti_dual_g6[2] * this.e412)) + (float4(right_anti_dual_g0[1]) * this.group3) + float4((right_anti_dual_g7.zxy * this.group2.yzx), right_anti_dual_g1[3] * this.e12345) - float4((right_anti_dual_g7.yzx * this.group2.zxy), right_anti_dual_g7[0] * this.e415) - float4((right_anti_dual_g8.zxy * this.group0.yzx), right_anti_dual_g7[1] * this.e425);
        float anti_wedge_g2 = (right_anti_dual_g0[1] * this.e5) + (other.e3215 * this.e12345) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g9[2] * this.e315) + (right_anti_dual_g9[3] * this.e415), (right_anti_dual_g9[0] * this.e125) + (right_anti_dual_g9[3] * this.e425), (right_anti_dual_g9[1] * this.e235) + (right_anti_dual_g9[3] * this.e435), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) + (right_anti_dual_g3 * float4(this.e12345)) - (right_anti_dual_g9.yzxx * float4(this.group2.zxy, this.e415));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + (right_anti_dual_g9.yzx * this.group0.zxy) - (right_anti_dual_g9.zxy * this.group0.yzx);
        float3 anti_wedge_g5 = (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9[3]) * this.group0.xyz) + (float3(this.e12345) * other.group6.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g6 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group1);
        float3 anti_wedge_g7 = (right_anti_dual_g7 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz);
        float3 anti_wedge_g8 = (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group2.xyz);
        float4 anti_wedge_g9 = right_anti_dual_g9 * float4(this.e12345);
        float anti_wedge_g10 = right_anti_dual_g10 * this.e12345;
        return MultiVector(
            /* scalar, e12345 */
            float2(anti_wedge_g0_x * other.scalar, (anti_wedge_g0_x * other.e12345) + (anti_wedge_g10 * other.e5) + (anti_wedge_g2 * other.e1234) + (anti_wedge_g1[0] * other.e4235) + (anti_wedge_g1[1] * other.e4315) + (anti_wedge_g1[2] * other.e4125) + (anti_wedge_g1[3] * other.e3215) + (anti_wedge_g9[0] * other.e1) + (anti_wedge_g9[1] * other.e2) + (anti_wedge_g9[2] * other.e3) + (anti_wedge_g9[3] * other.e4) + (right_anti_dual_g0[1] * other.scalar * this.e12345) - (anti_wedge_g4[0] * other.e235) - (anti_wedge_g4[1] * other.e315) - (anti_wedge_g4[2] * other.e125) - (anti_wedge_g5[0] * other.e415) - (anti_wedge_g5[1] * other.e425) - (anti_wedge_g5[2] * other.e435) - (anti_wedge_g7[0] * other.e15) - (anti_wedge_g7[1] * other.e25) - (anti_wedge_g7[2] * other.e35) - (anti_wedge_g8[0] * other.e41) - (anti_wedge_g8[1] * other.e42) - (anti_wedge_g8[2] * other.e43) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412) - (anti_wedge_g3[3] * other.e321) - (anti_wedge_g6[0] * other.e23) - (anti_wedge_g6[1] * other.e31) - (anti_wedge_g6[2] * other.e12) - (anti_wedge_g6[3] * other.e45)), 
            /* e1, e2, e3, e4 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group1), 
            /* e5 */
            (anti_wedge_g0_x * other.e5) + (anti_wedge_g2 * other.scalar), 
            /* e15, e25, e35, e45 */
            (anti_wedge_g3 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group3) + (float4(anti_wedge_g2) * other.group1) - (anti_wedge_g1 * float4(other.e5)), 
            /* e41, e42, e43 */
            (anti_wedge_g4 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group4.xyz) + (float3(other.e4) * anti_wedge_g1.xyz) - (float3(anti_wedge_g1[3]) * other.group1.xyz), 
            /* e23, e31, e12 */
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiScalar anti_project_orthogonally_onto(Plane other) {
        float4 anti_wedge_g0 = float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float4 anti_wedge_g0 = (right_anti_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4((right_anti_dual_g0.zxy * this.group0.yzx * float3(-1.0)), (right_anti_dual_g1 * this.e5) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4)) + float4((float3(right_anti_dual_g1) * this.group1.xyz), right_anti_dual_g0[1] * this.e2);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1 * this.e235) + (right_anti_dual_g0[3] * this.e423), (right_anti_dual_g1 * this.e315) + (right_anti_dual_g0[3] * this.e431), (right_anti_dual_g1 * this.e125) + (right_anti_dual_g0[3] * this.e412), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (right_anti_dual_g0.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g0[3]) * this.group1.xyz) + (right_anti_dual_g0.zxy * this.group2.yzx) - (right_anti_dual_g0.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g0 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0.zxyx * float4(anti_wedge_g0.yzx, anti_wedge_g3[0])) + (other.group0.wwwy * float4(anti_wedge_g1.xyz, anti_wedge_g3[1])) + float4((anti_wedge_g0.zxy * other.group0.yzx * float3(-1.0)), (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) + (right_anti_dual_g1 * other.e5 * this.e12345)), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) - (anti_wedge_g1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group0.yzx) - (anti_wedge_g2_xyz.yzx * other.group0.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(Scalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e415, e425, e435, e321 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e235, e315, e125, e5 */
            pow(float4(other.scalar), 2) * this.group2, 
            /* e1, e2, e3, e4 */
            pow(float4(other.scalar), 2) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public AntiScalar anti_project_orthogonally_onto(Sphere other) {
        float4 anti_wedge_g0 = float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiScalar(
            /* e12345 */
            (anti_wedge_g0[0] * other.e4235) + (anti_wedge_g0[1] * other.e4315) + (anti_wedge_g0[2] * other.e4125) + (anti_wedge_g0[3] * other.e3215) + (other.e3215 * other.e1234 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = (this.group0.zxyw * float4(right_anti_dual_g3.yzx, right_anti_dual_g0[3])) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (right_anti_dual_g2[1] * this.e431) - (right_anti_dual_g2[2] * this.e412)) + float4((right_anti_dual_g0.xyz * this.group0.www), right_anti_dual_g2[3] * this.e5) + float4((this.group1.xyz * right_anti_dual_g2.www), right_anti_dual_g3[0] * this.e1) - (this.group0.yzxx * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[0]));
        float4 anti_wedge_g1 = float4((right_anti_dual_g1[0] * this.e12345) + (right_anti_dual_g2[3] * this.e235), (right_anti_dual_g1[1] * this.e12345) + (right_anti_dual_g2[3] * this.e315), (right_anti_dual_g1[2] * this.e12345) + (right_anti_dual_g2[3] * this.e125), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + (this.group0 * float4(right_anti_dual_g3.www, right_anti_dual_g1[3])) - (right_anti_dual_g3.xyzx * this.group1.wwwx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual_g2.xyz) + (right_anti_dual_g3.zxy * this.group2.yzx) - (right_anti_dual_g3.yzx * this.group2.zxy);
        float4 anti_wedge_g3 = right_anti_dual_g3 * float4(this.e12345);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(anti_wedge_g0[3]) * other.group0) + (other.group3.wwwx * float4(anti_wedge_g1.xyz, anti_wedge_g3[0])) + float4(float3(0.0), (anti_wedge_g3[1] * other.e2) + (anti_wedge_g3[2] * other.e3) + (anti_wedge_g3[3] * other.e4) - (anti_wedge_g2_xyz[1] * other.e431) - (anti_wedge_g2_xyz[2] * other.e412) - (anti_wedge_g0[0] * other.e235) - (anti_wedge_g0[1] * other.e315) - (anti_wedge_g0[2] * other.e125) - (anti_wedge_g1[0] * other.e415) - (anti_wedge_g1[1] * other.e425) - (anti_wedge_g1[2] * other.e435) - (anti_wedge_g1[3] * other.e321)) + float4((anti_wedge_g0.yzx * other.group3.zxy), right_anti_dual_g2[3] * other.e5 * this.e12345) - float4((anti_wedge_g0.zxy * other.group3.yzx), anti_wedge_g2_xyz[0] * other.e423), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2_xyz[0] * other.e4) + (anti_wedge_g0[0] * other.e5), (anti_wedge_g2_xyz[1] * other.e4) + (anti_wedge_g0[1] * other.e5), (anti_wedge_g2_xyz[2] * other.e4) + (anti_wedge_g0[2] * other.e5), -(anti_wedge_g1[1] * other.e2) - (anti_wedge_g1[2] * other.e3)) + (float4(anti_wedge_g0[3]) * other.group1) - (anti_wedge_g1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g0[3]) * other.group2.xyz) + (float3(other.e5) * anti_wedge_g1.xyz) + (anti_wedge_g2_xyz.zxy * other.group3.yzx) - (anti_wedge_g2_xyz.yzx * other.group3.zxy)), anti_wedge_g0[3] * other.e5), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorEven {
    public VersorEven anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e12345);
        float4 anti_wedge_g1 = (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group1);
        float4 anti_wedge_g2 = (right_anti_dual_g2 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435));
        float4 anti_wedge_g3 = float4((right_anti_dual_g1[0] * this.e321) + (right_anti_dual_g1[3] * this.e415) + (right_anti_dual_g2[1] * this.e412) + (right_anti_dual_g3[0] * this.e12345), (right_anti_dual_g1[1] * this.e321) + (right_anti_dual_g1[3] * this.e425) + (right_anti_dual_g2[2] * this.e423) + (right_anti_dual_g3[1] * this.e12345), (right_anti_dual_g1[2] * this.e321) + (right_anti_dual_g1[3] * this.e435) + (right_anti_dual_g2[0] * this.e431) + (right_anti_dual_g3[2] * this.e12345), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + (right_anti_dual_g0.zxyw * float4(this.group2.yzx, this.e4)) + float4((this.group3.xyz * right_anti_dual_g0.www), right_anti_dual_g3[3] * this.e12345) - (right_anti_dual_g0.yzxx * float4(this.group2.zxy, this.e415)) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g0[1] * this.e425);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (anti_wedge_g0 * float4(other.scalar)) + (anti_wedge_g3.wwwx * float4(other.group1.xyz, other.e4235)) + float4(float3(0.0), (anti_wedge_g3[1] * other.e4315) + (anti_wedge_g3[2] * other.e4125) + (anti_wedge_g3[3] * other.e3215) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12) - (anti_wedge_g1[3] * other.e45) - (anti_wedge_g2[0] * other.e41) - (anti_wedge_g2[1] * other.e42) - (anti_wedge_g2[2] * other.e43)) + float4((anti_wedge_g3.zxy * other.group0.yzx), anti_wedge_g2[3] * other.e1234) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g0[0] * other.e15), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2[3] * other.e41) + (anti_wedge_g3[3] * other.e15), (anti_wedge_g2[3] * other.e42) + (anti_wedge_g3[3] * other.e25), (anti_wedge_g2[3] * other.e43) + (anti_wedge_g3[3] * other.e35), -(anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12)) + (anti_wedge_g1 * float4(other.scalar)) - (anti_wedge_g3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(anti_wedge_g2[3]) * other.group1.xyz) + (float3(other.scalar) * anti_wedge_g2.xyz) + (anti_wedge_g3.yzx * other.group2.zxy) - (anti_wedge_g3.zxy * other.group2.yzx)), anti_wedge_g2[3] * other.scalar), 
            /* e1, e2, e3, e4 */
            anti_wedge_g3 * float4(other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> operator /(VersorOdd lhs, AntiProjectOrthogonallyOntoInfix rhs) {
    return AntiProjectOrthogonallyOntoInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_orthogonally_onto(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4((right_anti_dual_g1[0] * this.e1234) + (right_anti_dual_g2[3] * this.e41), (right_anti_dual_g1[1] * this.e1234) + (right_anti_dual_g2[3] * this.e42), (right_anti_dual_g1[2] * this.e1234) + (right_anti_dual_g2[3] * this.e43), -(right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) + float4((right_anti_dual_g0.zxy * this.group3.yzx), right_anti_dual_g2[3] * this.scalar) - float4((right_anti_dual_g0.yzx * this.group3.zxy), right_anti_dual_g0[0] * this.e15);
        float4 anti_wedge_g1 = float4((right_anti_dual_g2[0] * this.e1234) + (right_anti_dual_g2[3] * this.e23), (right_anti_dual_g2[1] * this.e1234) + (right_anti_dual_g2[3] * this.e31), (right_anti_dual_g2[2] * this.e1234) + (right_anti_dual_g2[3] * this.e12), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((right_anti_dual_g0 * this.group3.www), right_anti_dual_g2[3] * this.e45) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float3 anti_wedge_g2_xyz = (float3(right_anti_dual_g2[3]) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group3.zxy) - (right_anti_dual_g2.zxy * this.group3.yzx);
        float anti_wedge_g2_w = right_anti_dual_g2[3] * this.e1234;
        float4 anti_wedge_g3 = float4(right_anti_dual_g2[3]) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (float4(other.scalar, other.scalar, other.scalar, 1.0) * float4(anti_wedge_g2_xyz, -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43))) + (other.group2 * float4(anti_wedge_g0.www, anti_wedge_g2_w)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g2_xyz[2] * other.e12) - (anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35)) + float4((anti_wedge_g2_xyz.zxy * other.group0.yzx), anti_wedge_g3[3] * other.scalar) - float4((anti_wedge_g2_xyz.yzx * other.group0.zxy), anti_wedge_g2_w * other.e23) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g2_xyz[1] * other.e31)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group3.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(right_anti_dual_g3.www, other.e423)) + float4(float3(0.0), (right_anti_dual_g1[3] * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_anti_dual_g3[0] * this.e41) - (right_anti_dual_g3[1] * this.e42) - (right_anti_dual_g3[2] * this.e43)) - float4((right_anti_dual_g3.xyz * this.group3.www), right_anti_dual_g2[3] * this.e45);
        float4 anti_wedge_g3 = (float4(right_anti_dual_g3[3]) * float4(this.group0.xyz, this.e45)) + (right_anti_dual_g3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g3[0] * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((right_anti_dual_g3.yzx * this.group1.zxy), right_anti_dual_g2[2] * this.e4125);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * float4(other.group3.yzx, other.e4)) - (other.group3.zxyw * float4(anti_wedge_g3.yzx, anti_wedge_g2[3])), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g2[3] * other.e321) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group3.www), anti_wedge_g3[0] * other.e423), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g3.yzxw * other.group1.zxyw) + (other.group2.wwwz * float4(anti_wedge_g2.xyz, anti_wedge_g3[2])) + float4((other.group0.xyz * anti_wedge_g3.www), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g1.yzx * other.group3.zxy), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_orthogonally_onto(AntiDualNum other) {
        float4 anti_wedge_g0 = float4(other.group0.yy, other.scalar, (other.e3215 * this.e1234) + (other.scalar * this.scalar)) * float4(this.group0.xyz, 1.0);
        float4 anti_wedge_g3 = float4(other.scalar) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            pow(float4(other.scalar), 2) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.yy, other.scalar, (anti_wedge_g0[3] * other.e3215) + (anti_wedge_g3[3] * other.scalar)) * float4(anti_wedge_g3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_orthogonally_onto(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = right_anti_dual_g0 * float4(this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(anti_wedge_g0[3]) * other.group0.xyz * float3(-1.0)), (anti_wedge_g0[0] * other.e235) + (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125) - (right_anti_dual_g0[0] * other.e321 * this.e4235) - (right_anti_dual_g0[1] * other.e321 * this.e4315) - (right_anti_dual_g0[2] * other.e321 * this.e4125) - (right_anti_dual_g0[3] * other.e321 * this.e3215)), 
            /* e1234 */
            anti_wedge_g0[3] * other.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(AntiFlector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e1234) * right_anti_dual_g1.xyz;
        float3 anti_wedge_g1_xyz = (right_anti_dual_g1.yzx * this.group3.zxy) - (right_anti_dual_g1.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4(float3(0.0), -(right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) + float4((this.group3.xyz * right_anti_dual_g1.www), right_anti_dual_g0[3] * this.e1234) - (right_anti_dual_g1.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (right_anti_dual_g1.zxyy * float4(this.group1.yzx, this.e25)) + (right_anti_dual_g1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[0])) + float4(float3(0.0), (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215)) - float4((right_anti_dual_g1.yzx * this.group1.zxy), right_anti_dual_g0[0] * this.e4235);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group1.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g3.yzx * other.group1.zxy)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group1.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) - float4((anti_wedge_g3.xyz * other.group1.www), anti_wedge_g2[3] * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (anti_wedge_g3[1] * other.e315) + (anti_wedge_g3[2] * other.e125) + (anti_wedge_g3[3] * other.e321) - (anti_wedge_g2[2] * other.e3)) + float4((anti_wedge_g1_xyz.yzx * other.group1.zxy), anti_wedge_g3[0] * other.e235) - (float4(other.e5) * float4(anti_wedge_g0, right_anti_dual_g1[3] * this.e1234)) - (anti_wedge_g2.wwwy * float4(other.group0.xyz, other.e2)) - (other.group1.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 anti_wedge_g0 = right_anti_dual_g0 * float3(this.e1234);
        float3 anti_wedge_g1_xyz = right_anti_dual_g1 * float3(this.e1234);
        float anti_wedge_g1_w = -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125);
        float4 anti_wedge_g2 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g1[1] * this.e4125), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g1[2] * this.e4235), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g1[0] * this.e4315), -(right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) - float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g0[0] * this.e23);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * anti_wedge_g2.www), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group1.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_w * other.e23) + (anti_wedge_g0[1] * other.e35), (anti_wedge_g1_w * other.e31) + (anti_wedge_g0[2] * other.e15), (anti_wedge_g1_w * other.e12) + (anti_wedge_g0[0] * other.e25), -(anti_wedge_g1_xyz[0] * other.e15) - (anti_wedge_g1_xyz[1] * other.e25) - (anti_wedge_g1_xyz[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_orthogonally_onto(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g0 * float4(this.group2.www, this.scalar)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) + float4((this.group0.xyz * right_anti_dual_g0.www), right_anti_dual_g1[3] * this.e1234);
        float4 anti_wedge_g1 = (float4(right_anti_dual_g0[3]) * this.group1) + (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_anti_dual_g1.xyz, -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)));
        float4 anti_wedge_g2 = float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual_g0.xyz) + (right_anti_dual_g1.yzx * this.group3.zxy) - (right_anti_dual_g1.zxy * this.group3.yzx)), right_anti_dual_g0[3] * this.e1234);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            anti_wedge_g0 * float4(other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g1.xyz)), anti_wedge_g1[3] * other.scalar), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(anti_wedge_g1.www, anti_wedge_g3[3])) + float4((anti_wedge_g0.yzx * other.group1.zxy), -(anti_wedge_g1[0] * other.e15) - (anti_wedge_g1[1] * other.e25) - (anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g3.xyz * other.group0.www), anti_wedge_g0[3] * other.e3215) - float4((anti_wedge_g0.zxy * other.group1.yzx), anti_wedge_g2[0] * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(AntiPlane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = float3(this.e1234) * right_anti_dual_g0.xyz;
        float3 anti_wedge_g1_xyz = (right_anti_dual_g0.yzx * this.group3.zxy) - (right_anti_dual_g0.zxy * this.group3.yzx);
        float4 anti_wedge_g2 = float4((this.group3.xyz * right_anti_dual_g0.www), -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = (right_anti_dual_g0.zxyx * float4(this.group1.yzx, this.e15)) + (right_anti_dual_g0.wwwy * float4(this.group0.xyz, this.e25)) + float4((right_anti_dual_g0.yzx * this.group1.zxy * float3(-1.0)), (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45));
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12, e45 */
            float4(((anti_wedge_g3.zxy * other.group0.yzx) - (anti_wedge_g3.yzx * other.group0.zxy)), anti_wedge_g2[3] * other.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g3.xyz, (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1_xyz.yzx * other.group0.zxy), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) - (float4(other.e5) * float4(anti_wedge_g0, right_anti_dual_g0[3] * this.e1234)) - (other.group0.yzxx * float4(anti_wedge_g1_xyz.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_orthogonally_onto(Circle other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxy * float4(right_anti_dual_g1.zxy, other.e431)) + float4((other.group2.xyz * this.group2.www), other.e423 * this.e4235);
        float anti_wedge_g1 = -(right_anti_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g1 * other.e321) + (anti_wedge_g0[2] * other.e125)) + (anti_wedge_g0.yzxy * float4(other.group1.zxy, other.e315)) + float4((float3(anti_wedge_g1) * other.group0.xyz), anti_wedge_g0[0] * other.e235), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_orthogonally_onto(CircleRotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g0 = float4(-(right_anti_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_anti_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_anti_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_anti_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxx * float4(right_anti_dual_g1.zxy, other.e423)) + float4((right_anti_dual_g2.xyz * this.group2.www), other.e431 * this.e4315);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(anti_wedge_g0[2] * other.e425) - (anti_wedge_g0[3] * other.e235), -(anti_wedge_g0[0] * other.e435) - (anti_wedge_g0[3] * other.e315), -(anti_wedge_g0[1] * other.e415) - (anti_wedge_g0[3] * other.e125), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125)) + (float4(-(right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g2[0] * this.e4235) - (right_anti_dual_g2[1] * this.e4315) - (right_anti_dual_g2[2] * this.e4125)) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            -(anti_wedge_g0[0] * other.e423) - (anti_wedge_g0[1] * other.e431) - (anti_wedge_g0[2] * other.e412) - (anti_wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group3.yzx) - (right_anti_dual_g0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = float4((right_anti_dual_g2[1] * this.e4125) + (right_anti_dual_g1[0] * this.e3215), (right_anti_dual_g2[2] * this.e4235) + (right_anti_dual_g1[1] * this.e3215), (right_anti_dual_g2[0] * this.e4315) + (right_anti_dual_g1[2] * this.e3215), -(right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45)) - float4((right_anti_dual_g2.zxy * this.group3.yzx), right_anti_dual_g0[0] * this.e15);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(anti_wedge_g2[3]) * other.group2.xyz), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g0[1] * other.e35) + (anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42), (anti_wedge_g0[2] * other.e15) + (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43), (anti_wedge_g0[0] * other.e25) + (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) - float4((anti_wedge_g0.zxy * other.group2.yzx), anti_wedge_g1[0] * other.e15) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group3.yzx) - (right_anti_dual_g0.yzx * this.group3.zxy);
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[0] * this.e3215) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[1] * this.e3215) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[2] * this.e3215) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = (float4(this.e3215) * float4(right_anti_dual_g1.xyz, right_anti_dual_g2[3])) + (this.group3.zxyx * float4(right_anti_dual_g2.yzx, right_anti_dual_g3[0])) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e1234) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) - float4((right_anti_dual_g2.zxy * this.group3.yzx), right_anti_dual_g0[0] * this.e15);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(anti_wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(anti_wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * anti_wedge_g2.www), (anti_wedge_g2[3] * other.e1234) - (anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g2[3] * other.e4235), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g2[3] * other.e4315), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g2[3] * other.e4125), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + float4((anti_wedge_g0.yzx * other.group2.zxy), anti_wedge_g2[3] * other.e3215) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((other.group0.zxy * anti_wedge_g2.yzx), anti_wedge_g1[1] * other.e25)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_orthogonally_onto(DualNum other) {
        float2 right_anti_dual_g0 = other.group0.xy * float2(-1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(((float4(right_anti_dual_g0[0]) * float4(this.group0.xyz, this.e45)).xyz * float3(other.group0.xx, other.e5) * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g0[0] * other.e5 * this.e1234) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_orthogonally_onto(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_anti_dual_g0.xyz, -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45))) - (right_anti_dual_g0.wwwx * float4(this.group3.xyz, this.e41));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * anti_wedge_g0.xyz), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_orthogonally_onto(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = float4(float3(0.0), (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e1234) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43) - (right_anti_dual_g0[3] * this.e45)) + float4((right_anti_dual_g0.xyz * this.group2.www), right_anti_dual_g1[0] * this.e4235) - (right_anti_dual_g0.wwwx * float4(this.group3.xyz, this.e41));
        return Flector(
            /* e15, e25, e35, e45 */
            float4(anti_wedge_g0[3]) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0 * float4(other.group0.www, other.e3215)) + float4((other.group1.xyz * anti_wedge_g0.www), -(anti_wedge_g0[0] * other.e15) - (anti_wedge_g0[1] * other.e25) - (anti_wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Plane anti_project_orthogonally_onto(Line other) {
        float4 anti_wedge_g0 = float4((other.e435 * this.e4315) + (other.e235 * this.e1234), (other.e415 * this.e4125) + (other.e315 * this.e1234), (other.e425 * this.e4235) + (other.e125 * this.e1234), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) - (this.group3.zxyx * float4(other.group0.yzx, other.e235));
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g0.yzxx * float4(other.group0.zxy, other.e235)) + float4((other.group0.yzx * anti_wedge_g0.zxy * float3(-1.0)), (anti_wedge_g0[1] * other.e315) + (anti_wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Flector anti_project_orthogonally_onto(Motor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 anti_wedge_g1 = float4((right_anti_dual_g1[0] * this.e1234) + (right_anti_dual_g1[3] * this.e41), (right_anti_dual_g1[1] * this.e1234) + (right_anti_dual_g1[3] * this.e42), (right_anti_dual_g1[2] * this.e1234) + (right_anti_dual_g1[3] * this.e43), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((right_anti_dual_g0.zxy * this.group3.yzx), right_anti_dual_g1[3] * this.e45) - (this.group3.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((anti_wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (anti_wedge_g1.yzxx * float4(other.group0.zxy, other.e235)) + float4(float3(0.0), (anti_wedge_g1[1] * other.e315) + (anti_wedge_g1[2] * other.e125)) - float4((anti_wedge_g1.zxy * other.group0.yzx), right_anti_dual_g1[3] * other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public MultiVector anti_project_orthogonally_onto(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float4 right_anti_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        float anti_wedge_g0_x = (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45);
        float4 anti_wedge_g1 = (this.group3.yzxx * float4(other.group6.zxy, other.e423)) + float4((other.group8.xyz * this.group2.www), other.e431 * this.e4315) + float4((right_anti_dual_g9.zxy * this.group1.yzx), other.e412 * this.e4125) + float4((this.group0.xyz * right_anti_dual_g9.www), right_anti_dual_g3_w * this.e1234) - (float4(right_anti_dual_g10) * float4(this.group2.xyz, this.e45)) - (right_anti_dual_g9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group7.xyz * this.group3.www), right_anti_dual_g9[0] * this.e41) - float4((other.group6.yzx * this.group3.zxy), right_anti_dual_g9[1] * this.e42);
        float anti_wedge_g2 = (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45) - (right_anti_dual_g3_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125);
        float4 anti_wedge_g3 = float4((right_anti_dual_g8[1] * this.e4125) + (right_anti_dual_g6[0] * this.e3215), (right_anti_dual_g8[2] * this.e4235) + (right_anti_dual_g6[1] * this.e3215), (right_anti_dual_g8[0] * this.e4315) + (right_anti_dual_g6[2] * this.e3215), -(right_anti_dual_g6[1] * this.e4315) - (right_anti_dual_g6[2] * this.e4125)) + (float4(right_anti_dual_g0[1]) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0]));
        float3 anti_wedge_g4 = (float3(right_anti_dual_g0[1]) * this.group0.xyz) + (float3(this.e1234) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * this.group3.yzx) - (right_anti_dual_g7.yzx * this.group3.zxy);
        float3 anti_wedge_g5 = (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) + (float3(right_anti_dual_g0[1]) * this.group1.xyz) - (float3(right_anti_dual_g6[3]) * this.group3.xyz);
        float4 anti_wedge_g6 = (right_anti_dual_g9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_anti_dual_g9.zxy * this.group3.yzx), right_anti_dual_g10 * this.e3215);
        float3 anti_wedge_g7 = (float3(this.e1234) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * this.group3.xyz);
        float3 anti_wedge_g8 = (float3(right_anti_dual_g9[3]) * this.group3.xyz) - (float3(this.e3215) * right_anti_dual_g9.xyz);
        float4 anti_wedge_g9 = float4(right_anti_dual_g0[1]) * this.group3;
        float anti_wedge_g10 = right_anti_dual_g0[1] * this.e1234;
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
            (anti_wedge_g5 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group5.xyz) + (anti_wedge_g1.zxy * other.group1.yzx) - (anti_wedge_g1.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((anti_wedge_g2 * other.e41) + (anti_wedge_g4[0] * other.e5) + (anti_wedge_g1[3] * other.e15) + (anti_wedge_g3[0] * other.e4), (anti_wedge_g2 * other.e42) + (anti_wedge_g4[1] * other.e5) + (anti_wedge_g1[3] * other.e25) + (anti_wedge_g3[1] * other.e4), (anti_wedge_g2 * other.e43) + (anti_wedge_g4[2] * other.e5) + (anti_wedge_g1[3] * other.e35) + (anti_wedge_g3[2] * other.e4), -(anti_wedge_g5[2] * other.e3) - (anti_wedge_g1[0] * other.e23) - (anti_wedge_g1[1] * other.e31) - (anti_wedge_g1[2] * other.e12)) + (anti_wedge_g6 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group6) - (other.group1.xyzy * float4(anti_wedge_g3.www, anti_wedge_g5[1])) - float4((anti_wedge_g1.xyz * other.group3.www), anti_wedge_g5[0] * other.e1), 
            /* e423, e431, e412 */
            (anti_wedge_g5 * float3(other.e4)) + (anti_wedge_g7 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group7.xyz) + (float3(anti_wedge_g1[3]) * other.group5.xyz) + (anti_wedge_g4.yzx * other.group1.zxy) + (other.group4.yzx * anti_wedge_g1.zxy) - (anti_wedge_g4.zxy * other.group1.yzx) - (other.group4.zxy * anti_wedge_g1.yzx), 
            /* e235, e315, e125 */
            (anti_wedge_g5 * float3(other.e5)) + (anti_wedge_g8 * float3(other.scalar)) + (float3(anti_wedge_g0_x) * other.group8.xyz) + (float3(anti_wedge_g2) * other.group5.xyz) + (anti_wedge_g1.yzx * other.group3.zxy) + (anti_wedge_g3.zxy * other.group1.yzx) - (anti_wedge_g1.zxy * other.group3.yzx) - (anti_wedge_g3.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g2 * other.e423) + (anti_wedge_g1[1] * other.e435) + (anti_wedge_g3[3] * other.e23) + (anti_wedge_g6[1] * other.e3), (anti_wedge_g2 * other.e431) + (anti_wedge_g1[2] * other.e415) + (anti_wedge_g3[3] * other.e31) + (anti_wedge_g6[2] * other.e1), (anti_wedge_g2 * other.e412) + (anti_wedge_g1[0] * other.e425) + (anti_wedge_g3[3] * other.e12) + (anti_wedge_g6[0] * other.e2), -(anti_wedge_g3[0] * other.e23) - (anti_wedge_g3[1] * other.e31) - (anti_wedge_g3[2] * other.e12) - (anti_wedge_g6[3] * other.e5)) + (anti_wedge_g9 * float4(other.scalar)) + (float4(anti_wedge_g0_x) * other.group9) + float4((anti_wedge_g5 * other.group3.www), anti_wedge_g1[1] * other.e315) + float4((anti_wedge_g8 * other.group1.www), anti_wedge_g1[2] * other.e125) + float4((anti_wedge_g4.yzx * other.group3.zxy), anti_wedge_g1[0] * other.e235) + float4((other.group4.yzx * anti_wedge_g3.zxy), anti_wedge_g2 * other.e321) - (other.group1.yzxz * float4(anti_wedge_g6.zxy, anti_wedge_g8[2])) - (other.group3.yzxx * float4(anti_wedge_g4.zxy, anti_wedge_g5[0])) - float4((anti_wedge_g7 * float3(other.e5)), anti_wedge_g5[1] * other.e25) - float4((other.group8.xyz * anti_wedge_g1.www), anti_wedge_g8[0] * other.e1) - float4((other.group4.zxy * anti_wedge_g3.yzx), anti_wedge_g5[2] * other.e35) - float4((anti_wedge_g1.zxy * other.group6.yzx), anti_wedge_g8[1] * other.e2), 
            /* e1234 */
            (anti_wedge_g0_x * other.e1234) + (anti_wedge_g10 * other.scalar) + (anti_wedge_g7[0] * other.e1) + (anti_wedge_g7[1] * other.e2) + (anti_wedge_g7[2] * other.e3) + (anti_wedge_g6[3] * other.e4) - (anti_wedge_g4[0] * other.e23) - (anti_wedge_g4[1] * other.e31) - (anti_wedge_g4[2] * other.e12) - (anti_wedge_g5[0] * other.e41) - (anti_wedge_g5[1] * other.e42) - (anti_wedge_g5[2] * other.e43) - (anti_wedge_g1[0] * other.e423) - (anti_wedge_g1[1] * other.e431) - (anti_wedge_g1[2] * other.e412) - (anti_wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Plane anti_project_orthogonally_onto(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e1234)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * this.group3.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_anti_dual_g0.zxy * this.group3.yzx), right_anti_dual_g1 * this.e3215);
        float4 anti_wedge_g2 = float4((this.group3.xyz * right_anti_dual_g0.www), -(right_anti_dual_g1 * this.e45) - (right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) - (right_anti_dual_g0.xyzx * float4(this.group3.www, this.e41));
        float4 anti_wedge_g3 = float4(-(right_anti_dual_g1 * this.e15) - (right_anti_dual_g0[1] * this.e12), -(right_anti_dual_g1 * this.e25) - (right_anti_dual_g0[2] * this.e23), -(right_anti_dual_g1 * this.e35) - (right_anti_dual_g0[0] * this.e31), (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45)) + (right_anti_dual_g0.zxyx * float4(this.group1.yzx, this.e15)) + (right_anti_dual_g0.wwwy * float4(this.group0.xyz, this.e25));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group0.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group0.yzxw) - float4((anti_wedge_g3.yzx * other.group0.zxy), anti_wedge_g2[3] * other.e5), 
            /* e15, e25, e35, e1234 */
            (float4(other.e5, other.e5, other.e5, 1.0) * float4(anti_wedge_g3.xyz, (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[1] * other.e3) + (anti_wedge_g2[0] * other.e4), (anti_wedge_g1[2] * other.e1) + (anti_wedge_g2[1] * other.e4), (anti_wedge_g1[0] * other.e2) + (anti_wedge_g2[2] * other.e4), -(anti_wedge_g2[1] * other.e2) - (anti_wedge_g2[2] * other.e3)) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (other.group0.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_orthogonally_onto(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            pow(float4(other.scalar), 2) * this.group0, 
            /* e23, e31, e12, e45 */
            pow(float4(other.scalar), 2) * this.group1, 
            /* e15, e25, e35, e1234 */
            pow(float4(other.scalar), 2) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            pow(float4(other.scalar), 2) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public Sphere anti_project_orthogonally_onto(Sphere other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float anti_wedge_g0 = (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(anti_wedge_g0) * other.group0, 
            /* e1234 */
            anti_wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public DipoleInversion anti_project_orthogonally_onto(VersorEven other) {
        float4 right_anti_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 anti_wedge_g0 = (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2[3]) * this.group3.xyz);
        float4 anti_wedge_g1 = (right_anti_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_anti_dual_g3.zxy, right_anti_dual_g2[3]));
        float4 anti_wedge_g2 = (this.group3.xyzx * float4(right_anti_dual_g3.www, right_anti_dual_g0[0])) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g1[3] * this.e1234) - (right_anti_dual_g3[0] * this.e41) - (right_anti_dual_g3[1] * this.e42) - (right_anti_dual_g3[2] * this.e43)) - float4((right_anti_dual_g3.xyz * this.group3.www), right_anti_dual_g2[3] * this.e45);
        float4 anti_wedge_g3 = (float4(right_anti_dual_g3[3]) * float4(this.group0.xyz, this.e45)) + (right_anti_dual_g3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_anti_dual_g2.xyz, right_anti_dual_g3[1])) + float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g3[0] * this.e15) - (float4(this.e3215) * float4(right_anti_dual_g0.xyz, right_anti_dual_g1[3])) - (right_anti_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_anti_dual_g1.yzx, right_anti_dual_g2[0])) - float4((right_anti_dual_g3.yzx * this.group1.zxy), right_anti_dual_g2[2] * this.e4125);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge_g3.xyz) - (float3(anti_wedge_g2[3]) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g3.zxyw * other.group3.yzxw) - float4((anti_wedge_g3.yzx * other.group3.zxy), anti_wedge_g2[3] * other.e5), 
            /* e15, e25, e35, e1234 */
            (other.group3.xyzx * float4(anti_wedge_g3.www, anti_wedge_g0[0])) + float4(float3(0.0), (anti_wedge_g0[1] * other.e2) + (anti_wedge_g0[2] * other.e3) + (anti_wedge_g1[3] * other.e4) - (anti_wedge_g3[0] * other.e423) - (anti_wedge_g3[1] * other.e431) - (anti_wedge_g3[2] * other.e412)) - float4((anti_wedge_g3.xyz * other.group2.www), anti_wedge_g2[3] * other.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(anti_wedge_g3[3]) * float4(other.group0.xyz, other.e321)) + (anti_wedge_g3.yzxz * float4(other.group1.zxy, other.e125)) + float4((anti_wedge_g1.yzx * other.group3.zxy), anti_wedge_g3[0] * other.e235) + float4((anti_wedge_g2.xyz * other.group3.www), anti_wedge_g3[1] * other.e315) - (float4(other.e5) * float4(anti_wedge_g0, anti_wedge_g1[3])) - (anti_wedge_g2.wwwy * float4(other.group2.xyz, other.e2)) - (other.group3.yzxx * float4(anti_wedge_g1.zxy, anti_wedge_g2[0])) - float4((anti_wedge_g3.zxy * other.group1.yzx), anti_wedge_g2[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiProjectOrthogonallyOntoInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.anti_project_orthogonally_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_project_orthogonally_onto(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 anti_wedge_g0 = (right_anti_dual_g0.zxyw * float4(this.group3.yzx, this.scalar)) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) + float4((right_anti_dual_g1.xyz * this.group2.www), right_anti_dual_g3[0] * this.e4235) + float4((this.group0.xyz * right_anti_dual_g0.www), right_anti_dual_g2[3] * this.e1234) - (right_anti_dual_g0.yzxx * float4(this.group3.zxy, this.e15));
        float4 anti_wedge_g1 = float4((right_anti_dual_g0[3] * this.e23) + (right_anti_dual_g2[0] * this.e1234), (right_anti_dual_g0[3] * this.e31) + (right_anti_dual_g2[1] * this.e1234), (right_anti_dual_g0[3] * this.e12) + (right_anti_dual_g2[2] * this.e1234), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + (right_anti_dual_g0 * float4(this.group3.www, this.e45)) - (right_anti_dual_g1.wwwx * this.group3.xyzx);
        float4 anti_wedge_g2 = float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual_g1.xyz) + (right_anti_dual_g2.yzx * this.group3.zxy) - (right_anti_dual_g2.zxy * this.group3.yzx)), right_anti_dual_g0[3] * this.e1234);
        float4 anti_wedge_g3 = float4(right_anti_dual_g0[3]) * this.group3;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(anti_wedge_g0[3]) * other.group0.xyz) + (float3(other.scalar) * anti_wedge_g0.xyz)), anti_wedge_g0[3] * other.scalar), 
            /* e23, e31, e12, e45 */
            (anti_wedge_g1 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group1), 
            /* e15, e25, e35, e1234 */
            (anti_wedge_g2 * float4(other.scalar)) + (float4(anti_wedge_g0[3]) * other.group2) + float4(float3(0.0), -(anti_wedge_g0[0] * other.e23) - (anti_wedge_g0[1] * other.e31) - (anti_wedge_g0[2] * other.e12) - (anti_wedge_g1[0] * other.e41) - (anti_wedge_g1[1] * other.e42) - (anti_wedge_g1[2] * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((anti_wedge_g1[0] * other.e45) + (anti_wedge_g1[3] * other.e23) + (anti_wedge_g2[2] * other.e42) + (anti_wedge_g3[0] * other.scalar), (anti_wedge_g1[1] * other.e45) + (anti_wedge_g1[3] * other.e31) + (anti_wedge_g2[0] * other.e43) + (anti_wedge_g3[1] * other.scalar), (anti_wedge_g1[2] * other.e45) + (anti_wedge_g1[3] * other.e12) + (anti_wedge_g2[1] * other.e41) + (anti_wedge_g3[2] * other.scalar), -(anti_wedge_g1[2] * other.e35) - (anti_wedge_g2[0] * other.e23) - (anti_wedge_g2[1] * other.e31) - (anti_wedge_g2[2] * other.e12)) + (anti_wedge_g0.yzxw * float4(other.group2.zxy, other.e3215)) + float4((other.group3.xyz * anti_wedge_g0.www), anti_wedge_g3[3] * other.scalar) - (other.group2.yzxx * float4(anti_wedge_g0.zxy, anti_wedge_g1[0])) - float4((anti_wedge_g2.yzx * other.group0.zxy), anti_wedge_g1[1] * other.e25)
        );
    }
}
