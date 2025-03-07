implementing cga3d;
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_dual_g2.xyz * this.group2.www;
        float wedge_g2_w = (right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), (wedge_g2_w * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_dual_g2.xyz * this.group2.www;
        float wedge_g2_w = (right_dual_g2[3] * this.scalar) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g3[0] * this.scalar) + (this.e35 * other.e431), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g3[1] * this.scalar) + (this.e15 * other.e412), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g3[2] * this.scalar) + (this.e25 * other.e423), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[2] * this.e12)) + float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g3[3] * this.scalar) - (right_dual_g2.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group0.zxy * this.group2.yzx), right_dual_g2[1] * this.e31);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2_w * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((wedge_g3.yzx * other.group2.zxy), wedge_g2_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_via_origin_onto(AntiDualNum other) {
        float4 wedge_g0 = float4(other.group0.xx, other.e3215, other.scalar) * float4(this.group0.xyz, this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.scalar), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge_g1.xyz, right_dual_g0[3] * this.scalar) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4((right_dual_g0[3] * this.e23) + (right_dual_g1[0] * this.scalar), (right_dual_g0[3] * this.e31) + (right_dual_g1[1] * this.scalar), (right_dual_g0[3] * this.e12) + (right_dual_g1[2] * this.scalar), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) + float4((this.group0.yzx * right_dual_g0.zxy), right_dual_g1[3] * this.scalar) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * other.group0.www * float3(-1.0)), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_dual_g0[3] * this.scalar * other.e321)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_dual_g0 * float3(this.scalar);
        float wedge_g0_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43))) + float4((this.group0.xyz * right_dual_g1.www), right_dual_g0[3] * this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * other.group0.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e3215) * wedge_g0.xyz)), wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public Scalar project_via_origin_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual_g0) * float4(other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (this.e42 * other.e125) + (this.e35 * other.e431), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (this.e43 * other.e235) + (this.e15 * other.e412), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (this.e41 * other.e315) + (this.e25 * other.e423), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (this.e12 * other.e125)) - float4((this.group0.zxy * other.group2.yzx), this.e23 * other.e235) - float4((other.group0.zxy * this.group2.yzx), this.e31 * other.e315);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e315) + (wedge_g3[3] * other.e415), (wedge_g3[0] * other.e125) + (wedge_g3[3] * other.e425), (wedge_g3[1] * other.e235) + (wedge_g3[3] * other.e435), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((other.group2.zxy * wedge_g3.yzx), wedge_g2_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = (float3(right_dual_g2[3]) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g2[3]) * this.group1);
        float wedge_g2_x = (right_dual_g2[0] * this.scalar) + (right_dual_g2[3] * this.e15);
        float wedge_g2_y = (right_dual_g2[1] * this.scalar) + (right_dual_g2[3] * this.e25);
        float wedge_g2_z = (right_dual_g2[2] * this.scalar) + (right_dual_g2[3] * this.e35);
        float wedge_g2_w = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (this.e35 * other.e431), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (this.e15 * other.e412), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (this.e25 * other.e423), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, right_dual_g1[0])) - float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g1[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e415) + (wedge_g0_xyz[0] * other.e12345), (wedge_g2_w * other.e425) + (wedge_g0_xyz[1] * other.e12345), (wedge_g2_w * other.e435) + (wedge_g0_xyz[2] * other.e12345), -(wedge_g2_y * other.e431) - (wedge_g2_z * other.e412) - (wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((other.group0.zxy * wedge_g3.yzx), right_dual_g2[3] * this.scalar * other.e12345) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g2_x * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g1[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g1[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g1[2] * other.e12345), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((other.group0.xyz * wedge_g3.www), wedge_g1[3] * other.e12345) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 wedge_g0 = right_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_dual_g2 * float3(this.scalar);
        float wedge_g2_w = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g0[0] * this.scalar) + (right_dual_g2[3] * this.e23) + (right_dual_g3[2] * this.e42), (right_dual_g0[1] * this.scalar) + (right_dual_g2[3] * this.e31) + (right_dual_g3[0] * this.e43), (right_dual_g0[2] * this.scalar) + (right_dual_g2[3] * this.e12) + (right_dual_g3[1] * this.e41), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) - float4((this.group0.zxy * right_dual_g3.yzx), right_dual_g2[0] * this.e41);
        float4 wedge_g1 = float4((right_dual_g1[0] * this.scalar) + (right_dual_g2[3] * this.e15), (right_dual_g1[1] * this.scalar) + (right_dual_g2[3] * this.e25), (right_dual_g1[2] * this.scalar) + (right_dual_g2[3] * this.e35), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + float4((this.group0.xyz * right_dual_g3.www), right_dual_g1[3] * this.scalar) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(right_dual_g3[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group2.zxy) - (right_dual_g3.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(right_dual_g3.xyz, right_dual_g2[3]);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[3] * this.scalar * other.e1234) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g2_xyz[0] * other.e41), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[0] * other.e3215), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[1] * other.e3215), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[2] * other.e3215), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (float4(wedge_g0[3]) * other.group1) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float4 wedge_g0 = float4(right_dual_g0[1]) * float4(this.group0.xyz, this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.e12345, wedge_g0[3] * other.e12345) * float4(wedge_g0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz * float3(right_dual_g0.yy, right_dual_g0[1]) * float3(other.group0.yy, other.e12345)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g0[0] * this.scalar * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g2_w = -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge_g2_w * other.e45), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group0.xyz), right_dual_g0[3] * this.scalar * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(((this.group0.yzx * right_dual_g1.zxy) - (this.group0.zxy * right_dual_g1.yzx)), -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45));
        float4 wedge_g1 = float4(float3(0.0), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((this.group0.xyz * right_dual_g1.www), right_dual_g0[3] * this.scalar) - (right_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(right_dual_g1[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g0.xyz) + (right_dual_g1.yzx * this.group2.zxy) - (right_dual_g1.zxy * this.group2.yzx);
        float3 wedge_g3_xyz = right_dual_g1.xyz * this.group2.www;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group1.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])) + float4(float3(0.0), (wedge_g3_xyz[1] * other.e4315) + (wedge_g3_xyz[2] * other.e4125) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * other.group1.zxy), 0.0) - float4((wedge_g2_xyz.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(Line other) {
        float3 wedge_g1_xyz = other.group0.xyz * this.group2.www;
        float wedge_g2_w = -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435);
        float4 wedge_g3 = float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group1.yzx), this.e15 * other.e415);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g2_w) * other.group1.xyz), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e315) + (wedge_g3[3] * other.e415), (wedge_g3[0] * other.e125) + (wedge_g3[3] * other.e425), (wedge_g3[1] * other.e235) + (wedge_g3[3] * other.e435), -(wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)) - float4((other.group1.zxy * wedge_g3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g0[3]) * float4(this.group0.xyz, this.scalar);
        float4 wedge_g1 = float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g0.xyz)), right_dual_g0[3] * this.e45);
        float wedge_g2_w = -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43);
        float4 wedge_g3 = (float4(this.e45, this.e45, this.e45, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12))) + (right_dual_g1.zxyw * float4(this.group0.yzx, this.scalar)) - (right_dual_g1.yzxx * float4(this.group0.zxy, this.e23));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0 * float4(float3(wedge_g2_w), wedge_g0[3])) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((wedge_g0.xyz * other.group0.www), wedge_g2_w * other.e5), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4((float3(wedge_g2_w) * other.group1.xyz), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(other.e12345) * float3((right_dual_g0[3] * this.e15) + (right_dual_g1[0] * this.scalar), (right_dual_g0[3] * this.e25) + (right_dual_g1[1] * this.scalar), (right_dual_g0[3] * this.e35) + (right_dual_g1[2] * this.scalar))) + (wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[1] * this.scalar) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float wedge_g2 = this.scalar * other.e3215;
        float4 wedge_g3 = (right_dual_g3 * float4(this.scalar)) + (float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45));
        float3 wedge_g4 = (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz);
        float3 wedge_g5 = (float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group6.xyz);
        float4 wedge_g6 = float4((right_dual_g1[3] * this.e15) + (right_dual_g6[0] * this.scalar), (right_dual_g1[3] * this.e25) + (right_dual_g6[1] * this.scalar), (right_dual_g1[3] * this.e35) + (right_dual_g6[2] * this.scalar), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((float3(other.e3215) * this.group0.xyz), right_dual_g6[3] * this.scalar) - (right_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g1[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g1.zxy) - (this.group0.zxy * right_dual_g1.yzx);
        float3 wedge_g8 = (right_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + (right_dual_g1.yzx * this.group2.zxy) - (right_dual_g1.zxy * this.group2.yzx);
        float4 wedge_g9 = float4((right_dual_g3[3] * this.e23) + (right_dual_g9[0] * this.scalar) + (this.e45 * other.e415) + (this.e35 * other.e431), (right_dual_g3[3] * this.e31) + (right_dual_g9[1] * this.scalar) + (this.e45 * other.e425) + (this.e15 * other.e412), (right_dual_g3[3] * this.e12) + (right_dual_g9[2] * this.scalar) + (this.e45 * other.e435) + (this.e25 * other.e423), -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12) - (this.e35 * other.e435)) + float4((this.group0.yzx * right_dual_g3.zxy), right_dual_g9[3] * this.scalar) - (this.group2.yzxy * float4(other.group7.zxy, other.e425)) - float4((this.group0.zxy * right_dual_g3.yzx), this.e15 * other.e415);
        float wedge_g10 = -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_dual_g0[0] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (this.group0.yzx * right_dual_g0.zxy) - (this.group0.zxy * right_dual_g0.yzx);
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_dual_g0.yzx * this.group2.zxy), 0.0) - float4((right_dual_g0.zxy * this.group2.yzx), 0.0);
        float4 wedge_g3 = right_dual_g0 * float4(this.scalar);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g0, -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125))) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(wedge_g1.xyz, wedge_g2[3])) + (other.group0.zxyx * float4(wedge_g2.yzx, wedge_g3[0])) + float4((wedge_g2.zxy * other.group0.yzx * float3(-1.0)), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public Scalar project_via_origin_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (this.scalar * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g0[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g0.zxy) - (this.group0.zxy * right_dual_g0.yzx);
        float4 wedge_g1 = float4((right_dual_g0[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g0[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g0[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (right_dual_g0.yzx * this.group2.zxy) - (right_dual_g0.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(right_dual_g0.xyz, other.e3215);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g2_xyz[0] * other.e1234), (wedge_g0[1] * other.e3215) + (wedge_g2_xyz[1] * other.e1234), (wedge_g0[2] * other.e3215) + (wedge_g2_xyz[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(wedge_g1.xyz, right_dual_g0[3] * this.scalar)) + (other.group0.zxyx * float4(wedge_g2_xyz.yzx, wedge_g3[0])) + float4((wedge_g2_xyz.zxy * other.group0.yzx * float3(-1.0)), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = (float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.scalar) * right_dual_g0.xyz);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0[3]) * this.group1);
        float4 wedge_g2 = (float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(right_dual_g2.xyz, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43))) + (this.group2 * float4(right_dual_g0.www, right_dual_g2[3]));
        float4 wedge_g3 = float4((right_dual_g0[1] * this.e35) + (right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g3[0] * this.scalar), (right_dual_g0[2] * this.e15) + (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g3[1] * this.scalar), (right_dual_g0[0] * this.e25) + (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g3[2] * this.scalar), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[2] * this.e12)) + float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g3[3] * this.scalar) - (right_dual_g2.yzxx * float4(this.group0.zxy, this.e23)) - float4((right_dual_g0.zxy * this.group2.yzx), right_dual_g2[1] * this.e31);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge_g3.yzx, right_dual_g0[3] * this.scalar)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((wedge_g0_xyz * other.group0.www), wedge_g2[3] * other.e5) + float4((other.group1.xyz * wedge_g2.www), wedge_g3[0] * other.e1) - (other.group0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.e12345) + (wedge_g2[3] * other.e235), (wedge_g1[1] * other.e12345) + (wedge_g2[3] * other.e315), (wedge_g1[2] * other.e12345) + (wedge_g2[3] * other.e125), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (other.group0 * float4(wedge_g3.www, wedge_g1[3])) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g0[0] * this.scalar) + (right_dual_g3[3] * this.e23), (right_dual_g0[1] * this.scalar) + (right_dual_g3[3] * this.e31), (right_dual_g0[2] * this.scalar) + (right_dual_g3[3] * this.e12), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((this.group0.yzx * right_dual_g3.zxy), right_dual_g0[3] * this.scalar) - float4((this.group0.zxy * right_dual_g3.yzx), right_dual_g2[0] * this.e41);
        float4 wedge_g1 = float4((right_dual_g1[0] * this.scalar) + (right_dual_g3[3] * this.e15), (right_dual_g1[1] * this.scalar) + (right_dual_g3[3] * this.e25), (right_dual_g1[2] * this.scalar) + (right_dual_g3[3] * this.e35), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + float4((this.group0.xyz * right_dual_g2.www), right_dual_g1[3] * this.scalar) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group2.zxy) - (right_dual_g3.zxy * this.group2.yzx);
        float4 wedge_g3 = right_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) + float4((other.group0.xyz * wedge_g0.www), right_dual_g2[3] * this.scalar * other.e1234) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[3] * other.e23), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[3] * other.e31), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[3] * other.e12), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (wedge_g0 * float4(other.group3.www, other.e45)) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g1[1] * this.e3) + (right_dual_g2[0] * this.e4), (right_dual_g1[2] * this.e1) + (right_dual_g2[1] * this.e4), (right_dual_g1[0] * this.e2) + (right_dual_g2[2] * this.e4), -(right_dual_g1[3] * this.e5) - (right_dual_g2[2] * this.e3)) - (this.group3.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((right_dual_g0 * this.group3.www), right_dual_g2[0] * this.e1);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[1] * other.e42), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((right_dual_g2[0] * this.e4) + (other.e423 * this.e5), (right_dual_g2[1] * this.e4) + (other.e431 * this.e5), (right_dual_g2[2] * this.e4) + (other.e412 * this.e5), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = (this.group3.wwwx * float4(right_dual_g1.xyz, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((right_dual_g2.zxy * this.group3.yzx), right_dual_g3[3] * this.e4) - (right_dual_g2.yzxx * float4(this.group3.zxy, this.e423));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[3] * other.e1), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[3] * other.e2), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[3] * other.e3), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[2] * other.e435)) + (wedge_g2.yzxw * float4(other.group0.zxy, other.e5)) - (wedge_g2.zxyx * float4(other.group0.yzx, other.e415)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g2[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_via_origin_onto(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float4(other.e3215) * float4(this.group0.xyz, this.e321)).xyz * float3(other.group0.xx, other.e3215) * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e4) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (float4(this.e4, this.e4, this.e4, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321))) - (right_dual_g0.wwwx * float4(this.group3.xyz, this.e423));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((wedge_g0.xyz * other.group0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (float4(this.e4) * float4(right_dual_g0.xyz, right_dual_g1[3])) + float4(float3(0.0), (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (right_dual_g0.wwwx * float4(this.group3.xyz, this.e423));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(other.group0.www, other.e5)) + float4((other.group1.xyz * wedge_g0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group3.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4((other.group0.zxy * wedge_g0.yzx * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiFlector project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + (float4(right_dual_g1[3]) * float4(this.group0.xyz, this.e321)) - (this.group3.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35)) - float4((wedge_g1.yzx * other.group0.zxy), right_dual_g1[3] * this.e4 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane project_via_origin_onto(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0) * float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((this.e4 * other.e235) + (this.e5 * other.e423), (this.e4 * other.e315) + (this.e5 * other.e431), (this.e4 * other.e125) + (this.e5 * other.e412), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = float4((right_dual_g1[0] * this.e5) + (this.e2 * other.e125), (right_dual_g1[1] * this.e5) + (this.e3 * other.e235), (right_dual_g1[2] * this.e5) + (this.e1 * other.e315), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group2.yzx * this.group3.zxy), this.e423 * other.e235);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[1] * other.e412), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[2] * other.e423), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[0] * other.e431), -(wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e315) - float4((other.group0.yzx * wedge_g2.zxy), wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e4) + (right_dual_g2[3] * this.e423) + (this.e3 * other.e431), (right_dual_g1[1] * this.e4) + (right_dual_g2[3] * this.e431) + (this.e1 * other.e412), (right_dual_g1[2] * this.e4) + (right_dual_g2[3] * this.e412) + (this.e2 * other.e423), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group0.zxy * this.group3.yzx), this.e235 * other.e423);
        float4 wedge_g1 = float4((right_dual_g2[0] * this.e4) + (right_dual_g2[3] * this.e415), (right_dual_g2[1] * this.e4) + (right_dual_g2[3] * this.e425), (right_dual_g2[2] * this.e4) + (right_dual_g2[3] * this.e435), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((other.group0.xyz * this.group3.www), right_dual_g2[3] * this.e321) - (right_dual_g1.wwwx * this.group3.xyzx);
        float3 wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group2.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group3.yzx) - (right_dual_g2.yzx * this.group3.zxy);
        float4 wedge_g3 = float4(right_dual_g2[3]) * float4(this.group3.xyz, this.e4);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g2_xyz, -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125))) + (other.group2 * float4(wedge_g0.www, right_dual_g2[3] * this.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g3[0] * other.e12345), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g3[1] * other.e12345), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g3[2] * other.e12345), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + float4((wedge_g2_xyz.yzx * other.group0.zxy), wedge_g3[3] * other.e12345) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g1[0] * other.e423) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g2[0] * this.e4) + (right_dual_g1[1] * this.e3), (right_dual_g2[1] * this.e4) + (right_dual_g1[2] * this.e1), (right_dual_g2[2] * this.e4) + (right_dual_g1[0] * this.e2), -(right_dual_g2[2] * this.e3) - (right_dual_g1[3] * this.e5)) - (this.group3.wwwx * float4(right_dual_g0, right_dual_g2[0])) - float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g2[1] * this.e2);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e41), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group3.xyz);
        float4 wedge_g1 = (right_dual_g3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(right_dual_g3.yzx, right_dual_g2[3]));
        float4 wedge_g2 = (this.group3.xyzx * float4(right_dual_g3.www, right_dual_g0[0])) + float4(float3(0.0), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g2[3] * this.e321) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412)) - float4((right_dual_g3.xyz * this.group3.www), right_dual_g3[0] * this.e423);
        float4 wedge_g3 = (right_dual_g3.yzxw * this.group1.zxyw) + (this.group2.wwwz * float4(right_dual_g2.xyz, right_dual_g3[2])) + float4((this.group0.xyz * right_dual_g3.www), right_dual_g3[0] * this.e235) + float4((right_dual_g1.yzx * this.group3.zxy), right_dual_g3[1] * this.e315) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0])) - float4((right_dual_g3.zxy * this.group1.yzx), right_dual_g2[2] * this.e3);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) + float4((other.group3.xyz * wedge_g3.www), wedge_g0[0] * other.e4235) - (wedge_g3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyx * float4(other.group1.yzx, other.e15)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (wedge_g1.yzxw * other.group3.zxyw) - (wedge_g2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge_g0 * other.group3.www), wedge_g2[0] * other.e4235) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[1] * other.e4315)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float4 wedge_g0 = float4(right_dual_g0.yy, right_dual_g0[1], right_dual_g0[0]) * float4(this.group0.xyz, this.e4);
        float4 wedge_g2 = float4(right_dual_g0[1]) * float4(this.group2.xyz, this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.e12345, (wedge_g0[3] * other.e5) + (wedge_g2[3] * other.e12345)) * float4(wedge_g2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[1]) * float4(other.e12345) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = right_dual_g0.xyz * this.group2.www;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3] * this.e4) * other.group0 * float4(-1.0), 
            /* e5 */
            (wedge_g0_xyz[0] * other.e15) + (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35) - (right_dual_g0[0] * this.e1 * other.e45) - (right_dual_g0[1] * this.e2 * other.e45) - (right_dual_g0[2] * this.e3 * other.e45) - (right_dual_g0[3] * this.e5 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(this.e4) * right_dual_g1.xyz;
        float3 wedge_g1_xyz = (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy);
        float4 wedge_g2 = float4(float3(0.0), -(right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((this.group3.xyz * right_dual_g1.www), right_dual_g0[3] * this.e4) - (right_dual_g1.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(right_dual_g1[3]) * float4(this.group0.xyz, this.e321)) + (right_dual_g1.yzxx * float4(this.group1.zxy, this.e235)) + (this.group2.wwwy * float4(right_dual_g0.xyz, right_dual_g1[1])) + float4(float3(0.0), (right_dual_g1[2] * this.e125) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3) - (right_dual_g0[3] * this.e5)) - float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g0[0] * this.e1);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group1.zxy) - (wedge_g3.zxy * other.group1.yzx)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(wedge_g3.www, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g3.xyz * other.group1.www), wedge_g2[3] * other.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g3[1] * other.e25) + (wedge_g3[2] * other.e35) + (wedge_g3[3] * other.e45) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group1.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, right_dual_g1[3] * this.e4)) - (wedge_g2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(Line other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435);
        float4 wedge_g2 = float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)) - float4((other.group1.yzx * this.group3.zxy), this.e423 * other.e235);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * wedge_g2.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group1.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_w * other.e415) + (wedge_g0[2] * other.e315), (wedge_g1_w * other.e425) + (wedge_g0[0] * other.e125), (wedge_g1_w * other.e435) + (wedge_g0[1] * other.e235), -(wedge_g1_xyz[0] * other.e235) - (wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g2[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (float4(this.e4, this.e4, this.e4, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412))) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g1[3] * this.e4);
        float4 wedge_g1 = (float4(right_dual_g0[3]) * this.group1) + (float4(this.e4, this.e4, this.e4, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e1) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3)));
        float4 wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * right_dual_g0.xyz) + (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy)), right_dual_g0[3] * this.e5);
        float4 wedge_g3 = float4(right_dual_g0[3]) * float4(this.group3.xyz, this.e4);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g3[0] * other.e12345), (wedge_g0[0] * other.e125) + (wedge_g3[1] * other.e12345), (wedge_g0[1] * other.e235) + (wedge_g3[2] * other.e12345), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + (other.group0 * float4(wedge_g1.www, wedge_g3[3])) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) - (right_dual_g3_w * this.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4);
        float4 wedge_g1 = float4(right_dual_g0[0]) * float4(this.group3.xyz, this.e4);
        float wedge_g2 = right_dual_g0[0] * this.e5;
        float4 wedge_g3 = (float4(other.e3215) * float4(this.group3.xyz, this.e4)) - (right_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group3.xyz);
        float3 wedge_g5 = (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy);
        float4 wedge_g6 = float4((this.e4 * other.e235) + (this.e5 * other.e423), (this.e4 * other.e315) + (this.e5 * other.e431), (this.e4 * other.e125) + (this.e5 * other.e412), -(this.e2 * other.e425) - (this.e3 * other.e435)) + (float4(right_dual_g0[0]) * this.group1) - float4((float3(right_dual_g3_w) * this.group3.xyz), this.e1 * other.e415);
        float3 wedge_g7 = (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx);
        float3 wedge_g8 = (float3(right_dual_g0[0]) * this.group2.xyz) + (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual_g8 * this.group2.www), right_dual_g1[0] * this.e235) + float4((right_dual_g6.yzx * this.group3.zxy), right_dual_g1[2] * this.e125) - (right_dual_g6.zxyw * this.group3.yzxw) - (this.group3.wwwx * float4(right_dual_g7, right_dual_g8[0])) - float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g8[1] * this.e2) - float4((this.group2.xyz * right_dual_g1.www), right_dual_g8[2] * this.e3);
        float wedge_g10 = (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3) + (right_dual_g6[3] * this.e4) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(this.e4) * right_dual_g0.xyz;
        float3 wedge_g1_xyz = (right_dual_g0.zxy * this.group3.yzx) - (right_dual_g0.yzx * this.group3.zxy);
        float4 wedge_g2 = float4((this.group3.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(right_dual_g0[3]) * float4(this.group0.xyz, this.e321)) + (right_dual_g0.yzxx * float4(this.group1.zxy, this.e235)) + float4((right_dual_g0.zxy * this.group1.yzx * float3(-1.0)), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group0.zxy) - (wedge_g3.zxy * other.group0.yzx)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g3.xyz, (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_xyz.zxy * other.group0.yzx), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) - (float4(other.e3215) * float4(wedge_g0, right_dual_g0[3] * this.e4)) - (other.group0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint project_via_origin_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group3.xyz);
        float4 wedge_g1 = float4((right_dual_g0.zxy * this.group3.yzx), this.e4 * other.e3215) - (right_dual_g0.yzxw * this.group3.zxyw);
        float4 wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g0.yzxx * float4(this.group1.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group0.zxy, other.e1234)) - (other.group0.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g3.xyz, (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1[2] * other.e4315) + (wedge_g2[0] * other.e1234), (wedge_g1[0] * other.e4125) + (wedge_g2[1] * other.e1234), (wedge_g1[1] * other.e4235) + (wedge_g2[2] * other.e1234), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (other.group0.zxyx * float4(wedge_g1.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (this.group3.zxyy * float4(right_dual_g0.yzx, right_dual_g3[1])) + float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[2] * this.e3) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412)) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g3[3] * this.e4) + float4((right_dual_g1.xyz * this.group2.www), right_dual_g3[0] * this.e1) - float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g2[0] * this.e423);
        float4 wedge_g1 = float4((right_dual_g0[0] * this.e5) + (right_dual_g2[0] * this.e4), (right_dual_g0[1] * this.e5) + (right_dual_g2[1] * this.e4), (right_dual_g0[2] * this.e5) + (right_dual_g2[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + (float4(right_dual_g0[3]) * this.group1) - (right_dual_g1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group3.yzx) - (right_dual_g2.yzx * this.group3.zxy)), right_dual_g0[3] * this.e5);
        float4 wedge_g3 = float4(right_dual_g0[3]) * float4(this.group3.xyz, this.e4);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g0[3] * other.e1) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g0[3] * other.e2) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g0[3] * other.e3) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + (other.group0.zxyw * float4(wedge_g2.yzx, wedge_g3[3])) + float4((wedge_g3.xyz * other.group0.www), wedge_g0[3] * other.e4) - (other.group0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * this.group3.xyz);
        float4 wedge_g1 = float4((right_dual_g3.zxy * this.group3.yzx), right_dual_g2[3] * this.e4) - (right_dual_g3.yzxw * this.group3.zxyw);
        float4 wedge_g2 = float4(float3(0.0), (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321)) + float4((this.group3.xyz * right_dual_g2.www), right_dual_g1[3] * this.e4) - (right_dual_g3.xyzx * float4(this.group3.www, this.e423));
        float4 wedge_g3 = (float4(right_dual_g2[3]) * float4(this.group0.xyz, this.e321)) + (right_dual_g3.yzxx * float4(this.group1.zxy, this.e235)) + (this.group2.wwwy * float4(right_dual_g2.xyz, right_dual_g3[1])) + float4((right_dual_g1.yzx * this.group3.zxy), right_dual_g3[2] * this.e125) - (float4(this.e5) * float4(right_dual_g0.xyz, right_dual_g1[3])) - (this.group3.yzxz * float4(right_dual_g1.zxy, right_dual_g2[2])) - float4((right_dual_g3.zxy * this.group1.yzx), right_dual_g2[0] * this.e1) - float4((this.group2.xyz * right_dual_g3.www), right_dual_g2[1] * this.e2);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge_g3.www, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g3[0] * other.e41) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - float4((wedge_g3.xyz * other.group3.www), wedge_g2[3] * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz * float3(-1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4(this.scalar) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[3] * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2[3] * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2[3] * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2[3] * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g3.yzx * other.group2.zxy), wedge_g2[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum project_via_origin_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * other.group0.www * float3(-1.0)), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (this.scalar * pow(other.e321, 2))), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * other.group0.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e3215) * wedge_g0.xyz)), wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum project_via_origin_onto(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, right_dual_g2[3]);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g2_xyz = right_dual_g2.xyz * float3(this.group0.yy, this.scalar);
        float wedge_g3_w = right_dual_g2[3] * this.e3215;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g0.xyz, -(wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321))) + float4(float3(0.0), wedge_g0[3] * other.e12345) - float4(float3(0.0), wedge_g2_xyz[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g3_w) * other.group0.xyz), wedge_g1[3] * other.e12345) + float4((float3(other.e12345) * wedge_g1.xyz), 0.0) - (other.group1.wwwx * float4(float3(0.0), wedge_g3_w)), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3_w) * other.group1.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g3_w * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz * float3(-1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(this.group0.yy, this.scalar, this.e3215) * float4((other.group0.xyz * float3(-1.0)), right_dual_g2[3]);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * float4(right_dual_g2.xyz, right_dual_g3[3]);
        float4 wedge_g3 = float4(this.scalar) * float4(right_dual_g3.xyz, right_dual_g2[3]);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((other.group0.xyz * wedge_g0.www), wedge_g2[3] * other.e1234) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g2[0] * other.e41), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g2[0] * other.e1234), (wedge_g0[1] * other.e3215) + (wedge_g2[1] * other.e1234), (wedge_g0[2] * other.e3215) + (wedge_g2[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (float4(wedge_g0[3]) * other.group1) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiDualNum project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * float2((right_dual_g0[0] * this.scalar) + (right_dual_g0[1] * this.e3215), right_dual_g0[1] * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * other.e45 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_via_origin_onto(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * wedge_g0.www * float3(-1.0)), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (wedge_g0[3] * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public AntiMotor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = right_dual_g0 * float4(this.scalar);
        float4 wedge_g1 = float4(this.group0.yy, this.scalar, (right_dual_g0[3] * this.e3215) + (right_dual_g1[3] * this.scalar)) * float4(right_dual_g1.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * wedge_g0.xyz), (wedge_g0[3] * other.e12345) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[1] * this.scalar) + (right_dual_g1[3] * this.e3215);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float wedge_g2 = this.scalar * other.e3215;
        float4 wedge_g3 = float4(this.scalar) * float4(other.group8.xyz, other.e321 * -1.0);
        float3 wedge_g4 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group6.xyz;
        float4 wedge_g6 = float4(this.scalar) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g7 = float3(this.scalar) * other.group4.xyz * float3(-1.0);
        float3 wedge_g8 = float3(this.scalar) * other.group3.xyz * float3(-1.0);
        float4 wedge_g9 = float4(this.group0.yy, this.scalar, (right_dual_g0[0] * this.e3215) + (right_dual_g9[3] * this.scalar)) * float4(right_dual_g9.xyz, 1.0);
        float wedge_g10 = this.scalar * other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_dual_g0[0] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (this.scalar * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_w = right_dual_g0[3] * this.e3215;
        float4 wedge_g3 = right_dual_g0 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (wedge_g3[0] * other.e4235) + (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) + (this.scalar * other.e3215 * other.e1234)), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), wedge_g0_w * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0_w) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = right_dual_g0 * float4(this.scalar);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4(this.group0.yy, this.scalar, (right_dual_g0[3] * this.e3215) + (right_dual_g3[3] * this.scalar)) * float4(right_dual_g3.xyz, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge_g3.yzx, wedge_g0[3])) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((wedge_g0.xyz * other.group0.www), wedge_g2[3] * other.e5) + float4((other.group1.xyz * wedge_g2.www), wedge_g3[0] * other.e1) - (other.group0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.e12345) + (wedge_g2[3] * other.e235), (wedge_g1[1] * other.e12345) + (wedge_g2[3] * other.e315), (wedge_g1[2] * other.e12345) + (wedge_g2[3] * other.e125), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (other.group0 * float4(wedge_g3.www, wedge_g1[3])) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiDualNum {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(this.group0.yy, this.scalar, (right_dual_g0[3] * this.scalar) + (right_dual_g3[3] * this.e3215)) * float4(right_dual_g0.xyz, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g3 = right_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) + float4((other.group0.xyz * wedge_g0.www), wedge_g2[3] * other.e1234) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[3] * other.e23) + (wedge_g2[0] * other.e1234), (wedge_g0[3] * other.e31) + (wedge_g2[1] * other.e1234), (wedge_g0[3] * other.e12) + (wedge_g2[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (wedge_g0 * float4(other.group3.www, other.e45)) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float wedge_g0 = (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_via_origin_onto(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float wedge_g0 = this.e321 * other.e321;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_via_origin_onto(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle project_via_origin_onto(Circle other) {
        float wedge_g0 = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(float3(0.0), right_dual_g2[3] * this.e321);
        float3 wedge_g2_xyz = this.group0.xyz * right_dual_g2.www;
        float wedge_g2_w = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(wedge_g2_w) * other.group0.xyz), wedge_g2_w * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2_w) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e235) + (wedge_g2_xyz[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g2_xyz[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g2_xyz[2] * other.e12345), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)), 
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e321) * wedge_g1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g2_xyz.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = this.group0.xyz * right_dual_g2.www * float3(-1.0);
        float wedge_g0_w = (right_dual_g3[0] * this.e235) + (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125) + (right_dual_g3[3] * this.e321);
        float wedge_g1 = right_dual_g2[3] * this.e321 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (wedge_g0_xyz * float3(other.e1234)) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0_xyz.yzx * other.group3.zxy), wedge_g0_w * other.e1234) - (other.group3.yzxw * float4(wedge_g0_xyz.zxy, wedge_g1)), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g0_w) * other.group3.xyz) - (wedge_g0_xyz * other.group3.www)), -(wedge_g0_w * other.e41) - (wedge_g1 * other.e45) - (wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge_g1 * other.e15) - (wedge_g0_xyz[1] * other.e12), -(wedge_g1 * other.e25) - (wedge_g0_xyz[2] * other.e23), -(wedge_g1 * other.e35) - (wedge_g0_xyz[0] * other.e31), (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35)) + (float4(wedge_g0_w) * float4(other.group0.xyz, other.e45)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g0_w * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_via_origin_onto(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_x = (right_dual_g1[0] * this.e235) + (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125) + (right_dual_g1[3] * this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(wedge_g0_x) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), wedge_g0_x * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector project_via_origin_onto(Motor other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(other.e12345), 
            /* e1, e2, e3, e5 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4(float3(0.0), right_dual_g0[0] * this.e321);
        float3 wedge_g8 = float3(right_dual_g0[0]) * this.group0.xyz;
        float4 wedge_g9 = float4((float3(right_dual_g1[3]) * this.group0.xyz * float3(-1.0)), (right_dual_g1[0] * this.e235) + (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125) + (this.e321 * other.e3215));
        float wedge_g10 = right_dual_g1[3] * this.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(((float3(wedge_g6[3]) * other.group6.xyz) + (float3(other.e321) * wedge_g6.xyz)), wedge_g10 * other.e45) * float4(1.0, 1.0, 1.0, -1.0)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4(float3(0.0), wedge_g9[2] * other.e43) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3((right_dual_g0[0] * this.e235) + (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125) + (right_dual_g0[3] * this.e321)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public Circle project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(right_dual_g0[3]) * this.group0.xyz * float3(-1.0);
        float wedge_g0_w = (right_dual_g0[0] * this.e235) + (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125) + (this.e321 * other.e3215);
        float wedge_g1 = right_dual_g0[3] * this.e321 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (wedge_g0_xyz * float3(other.e1234)) - (float3(wedge_g1) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0_xyz.yzx * other.group0.zxy), wedge_g0_w * other.e1234) - (other.group0.yzxw * float4(wedge_g0_xyz.zxy, wedge_g1)), 
            /* e235, e315, e125 */
            (float3(wedge_g0_w) * other.group0.xyz) - (wedge_g0_xyz * float3(other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(float3(0.0), right_dual_g0[3] * this.e321);
        float4 wedge_g2 = float4((float3(right_dual_g0[3]) * this.group0.xyz), (this.e321 * other.e321) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(wedge_g2[3]) * other.group0.xyz), wedge_g2[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.group0.www, other.e5)) + float4((other.group2.xyz * wedge_g2.www), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[1] * other.e412) + (wedge_g2[3] * other.e1), (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[2] * other.e423) + (wedge_g2[3] * other.e2), (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[0] * other.e431) + (wedge_g2[3] * other.e3), -(wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4(float3(0.0), wedge_g2[3] * other.e4) - float4((wedge_g2.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlatPoint {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(right_dual_g3[3]) * this.group0.xyz * float3(-1.0);
        float wedge_g0_w = (right_dual_g3[0] * this.e235) + (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125) + (this.e321 * other.e3215);
        float wedge_g1 = right_dual_g3[3] * this.e321 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (wedge_g0_xyz * float3(other.e1234)) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz.yzx * other.group3.zxy) - (wedge_g0_xyz.zxy * other.group3.yzx)), (wedge_g0_w * other.e1234) - (wedge_g1 * other.e3215)), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g0_w) * other.group3.xyz) - (wedge_g0_xyz * other.group3.www)), -(wedge_g0_w * other.e41) - (wedge_g1 * other.e45) - (wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge_g1 * other.e15) - (wedge_g0_xyz[1] * other.e12), -(wedge_g1 * other.e25) - (wedge_g0_xyz[2] * other.e23), -(wedge_g1 * other.e35) - (wedge_g0_xyz[0] * other.e31), (wedge_g0_w * other.e45) + (wedge_g0_xyz[2] * other.e35)) + (float4(wedge_g0_xyz, wedge_g0_w).wwwy * float4(other.group0.xyz, other.e25)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g0_w * other.e15)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiFlector> operator /(AntiFlector lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public RoundPoint project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g1.yzx * this.group1.zxy), -(right_dual_g1[3] * this.e5) - (right_dual_g2[2] * this.e3)) - (this.group1.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((right_dual_g0 * this.group1.www), right_dual_g2[0] * this.e1);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[1] * other.e42), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group1.xyzx);
        float4 wedge_g2 = (this.group1.yzxx * float4(right_dual_g2.zxy, right_dual_g3[0])) + (this.group1.wwwy * float4(right_dual_g1.xyz, right_dual_g3[1])) + float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[2] * this.e3) - (right_dual_g1[3] * this.e321) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((right_dual_g2.yzx * this.group1.zxy), other.e423 * this.e235);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[3] * other.e1), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[3] * other.e2), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[3] * other.e3), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[2] * other.e435)) + (wedge_g2.yzxw * float4(other.group0.zxy, other.e5)) - (wedge_g2.zxyx * float4(other.group0.yzx, other.e415)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g2[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(AntiFlatPoint other) {
        float4 wedge_g0 = float4(other.e321 * -1.0) * float4(this.group1.xyz, this.e321) * float4(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((wedge_g0.xyz * other.group0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((float3(right_dual_g0[3]) * this.group1.xyz * float3(-1.0)), (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) - (right_dual_g0[3] * this.e321));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(other.group0.www, other.e5)) + float4((other.group1.xyz * wedge_g0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiPlane project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g0.yzx * this.group1.zxy), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group1.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4((other.group0.zxy * wedge_g0.yzx * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4(float3(0.0), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((right_dual_g0.yzx * this.group1.zxy), right_dual_g1[3] * this.e321) - (this.group1.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35)) - float4((wedge_g1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiPlane project_via_origin_onto(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group1.xyzx);
        float4 wedge_g2 = float4((right_dual_g1[0] * this.e5) + (this.e2 * other.e125), (right_dual_g1[1] * this.e5) + (this.e3 * other.e235), (right_dual_g1[2] * this.e5) + (this.e1 * other.e315), -(right_dual_g1[3] * this.e321) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group2.yzx * this.group1.zxy), this.e235 * other.e423);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[1] * other.e412), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[2] * other.e423), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[0] * other.e431), -(wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e315) - float4((other.group0.yzx * wedge_g2.zxy), wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(this.e3 * other.e431, this.e1 * other.e412, this.e2 * other.e423, -(right_dual_g1[3] * this.e321) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group0.zxy * this.group1.yzx), this.e235 * other.e423);
        float4 wedge_g1 = float4(float3(0.0), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((other.group0.xyz * this.group1.www), right_dual_g2[3] * this.e321) - (right_dual_g1.wwwx * this.group1.xyzx);
        float3 wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group0.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group1.yzx) - (right_dual_g2.yzx * this.group1.zxy);
        float3 wedge_g3_xyz = this.group1.xyz * right_dual_g2.www;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g2_xyz, -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125))) + (other.group2 * float4(wedge_g0.www, right_dual_g2[3] * this.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g3_xyz[0] * other.e12345) + (wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g3_xyz[1] * other.e12345) + (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g3_xyz[2] * other.e12345) + (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g1[0] * other.e423) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public RoundPoint project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g1.yzx * this.group1.zxy), -(right_dual_g2[2] * this.e3) - (right_dual_g1[3] * this.e5)) - (this.group1.wwwx * float4(right_dual_g0, right_dual_g2[0])) - float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g2[1] * this.e2);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e41), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g2[3]) * this.group1.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy);
        float wedge_g1_w = right_dual_g2[3] * this.e5 * -1.0;
        float4 wedge_g2 = (this.group1.xyzx * float4(right_dual_g3.www, right_dual_g0[0])) + float4(float3(0.0), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)) - float4((right_dual_g3.xyz * this.group1.www), right_dual_g2[3] * this.e321);
        float4 wedge_g3 = float4(float3(0.0), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125) + (right_dual_g3[3] * this.e321) - (right_dual_g2[2] * this.e3)) + float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g3[0] * this.e235) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0]));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) + float4((other.group3.xyz * wedge_g3.www), wedge_g0[0] * other.e4235) - (wedge_g3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyx * float4(other.group1.yzx, other.e15)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (wedge_g2.wwwz * float4(other.group2.xyz, other.e4125)) - (other.group3.zxyw * float4(wedge_g1_xyz.yzx, wedge_g1_w)) - float4((wedge_g0 * other.group3.www), wedge_g2[0] * other.e4235) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[1] * other.e4315)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public DualNum project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return DualNum(
            /* e5, e12345 */
            (float2(-(right_dual_g0[0] * this.e1 * other.e45) - (right_dual_g0[1] * this.e2 * other.e45) - (right_dual_g0[2] * this.e3 * other.e45) - (right_dual_g0[3] * this.e5 * other.e45), 0.0) + float2(0.0, 1.0)) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = (right_dual_g1.zxy * this.group1.yzx) - (right_dual_g1.yzx * this.group1.zxy);
        float4 wedge_g1 = (right_dual_g1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125) + (right_dual_g1[3] * this.e321) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3) - (right_dual_g0[3] * this.e5)) - (this.group1.wwwx * float4(right_dual_g1.xyz, right_dual_g0[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((other.group1.xyz * wedge_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g1[3] * other.e45) - (other.group1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiPlane project_via_origin_onto(Line other) {
        float4 wedge_g0 = float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group1.zxyx * float4(other.group1.yzx, other.e415));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g0[0] * this.e5) + (right_dual_g1[2] * this.e2), (right_dual_g0[1] * this.e5) + (right_dual_g1[0] * this.e3), (right_dual_g0[2] * this.e5) + (right_dual_g1[1] * this.e1), -(right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3)) + (float4(right_dual_g0[3]) * this.group0) - (this.group1.zxyx * float4(right_dual_g1.yzx, right_dual_g0[0]));
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(other.e12345), 
            /* e1, e2, e3, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g1.xyz, -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))) + (other.group0 * float4(wedge_g0.www, wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) - (right_dual_g3_w * this.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4);
        float4 wedge_g1 = float4((this.group1.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0);
        float wedge_g2 = right_dual_g0[0] * this.e5;
        float4 wedge_g3 = float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.e5) * right_dual_g1.xyz)), right_dual_g1[3] * this.e5 * -1.0);
        float3 wedge_g4 = float3(right_dual_g1[3]) * this.group1.xyz * float3(-1.0);
        float3 wedge_g5 = (right_dual_g1.zxy * this.group1.yzx) - (right_dual_g1.yzx * this.group1.zxy);
        float4 wedge_g6 = float4(float3(0.0), -(this.e2 * other.e425) - (this.e3 * other.e435)) + float4((other.group7.xyz * this.group1.www), right_dual_g0[0] * this.e321) - float4((float3(right_dual_g3_w) * this.group1.xyz), this.e1 * other.e415);
        float3 wedge_g7 = (other.group7.yzx * this.group1.zxy) - (other.group7.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group1.yzx) - (other.group8.yzx * this.group1.zxy);
        float4 wedge_g9 = float4(float3(0.0), (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125) + (this.e321 * other.e3215) - (right_dual_g6[3] * this.e5)) + float4((right_dual_g6.yzx * this.group1.zxy), right_dual_g1[0] * this.e235) - (this.group1.wwwx * float4(right_dual_g7, right_dual_g8[0])) - float4((right_dual_g6.zxy * this.group1.yzx), right_dual_g8[2] * this.e3) - float4((this.group0.xyz * right_dual_g1.www), right_dual_g8[1] * this.e2);
        float wedge_g10 = (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3) - (right_dual_g1[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiFlector project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy);
        float4 wedge_g1 = (float4(this.e5, this.e5, this.e5, 1.0) * float4(right_dual_g0.xyz, (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125) + (right_dual_g0[3] * this.e321)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_dual_g0.wwwx * float4(this.group1.xyz, this.e235));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((other.group0.xyz * wedge_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0_xyz.zxy * other.group0.yzx), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (other.group0.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public RoundPoint project_via_origin_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group1.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy);
        float wedge_g1_w = right_dual_g0[3] * this.e5 * -1.0;
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) - (float3(this.e5) * right_dual_g0.xyz);
        float wedge_g2_w = right_dual_g0[3] * this.e321 * -1.0;
        float3 wedge_g3_xyz = float3(right_dual_g0[3]) * this.group0.xyz * float3(-1.0);
        float wedge_g3_w = (right_dual_g0[0] * this.e235) + (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125) + (this.e321 * other.e3215);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (wedge_g3_xyz * float3(other.e1234)) - (float3(wedge_g2_w) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g3_xyz.yzx * other.group0.zxy), wedge_g3_w * other.e1234) - (other.group0.yzxw * float4(wedge_g3_xyz.zxy, wedge_g2_w)), 
            /* e235, e315, e125, e4 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g3_xyz, (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(float3(wedge_g3_w), wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_xyz[2] * other.e4315) + (wedge_g2_xyz[0] * other.e1234), (wedge_g1_xyz[0] * other.e4125) + (wedge_g2_xyz[1] * other.e1234), (wedge_g1_xyz[1] * other.e4235) + (wedge_g2_xyz[2] * other.e1234), -(wedge_g2_xyz[1] * other.e4315) - (wedge_g2_xyz[2] * other.e4125)) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (other.group0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (this.group1.zxyx * float4(right_dual_g0.yzx, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[3] * this.e321)) - (right_dual_g0.zxyx * float4(this.group1.yzx, this.e235));
        float4 wedge_g1 = (right_dual_g0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group1.xyzx);
        float4 wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group1.yzx) - (right_dual_g2.yzx * this.group1.zxy)), right_dual_g0[3] * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g0[3] * other.e1) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g0[3] * other.e2) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g0[3] * other.e3) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + (other.group0.zxyw * float4(wedge_g2.yzx, 0.0)) + float4((this.group1.xyz * right_dual_g0.www * other.group0.www), wedge_g0[3] * other.e4) - (other.group0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiFlector {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g3[3]) * this.group1.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy);
        float wedge_g1_w = right_dual_g3[3] * this.e5 * -1.0;
        float4 wedge_g2 = (this.group1.xyzx * float4(right_dual_g2.www, right_dual_g0[0])) + float4(float3(0.0), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)) - (right_dual_g3 * float4(this.group1.www, this.e321));
        float4 wedge_g3 = float4(float3(0.0), (right_dual_g2[3] * this.e321) + (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125) - (right_dual_g1[3] * this.e5)) + float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g3[0] * this.e235) - (this.group1.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - (this.group1.wwwz * float4(right_dual_g0.xyz, right_dual_g2[2])) - float4((this.group0.xyz * right_dual_g3.www), right_dual_g2[0] * this.e1);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge_g3.www, wedge_g0[0])) + float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g3[0] * other.e41) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - float4((wedge_g3.xyz * other.group3.www), wedge_g2[3] * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiLine> operator /(AntiLine lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[3] * this.e23) + (other.e431 * this.e35), (right_dual_g1[3] * this.e31) + (other.e412 * this.e15), (right_dual_g1[3] * this.e12) + (other.e423 * this.e25), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - float4((other.group0.zxy * this.group1.yzx), right_dual_g2[0] * this.e23);
        float wedge_g1 = -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4((wedge_g0.yzx * other.group2.zxy * float3(-1.0)), (wedge_g1 * other.e5) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_via_origin_onto(AntiFlatPoint other) {
        float3 wedge_g0_xyz = float3(other.e321 * -1.0) * this.group0.xyz;
        return AntiLine(
            /* e23, e31, e12 */
            wedge_g0_xyz * float3(other.e321) * float3(-1.0), 
            /* e15, e25, e35 */
            (wedge_g0_xyz.zxy * other.group0.yzx) - (wedge_g0_xyz.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float3 wedge_g0_xyz = float3(other.e321 * -1.0) * this.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * other.group0.www * float3(-1.0)), (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz.zxy * other.group0.yzx) - (wedge_g0_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), wedge_g0_w * other.scalar), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual_g0) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public Dipole project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[3] * this.e23) + (this.e35 * other.e431), (right_dual_g1[3] * this.e31) + (this.e15 * other.e412), (right_dual_g1[3] * this.e12) + (this.e25 * other.e423), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (this.e31 * other.e315) - (this.e12 * other.e125)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e235);
        float wedge_g1 = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g1_xyz = this.group0.xyz * right_dual_g2.www;
        float3 wedge_g2_xyz = float3(right_dual_g2[3]) * this.group1.xyz;
        float wedge_g2_w = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4((right_dual_g1[3] * this.e23) + (this.e35 * other.e431), (right_dual_g1[3] * this.e31) + (this.e15 * other.e412), (right_dual_g1[3] * this.e12) + (this.e25 * other.e423), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - float4((this.group1.yzx * other.group0.zxy), right_dual_g2[0] * this.e23);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e415) + (wedge_g3[1] * other.e412), (wedge_g2_w * other.e425) + (wedge_g3[2] * other.e423), (wedge_g2_w * other.e435) + (wedge_g3[0] * other.e431), -(wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412)) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g2_xyz[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g1_xyz[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g1_xyz[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g1_xyz[2] * other.e12345), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((other.group0.xyz * wedge_g3.www), 0.0) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public Dipole project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g2[3]) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(right_dual_g2[3]) * this.group1.xyz;
        float wedge_g1_w = -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12);
        float4 wedge_g2 = float4((right_dual_g3[1] * this.e35) + (right_dual_g3[3] * this.e23), (right_dual_g3[2] * this.e15) + (right_dual_g3[3] * this.e31), (right_dual_g3[0] * this.e25) + (right_dual_g3[3] * this.e12), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - float4((this.group1.yzx * right_dual_g3.zxy), right_dual_g0[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[2] * other.e4315) + (wedge_g1_xyz[0] * other.e1234) + (wedge_g2[3] * other.e41), (wedge_g0[0] * other.e4125) + (wedge_g1_xyz[1] * other.e1234) + (wedge_g2[3] * other.e42), (wedge_g0[1] * other.e4235) + (wedge_g1_xyz[2] * other.e1234) + (wedge_g2[3] * other.e43), -(wedge_g1_w * other.e45) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[0] * other.e1234) + (wedge_g2[3] * other.e23), (wedge_g2[1] * other.e1234) + (wedge_g2[3] * other.e31), (wedge_g2[2] * other.e1234) + (wedge_g2[3] * other.e12), -(wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)) + float4((wedge_g0 * other.group3.www), wedge_g2[3] * other.e45) - (other.group3.xyzx * float4(wedge_g1_xyz, wedge_g1_w).wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(other.e3215)) + (float3(wedge_g2[3]) * other.group2.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g1[1] * this.e35) + (right_dual_g1[3] * this.e23), (right_dual_g1[2] * this.e15) + (right_dual_g1[3] * this.e31), (right_dual_g1[0] * this.e25) + (right_dual_g1[3] * this.e12), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - (right_dual_g1.zxyx * float4(this.group1.yzx, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0[3]) * float4(other.group1.xyz, other.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public FlatPoint project_via_origin_onto(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(-(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiMotor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = this.group0.xyz * right_dual_g0.www;
        float wedge_g1_w = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float3(other.e12345)), -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1_w) * other.group0.xyz) + (float3(right_dual_g0[3]) * float3(other.e12345) * this.group1.xyz)), wedge_g1_w * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float wedge_g0_y = -(right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12);
        float4 wedge_g3 = float4((this.group1.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0);
        float3 wedge_g5 = float3(right_dual_g0[0]) * this.group0.xyz;
        float4 wedge_g6 = float4((float3(right_dual_g1[3]) * this.group1.xyz), -(right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12));
        float3 wedge_g7 = float3(right_dual_g1[3]) * this.group0.xyz;
        float3 wedge_g8 = (float3(other.e3215) * this.group0.xyz) + (this.group1.zxy * right_dual_g1.yzx) - (this.group1.yzx * right_dual_g1.zxy);
        float4 wedge_g9 = float4((right_dual_g3_w * this.e23) + (this.e35 * other.e431), (right_dual_g3_w * this.e31) + (this.e15 * other.e412), (right_dual_g3_w * this.e12) + (this.e25 * other.e423), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group1.yzx * other.group7.zxy), this.e23 * other.e235);
        float wedge_g10 = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public AntiLine project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g0[1] * this.e35) + (right_dual_g0[3] * this.e23), (right_dual_g0[2] * this.e15) + (right_dual_g0[3] * this.e31), (right_dual_g0[0] * this.e25) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.zxyx * float4(this.group1.yzx, this.e23));
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0[3]) * other.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public Dipole project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(right_dual_g0[3]) * this.group1.xyz;
        float3 wedge_g2 = (float3(other.e3215) * this.group0.xyz) + (this.group1.zxy * right_dual_g0.yzx) - (this.group1.yzx * right_dual_g0.zxy);
        return Dipole(
            /* e41, e42, e43 */
            (wedge_g1_xyz * float3(other.e1234)) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g2[0] * other.e1234), (wedge_g0[1] * other.e3215) + (wedge_g2[1] * other.e1234), (wedge_g0[2] * other.e3215) + (wedge_g2[2] * other.e1234), -(wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)) - (other.group0.xyzx * float4(wedge_g1_xyz, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)).wwwx), 
            /* e15, e25, e35 */
            (wedge_g1_xyz * float3(other.e3215)) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g0.www), 0.0);
        float3 wedge_g2_xyz = float3(right_dual_g0[3]) * this.group1.xyz;
        float wedge_g2_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        float4 wedge_g3 = float4((right_dual_g0[1] * this.e35) + (right_dual_g1[3] * this.e23), (right_dual_g0[2] * this.e15) + (right_dual_g1[3] * this.e31), (right_dual_g0[0] * this.e25) + (right_dual_g1[3] * this.e12), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - float4((this.group1.yzx * right_dual_g0.zxy), right_dual_g2[0] * this.e23);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), wedge_g2_w * other.e5) + float4(float3(0.0), (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((float3(wedge_g2_w) * other.group1.xyz), wedge_g3[0] * other.e1) - float4((wedge_g3.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiLine {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g3[3]) * this.group0.xyz;
        float3 wedge_g1_xyz = float3(right_dual_g3[3]) * this.group1.xyz;
        float wedge_g1_w = -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12);
        float4 wedge_g2 = float4((right_dual_g2[3] * this.e23) + (right_dual_g3[1] * this.e35), (right_dual_g2[3] * this.e31) + (right_dual_g3[2] * this.e15), (right_dual_g2[3] * this.e12) + (right_dual_g3[0] * this.e25), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - float4((this.group1.yzx * right_dual_g3.zxy), right_dual_g1[0] * this.e23);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g1_xyz[0] * other.e1234) + (wedge_g2[3] * other.e41), (wedge_g1_xyz[1] * other.e1234) + (wedge_g2[3] * other.e42), (wedge_g1_xyz[2] * other.e1234) + (wedge_g2[3] * other.e43), -(wedge_g1_w * other.e45) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1_xyz[0] * other.e23) - (wedge_g1_xyz[1] * other.e31) - (wedge_g1_xyz[2] * other.e12) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0.zxy * other.group3.yzx), wedge_g2[3] * other.scalar) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[0] * other.e1234) + (wedge_g2[3] * other.e23), (wedge_g2[1] * other.e1234) + (wedge_g2[3] * other.e31), (wedge_g2[2] * other.e1234) + (wedge_g2[3] * other.e12), -(wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)) + float4((wedge_g0 * other.group3.www), wedge_g2[3] * other.e45) - (other.group3.xyzx * float4(wedge_g1_xyz, wedge_g1_w).wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(other.e3215)) + (float3(wedge_g2[3]) * other.group2.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiMotor> operator /(AntiMotor lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_dual_g2.xyz * this.group0.www;
        float wedge_g2_w = (right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), (wedge_g2_w * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_dual_g2.xyz * this.group0.www;
        float wedge_g2_w = (right_dual_g2[3] * this.scalar) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = float4((right_dual_g1[3] * this.e23) + (right_dual_g3[0] * this.scalar), (right_dual_g1[3] * this.e31) + (right_dual_g3[1] * this.scalar), (right_dual_g1[3] * this.e12) + (right_dual_g3[2] * this.scalar), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) + float4((other.group0.yzx * this.group1.zxy), right_dual_g3[3] * this.scalar) - float4((other.group0.zxy * this.group1.yzx), right_dual_g2[0] * this.e23);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2_w * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((wedge_g3.yzx * other.group2.zxy), wedge_g2_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiDualNum project_via_origin_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g1_xyz = float3(right_dual_g0[3]) * this.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge_g1_xyz, right_dual_g0[3] * this.scalar) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1_xyz.zxy * other.group0.yzx) - (wedge_g1_xyz.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = (float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12))) + (this.group0 * float4(right_dual_g0.www, right_dual_g1[3]));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * other.group0.www * float3(-1.0)), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_dual_g0[3] * other.e321 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_dual_g0 * float3(this.scalar);
        float wedge_g0_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = right_dual_g0.xyz * this.group0.www;
        float wedge_g0_w = (right_dual_g0[3] * this.scalar) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), (wedge_g0_w * other.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(other.e3215)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_via_origin_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = float4((right_dual_g1[3] * this.e23) + (this.e35 * other.e431), (right_dual_g1[3] * this.e31) + (this.e15 * other.e412), (right_dual_g1[3] * this.e12) + (this.e25 * other.e423), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (this.e31 * other.e315) - (this.e12 * other.e125)) - float4((other.group0.zxy * this.group1.yzx), this.e23 * other.e235);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e315) + (wedge_g3[3] * other.e415), (wedge_g3[0] * other.e125) + (wedge_g3[3] * other.e425), (wedge_g3[1] * other.e235) + (wedge_g3[3] * other.e435), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((other.group2.zxy * wedge_g3.yzx), wedge_g2_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, right_dual_g2[3]);
        float3 wedge_g1_xyz = (float3(right_dual_g2[3]) * this.group0.xyz) + (float3(this.scalar) * right_dual_g1.xyz);
        float wedge_g1_w = right_dual_g1[3] * this.scalar;
        float wedge_g2_x = (right_dual_g2[0] * this.scalar) + (right_dual_g2[3] * this.e15);
        float wedge_g2_y = (right_dual_g2[1] * this.scalar) + (right_dual_g2[3] * this.e25);
        float wedge_g2_z = (right_dual_g2[2] * this.scalar) + (right_dual_g2[3] * this.e35);
        float wedge_g2_w = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        float4 wedge_g3 = (this.group1.zxyw * float4(other.group0.yzx, right_dual_g2[3])) + float4((this.group0.xyz * right_dual_g1.www), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - (this.group1.yzxx * float4(other.group0.zxy, right_dual_g1[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e415) + (wedge_g0[0] * other.e12345), (wedge_g2_w * other.e425) + (wedge_g0[1] * other.e12345), (wedge_g2_w * other.e435) + (wedge_g0[2] * other.e12345), -(wedge_g1_w * other.e321) - (wedge_g2_y * other.e431) - (wedge_g2_z * other.e412) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((other.group0.zxy * wedge_g3.yzx), wedge_g0[3] * other.e12345) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g2_x * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g1_xyz[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g1_xyz[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g1_xyz[2] * other.e12345), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((other.group0.xyz * wedge_g3.www), wedge_g1_w * other.e12345) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * float3(wedge_g2_x, wedge_g2_y, wedge_g2_z)) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiCircleRotor project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz * float3(-1.0);
        float wedge_g2_w = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = (right_dual_g0 * this.group0.www) + (this.group0.xyz * right_dual_g2.www);
        float wedge_g0_w = (right_dual_g2[3] * this.e3215) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + float4((this.group1.xyz * right_dual_g2.www), -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12));
        float3 wedge_g2_xyz = (float3(right_dual_g3[3]) * this.group0.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group1.zxy) - (right_dual_g3.zxy * this.group1.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(right_dual_g3.xyz, right_dual_g2[3]);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge_g0_xyz.zxy, wedge_g3[1])) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g0_xyz[0] * other.e15) - (wedge_g0_xyz[1] * other.e25) - (wedge_g0_xyz[2] * other.e35) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((float3(wedge_g0_w) * other.group0.xyz), right_dual_g3[3] * this.scalar * other.e1234) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) - float4((wedge_g0_xyz.yzx * other.group3.zxy), wedge_g2_xyz[0] * other.e41), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0_xyz[0] * other.e3215) + (wedge_g2_xyz[0] * other.e1234), (wedge_g0_xyz[1] * other.e3215) + (wedge_g2_xyz[1] * other.e1234), (wedge_g0_xyz[2] * other.e3215) + (wedge_g2_xyz[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (float4(wedge_g0_w) * other.group1) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0_w) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0_w * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0_w) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * float4(right_dual_g0.yy, right_dual_g0[1], (right_dual_g0[0] * this.scalar) + (right_dual_g0[1] * this.e3215)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_via_origin_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * other.e45 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g1[1] * this.e35) + (right_dual_g1[3] * this.e23), (right_dual_g1[2] * this.e15) + (right_dual_g1[3] * this.e31), (right_dual_g1[0] * this.e25) + (right_dual_g1[3] * this.e12), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + (float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)) - (right_dual_g1.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * wedge_g0.www * float3(-1.0)), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (wedge_g0[3] * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(Line other) {
        float3 wedge_g0_xyz = other.group0.xyz * this.group0.www;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(-(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = (float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.scalar) * right_dual_g0.xyz);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float3(other.e12345)), (right_dual_g0[3] * this.scalar * other.e12345) - (wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[1] * this.scalar) + (right_dual_g1[3] * this.e3215) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float wedge_g2 = this.scalar * other.e3215;
        float4 wedge_g3 = float4(((float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz)), right_dual_g3_w * this.scalar);
        float3 wedge_g4 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g5 = (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group6.xyz);
        float4 wedge_g6 = (right_dual_g6 * float4(this.scalar)) + float4((this.group1.xyz * right_dual_g1.www), -(right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12));
        float3 wedge_g7 = (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g1[3]) * this.group0.xyz);
        float3 wedge_g8 = (float3(other.e3215) * this.group0.xyz) + (right_dual_g1.yzx * this.group1.zxy) - (float3(this.scalar) * other.group3.xyz) - (right_dual_g1.zxy * this.group1.yzx);
        float4 wedge_g9 = (float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(right_dual_g9.xyz, -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e25 * other.e425) - (this.e35 * other.e435))) + (this.group0 * float4(float3(right_dual_g3_w), right_dual_g9[3])) + (this.group1.zxyw * float4(other.group7.yzx, right_dual_g0[0])) - (this.group1.yzxx * float4(other.group7.zxy, other.e415));
        float wedge_g10 = -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_dual_g0[0] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public AntiMotor project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g0[1] * this.e35) + (right_dual_g0[3] * this.e23), (right_dual_g0[2] * this.e15) + (right_dual_g0[3] * this.e31), (right_dual_g0[0] * this.e25) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.zxyx * float4(this.group1.yzx, this.e23));
        float4 wedge_g1 = right_dual_g0 * float4(this.scalar);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * wedge_g0.www * float3(-1.0)), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_via_origin_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (this.scalar * other.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(right_dual_g0[3]) * float4(this.group0.xyz, this.e3215);
        float3 wedge_g1_xyz = float3(right_dual_g0[3]) * this.group1.xyz;
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group0.xyz) + (right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx);
        float4 wedge_g3 = right_dual_g0 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4((wedge_g0.yzx * other.group0.zxy * float3(-1.0)), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) + (this.scalar * other.e3215 * other.e1234)) + float4((wedge_g1_xyz * float3(other.e1234)), wedge_g3[1] * other.e4315), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[0] * other.e3215), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[1] * other.e3215), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[2] * other.e3215), -(wedge_g1_xyz[1] * other.e4315) - (wedge_g1_xyz[2] * other.e4125)) - (other.group0.xyzx * float4(wedge_g1_xyz, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)).wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g1_xyz * float3(other.e3215)) + (wedge_g2_xyz.yzx * other.group0.zxy) - (wedge_g2_xyz.zxy * other.group0.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = right_dual_g0 * float4(this.scalar);
        float3 wedge_g1_xyz = (float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.scalar) * right_dual_g1.xyz);
        float wedge_g1_w = right_dual_g1[3] * this.scalar;
        float4 wedge_g2 = (right_dual_g2 * float4(this.scalar)) + float4((this.group1.xyz * right_dual_g0.www), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12));
        float4 wedge_g3 = (this.group0 * float4(right_dual_g1.www, right_dual_g3[3])) + float4((right_dual_g0.yzx * this.group1.zxy), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) + float4((right_dual_g3.xyz * this.group0.www), right_dual_g0[3] * this.e3215) - float4((right_dual_g0.zxy * this.group1.yzx), right_dual_g2[0] * this.e23);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge_g3.yzx, wedge_g0[3])) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g1_w * other.e321) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((wedge_g0.xyz * other.group0.www), wedge_g2[3] * other.e5) + float4((other.group1.xyz * wedge_g2.www), wedge_g3[0] * other.e1) - (other.group0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1_xyz[0] * other.e12345) + (wedge_g2[3] * other.e235), (wedge_g1_xyz[1] * other.e12345) + (wedge_g2[3] * other.e315), (wedge_g1_xyz[2] * other.e12345) + (wedge_g2[3] * other.e125), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (other.group0 * float4(wedge_g3.www, wedge_g1_w)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiMotor {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (this.group0 * float4(right_dual_g3.www, right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((right_dual_g0.xyz * this.group0.www), right_dual_g3[3] * this.e3215);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + float4((this.group1.xyz * right_dual_g3.www), -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12));
        float3 wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group0.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group1.zxy) - (right_dual_g3.zxy * this.group1.yzx);
        float4 wedge_g3 = right_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) + float4((other.group0.xyz * wedge_g0.www), right_dual_g2[3] * this.scalar * other.e1234) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[3] * other.e23), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[3] * other.e31), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[3] * other.e12), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (wedge_g0 * float4(other.group3.www, other.e45)) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiPlane> operator /(AntiPlane lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g1.yzx * this.group0.zxy), -(right_dual_g1[3] * this.e5) - (right_dual_g2[2] * this.e3)) - (this.group0.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((right_dual_g0 * this.group0.www), right_dual_g2[0] * this.e1);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[1] * other.e42), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = (this.group0.yzxx * float4(right_dual_g2.zxy, right_dual_g3[0])) + (this.group0.wwwy * float4(right_dual_g1.xyz, right_dual_g3[1])) + float4((right_dual_g2.yzx * this.group0.zxy * float3(-1.0)), (right_dual_g2[3] * this.e5) + (right_dual_g3[2] * this.e3));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[3] * other.e1), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[3] * other.e2), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[3] * other.e3), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[2] * other.e435)) + (wedge_g2.yzxw * float4(other.group0.zxy, other.e5)) - (wedge_g2.zxyx * float4(other.group0.yzx, other.e415)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g2[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_via_origin_onto(AntiFlatPoint other) {
        float3 wedge_g0 = float3(other.e321 * -1.0) * this.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((wedge_g0 * float3(other.e321)), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((float3(other.e321 * -1.0) * this.group0.xyz * float3(-1.0)), (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(other.group0.www, other.e5)) + float4((other.group1.xyz * wedge_g0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g0.yzx * this.group0.zxy), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4((other.group0.zxy * wedge_g0.yzx * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4((right_dual_g0.yzx * this.group0.zxy), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35)) - float4((wedge_g1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_via_origin_onto(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_via_origin_onto(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group0.xyzx);
        float3 wedge_g2 = (float3(this.e5) * right_dual_g1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g2[1] * other.e412) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g2[2] * other.e423) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g2[0] * other.e431) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g1[0] * other.e423), 
            /* e5 */
            -(wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((float3(this.e5) * other.group0.xyz), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = float4((float3(this.e5) * right_dual_g1.xyz), 0.0) + float4((right_dual_g2.zxy * this.group0.yzx), 0.0) - float4((right_dual_g2.yzx * this.group0.zxy), 0.0);
        float4 wedge_g3 = float4(right_dual_g2[3]) * this.group0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(other.e12345), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(other.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * wedge_g2.xyz), (wedge_g2[3] * other.e12345) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[1] * other.e412) + (wedge_g3[0] * other.e12345), (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[2] * other.e423) + (wedge_g3[1] * other.e12345), (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[0] * other.e431) + (wedge_g3[2] * other.e12345), -(wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) + (other.group2.yzxw * float4(wedge_g0.zxy, wedge_g3[3])) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0])) - float4((other.group0.yzx * wedge_g2.zxy), wedge_g1[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g1.yzx * this.group0.zxy), -(right_dual_g2[2] * this.e3) - (right_dual_g1[3] * this.e5)) - (this.group0.wwwx * float4(right_dual_g0, right_dual_g2[0])) - float4((right_dual_g1.zxy * this.group0.yzx), right_dual_g2[1] * this.e2);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e41), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g2[3]) * this.group0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (right_dual_g3.zxy * this.group0.yzx) - (right_dual_g3.yzx * this.group0.zxy);
        float wedge_g1_w = right_dual_g2[3] * this.e5 * -1.0;
        float4 wedge_g2 = (float4(this.e5, this.e5, this.e5, 1.0) * float4(right_dual_g3.xyz, (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g3.www, right_dual_g0[0]));
        float4 wedge_g3 = float4((right_dual_g1.yzx * this.group0.zxy), -(right_dual_g2[1] * this.e2) - (right_dual_g2[2] * this.e3)) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0]));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) + float4((other.group3.xyz * wedge_g3.www), wedge_g0[0] * other.e4235) - (wedge_g3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyx * float4(other.group1.yzx, other.e15)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (wedge_g2.wwwz * float4(other.group2.xyz, other.e4125)) - (other.group3.zxyw * float4(wedge_g1_xyz.yzx, wedge_g1_w)) - float4((wedge_g0 * other.group3.www), wedge_g2[0] * other.e4235) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[1] * other.e4315)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_via_origin_onto(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public DualNum project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return DualNum(
            /* e5, e12345 */
            (float2(-(right_dual_g0[0] * this.e1 * other.e45) - (right_dual_g0[1] * this.e2 * other.e45) - (right_dual_g0[2] * this.e3 * other.e45) - (right_dual_g0[3] * this.e5 * other.e45), 0.0) + float2(0.0, 1.0)) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy);
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g1.www), -(right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3) - (right_dual_g0[3] * this.e5)) - (this.group0.wwwx * float4(right_dual_g1.xyz, right_dual_g0[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((other.group1.xyz * wedge_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0_xyz.zxy * other.group1.yzx), wedge_g1[3] * other.e45) - (other.group1.zxyx * float4(wedge_g0_xyz.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_via_origin_onto(Line other) {
        float4 wedge_g0 = float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group0.zxyx * float4(other.group1.yzx, other.e415));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiFlector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g0[0] * this.e5) + (right_dual_g1[2] * this.e2), (right_dual_g0[1] * this.e5) + (right_dual_g1[0] * this.e3), (right_dual_g0[2] * this.e5) + (right_dual_g1[1] * this.e1), -(right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3)) - (this.group0.zxyx * float4(right_dual_g1.yzx, right_dual_g0[0]));
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            wedge_g0 * float4(other.e12345), 
            /* e1, e2, e3, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g1.xyz, -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435))) + (other.group0 * float4(wedge_g0.www, wedge_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) - (this.e5 * other.e4);
        float4 wedge_g1 = float4((this.group0.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0);
        float wedge_g2 = right_dual_g0[0] * this.e5;
        float4 wedge_g3 = float4(((float3(other.e3215) * this.group0.xyz) - (float3(this.e5) * right_dual_g1.xyz)), right_dual_g1[3] * this.e5 * -1.0);
        float3 wedge_g4 = float3(right_dual_g1[3]) * this.group0.xyz * float3(-1.0);
        float3 wedge_g5 = (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy);
        float4 wedge_g6 = float4(((float3(this.e5) * other.group7.xyz) - (float3(other.e321 * -1.0) * this.group0.xyz)), -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435));
        float3 wedge_g7 = (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group0.yzx) - (other.group8.yzx * this.group0.zxy);
        float4 wedge_g9 = float4((right_dual_g6.yzx * this.group0.zxy), -(right_dual_g8[2] * this.e3) - (right_dual_g6[3] * this.e5)) - (this.group0.wwwx * float4(right_dual_g7, right_dual_g8[0])) - float4((right_dual_g6.zxy * this.group0.yzx), right_dual_g8[1] * this.e2);
        float wedge_g10 = (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiPlane project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy);
        float3 wedge_g1 = (float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e5) * right_dual_g0.xyz);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((wedge_g0.zxy * other.group0.yzx), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (other.group0.zxyx * float4(wedge_g0.yzx, wedge_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_via_origin_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) - (this.e5 * other.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public RoundPoint project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy);
        float wedge_g1_w = right_dual_g0[3] * this.e5 * -1.0;
        float3 wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e5) * right_dual_g0.xyz);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[0] * other.e3215) - (wedge_g1_xyz[1] * other.e4125), -(wedge_g0[1] * other.e3215) - (wedge_g1_xyz[2] * other.e4235), -(wedge_g0[2] * other.e3215) - (wedge_g1_xyz[0] * other.e4315), (wedge_g1_w * other.e1234) + (wedge_g0[2] * other.e4125)) + (other.group0.yzxy * float4(wedge_g1_xyz.zxy, wedge_g0[1])) + float4((wedge_g2 * float3(other.e1234)), wedge_g0[0] * other.e4235), 
            /* e5 */
            -(wedge_g1_w * other.e3215) - (wedge_g2[0] * other.e4235) - (wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g3[0])) + float4((right_dual_g0.zxy * this.group0.yzx * float3(-1.0)), (right_dual_g2[3] * this.e5) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3));
        float4 wedge_g1 = (float4(this.e5, this.e5, this.e5, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3))) - (right_dual_g1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = float4(((float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group0.yzx) - (right_dual_g2.yzx * this.group0.zxy)), right_dual_g0[3] * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g0[3] * other.e1) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g0[3] * other.e2) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g0[3] * other.e3) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + (other.group0.zxyw * float4(wedge_g2.yzx, 0.0)) + float4((this.group0.xyz * right_dual_g0.www * other.group0.www), wedge_g0[3] * other.e4) - (other.group0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(right_dual_g3[3]) * this.group0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (right_dual_g3.zxy * this.group0.yzx) - (right_dual_g3.yzx * this.group0.zxy);
        float wedge_g1_w = right_dual_g3[3] * this.e5 * -1.0;
        float4 wedge_g2 = (float4(this.e5, this.e5, this.e5, 1.0) * float4(right_dual_g3.xyz, (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g2.www, right_dual_g0[0]));
        float4 wedge_g3 = float4((right_dual_g1.yzx * this.group0.zxy), -(right_dual_g1[3] * this.e5) - (right_dual_g2[2] * this.e3)) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0])) - (this.group0.wwwy * float4(right_dual_g0.xyz, right_dual_g2[1]));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge_g3.www, wedge_g0[0])) + float4(float3(0.0), (wedge_g1_w * other.e1234) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) - (wedge_g3[0] * other.e41) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - float4((wedge_g3.xyz * other.group3.www), wedge_g2[3] * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1_xyz.zxy * other.group3.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, wedge_g1_w)) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0])) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<AntiScalar> operator /(AntiScalar lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(ProjectViaOriginOntoInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiScalar {
    public AntiScalar project_via_origin_onto(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(ProjectViaOriginOntoInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiScalar {
    public CircleRotor project_via_origin_onto(CircleRotor other) {
        float wedge_g0 = this.e12345 * other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiScalar {
    public DualNum project_via_origin_onto(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345 * other.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiScalar {
    public Motor project_via_origin_onto(Motor other) {
        float wedge_g0 = this.e12345 * other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(wedge_g0) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiScalar {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float wedge_g0 = other.group0.yx[0] * this.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge_g0) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group1, 
            /* e5 */
            wedge_g0 * other.e5, 
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * other.group3, 
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group6, 
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group9, 
            /* e1234 */
            wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension AntiScalar {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float wedge_g0 = this.e12345 * other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(wedge_g0) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Circle> operator /(Circle lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint project_via_origin_onto(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group0.xyz * float3(other.group0.xx, other.e3215) * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiFlatPoint project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiFlector project_via_origin_onto(AntiMotor other) {
        float4 wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g0.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4((wedge_g0.yzx * other.group0.zxy * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public Circle project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual_g0) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public Circle project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g2[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g2[3]) * this.group1;
        float3 wedge_g2_xyz = float3(right_dual_g2[3]) * this.group2.xyz;
        float wedge_g2_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2_w) * other.group0.xyz)), wedge_g2_w * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2_w) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e235) + (wedge_g2_xyz[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g2_xyz[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g2_xyz[2] * other.e12345), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g2_xyz[1] * other.e412) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g2_xyz[2] * other.e423) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g2_xyz[0] * other.e431) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g1[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g2[3] * this.e235) - (right_dual_g3[2] * this.e425), -(right_dual_g2[3] * this.e315) - (right_dual_g3[0] * this.e435), -(right_dual_g2[3] * this.e125) - (right_dual_g3[1] * this.e415), (right_dual_g3[2] * this.e125) + (right_dual_g3[3] * this.e321)) + (right_dual_g3.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual_g3.yzx * this.group1.zxy), right_dual_g3[1] * this.e315);
        float wedge_g1 = -(right_dual_g2[3] * this.e321) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g0.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g0.zxy, wedge_g1)), 
            /* e235, e315, e125, e4 */
            float4((other.group3.xyz * wedge_g0.www), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge_g1 * other.e15) - (wedge_g0[1] * other.e12), -(wedge_g1 * other.e25) - (wedge_g0[2] * other.e23), -(wedge_g1 * other.e35) - (wedge_g0[0] * other.e31), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) + (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public Circle project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g1.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual_g1.zxy * this.group1.yzx * float3(-1.0)), (right_dual_g1[2] * this.e125) + (right_dual_g1[3] * this.e321)) + float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g1[1] * this.e315);
        float wedge_g1 = -(right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) - (float3(other.e3215) * wedge_g0.xyz)), wedge_g1 * other.e45 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g1) * other.group0.xyz * float3(-1.0)), (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public Line project_via_origin_onto(Line other) {
        float wedge_g0 = -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public VersorEven project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float4 wedge_g2 = float4((float3(right_dual_g0[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g2[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.group0.www, other.e5)) + float4((other.group1.xyz * wedge_g2.www), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g1[3] * other.e435), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4(right_dual_g0[0]) * this.group1;
        float3 wedge_g7 = float3(right_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g8 = float3(right_dual_g0[0]) * this.group2.xyz;
        float4 wedge_g9 = float4(-(right_dual_g1[2] * this.e425) - (right_dual_g1[3] * this.e235), -(right_dual_g1[0] * this.e435) - (right_dual_g1[3] * this.e315), -(right_dual_g1[1] * this.e415) - (right_dual_g1[3] * this.e125), (right_dual_g1[2] * this.e125) + (this.e321 * other.e3215)) + (right_dual_g1.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e3215) * this.group0.xyz), right_dual_g1[0] * this.e235);
        float wedge_g10 = -(right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public Circle project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0.wwwx * float4(this.group0.xyz, this.e235)) + float4((right_dual_g0.zxy * this.group1.yzx * float3(-1.0)), (right_dual_g0[2] * this.e125) + (right_dual_g0[3] * this.e321)) + float4((right_dual_g0.yzx * this.group1.zxy), right_dual_g0[1] * this.e315);
        float wedge_g1 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public Circle project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[2] * this.e125) + (this.e321 * other.e3215)) + (right_dual_g0.yzxy * float4(this.group1.zxy, this.e315)) + float4((float3(other.e3215) * this.group0.xyz), right_dual_g0[0] * this.e235);
        float wedge_g1 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.yzx * other.group0.zxy), wedge_g0[3] * other.e1234) - (other.group0.yzxw * float4(wedge_g0.zxy, wedge_g1)), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float4 wedge_g2 = float4((float3(right_dual_g0[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2[3]) * other.group0.xyz)), wedge_g2[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.group0.www, other.e5)) + float4((other.group2.xyz * wedge_g2.www), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[1] * other.e412) + (wedge_g2[3] * other.e1), (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[2] * other.e423) + (wedge_g2[3] * other.e2), (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[0] * other.e431) + (wedge_g2[3] * other.e3), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((wedge_g0.zxy * other.group2.yzx), wedge_g2[3] * other.e4) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e415) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g0[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Circle {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g3[2] * this.e425) - (right_dual_g3[3] * this.e235), -(right_dual_g3[0] * this.e435) - (right_dual_g3[3] * this.e315), -(right_dual_g3[1] * this.e415) - (right_dual_g3[3] * this.e125), (right_dual_g2[3] * this.e321) + (right_dual_g3[2] * this.e125)) + (right_dual_g3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual_g2.www), right_dual_g3[0] * this.e235);
        float wedge_g1 = -(right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g0.yzxw * float4(other.group3.zxy, other.e1234)) - float4((wedge_g0.zxy * other.group3.yzx), wedge_g1 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4((other.group3.xyz * wedge_g0.www), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge_g1 * other.e15) - (wedge_g0[1] * other.e12), -(wedge_g1 * other.e25) - (wedge_g0[2] * other.e23), -(wedge_g1 * other.e35) - (wedge_g0[0] * other.e31), (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + (wedge_g0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<CircleRotor> operator /(CircleRotor lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint project_via_origin_onto(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group0.xyz * float3(other.group0.xx, other.e3215) * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiFlatPoint project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(-(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiFlector project_via_origin_onto(AntiMotor other) {
        float4 wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e321);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g0.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4((wedge_g0.yzx * other.group0.zxy * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public CircleRotor project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual_g0) * float4(other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public Circle project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g2[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g2[3]) * this.group1;
        float3 wedge_g2_xyz = this.group2.xyz * right_dual_g2.www;
        float wedge_g2_w = (right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2_w) * other.group0.xyz)), wedge_g2_w * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2_w) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((wedge_g2_w * other.e235) + (wedge_g2_xyz[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g2_xyz[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g2_xyz[2] * other.e12345), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g2_xyz[1] * other.e412) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g2_xyz[2] * other.e423) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g2_xyz[0] * other.e431) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g1[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g2[3] * this.e235) - (right_dual_g3[2] * this.e425), -(right_dual_g2[3] * this.e315) - (right_dual_g3[0] * this.e435), -(right_dual_g2[3] * this.e125) - (right_dual_g3[1] * this.e415), (right_dual_g3[2] * this.e125) + (right_dual_g3[3] * this.e321)) + (right_dual_g3.yzxy * float4(this.group1.zxy, this.e315)) + float4((this.group0.xyz * right_dual_g3.www), right_dual_g3[0] * this.e235);
        float wedge_g1 = -(right_dual_g2[3] * this.e321) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g0.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g0.zxy, wedge_g1)), 
            /* e235, e315, e125, e4 */
            float4((other.group3.xyz * wedge_g0.www), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge_g1 * other.e15) - (wedge_g0[1] * other.e12), -(wedge_g1 * other.e25) - (wedge_g0[2] * other.e23), -(wedge_g1 * other.e35) - (wedge_g0[0] * other.e31), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35)) + (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float4 wedge_g2 = float4(right_dual_g0[1]) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4((float3(right_dual_g0[1]) * this.group0.xyz), wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            wedge_g2 * float4(other.group0.yy, other.e12345, other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual_g1.zxy * this.group1.yzx * float3(-1.0)), (right_dual_g1[2] * this.e125) + (right_dual_g1[3] * this.e321)) + float4((this.group0.xyz * right_dual_g1.www), right_dual_g1[0] * this.e235);
        float wedge_g1 = -(right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) - (float3(other.e3215) * wedge_g0.xyz)), wedge_g1 * other.e45 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(wedge_g1) * other.group0.xyz * float3(-1.0)), (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public Line project_via_origin_onto(Line other) {
        float wedge_g0 = -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float4 wedge_g2 = float4((this.group2.xyz * right_dual_g0.www), (right_dual_g0[3] * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(wedge_g0, wedge_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g2[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.group0.www, other.e5)) + float4((other.group1.xyz * wedge_g2.www), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g1[3] * other.e435), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g0[0] * this.e12345) + (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4(right_dual_g0[0]) * this.group1;
        float3 wedge_g7 = float3(right_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g8 = float3(right_dual_g0[0]) * this.group2.xyz;
        float4 wedge_g9 = float4(-(right_dual_g1[2] * this.e425) - (right_dual_g1[3] * this.e235), -(right_dual_g1[0] * this.e435) - (right_dual_g1[3] * this.e315), -(right_dual_g1[1] * this.e415) - (right_dual_g1[3] * this.e125), (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g1.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g10 = -(right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g7.zxy * other.group8.yzx), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public Circle project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual_g0.zxy * this.group1.yzx * float3(-1.0)), (right_dual_g0[2] * this.e125) + (right_dual_g0[3] * this.e321)) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g0[0] * this.e235);
        float wedge_g1 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g1) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx)), wedge_g1 * other.e3215 * -1.0), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public Circle project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g0.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((wedge_g0.yzx * other.group0.zxy), wedge_g0[3] * other.e1234) - (other.group0.yzxw * float4(wedge_g0.zxy, wedge_g1)), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float4 wedge_g2 = float4((float3(right_dual_g0[3]) * this.group2.xyz), (right_dual_g0[3] * this.e12345) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412));
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((wedge_g0 * float3(other.e12345)) + (float3(wedge_g2[3]) * other.group0.xyz)), wedge_g2[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g2[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.group0.www, other.e5)) + float4((other.group2.xyz * wedge_g2.www), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[1] * other.e412) + (wedge_g2[3] * other.e1), (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[2] * other.e423) + (wedge_g2[3] * other.e2), (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[0] * other.e431) + (wedge_g2[3] * other.e3), -(wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)) + float4((wedge_g0.zxy * other.group2.yzx), wedge_g2[3] * other.e4) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g0[0] * other.e415) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g0[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g3[2] * this.e425) - (right_dual_g3[3] * this.e235), -(right_dual_g3[0] * this.e435) - (right_dual_g3[3] * this.e315), -(right_dual_g3[1] * this.e415) - (right_dual_g3[3] * this.e125), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g3.yzxx * float4(this.group1.zxy, this.e235));
        float wedge_g1 = -(right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g0.xyz) - (float3(wedge_g1) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g0.yzxw * float4(other.group3.zxy, other.e1234)) - float4((wedge_g0.zxy * other.group3.yzx), wedge_g1 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4((other.group3.xyz * wedge_g0.www), -(wedge_g1 * other.e45) - (wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(wedge_g1 * other.e15) - (wedge_g0[1] * other.e12), -(wedge_g1 * other.e25) - (wedge_g0[2] * other.e23), -(wedge_g1 * other.e35) - (wedge_g0[0] * other.e31), (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + (wedge_g0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Dipole> operator /(Dipole lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (other.e423 * this.e25), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - float4((other.group0.zxy * this.group2.yzx), right_dual_g1[1] * this.e25) - float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g1[0] * this.e15);
        float wedge_g1 = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4((wedge_g0.yzx * other.group2.zxy * float3(-1.0)), (wedge_g1 * other.e5) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public FlatPoint project_via_origin_onto(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group0.xyz * float3(other.group0.xx, other.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public AntiLine project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321) * wedge_g0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0.xyz * other.group0.www * float3(-1.0)), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public AntiLine project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(right_dual_g1[3]) * this.group0.xyz;
        float wedge_g0_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), (wedge_g0_w * other.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(other.e3215)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public Dipole project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual_g0) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public Dipole project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (other.e431 * this.e35) + (other.e125 * this.e42), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (other.e412 * this.e15) + (other.e235 * this.e43), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (other.e423 * this.e25) + (other.e315 * this.e41), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e235 * this.e23) - float4((other.group2.yzx * this.group0.zxy), other.e315 * this.e31);
        float wedge_g1 = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g2[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g2[3]) * this.group1;
        float3 wedge_g2_xyz = float3(right_dual_g2[3]) * this.group2.xyz;
        float wedge_g2_w = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (other.e423 * this.e25), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - float4((other.group0.zxy * this.group2.yzx), right_dual_g1[0] * this.e15) - float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g1[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e415) + (wedge_g0[0] * other.e12345) + (wedge_g3[1] * other.e412), (wedge_g2_w * other.e425) + (wedge_g0[1] * other.e12345) + (wedge_g3[2] * other.e423), (wedge_g2_w * other.e435) + (wedge_g0[2] * other.e12345) + (wedge_g3[0] * other.e431), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g2_xyz[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g1[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g1[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g1[2] * other.e12345), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((other.group0.xyz * wedge_g3.www), wedge_g1[3] * other.e12345) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public Dipole project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g2[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g3.zxy) - (this.group0.zxy * right_dual_g3.yzx);
        float4 wedge_g1 = float4((right_dual_g2[3] * this.e15) + (right_dual_g3[3] * this.e41), (right_dual_g2[3] * this.e25) + (right_dual_g3[3] * this.e42), (right_dual_g2[3] * this.e35) + (right_dual_g3[3] * this.e43), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) - (right_dual_g3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((right_dual_g3[1] * this.e35) + (right_dual_g3[3] * this.e23), (right_dual_g3[2] * this.e15) + (right_dual_g3[3] * this.e31), (right_dual_g3[0] * this.e25) + (right_dual_g3[3] * this.e12), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) - float4((this.group2.yzx * right_dual_g3.zxy), right_dual_g0[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[2] * other.e4315) + (wedge_g1[0] * other.e1234) + (wedge_g2[3] * other.e41), (wedge_g0[0] * other.e4125) + (wedge_g1[1] * other.e1234) + (wedge_g2[3] * other.e42), (wedge_g0[1] * other.e4235) + (wedge_g1[2] * other.e1234) + (wedge_g2[3] * other.e43), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[0] * other.e1234) + (wedge_g2[3] * other.e23), (wedge_g2[1] * other.e1234) + (wedge_g2[3] * other.e31), (wedge_g2[2] * other.e1234) + (wedge_g2[3] * other.e12), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * other.group3.www), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public Dipole project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public FlatPoint project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (this.group0.yzx * right_dual_g1.zxy) - (this.group0.zxy * right_dual_g1.yzx);
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g1.www), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - (right_dual_g1.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((right_dual_g1[1] * this.e35) + (right_dual_g1[3] * this.e23), (right_dual_g1[2] * this.e15) + (right_dual_g1[3] * this.e31), (right_dual_g1[0] * this.e25) + (right_dual_g1[3] * this.e12), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) - float4((this.group2.yzx * right_dual_g1.zxy), right_dual_g0[0] * this.e41);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((wedge_g0.zxy * other.group1.yzx) - (wedge_g0.yzx * other.group1.zxy)), -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * other.group1.www), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group1.zxy), 0.0) - float4((wedge_g2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public Dipole project_via_origin_onto(Line other) {
        float4 wedge_g0 = float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group1.yzx), this.e15 * other.e415);
        float wedge_g1 = -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g1) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g1) * other.group1.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group0.xyz) + (other.group1.yzx * wedge_g0.zxy) - (other.group1.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float wedge_g2_w = -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43);
        float4 wedge_g3 = float4((right_dual_g0[0] * this.e45) + (right_dual_g1[2] * this.e42), (right_dual_g0[1] * this.e45) + (right_dual_g1[0] * this.e43), (right_dual_g0[2] * this.e45) + (right_dual_g1[1] * this.e41), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - float4((this.group0.zxy * right_dual_g1.yzx), right_dual_g0[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((wedge_g0 * other.group0.www) + (float3(wedge_g2_w) * other.group0.xyz)), (wedge_g2_w * other.e5) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4((float3(wedge_g2_w) * other.group1.xyz), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (wedge_g3.zxy * other.group1.yzx) + (float3(right_dual_g0[3]) * float3(other.e12345) * this.group2.xyz) - (wedge_g3.yzx * other.group1.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = -(right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45);
        float4 wedge_g3 = float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(right_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g5 = float3(right_dual_g0[0]) * this.group1.xyz;
        float4 wedge_g6 = float4((right_dual_g1[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g1[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g1[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - (right_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (float3(right_dual_g1[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g1.zxy) - (this.group0.zxy * right_dual_g1.yzx);
        float3 wedge_g8 = (float3(other.e3215) * this.group1.xyz) + (this.group2.zxy * right_dual_g1.yzx) - (this.group2.yzx * right_dual_g1.zxy);
        float4 wedge_g9 = float4((right_dual_g3_w * this.e23) + (this.e42 * other.e125) + (this.e45 * other.e415) + (this.e35 * other.e431), (right_dual_g3_w * this.e31) + (this.e43 * other.e235) + (this.e45 * other.e425) + (this.e15 * other.e412), (right_dual_g3_w * this.e12) + (this.e41 * other.e315) + (this.e45 * other.e435) + (this.e25 * other.e423), -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group8.yzx), this.e15 * other.e415) - float4((this.group2.yzx * other.group7.zxy), this.e25 * other.e425);
        float wedge_g10 = -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public Dipole project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (this.group0.yzx * right_dual_g0.zxy) - (this.group0.zxy * right_dual_g0.yzx);
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float3 wedge_g2 = (float3(right_dual_g0[3]) * this.group1.xyz) + (this.group2.zxy * right_dual_g0.yzx) - (this.group2.yzx * right_dual_g0.zxy);
        return Dipole(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g0, -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125))) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public Dipole project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g0[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g0.zxy) - (this.group0.zxy * right_dual_g0.yzx);
        float4 wedge_g1 = float4((right_dual_g0[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g0[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g0[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float3 wedge_g2 = (float3(other.e3215) * this.group1.xyz) + (this.group2.zxy * right_dual_g0.yzx) - (this.group2.yzx * right_dual_g0.zxy);
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g2[0] * other.e1234), (wedge_g0[1] * other.e3215) + (wedge_g2[1] * other.e1234), (wedge_g0[2] * other.e3215) + (wedge_g2[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float3 wedge_g2_xyz = float3(right_dual_g0[3]) * this.group2.xyz;
        float wedge_g2_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        float4 wedge_g3 = float4((right_dual_g0[1] * this.e35) + (right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42), (right_dual_g0[2] * this.e15) + (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43), (right_dual_g0[0] * this.e25) + (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g1[0] * this.e15) - float4((this.group2.yzx * right_dual_g0.zxy), right_dual_g1[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((wedge_g0 * other.group0.www), wedge_g2_w * other.e5) + float4((float3(wedge_g2_w) * other.group1.xyz), wedge_g3[0] * other.e1) - float4((wedge_g3.zxy * other.group0.yzx), wedge_g0[0] * other.e235), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Dipole {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g3[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g3.zxy) - (this.group0.zxy * right_dual_g3.yzx);
        float4 wedge_g1 = float4((right_dual_g2[3] * this.e41) + (right_dual_g3[3] * this.e15), (right_dual_g2[3] * this.e42) + (right_dual_g3[3] * this.e25), (right_dual_g2[3] * this.e43) + (right_dual_g3[3] * this.e35), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) - (right_dual_g3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = float4((right_dual_g2[3] * this.e23) + (right_dual_g3[1] * this.e35), (right_dual_g2[3] * this.e31) + (right_dual_g3[2] * this.e15), (right_dual_g2[3] * this.e12) + (right_dual_g3[0] * this.e25), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) - float4((this.group2.yzx * right_dual_g3.zxy), right_dual_g2[0] * this.e41);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g1[0] * other.e1234) + (wedge_g2[3] * other.e41), (wedge_g1[1] * other.e1234) + (wedge_g2[3] * other.e42), (wedge_g1[2] * other.e1234) + (wedge_g2[3] * other.e43), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0.zxy * other.group3.yzx), wedge_g2[3] * other.scalar) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[0] * other.e1234) + (wedge_g2[3] * other.e23), (wedge_g2[1] * other.e1234) + (wedge_g2[3] * other.e31), (wedge_g2[2] * other.e1234) + (wedge_g2[3] * other.e12), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * other.group3.www), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (other.e423 * this.e25), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, right_dual_g1[0])) - float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g1[1] * this.e25);
        float wedge_g1 = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4((wedge_g0.yzx * other.group2.zxy * float3(-1.0)), (wedge_g1 * other.e5) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public AntiMotor project_via_origin_onto(AntiDualNum other) {
        float4 wedge_g0 = float4(other.e3215) * float4(this.group0.xyz, this.e1234);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.scalar), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public AntiLine project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321) * wedge_g0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g0.xyz * other.group0.www * float3(-1.0)), (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public AntiLine project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        return AntiLine(
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(right_dual_g1[3]) * this.group0.xyz;
        float wedge_g0_w = (right_dual_g1[3] * this.e1234) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), (wedge_g0_w * other.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0_xyz * float3(other.e3215)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g0) * float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public Dipole project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (other.e431 * this.e35) + (other.e125 * this.e42), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (other.e412 * this.e15) + (other.e235 * this.e43), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (other.e423 * this.e25) + (other.e315 * this.e41), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e235 * this.e23) - float4((other.group2.yzx * this.group0.zxy), other.e315 * this.e31);
        float wedge_g1 = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return Dipole(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g2[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g2[3]) * this.group1;
        float3 wedge_g2_xyz = this.group2.xyz * right_dual_g2.www;
        float wedge_g2_w = (right_dual_g2[3] * this.e1234) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (right_dual_g2[3] * this.e4235), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (right_dual_g2[3] * this.e4315), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (right_dual_g2[3] * this.e4125), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) + float4((other.group0.yzx * this.group2.zxy), right_dual_g2[3] * this.e3215) - (this.group2.yzxx * float4(other.group0.zxy, right_dual_g1[0])) - float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g1[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g2_w * other.e415) + (wedge_g0[0] * other.e12345) + (wedge_g3[1] * other.e412), (wedge_g2_w * other.e425) + (wedge_g0[1] * other.e12345) + (wedge_g3[2] * other.e423), (wedge_g2_w * other.e435) + (wedge_g0[2] * other.e12345) + (wedge_g3[0] * other.e431), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g2_xyz[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g1[0] * other.e12345), (wedge_g2_w * other.e315) + (wedge_g1[1] * other.e12345), (wedge_g2_w * other.e125) + (wedge_g1[2] * other.e12345), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((other.group0.xyz * wedge_g3.www), wedge_g1[3] * other.e12345) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public Dipole project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g2[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g3.zxy) - (this.group0.zxy * right_dual_g3.yzx);
        float4 wedge_g1 = float4((right_dual_g2[3] * this.e15) + (right_dual_g3[3] * this.e41), (right_dual_g2[3] * this.e25) + (right_dual_g3[3] * this.e42), (right_dual_g2[3] * this.e35) + (right_dual_g3[3] * this.e43), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) - (right_dual_g3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (right_dual_g3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((right_dual_g3.yzx * this.group2.zxy), right_dual_g2[3] * this.e3215) - (this.group2.yzxx * float4(right_dual_g3.zxy, right_dual_g0[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0[2] * other.e4315) + (wedge_g1[0] * other.e1234) + (wedge_g2[3] * other.e41), (wedge_g0[0] * other.e4125) + (wedge_g1[1] * other.e1234) + (wedge_g2[3] * other.e42), (wedge_g0[1] * other.e4235) + (wedge_g1[2] * other.e1234) + (wedge_g2[3] * other.e43), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[0] * other.e1234) + (wedge_g2[3] * other.e23), (wedge_g2[1] * other.e1234) + (wedge_g2[3] * other.e31), (wedge_g2[2] * other.e1234) + (wedge_g2[3] * other.e12), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * other.group3.www), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float4 wedge_g2 = float4(right_dual_g0[1]) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.e12345, other.e5) * float4((float3(right_dual_g0[1]) * this.group0.xyz), wedge_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public FlatPoint project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(-(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (this.group0.yzx * right_dual_g1.zxy) - (this.group0.zxy * right_dual_g1.yzx);
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g1.www), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - (right_dual_g1.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (right_dual_g1.yzxx * float4(this.group2.zxy, this.e4235)) + (right_dual_g1.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) - float4((right_dual_g1.zxy * this.group2.yzx), right_dual_g0[0] * this.e41);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((wedge_g0.zxy * other.group1.yzx) - (wedge_g0.yzx * other.group1.zxy)), -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * other.group1.www), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g2[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group1.zxy), 0.0) - float4((wedge_g2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public Dipole project_via_origin_onto(Line other) {
        float4 wedge_g0 = float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group1.yzx), this.e15 * other.e415);
        float wedge_g1 = -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g1) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g1) * other.group1.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group0.xyz) + (other.group1.yzx * wedge_g0.zxy) - (other.group1.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float wedge_g2_w = (right_dual_g0[3] * this.e1234) - (right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43);
        float4 wedge_g3 = float4((right_dual_g0[0] * this.e45) + (right_dual_g0[3] * this.e4235), (right_dual_g0[1] * this.e45) + (right_dual_g0[3] * this.e4315), (right_dual_g0[2] * this.e45) + (right_dual_g0[3] * this.e4125), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((this.group0.yzx * right_dual_g1.zxy), right_dual_g0[3] * this.e3215) - float4((this.group0.zxy * right_dual_g1.yzx), right_dual_g0[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((wedge_g0 * other.group0.www) + (float3(wedge_g2_w) * other.group0.xyz)), (wedge_g2_w * other.e5) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4((float3(wedge_g2_w) * other.group1.xyz), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (wedge_g3.zxy * other.group1.yzx) + (float3(other.e12345) * this.group2.xyz * right_dual_g0.www) - (wedge_g3.yzx * other.group1.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (this.e1234 * other.e3215) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45);
        float4 wedge_g3 = float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45);
        float3 wedge_g4 = float3(right_dual_g0[0]) * this.group0.xyz;
        float3 wedge_g5 = float3(right_dual_g0[0]) * this.group1.xyz;
        float4 wedge_g6 = float4((right_dual_g1[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g1[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g1[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - (right_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (float3(right_dual_g1[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g1.zxy) - (this.group0.zxy * right_dual_g1.yzx);
        float3 wedge_g8 = (float3(other.e3215) * this.group1.xyz) + (right_dual_g1.yzx * this.group2.zxy) - (right_dual_g1.zxy * this.group2.yzx);
        float4 wedge_g9 = float4((right_dual_g3_w * this.e23) + (this.e42 * other.e125) + (this.e45 * other.e415) + (this.e35 * other.e431), (right_dual_g3_w * this.e31) + (this.e43 * other.e235) + (this.e45 * other.e425) + (this.e15 * other.e412), (right_dual_g3_w * this.e12) + (this.e41 * other.e315) + (this.e45 * other.e435) + (this.e25 * other.e423), -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e35 * other.e435)) + (float4(right_dual_g0[0]) * this.group3) - (this.group2.yzxy * float4(other.group7.zxy, other.e425)) - float4((this.group0.zxy * other.group8.yzx), this.e15 * other.e415);
        float wedge_g10 = (right_dual_g0[0] * this.e1234) - (this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (this.group0.yzx * right_dual_g0.zxy) - (this.group0.zxy * right_dual_g0.yzx);
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (right_dual_g0.yzxx * float4(this.group2.zxy, this.e4235)) + (right_dual_g0.wwwy * float4(this.group1.xyz, this.e4315)) + float4((right_dual_g0.zxy * this.group2.yzx * float3(-1.0)), (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234));
        return DipoleInversion(
            /* e41, e42, e43 */
            (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g0, -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125))) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2.yzx * other.group0.zxy), 0.0) - float4((wedge_g2.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public DipoleInversion project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g0[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g0.zxy) - (this.group0.zxy * right_dual_g0.yzx);
        float4 wedge_g1 = float4((right_dual_g0[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g0[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g0[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + (right_dual_g0.yzxx * float4(this.group2.zxy, this.e4235)) + float4((right_dual_g0.zxy * this.group2.yzx * float3(-1.0)), (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215));
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group0.yzx) - (wedge_g0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g2[0] * other.e1234), (wedge_g0[1] * other.e3215) + (wedge_g2[1] * other.e1234), (wedge_g0[2] * other.e3215) + (wedge_g2[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group0.zxy) - (wedge_g2.zxy * other.group0.yzx)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(right_dual_g0[3]) * this.group0.xyz;
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float3 wedge_g2_xyz = float3(right_dual_g0[3]) * this.group2.xyz;
        float wedge_g2_w = (right_dual_g0[3] * this.e1234) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        float4 wedge_g3 = float4((right_dual_g0[1] * this.e35) + (right_dual_g0[3] * this.e4235) + (right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23), (right_dual_g0[2] * this.e15) + (right_dual_g0[3] * this.e4315) + (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31), (right_dual_g0[0] * this.e25) + (right_dual_g0[3] * this.e4125) + (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[2] * this.e12)) + float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g0[3] * this.e3215) - (right_dual_g2.yzxx * float4(this.group0.zxy, this.e23)) - float4((right_dual_g0.zxy * this.group2.yzx), right_dual_g2[1] * this.e31);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g3.yzxy * float4(other.group0.zxy, other.e2)) + float4(float3(0.0), (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[0] * other.e423) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) + float4((wedge_g0 * other.group0.www), wedge_g2_w * other.e5) + float4((float3(wedge_g2_w) * other.group1.xyz), wedge_g3[0] * other.e1) - float4((wedge_g3.zxy * other.group0.yzx), wedge_g0[0] * other.e235), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((wedge_g2_xyz * float3(other.e12345)) + (float3(wedge_g3[3]) * other.group1.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2_w * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DipoleInversion {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g3[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g3.zxy) - (this.group0.zxy * right_dual_g3.yzx);
        float4 wedge_g1 = float4((right_dual_g2[3] * this.e41) + (right_dual_g3[3] * this.e15), (right_dual_g2[3] * this.e42) + (right_dual_g3[3] * this.e25), (right_dual_g2[3] * this.e43) + (right_dual_g3[3] * this.e35), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) - (right_dual_g3.xyzx * this.group1.wwwx);
        float4 wedge_g2 = (float4(right_dual_g2[3]) * float4(this.group1.xyz, this.e1234)) + (right_dual_g3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) - float4((right_dual_g3.zxy * this.group2.yzx), right_dual_g2[0] * this.e41);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g1[0] * other.e1234) + (wedge_g2[3] * other.e41), (wedge_g1[1] * other.e1234) + (wedge_g2[3] * other.e42), (wedge_g1[2] * other.e1234) + (wedge_g2[3] * other.e43), -(wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g0.zxy * other.group3.yzx), wedge_g2[3] * other.scalar) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g0[0] * other.e15), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[0] * other.e1234) + (wedge_g2[3] * other.e23), (wedge_g2[1] * other.e1234) + (wedge_g2[3] * other.e31), (wedge_g2[2] * other.e1234) + (wedge_g2[3] * other.e12), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + float4((wedge_g0 * other.group3.www), wedge_g2[3] * other.e45) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g2[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g2[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g2[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<DualNum> operator /(DualNum lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_via_origin_onto(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.e5) * float4((other.group0.xyz * float3(-1.0)), other.e45) * float4(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group0.xyz, other.e4 * -1.0);
        float3 wedge_g1_xyz = (other.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(this.group0.xx, this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0[3]) * other.group2.xyz), (wedge_g0[3] * other.e4) - (wedge_g0[0] * other.e423) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[0] * other.e321) + (wedge_g0[3] * other.e1), (wedge_g0[1] * other.e321) + (wedge_g0[3] * other.e2), (wedge_g0[2] * other.e321) + (wedge_g0[3] * other.e3), -(wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125)) + float4((wedge_g1_xyz.yzx * other.group0.zxy), wedge_g0[3] * other.e5) - float4((wedge_g1_xyz.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public DualNum project_via_origin_onto(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345) * float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_via_origin_onto(Circle other) {
        float3 wedge_g0 = float3(this.e5) * other.group0.xyz;
        float3 wedge_g1 = float3(this.e5) * (other.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[0] * other.e321) + (wedge_g1[1] * other.e412), (wedge_g0[1] * other.e321) + (wedge_g1[2] * other.e423), (wedge_g0[2] * other.e321) + (wedge_g1[0] * other.e431), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) - float4((wedge_g1.zxy * other.group0.yzx), wedge_g0[0] * other.e423), 
            /* e5 */
            -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(this.group0.xx, this.e5, this.e12345) * float4(other.group0.xyz, right_dual_g2[3]);
        float4 wedge_g1 = float4(this.e5) * float4((other.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz, right_dual_g2[3]);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g1.xyz, -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435))) + (other.group2 * float4(wedge_g0.www, wedge_g1[3])), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[0] * other.e321) + (wedge_g1[1] * other.e412), (wedge_g0[1] * other.e321) + (wedge_g1[2] * other.e423), (wedge_g0[2] * other.e321) + (wedge_g1[0] * other.e431), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) - float4((other.group0.yzx * wedge_g1.zxy), wedge_g0[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_via_origin_onto(Dipole other) {
        float4 wedge_g0 = float4(this.e5) * float4((other.group0.xyz * float3(-1.0)), other.e45) * float4(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 wedge_g0 = float4(this.e5) * float4((other.group3 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, other.e1234) * float4(-1.0);
        float4 wedge_g1 = float4(this.e5) * float4((other.group0.xyz * float3(-1.0)), other.e45) * float4(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) - (wedge_g1.zxy * other.group3.yzx)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((other.group3.xyz * wedge_g1.www), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g1[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g1.zxyx * float4(other.group1.yzx, other.e15)) + (other.group2.wwwy * float4(wedge_g0.xyz, wedge_g1[1])) + float4(float3(0.0), (wedge_g1[2] * other.e35) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), wedge_g0[0] * other.e4235)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public DualNum project_via_origin_onto(DualNum other) {
        float2 wedge_g0 = float2(other.e12345 * -1.0) * this.group0.xy;
        return DualNum(
            /* e5, e12345 */
            float2((wedge_g0[0] * other.e12345) + (wedge_g0[1] * other.e5), wedge_g0[1] * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public DualNum project_via_origin_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e45 * other.e45 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public AntiFlector project_via_origin_onto(Flector other) {
        float3 wedge_g0_xyz = (other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz * float3(this.group0.xx, this.e5) * float3(-1.0);
        float4 wedge_g1 = float4(float3(0.0), this.e5 * other.e45) * float4(0.0, 0.0, 0.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float3(wedge_g1[3]) * other.group1.xyz) - (float3(other.e3215) * wedge_g1.xyz)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g1[0] * other.e15) + (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35) + (wedge_g1[3] * other.e45) - (wedge_g0_xyz[0] * other.e4235) - (wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public DualNum project_via_origin_onto(Line other) {
        float3 wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e5);
        return DualNum(
            /* e5, e12345 */
            float2(-(wedge_g0_xyz[0] * other.e415) - (wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Motor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_w = right_dual_g0[3] * this.e12345;
        float4 wedge_g1 = right_dual_g0 * float4(this.e5);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(wedge_g0_w) * other.group0.xyz), wedge_g0_w * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_w) * other.group1) + float4(float3(0.0), -(wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float wedge_g0_y = (right_dual_g0[0] * this.e12345) - (this.e5 * other.e4);
        float wedge_g2 = right_dual_g0[0] * this.e5;
        float4 wedge_g3 = float4(this.e5) * float4(other.group9.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g6 = float4((other.group7.xyz * float3(this.group0.xx, this.e5)), 0.0);
        float3 wedge_g8 = float3(this.e5) * other.group6.xyz;
        float4 wedge_g9 = float4(this.e5) * float4((other.group4.xyz * float3(-1.0)), other.e45) * float4(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g2 * other.e1234) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), wedge_g3[3] * other.e1234) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((float3(other.e1234) * wedge_g3.xyz), 0.0) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4(float3(0.0), wedge_g6[2] * other.e412) - float4(float3(0.0), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, 0.0)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public DualNum project_via_origin_onto(Plane other) {
        float3 wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.e5);
        return DualNum(
            /* e5, e12345 */
            float2(-(wedge_g0_xyz[0] * other.e4235) - (wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_via_origin_onto(RoundPoint other) {
        float wedge_g0 = this.e5 * other.e4 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public RoundPoint project_via_origin_onto(Sphere other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.e4235, other.e4315, other.e4125, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            wedge_g0 * float4(other.e1234), 
            /* e5 */
            -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(this.group0.xx, this.e5, (right_dual_g0[3] * this.e12345) - (this.e5 * other.e4)) * float4(right_dual_g0.xyz, 1.0);
        float4 wedge_g1 = float4(this.e5) * float4((other.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz, right_dual_g0[3]);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[3] * other.e1) + (wedge_g1[1] * other.e412), (wedge_g0[3] * other.e2) + (wedge_g1[2] * other.e423), (wedge_g0[3] * other.e3) + (wedge_g1[0] * other.e431), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + (wedge_g0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 wedge_g0 = float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(this.e5) * float4((other.group0 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, other.e45) * float4(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) - (wedge_g1.zxy * other.group3.yzx)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((other.group3.xyz * wedge_g1.www), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyy * float4(other.group1.yzx, other.e25)) + (wedge_g1.wwwz * float4(other.group0.xyz, other.e35)) + (other.group2.wwwx * float4(wedge_g0.xyz, wedge_g1[0])) + float4(float3(0.0), (wedge_g1[3] * other.e45) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), wedge_g0[0] * other.e4235)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<FlatPoint> operator /(FlatPoint lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (other.e423 * this.e25), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, right_dual_g1[0]));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4((wedge_g0.yzx * other.group2.zxy * float3(-1.0)), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_via_origin_onto(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public Dipole project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (other.e423 * this.e25), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, right_dual_g1[0]));
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        float4 wedge_g1 = float4((right_dual_g1[0] * this.e45) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (other.e423 * this.e25), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, right_dual_g1[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((other.group0.zxy * wedge_g1.yzx) - (other.group0.yzx * wedge_g1.zxy)), -(wedge_g0[0] * other.e423) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g1.www), wedge_g0[3] * other.e12345) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public Dipole project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((this.group0.xyz * (other.group2 * float4(-1.0, -1.0, -1.0, 1.0)).www), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45)) - (this.group0.wwwx * float4(right_dual_g3.xyz, right_dual_g0[0]));
        float3 wedge_g1_xyz = (right_dual_g3.yzx * this.group0.zxy) - (right_dual_g3.zxy * this.group0.yzx);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(wedge_g0.xyz, -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))) + float4((other.group0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + (float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(wedge_g1_xyz, -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125))), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_via_origin_onto(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_via_origin_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public Flector project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(this.e45) * float4(right_dual_g1.xyz, other.e45) * float4(-1.0);
        float3 wedge_g1_xyz = (right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g1_xyz[1] * other.e4125) + (wedge_g0[0] * other.e3215), (wedge_g1_xyz[2] * other.e4235) + (wedge_g0[1] * other.e3215), (wedge_g1_xyz[0] * other.e4315) + (wedge_g0[2] * other.e3215), -(wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) + (float4(wedge_g0[3]) * other.group0) - (other.group1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_via_origin_onto(Line other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz;
        float wedge_g0_w = -(this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_w * other.e415) + (wedge_g0_xyz[2] * other.e315), (wedge_g0_w * other.e425) + (wedge_g0_xyz[0] * other.e125), (wedge_g0_w * other.e435) + (wedge_g0_xyz[1] * other.e235), -(wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)) - float4((wedge_g0_xyz.yzx * other.group1.zxy), wedge_g0_w * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public Flector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4((float3(this.e45) * right_dual_g0.xyz), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g1[2] * other.e315) + (wedge_g1[3] * other.e415), (wedge_g1[0] * other.e125) + (wedge_g1[3] * other.e425), (wedge_g1[1] * other.e235) + (wedge_g1[3] * other.e435), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + (float4(right_dual_g0[3]) * float4(other.e12345) * this.group0) - (wedge_g1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float wedge_g0_y = -(right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (this.e45 * other.e45);
        float4 wedge_g3 = float4(other.group0.yx[0] * -1.0) * this.group0;
        float4 wedge_g6 = float4(((float3(right_dual_g1[3]) * this.group0.xyz) - (float3(this.e45) * right_dual_g1.xyz)), 0.0);
        float3 wedge_g8 = (right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx);
        float4 wedge_g9 = float4((this.e35 * other.e431) + (this.e45 * other.e415), (this.e15 * other.e412) + (this.e45 * other.e425), (this.e25 * other.e423) + (this.e45 * other.e435), -(this.e25 * other.e425) - (this.e35 * other.e435)) - (this.group0.yzxx * float4(other.group7.zxy, other.e415));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), wedge_g3[3] * other.e1234) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((float3(other.e1234) * wedge_g3.xyz), 0.0) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4(float3(0.0), wedge_g6[2] * other.e412) - float4(float3(0.0), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, 0.0)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public FlatPoint project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(this.e45) * right_dual_g0.xyz * float3(-1.0);
        float3 wedge_g1 = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g1[1] * other.e4125), (wedge_g0[1] * other.e3215) + (wedge_g1[2] * other.e4235), (wedge_g0[2] * other.e3215) + (wedge_g1[0] * other.e4315), -(wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public Dipole project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e45) * right_dual_g0.xyz);
        float3 wedge_g1 = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return Dipole(
            /* e41, e42, e43 */
            wedge_g0 * float3(other.e1234), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * float3(other.e1234)), -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)), 
            /* e15, e25, e35 */
            (wedge_g0 * float3(other.e3215)) + (wedge_g1.yzx * other.group0.zxy) - (wedge_g1.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g0[3]) * this.group0;
        float4 wedge_g1 = float4((right_dual_g0[1] * this.e35) + (right_dual_g1[0] * this.e45), (right_dual_g0[2] * this.e15) + (right_dual_g1[1] * this.e45), (right_dual_g0[0] * this.e25) + (right_dual_g1[2] * this.e45), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) - (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (wedge_g1[3] * other.e4) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0])), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(wedge_g1.www, wedge_g0[3])) + float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension FlatPoint {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((this.group0.xyz * right_dual_g3.www), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (this.e45 * other.e45)) - (this.group0.wwwx * float4(right_dual_g3.xyz, right_dual_g0[0]));
        float3 wedge_g1_xyz = (right_dual_g3.yzx * this.group0.zxy) - (right_dual_g3.zxy * this.group0.yzx);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.group2.www, other.scalar)) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) + float4((other.group0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + (float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(wedge_g1_xyz, -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125))), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Flector> operator /(Flector lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(wedge_g0) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (other.e423 * this.e25), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, right_dual_g1[0]));
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g0[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g0.zxyx * float4(other.group2.yzx, other.e1)) + float4((wedge_g0.yzx * other.group2.zxy * float3(-1.0)), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Dipole project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e45) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (other.e423 * this.e25), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, right_dual_g1[0]));
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(wedge_g0[3]) * other.group1.xyz) + (other.group2.yzx * wedge_g0.zxy) - (other.group2.zxy * wedge_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g2[3]) * this.group0;
        float4 wedge_g1 = float4((right_dual_g1[0] * this.e45) + (right_dual_g2[3] * this.e4235), (right_dual_g1[1] * this.e45) + (right_dual_g2[3] * this.e4315), (right_dual_g1[2] * this.e45) + (right_dual_g2[3] * this.e4125), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) + float4((other.group0.yzx * this.group0.zxy), right_dual_g2[3] * this.e3215) - (this.group0.yzxx * float4(other.group0.zxy, right_dual_g1[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((other.group0.zxy * wedge_g1.yzx) - (other.group0.yzx * wedge_g1.zxy)), -(wedge_g0[0] * other.e423) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((other.group0.xyz * wedge_g1.www), wedge_g0[3] * other.e12345) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Dipole project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45);
        return Dipole(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (float4(other.e1234) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45)) - (this.group0.wwwx * float4(right_dual_g3.xyz, right_dual_g0[0]));
        float3 wedge_g1_xyz = (right_dual_g3.yzx * this.group0.zxy) - (right_dual_g3.zxy * this.group0.yzx);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(wedge_g0.xyz, -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))) + float4((other.group0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + (float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(wedge_g1_xyz, -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125))), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public FlatPoint project_via_origin_onto(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e45 * this.e45 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(this.e45) * right_dual_g1.xyz * float3(-1.0);
        float wedge_g0_w = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) - (other.e45 * this.e45);
        float3 wedge_g1_xyz = (right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_xyz[0] * other.e3215) + (wedge_g1_xyz[1] * other.e4125), (wedge_g0_xyz[1] * other.e3215) + (wedge_g1_xyz[2] * other.e4235), (wedge_g0_xyz[2] * other.e3215) + (wedge_g1_xyz[0] * other.e4315), -(wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125)) + (float4(wedge_g0_w) * other.group0) - (other.group1.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0_xyz[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0_w) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public FlatPoint project_via_origin_onto(Line other) {
        float3 wedge_g0_xyz = float3(this.e45) * other.group0.xyz;
        float wedge_g0_w = -(this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_w * other.e415) + (wedge_g0_xyz[2] * other.e315), (wedge_g0_w * other.e425) + (wedge_g0_xyz[0] * other.e125), (wedge_g0_w * other.e435) + (wedge_g0_xyz[1] * other.e235), -(wedge_g0_xyz[1] * other.e425) - (wedge_g0_xyz[2] * other.e435)) - float4((wedge_g0_xyz.yzx * other.group1.zxy), wedge_g0_w * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = (right_dual_g0 * float4(this.group0.www, this.e3215)) + float4((this.group1.xyz * right_dual_g0.www), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35));
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g1[2] * other.e315) + (wedge_g1[3] * other.e415), (wedge_g1[0] * other.e125) + (wedge_g1[3] * other.e425), (wedge_g1[1] * other.e235) + (wedge_g1[3] * other.e435), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + (float4(right_dual_g0[3]) * float4(other.e12345) * this.group0) - (wedge_g1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float wedge_g0_y = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (this.e45 * other.e45);
        float4 wedge_g3 = float4(right_dual_g0[0]) * this.group0;
        float4 wedge_g6 = float4(((float3(right_dual_g1[3]) * this.group0.xyz) - (float3(this.e45) * right_dual_g1.xyz)), 0.0);
        float3 wedge_g8 = (right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx);
        float4 wedge_g9 = float4((this.e35 * other.e431) + (this.e45 * other.e415), (this.e15 * other.e412) + (this.e45 * other.e425), (this.e25 * other.e423) + (this.e45 * other.e435), -(this.e25 * other.e425) - (this.e35 * other.e435)) + (float4(right_dual_g0[0]) * this.group1) - (this.group0.yzxx * float4(other.group7.zxy, other.e415));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), wedge_g3[3] * other.e1234) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((float3(other.e1234) * wedge_g3.xyz), 0.0) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4(float3(0.0), wedge_g6[2] * other.e412) - float4(float3(0.0), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, 0.0)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(this.e45) * right_dual_g0.xyz * float3(-1.0);
        float3 wedge_g1_xyz = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g0_xyz[0] * other.e3215) + (wedge_g1_xyz[1] * other.e4125), (wedge_g0_xyz[1] * other.e3215) + (wedge_g1_xyz[2] * other.e4235), (wedge_g0_xyz[2] * other.e3215) + (wedge_g1_xyz[0] * other.e4315), -(wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125)) - (other.group0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0_xyz[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public DipoleInversion project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (float4(this.e45, this.e45, this.e45, 1.0) * float4(right_dual_g0.xyz, (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_dual_g0.wwwx * float4(this.group0.xyz, this.e4235));
        float3 wedge_g1_xyz = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e23, e31, e12, e45 */
            float4((wedge_g1_xyz * float3(other.e1234)), -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group0.zxy) - (wedge_g1_xyz.zxy * other.group0.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g0[3]) * this.group0;
        float4 wedge_g1 = float4((right_dual_g0[3] * this.e4235) + (right_dual_g1[0] * this.e45), (right_dual_g0[3] * this.e4315) + (right_dual_g1[1] * this.e45), (right_dual_g0[3] * this.e4125) + (right_dual_g1[2] * this.e45), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35)) + (right_dual_g0.yzxw * float4(this.group0.zxy, this.e3215)) - (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (wedge_g1[3] * other.e4) - (wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412) - (wedge_g0[3] * other.e321)) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0])), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(wedge_g1.www, wedge_g0[3])) + float4(float3(0.0), -(wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) - (wedge_g1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g1[3]) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * wedge_g0.xyz), 0.0) + float4((wedge_g1.zxy * other.group2.yzx), 0.0) - float4((wedge_g1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g1 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g3.wwwx * float4(this.group0.xyz, this.e4235)) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (this.e45 * other.e45)) - (this.group0.wwwx * float4(right_dual_g3.xyz, right_dual_g0[0]));
        float3 wedge_g1_xyz = (right_dual_g3.yzx * this.group0.zxy) - (right_dual_g3.zxy * this.group0.yzx);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0 * float4(other.group2.www, other.scalar)) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e41) - (wedge_g1_xyz[1] * other.e42) - (wedge_g1_xyz[2] * other.e43) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)) + float4((other.group0.xyz * wedge_g0.www), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(wedge_g0[3]) * other.group1) + (float4(other.e1234, other.e1234, other.e1234, 1.0) * float4(wedge_g1_xyz, -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125))), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g0.xyz) + (wedge_g1_xyz.yzx * other.group3.zxy) - (wedge_g1_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Line> operator /(Line lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(wedge_g0) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Line project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Line(
            /* e415, e425, e435 */
            float3(right_dual_g0) * float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Circle project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        return Circle(
            /* e423, e431, e412 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0) * other.group1, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = float3(right_dual_g2[3]) * this.group0.xyz;
        float wedge_g0_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float3 wedge_g1_xyz = this.group1.xyz * right_dual_g2.www;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0_w) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g1_xyz, -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435))) + (other.group2 * float4(float3(wedge_g0_w), 0.0)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0_xyz[0] * other.e321) + (wedge_g1_xyz[1] * other.e412), (wedge_g0_xyz[1] * other.e321) + (wedge_g1_xyz[2] * other.e423), (wedge_g0_xyz[2] * other.e321) + (wedge_g1_xyz[0] * other.e431), -(wedge_g0_xyz[1] * other.e431) - (wedge_g0_xyz[2] * other.e412)) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g2[3] * this.e235) - (right_dual_g3[2] * this.e425), -(right_dual_g2[3] * this.e315) - (right_dual_g3[0] * this.e435), -(right_dual_g2[3] * this.e125) - (right_dual_g3[1] * this.e415), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125)) + (right_dual_g3.yzxx * float4(this.group0.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group3.zxy) - (wedge_g0.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4((other.group3.xyz * wedge_g0.www), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g0[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + float4((wedge_g0.yzx * other.group1.zxy * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Line project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return Line(
            /* e415, e425, e435 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Motor project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g1.yzxx * float4(this.group0.zxy, this.e235)) + float4((this.group0.yzx * right_dual_g1.zxy * float3(-1.0)), (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g0.xyz, (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)) * float4(-1.0, -1.0, -1.0, 1.0)) + (wedge_g0.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Line project_via_origin_onto(Line other) {
        float wedge_g0 = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435);
        return Line(
            /* e415, e425, e435 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Motor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = float3(right_dual_g0[3]) * this.group0.xyz;
        float wedge_g0_w = -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435);
        float4 wedge_g1 = float4((this.group1.xyz * right_dual_g0.www), 0.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group0.xyz)), wedge_g0_w * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_w) * other.group1) + float4(float3(0.0), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412);
        float4 wedge_g6 = float4((this.group0.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0);
        float3 wedge_g8 = float3(right_dual_g0[0]) * this.group1.xyz;
        float4 wedge_g9 = float4(-(right_dual_g1[2] * this.e425) - (right_dual_g1[3] * this.e235), -(right_dual_g1[0] * this.e435) - (right_dual_g1[3] * this.e315), -(right_dual_g1[1] * this.e415) - (right_dual_g1[3] * this.e125), (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125)) + (right_dual_g1.yzxx * float4(this.group0.zxy, this.e235));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) + float4(((float3(wedge_g6[3]) * other.group6.xyz) + (float3(other.e321) * wedge_g6.xyz)), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4(float3(0.0), wedge_g6[2] * other.e412) - float4(float3(0.0), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, 0.0)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Line project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0.yzxx * float4(this.group0.zxy, this.e235)) + float4((this.group0.yzx * right_dual_g0.zxy * float3(-1.0)), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125));
        return Line(
            /* e415, e425, e435 */
            (wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Circle project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125)) + (right_dual_g0.yzxx * float4(this.group0.zxy, this.e235));
        return Circle(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group0.zxy) - (wedge_g0.zxy * other.group0.yzx)), wedge_g0[3] * other.e1234), 
            /* e235, e315, e125 */
            (float3(wedge_g0[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((float3(right_dual_g0[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435));
        float4 wedge_g1 = float4((this.group1.xyz * right_dual_g0.www), 0.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[3] * other.e1) + (wedge_g1[1] * other.e412), (wedge_g0[3] * other.e2) + (wedge_g1[2] * other.e423), (wedge_g0[3] * other.e3) + (wedge_g1[0] * other.e431), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + (wedge_g0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(-(right_dual_g3[2] * this.e425) - (right_dual_g3[3] * this.e235), -(right_dual_g3[0] * this.e435) - (right_dual_g3[3] * this.e315), -(right_dual_g3[1] * this.e415) - (right_dual_g3[3] * this.e125), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125)) + (right_dual_g3.yzxx * float4(this.group0.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0.yzx * other.group3.zxy) - (wedge_g0.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4((other.group3.xyz * wedge_g0.www), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group1.yzx, other.e15)) + (wedge_g0.wwwy * float4(other.group0.xyz, other.e25)) + float4((wedge_g0.yzx * other.group1.zxy * float3(-1.0)), (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Motor> operator /(Motor lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public RoundPoint project_via_origin_onto(AntiCircleRotor other) {
        float4 wedge_g0 = float4(this.e5) * float4((other.group0.xyz * float3(-1.0)), other.e45) * float4(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        float wedge_g0_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5);
        float3 wedge_g1_xyz = right_dual_g1.xyz * this.group1.www;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0_w) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0_w) * other.group2.xyz), (wedge_g0_w * other.e4) - (wedge_g0_xyz[0] * other.e423) - (wedge_g0_xyz[1] * other.e431) - (wedge_g0_xyz[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0_w * other.e1) + (wedge_g0_xyz[0] * other.e321), (wedge_g0_w * other.e2) + (wedge_g0_xyz[1] * other.e321), (wedge_g0_w * other.e3) + (wedge_g0_xyz[2] * other.e321), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)) + float4((wedge_g1_xyz.yzx * other.group0.zxy), wedge_g0_w * other.e5) - float4((wedge_g1_xyz.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = float3(this.e5) * other.group0.xyz;
        float wedge_g0_w = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float3 wedge_g1_xyz = right_dual_g1.xyz * this.group1.www;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g0_w) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g0_w) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g0_w) * other.group2.xyz), -(wedge_g0_xyz[0] * other.e423) - (wedge_g0_xyz[1] * other.e431) - (wedge_g0_xyz[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0_xyz[0] * other.e321) + (wedge_g1_xyz[1] * other.e412), (wedge_g0_xyz[1] * other.e321) + (wedge_g1_xyz[2] * other.e423), (wedge_g0_xyz[2] * other.e321) + (wedge_g1_xyz[0] * other.e431), -(wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0_xyz[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = (other.group0.xyz * this.group1.www) + (this.group0.xyz * right_dual_g2.www);
        float wedge_g0_w = (right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float3 wedge_g1_xyz = (float3(right_dual_g2[3]) * this.group1.xyz) + (float3(this.e5) * right_dual_g1.xyz);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0_w) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group1.xyz)), wedge_g0_w * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g1_xyz, -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435))) + (other.group2 * float4(float3(wedge_g0_w), right_dual_g2[3] * this.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0_xyz[0] * other.e321) + (wedge_g1_xyz[1] * other.e412), (wedge_g0_xyz[1] * other.e321) + (wedge_g1_xyz[2] * other.e423), (wedge_g0_xyz[2] * other.e321) + (wedge_g1_xyz[0] * other.e431), -(wedge_g0_xyz[1] * other.e431) - (wedge_g0_xyz[2] * other.e412)) - float4((wedge_g1_xyz.zxy * other.group0.yzx), wedge_g0_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public RoundPoint project_via_origin_onto(Dipole other) {
        float4 wedge_g0 = float4(this.e5) * float4((other.group0.xyz * float3(-1.0)), other.e45) * float4(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g0[1] * other.e42) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(this.e5) * float4(right_dual_g3.xyz, right_dual_g2[3]) * float4(-1.0);
        float4 wedge_g1 = float4(-(right_dual_g2[3] * this.e235) - (right_dual_g3[2] * this.e425), -(right_dual_g2[3] * this.e315) - (right_dual_g3[0] * this.e435), -(right_dual_g2[3] * this.e125) - (right_dual_g3[1] * this.e415), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125)) + (right_dual_g3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4((other.group0.xyz * float3(-1.0)), other.e45));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) - (wedge_g1.zxy * other.group3.yzx)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((other.group3.xyz * wedge_g1.www), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g1[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g1.zxyx * float4(other.group1.yzx, other.e15)) + (other.group2.wwwy * float4(wedge_g0.xyz, wedge_g1[1])) + float4(float3(0.0), (wedge_g1[2] * other.e35) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), wedge_g0[0] * other.e4235)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float4 wedge_g0 = float4(right_dual_g0[1]) * this.group0;
        float4 wedge_g1 = float4(right_dual_g0[1]) * this.group1;
        return Motor(
            /* e415, e425, e435, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.e12345, (wedge_g0[3] * other.e5) + (wedge_g1[3] * other.e12345)) * float4(wedge_g1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public DualNum project_via_origin_onto(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e45 * other.e45 * this.e5, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = right_dual_g1.xyz * this.group1.www * float3(-1.0);
        float4 wedge_g1 = (right_dual_g1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125)) - float4((right_dual_g1.zxy * this.group0.yzx), other.e45 * this.e5);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g1.yzx * other.group1.zxy) - (wedge_g1.zxy * other.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (wedge_g1.wwwx * float4(other.group1.xyz, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35) + (wedge_g1[3] * other.e45) - (wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125)) - (other.group1.wwwx * float4(wedge_g1.xyz, wedge_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Line other) {
        float wedge_g0_w = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435);
        float3 wedge_g1_xyz = other.group0.xyz * this.group1.www;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(wedge_g0_w) * other.group1.xyz), -(wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = this.group0.xyz * right_dual_g0.www;
        float wedge_g0_w = (right_dual_g0[3] * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435);
        float4 wedge_g1 = float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e5) * right_dual_g0.xyz)), right_dual_g0[3] * this.e5);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g0_xyz * float3(other.e12345)) + (float3(wedge_g0_w) * other.group0.xyz)), wedge_g0_w * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_w) * other.group1) + float4(float3(0.0), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g0[0] * this.e12345) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4);
        float wedge_g2 = right_dual_g0[0] * this.e5;
        float4 wedge_g3 = right_dual_g1 * float4(this.e5) * float4(-1.0);
        float4 wedge_g6 = float4(((float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e5) * other.group7.xyz)), 0.0);
        float3 wedge_g8 = (float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.e5) * other.group6.xyz);
        float4 wedge_g9 = float4(-(right_dual_g1[2] * this.e425) - (right_dual_g1[3] * this.e235), -(right_dual_g1[0] * this.e435) - (right_dual_g1[3] * this.e315), -(right_dual_g1[1] * this.e415) - (right_dual_g1[3] * this.e125), (right_dual_g1[1] * this.e315) + (right_dual_g1[2] * this.e125)) + (right_dual_g1.yzxx * float4(this.group0.zxy, this.e235)) - float4((other.group4.xyz * this.group1.www * float3(-1.0)), this.e5 * other.e45);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g2 * other.e1234) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4(float3(0.0), wedge_g3[3] * other.e1234) + float4((float3(wedge_g6[3]) * other.group6.xyz), 0.0) + float4((float3(other.e321) * wedge_g6.xyz), 0.0) + float4((float3(other.e1234) * wedge_g3.xyz), 0.0) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((wedge_g8.yzx * other.group7.zxy), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g6[0] * other.e423) - float4(float3(0.0), wedge_g6[1] * other.e431) - float4(float3(0.0), wedge_g6[2] * other.e412) - float4(float3(0.0), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, 0.0)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = right_dual_g0.xyz * this.group1.www * float3(-1.0);
        float4 wedge_g1 = (right_dual_g0.yzxx * float4(this.group0.zxy, this.e235)) + float4((right_dual_g0.zxy * this.group0.yzx * float3(-1.0)), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125));
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((wedge_g1.yzx * other.group0.zxy) - (wedge_g1.zxy * other.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz * wedge_g1.www), -(wedge_g0_xyz[1] * other.e4315) - (wedge_g0_xyz[2] * other.e4125)) - (other.group0.wwwx * float4(wedge_g1.xyz, wedge_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public RoundPoint project_via_origin_onto(RoundPoint other) {
        float wedge_g0 = this.e5 * other.e4 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = right_dual_g0 * float4(this.e5) * float4(-1.0);
        float4 wedge_g1 = float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125)) + (right_dual_g0.yzxx * float4(this.group0.zxy, this.e235));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group0.zxy) - (wedge_g1.zxy * other.group0.yzx)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g1.xyz)), wedge_g0[3] * other.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(other.e1234) * wedge_g0.xyz), -(wedge_g0[0] * other.e4235) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (float4(right_dual_g0[3]) * this.group0) + (float4(this.e5) * float4(right_dual_g0.xyz, other.e4 * -1.0)) + float4(float3(0.0), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435));
        float4 wedge_g1 = float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e5) * right_dual_g1.xyz)), right_dual_g0[3] * this.e5);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(wedge_g0[3]) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e321), 
            /* e235, e315, e125, e5 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[3] * other.e1) + (wedge_g1[1] * other.e412), (wedge_g0[3] * other.e2) + (wedge_g1[2] * other.e423), (wedge_g0[3] * other.e3) + (wedge_g1[0] * other.e431), -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412)) + (wedge_g0 * float4(other.group1.www, other.e4)) - (other.group0.yzxx * float4(wedge_g1.zxy, wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = right_dual_g3 * float4(this.e5) * float4(-1.0);
        float4 wedge_g1 = float4(-(right_dual_g0[0] * this.e5) - (right_dual_g3[3] * this.e235), -(right_dual_g0[1] * this.e5) - (right_dual_g3[3] * this.e315), -(right_dual_g0[2] * this.e5) - (right_dual_g3[3] * this.e125), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125)) + (right_dual_g3.yzxx * float4(this.group0.zxy, this.e235)) - float4((right_dual_g3.zxy * this.group0.yzx), this.e5 * other.e45);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234) * wedge_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1.yzx * other.group3.zxy) - (wedge_g1.zxy * other.group3.yzx)), wedge_g1[3] * other.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(wedge_g1[1] * other.e42) - (wedge_g1[2] * other.e43)) + float4((other.group3.xyz * wedge_g1.www), wedge_g0[3] * other.e1234) - (wedge_g1.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyy * float4(other.group1.yzx, other.e25)) + (wedge_g1.wwwz * float4(other.group0.xyz, other.e35)) + (other.group2.wwwx * float4(wedge_g0.xyz, wedge_g1[0])) + float4(float3(0.0), (wedge_g1[3] * other.e45) - (wedge_g0[1] * other.e4315) - (wedge_g0[2] * other.e4125) - (wedge_g0[3] * other.e3215)) - float4((wedge_g1.yzx * other.group1.zxy), wedge_g0[0] * other.e4235)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<MultiVector> operator /(MultiVector lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43);
        float4 wedge_g6 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g7 = right_dual_g0 * float3(this.scalar);
        float3 wedge_g8 = float3(this.scalar) * right_dual_g2.xyz;
        float4 wedge_g9 = float4((right_dual_g1[1] * this.e3) + (right_dual_g2[0] * this.e4), (right_dual_g1[2] * this.e1) + (right_dual_g2[1] * this.e4), (right_dual_g1[0] * this.e2) + (right_dual_g2[2] * this.e4), -(right_dual_g2[1] * this.e2) - (right_dual_g2[2] * this.e3)) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (this.group1.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0]));
        float wedge_g10 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g9[2] * other.e31) + (wedge_g9[3] * other.e41), (wedge_g9[0] * other.e12) + (wedge_g9[3] * other.e42), (wedge_g9[1] * other.e23) + (wedge_g9[3] * other.e43), -(wedge_g10 * other.e45) - (wedge_g9[2] * other.e43)) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge_g10) * other.group2.xyz), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45), 
            /* e15, e25, e35, e45 */
            float4(wedge_g0_y) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g2[3] * this.e5) + (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 wedge_g3 = float4(this.scalar) * float4(right_dual_g2.xyz, right_dual_g1[3]);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = float3(this.scalar) * right_dual_g1.xyz;
        float4 wedge_g6 = float4((right_dual_g2[0] * this.e4) + (other.e423 * this.e5), (right_dual_g2[1] * this.e4) + (other.e431 * this.e5), (right_dual_g2[2] * this.e4) + (other.e412 * this.e5), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group1.yzx) - (right_dual_g2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (other.e431 * this.e35), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (other.e412 * this.e15), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (other.e423 * this.e25), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[2] * this.e12)) + (right_dual_g3 * float4(this.scalar)) - (right_dual_g2.yzxy * float4(this.group4.zxy, this.e31)) - float4((other.group0.zxy * this.group3.yzx), right_dual_g2[0] * this.e23);
        float wedge_g10 = (right_dual_g2[3] * this.scalar) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g7[2] * other.e315) + (wedge_g8[1] * other.e412) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g7[0] * other.e125) + (wedge_g8[2] * other.e423) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g7[1] * other.e235) + (wedge_g8[0] * other.e431) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435) - (wedge_g6[2] * other.e412)) + (float4(wedge_g0_y) * float4(other.group3.xyz, other.e4)) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g6[1] * other.e431) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (wedge_g0_y * other.e5) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(wedge_g10) * other.group1.xyz) + (other.group0.zxy * wedge_g9.yzx) - (other.group0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge_g10) * other.group2.xyz) + (float3(wedge_g9[3]) * other.group0.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0_y) * other.group1, 
            /* e423, e431, e412 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiDualNum other) {
        float wedge_g0_y = (other.e3215 * this.e1234) + (other.scalar * this.scalar);
        return MultiVector(
            /* scalar, e12345 */
            float2(wedge_g0_y * other.scalar, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e3215 * other.e3215 * this.e4 * -1.0, 
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
            float3(other.e3215) * (float4(other.e3215) * float4(this.group7.xyz, this.e321)).xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g0_y * other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        float3 wedge_g6_xyz = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group1.xyz);
        float4 wedge_g9 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group4.zxy, this.e23));
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[3] * other.e321 * this.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6_xyz * float3(other.e321)), 0.0), 
            /* e5 */
            -(wedge_g6_xyz[0] * other.e235) - (wedge_g6_xyz[1] * other.e315) - (wedge_g6_xyz[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * wedge_g9.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), wedge_g0_y * other.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        float3 wedge_g6_xyz = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group1.xyz);
        float4 wedge_g9 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (right_dual_g0.yzxx * float4(this.group4.zxy, this.e23));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) - (right_dual_g0[3] * other.e321 * this.scalar), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g6_xyz * float3(other.e321)) + (float3(wedge_g0_y) * other.group1.xyz)), 0.0), 
            /* e5 */
            (wedge_g0_y * other.e5) - (wedge_g6_xyz[0] * other.e235) - (wedge_g6_xyz[1] * other.e315) - (wedge_g6_xyz[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * wedge_g9.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), wedge_g0_y * other.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float wedge_g0_y = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        float3 wedge_g6_xyz = right_dual_g0 * float3(this.group0.xx, this.scalar);
        float4 wedge_g9 = float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group1.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g6_xyz[0] * other.e23) - (wedge_g6_xyz[1] * other.e31) - (wedge_g6_xyz[2] * other.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((other.group0.yzx * wedge_g9.zxy) - (other.group0.zxy * wedge_g9.yzx)), 0.0), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35), 
            /* e15, e25, e35, e45 */
            float4((float3(wedge_g0_y) * other.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g0[3] * this.scalar) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        float3 wedge_g6_xyz = (float3(right_dual_g1[3]) * this.group4.xyz) + (float3(this.scalar) * right_dual_g0.xyz);
        float4 wedge_g9 = float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((this.group7.xyz * right_dual_g1.www), right_dual_g1[3] * this.e321) - (this.group1.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) - (wedge_g6_xyz[0] * other.e23) - (wedge_g6_xyz[1] * other.e31) - (wedge_g6_xyz[2] * other.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g9.zxy * other.group0.yzx) - (wedge_g9.yzx * other.group0.zxy)), 0.0), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) - (right_dual_g1[3] * other.e3215 * this.e4), 
            /* e15, e25, e35, e45 */
            float4(((wedge_g6_xyz * float3(other.e3215)) + (float3(wedge_g0_y) * other.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * wedge_g9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), wedge_g0_y * other.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4);
        float4 wedge_g9 = right_dual_g0 * float4(this.scalar);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(wedge_g0_y) * other.group0.xyz), 0.0), 
            /* e5 */
            wedge_g0_y * other.e5, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0) * float2(other.e12345) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e5 */
            right_dual_g0 * other.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0) * float4(other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual_g0) * float3(other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0) * float3(other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * float4(other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual_g0) * float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0) * float3(other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * float4(other.e12345) * this.group9, 
            /* e1234 */
            right_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group2.xyz, right_dual_g1[3]);
        float3 wedge_g4 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g5 = float3(this.scalar) * right_dual_g1.xyz;
        float4 wedge_g6 = float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(this.e5) * right_dual_g1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (other.e431 * this.e35) + (other.e125 * this.e42), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (other.e412 * this.e15) + (other.e235 * this.e43), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (other.e423 * this.e25) + (other.e315 * this.e41), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group3.yzx), other.e235 * this.e23) - float4((other.group2.yzx * this.group4.zxy), other.e315 * this.e31);
        float wedge_g10 = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g7[2] * other.e315) + (wedge_g8[1] * other.e412) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g7[0] * other.e125) + (wedge_g8[2] * other.e423) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g7[1] * other.e235) + (wedge_g8[0] * other.e431) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435) - (wedge_g6[2] * other.e412)) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g6[1] * other.e431) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            -(wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(wedge_g10) * other.group1.xyz) + (other.group0.zxy * wedge_g9.yzx) - (other.group0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge_g10) * other.group2.xyz) + (float3(wedge_g9[3]) * other.group0.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(wedge_g0_y) * other.group1, 
            /* e423, e431, e412 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 wedge_g1 = float4(right_dual_g2[3]) * this.group1;
        float4 wedge_g3 = (float4(right_dual_g2[3]) * this.group3) + (float4(this.scalar) * float4(right_dual_g2.xyz, right_dual_g1[3]));
        float3 wedge_g4 = (float3(right_dual_g2[3]) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz);
        float3 wedge_g5 = (float3(right_dual_g2[3]) * this.group5.xyz) + (float3(this.scalar) * right_dual_g1.xyz);
        float4 wedge_g6 = float4((right_dual_g2[0] * this.e4) + (right_dual_g2[3] * this.e415), (right_dual_g2[1] * this.e4) + (right_dual_g2[3] * this.e425), (right_dual_g2[2] * this.e4) + (right_dual_g2[3] * this.e435), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((float3(this.e5) * other.group0.xyz), right_dual_g2[3] * this.e321) - (right_dual_g1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(right_dual_g2[3]) * this.group7.xyz) + (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(right_dual_g2[3]) * this.group8.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group1.yzx) - (right_dual_g2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (right_dual_g2[3] * this.e4235), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (right_dual_g2[3] * this.e4315), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (right_dual_g2[3] * this.e4125), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[2] * this.e12)) + float4((other.group0.yzx * this.group3.zxy), right_dual_g2[3] * this.e3215) - (right_dual_g2.yzxy * float4(this.group4.zxy, this.e31)) - float4((other.group0.zxy * this.group3.yzx), right_dual_g2[0] * this.e23);
        float wedge_g10 = (right_dual_g2[3] * this.e1234) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g2[3] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g7[2] * other.e315) + (wedge_g1[0] * other.e12345) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g7[0] * other.e125) + (wedge_g1[1] * other.e12345) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g7[1] * other.e235) + (wedge_g1[2] * other.e12345) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435) - (wedge_g6[2] * other.e412)) + float4((wedge_g8.yzx * other.group0.zxy), wedge_g1[3] * other.e12345) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g6[1] * other.e431) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g6[0] * other.e423), 
            /* e5 */
            (right_dual_g2[3] * other.e12345 * this.e5) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g3[0] * other.e12345) + (wedge_g9[2] * other.e315), (wedge_g3[1] * other.e12345) + (wedge_g9[0] * other.e125), (wedge_g3[2] * other.e12345) + (wedge_g9[1] * other.e235), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + float4((other.group1.xyz * wedge_g9.www), wedge_g3[3] * other.e12345) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g10) * other.group1.xyz) + (other.group0.zxy * wedge_g9.yzx) - (other.group0.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (float3(wedge_g10) * other.group2.xyz) + (float3(wedge_g9[3]) * other.group0.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group0.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(other.e12345), 
            /* e1234 */
            wedge_g10 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float wedge_g0_y = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        float4 wedge_g6 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g7 = right_dual_g0 * float3(this.scalar);
        float3 wedge_g8 = right_dual_g2 * float3(this.scalar);
        float4 wedge_g9 = float4((right_dual_g2[0] * this.e4) + (right_dual_g1[1] * this.e3), (right_dual_g2[1] * this.e4) + (right_dual_g1[2] * this.e1), (right_dual_g2[2] * this.e4) + (right_dual_g1[0] * this.e2), -(right_dual_g2[2] * this.e3) - (right_dual_g1[3] * this.e5)) - (this.group1.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((right_dual_g0 * float3(this.e5)), right_dual_g2[0] * this.e1);
        float wedge_g10 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g9[2] * other.e31) + (wedge_g9[3] * other.e41), (wedge_g9[0] * other.e12) + (wedge_g9[3] * other.e42), (wedge_g9[1] * other.e23) + (wedge_g9[3] * other.e43), -(wedge_g10 * other.e45) - (wedge_g9[2] * other.e43)) - (wedge_g9.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge_g10) * other.group2.xyz), wedge_g9[0] * other.e41), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45), 
            /* e15, e25, e35, e45 */
            float4(wedge_g0_y) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g2[3] * this.e3215) + (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43);
        float4 wedge_g1 = float4(this.scalar) * float4(right_dual_g3.xyz, right_dual_g2[3]);
        float4 wedge_g3 = (float4(right_dual_g3[3]) * this.group1) - (float4(this.e5) * float4(right_dual_g3.xyz, right_dual_g2[3]));
        float3 wedge_g4 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group1.xyz);
        float3 wedge_g5 = (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((right_dual_g2[3] * this.e15) + (right_dual_g3[3] * this.e41), (right_dual_g2[3] * this.e25) + (right_dual_g3[3] * this.e42), (right_dual_g2[3] * this.e35) + (right_dual_g3[3] * this.e43), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - float4((right_dual_g3.xyz * this.group3.www), right_dual_g3[0] * this.e23);
        float3 wedge_g7 = (right_dual_g0 * float3(this.scalar)) + (float3(right_dual_g2[3]) * this.group5.xyz) + (this.group4.yzx * right_dual_g3.zxy) - (this.group4.zxy * right_dual_g3.yzx);
        float3 wedge_g8 = (float3(right_dual_g3[3]) * this.group5.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group3.zxy) - (right_dual_g3.zxy * this.group3.yzx);
        float4 wedge_g9 = (right_dual_g3.yzxw * this.group6.zxyw) + (right_dual_g3.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g3[1] * this.e315) + float4((right_dual_g2.xyz * this.group1.www), right_dual_g3[2] * this.e125) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (this.group1.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((this.group8.xyz * right_dual_g2.www), right_dual_g2[0] * this.e1) - float4((right_dual_g3.zxy * this.group6.yzx), right_dual_g2[2] * this.e3);
        float wedge_g10 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g2[3] * this.e321) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (right_dual_g3[3] * other.e1234 * this.scalar) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (wedge_g3 * float4(other.e1234)) + (other.group3.yzxy * float4(wedge_g5.zxy, wedge_g4[1])) + float4((other.group0.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g9.zxy * other.group1.yzx), wedge_g4[2] * other.e4125) - (float4(wedge_g10) * float4(other.group2.xyz, other.e45)) - (wedge_g9.yzxz * float4(other.group1.zxy, other.e43)) - float4((wedge_g4 * other.group3.www), wedge_g9[0] * other.e41) - float4((wedge_g5.yzx * other.group3.zxy), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + (float4(wedge_g0_y) * float4(other.group2.xyz, other.e45)) - float4((wedge_g8.zxy * other.group3.yzx), wedge_g6[0] * other.e4235), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group0.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group3.yzx) - (wedge_g7.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group1.xyz) - (float3(wedge_g6[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g9.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group3.xyz), 
            /* e235, e315, e125 */
            (float3(wedge_g9[3]) * other.group3.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0_y) * other.group3, 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float wedge_g0_y = (right_dual_g0[0] * this.e4) + (right_dual_g0[1] * this.e12345);
        float wedge_g10 = right_dual_g0[1] * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (right_dual_g0[1] * other.e12345 * this.scalar), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e5 */
            (wedge_g0_y * other.e5) + (right_dual_g0[1] * other.e12345 * this.e5), 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * float3(other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * float4(right_dual_g0.yy, right_dual_g0[1], (right_dual_g0[0] * this.scalar) + (right_dual_g0[1] * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234 */
            wedge_g10 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g9_xyz = float3(this.e4) * right_dual_g0.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[3] * other.e45 * this.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3] * this.e4) * other.group0 * float4(-1.0), 
            /* e5 */
            (wedge_g9_xyz[0] * other.e15) + (wedge_g9_xyz[1] * other.e25) + (wedge_g9_xyz[2] * other.e35) - (right_dual_g0[0] * other.e45 * this.e1) - (right_dual_g0[1] * other.e45 * this.e2) - (right_dual_g0[2] * other.e45 * this.e3) - (right_dual_g0[3] * other.e45 * this.e5), 
            /* e15, e25, e35, e45 */
            float4(-(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) * other.group0, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45);
        float3 wedge_g1_xyz = right_dual_g1.xyz * float3(this.group0.xx, this.scalar);
        float3 wedge_g3_xyz = (float3(right_dual_g1[3]) * this.group1.xyz) - (float3(this.e5) * right_dual_g1.xyz);
        float3 wedge_g4 = float3(this.e4) * right_dual_g1.xyz;
        float3 wedge_g5 = (right_dual_g1.zxy * this.group1.yzx) - (right_dual_g1.yzx * this.group1.zxy);
        float4 wedge_g6 = float4(float3(0.0), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((this.group4.xyz * right_dual_g1.www), right_dual_g0[3] * this.scalar) - float4((right_dual_g1.xyz * this.group3.www), right_dual_g1[0] * this.e23);
        float3 wedge_g7 = (this.group4.yzx * right_dual_g1.zxy) - (this.group4.zxy * right_dual_g1.yzx);
        float3 wedge_g8 = (float3(right_dual_g1[3]) * this.group5.xyz) + (float3(this.scalar) * right_dual_g0.xyz) + (right_dual_g1.yzx * this.group3.zxy) - (right_dual_g1.zxy * this.group3.yzx);
        float4 wedge_g9 = (right_dual_g1.wwwx * float4(this.group7.xyz, this.e235)) + float4(float3(0.0), (right_dual_g1[3] * this.e321) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3) - (right_dual_g0[3] * this.e5)) + float4((right_dual_g1.yzx * this.group6.zxy), right_dual_g1[2] * this.e125) + float4((right_dual_g0.xyz * this.group1.www), right_dual_g1[1] * this.e315) - float4((right_dual_g1.zxy * this.group6.yzx), right_dual_g0[0] * this.e1);
        float wedge_g10 = (right_dual_g0[3] * this.e4) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * other.e4235) + (wedge_g1_xyz[1] * other.e4315) + (wedge_g1_xyz[2] * other.e4125) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(wedge_g10 * other.e15) - (wedge_g5[1] * other.e4125), -(wedge_g10 * other.e25) - (wedge_g5[2] * other.e4235), -(wedge_g10 * other.e35) - (wedge_g5[0] * other.e4315), (wedge_g4[1] * other.e4315) + (wedge_g4[2] * other.e4125)) + (other.group1.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) - float4((wedge_g4 * other.group1.www), wedge_g10 * other.e45), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g3_xyz[0] * other.e4235) - (wedge_g3_xyz[1] * other.e4315) - (wedge_g3_xyz[2] * other.e4125) - (right_dual_g1[3] * other.e3215 * this.e4), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + (float4(wedge_g0_y) * other.group0) - float4((wedge_g8.zxy * other.group1.yzx), wedge_g6[0] * other.e4235), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * other.group1.yzx) - (wedge_g7.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) - (float3(wedge_g6[3]) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g9.yzx * other.group1.zxy) - (wedge_g9.zxy * other.group1.yzx)), wedge_g10 * other.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(wedge_g10) * other.group1.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(wedge_g9[3]) * other.group1.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0_y) * other.group1, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Line other) {
        float wedge_g0_y = -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float3 wedge_g5 = float3(this.scalar) * other.group0.xyz;
        float3 wedge_g6_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g6_w = -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3);
        float3 wedge_g7 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g8 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((other.e415 * this.e45) + (other.e125 * this.e42), (other.e425 * this.e45) + (other.e235 * this.e43), (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group1.yzx * this.group4.zxy), other.e415 * this.e15);
        float wedge_g10 = -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge_g6_w * other.e415) + (wedge_g7[2] * other.e315), (wedge_g6_w * other.e425) + (wedge_g7[0] * other.e125), (wedge_g6_w * other.e435) + (wedge_g7[1] * other.e235), -(wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) - float4((wedge_g7.yzx * other.group1.zxy), wedge_g7[0] * other.e415), 
            /* e5 */
            -(wedge_g6_xyz[0] * other.e235) - (wedge_g6_xyz[1] * other.e315) - (wedge_g6_xyz[2] * other.e125) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435), 
            /* e15, e25, e35, e45 */
            float4((wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e41, e42, e43 */
            float3(wedge_g10) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g10) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(wedge_g0_y) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(wedge_g0_y) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[3] * this.e12345) + (right_dual_g1[3] * this.e4) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412);
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float3 wedge_g3_xyz = (float3(right_dual_g0[3]) * this.group3.xyz) + (float3(this.scalar) * right_dual_g1.xyz);
        float3 wedge_g4 = float3(right_dual_g0[3]) * this.group4.xyz;
        float3 wedge_g5 = (float3(right_dual_g0[3]) * this.group5.xyz) + (float3(this.scalar) * right_dual_g0.xyz);
        float4 wedge_g6 = (float4(right_dual_g0[3]) * this.group6) + (float4(this.e4, this.e4, this.e4, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e1) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3)));
        float3 wedge_g7 = (float3(right_dual_g0[3]) * this.group7.xyz) + (float3(this.e4) * right_dual_g0.xyz);
        float3 wedge_g8 = (float3(right_dual_g0[3]) * this.group8.xyz) + (float3(this.e5) * right_dual_g0.xyz) + (right_dual_g1.zxy * this.group1.yzx) - (right_dual_g1.yzx * this.group1.zxy);
        float4 wedge_g9 = (right_dual_g0 * float4(this.group3.www, this.e3215)) + (right_dual_g1.zxyw * float4(this.group4.yzx, this.scalar)) + float4((this.group9.xyz * right_dual_g0.www), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - (right_dual_g1.yzxx * float4(this.group4.zxy, this.e23));
        float wedge_g10 = (right_dual_g0[3] * this.e1234) - (right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (right_dual_g0[3] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g1[0] * other.e12345) + (wedge_g6[3] * other.e415), (wedge_g1[1] * other.e12345) + (wedge_g6[3] * other.e425), (wedge_g1[2] * other.e12345) + (wedge_g6[3] * other.e435), -(wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + float4((wedge_g7.zxy * other.group1.yzx), wedge_g1[3] * other.e12345) - float4((wedge_g7.yzx * other.group1.zxy), wedge_g7[0] * other.e415), 
            /* e5 */
            (wedge_g0_y * other.e5) + (right_dual_g0[3] * other.e12345 * this.e5) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g3_xyz[0] * other.e12345) + (wedge_g9[2] * other.e315), (wedge_g3_xyz[1] * other.e12345) + (wedge_g9[0] * other.e125), (wedge_g3_xyz[2] * other.e12345) + (wedge_g9[1] * other.e235), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + (other.group0 * float4(wedge_g9.www, right_dual_g0[3] * this.e45)) - (wedge_g9.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g10) * other.group0.xyz), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (float3(wedge_g10) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0_y) * other.group0.xyz) + (float3(other.e12345) * wedge_g6.xyz)), wedge_g6[3] * other.e12345), 
            /* e423, e431, e412 */
            wedge_g7 * float3(other.e12345), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(other.e12345), 
            /* e1234 */
            wedge_g10 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        float wedge_g0_y = (right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345) + (right_dual_g0[1] * this.scalar) + (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) + (other.e3215 * this.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0[0]) * this.group1);
        float wedge_g2 = (right_dual_g0[0] * this.e5) + (other.e3215 * this.scalar);
        float4 wedge_g3 = (right_dual_g3 * float4(this.scalar)) + (float4(right_dual_g0[0]) * this.group3) + (float4(other.e3215) * this.group1) - (right_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(right_dual_g0[0]) * this.group4.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group1.xyz);
        float3 wedge_g5 = (float3(right_dual_g0[0]) * this.group5.xyz) + (float3(this.scalar) * other.group6.xyz) + (right_dual_g1.zxy * this.group1.yzx) - (right_dual_g1.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((right_dual_g1[3] * this.e15) + (right_dual_g3[0] * this.e4) + (other.e423 * this.e5) + (other.e3215 * this.e41), (right_dual_g1[3] * this.e25) + (right_dual_g3[1] * this.e4) + (other.e431 * this.e5) + (other.e3215 * this.e42), (right_dual_g1[3] * this.e35) + (right_dual_g3[2] * this.e4) + (other.e412 * this.e5) + (other.e3215 * this.e43), -(right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (other.e435 * this.e3)) + (right_dual_g6 * float4(this.scalar)) + (float4(right_dual_g0[0]) * this.group6) - (this.group1.xyzy * float4(right_dual_g3.www, other.e425)) - float4((right_dual_g1.xyz * this.group3.www), other.e415 * this.e1);
        float3 wedge_g7 = (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g0[0]) * this.group7.xyz) + (float3(right_dual_g1[3]) * this.group5.xyz) + (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group1.zxy) + (this.group4.yzx * right_dual_g1.zxy) - (other.group7.zxy * this.group1.yzx) - (this.group4.zxy * right_dual_g1.yzx);
        float3 wedge_g8 = (right_dual_g8 * float3(this.scalar)) + (float3(right_dual_g0[0]) * this.group8.xyz) + (float3(other.e3215) * this.group5.xyz) + (float3(this.e5) * other.group6.xyz) + (right_dual_g1.yzx * this.group3.zxy) + (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g3.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((right_dual_g1[1] * this.e435) + (right_dual_g3[3] * this.e23) + (right_dual_g6[1] * this.e3) + (other.e3215 * this.e423), (right_dual_g1[2] * this.e415) + (right_dual_g3[3] * this.e31) + (right_dual_g6[2] * this.e1) + (other.e3215 * this.e431), (right_dual_g1[0] * this.e425) + (right_dual_g3[3] * this.e12) + (right_dual_g6[0] * this.e2) + (other.e3215 * this.e412), -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12) - (right_dual_g6[3] * this.e5)) + (right_dual_g9 * float4(this.scalar)) + (float4(right_dual_g0[0]) * this.group9) + float4((right_dual_g8 * this.group1.www), right_dual_g1[2] * this.e125) + float4((other.group7.yzx * this.group3.zxy), right_dual_g1[0] * this.e235) + float4((this.group4.yzx * right_dual_g3.zxy), other.e3215 * this.e321) + float4((other.group6.xyz * this.group3.www), right_dual_g1[1] * this.e315) - (this.group1.yzxz * float4(right_dual_g6.zxy, right_dual_g8[2])) - (this.group3.yzxx * float4(other.group7.zxy, other.e415)) - float4((right_dual_g7 * float3(this.e5)), other.e425 * this.e25) - float4((this.group8.xyz * right_dual_g1.www), right_dual_g8[0] * this.e1) - float4((this.group4.zxy * right_dual_g3.yzx), other.e435 * this.e35) - float4((right_dual_g1.zxy * this.group6.yzx), right_dual_g8[1] * this.e2);
        float wedge_g10 = (right_dual_g10 * this.scalar) + (right_dual_g0[0] * this.e1234) + (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3) + (right_dual_g6[3] * this.e4) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_dual_g0[0] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g1_xyz = right_dual_g0.xyz * float3(this.group0.xx, this.scalar);
        float3 wedge_g3_xyz = (float3(right_dual_g0[3]) * this.group1.xyz) - (float3(this.e5) * right_dual_g0.xyz);
        float3 wedge_g4 = float3(this.e4) * right_dual_g0.xyz;
        float3 wedge_g5 = (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy);
        float3 wedge_g6_xyz = (this.group4.xyz * right_dual_g0.www) - (right_dual_g0.xyz * this.group3.www);
        float3 wedge_g7 = (this.group4.yzx * right_dual_g0.zxy) - (this.group4.zxy * right_dual_g0.yzx);
        float3 wedge_g8 = (float3(right_dual_g0[3]) * this.group5.xyz) + (right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx);
        float4 wedge_g9 = (right_dual_g0.wwwx * float4(this.group7.xyz, this.e235)) + float4((right_dual_g0.zxy * this.group6.yzx * float3(-1.0)), (right_dual_g0[2] * this.e125) + (right_dual_g0[3] * this.e321)) + float4((right_dual_g0.yzx * this.group6.zxy), right_dual_g0[1] * this.e315);
        float wedge_g10 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1_xyz[0] * other.e4235) + (wedge_g1_xyz[1] * other.e4315) + (wedge_g1_xyz[2] * other.e4125), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(wedge_g4[0] * other.e3215) - (wedge_g5[1] * other.e4125), -(wedge_g4[1] * other.e3215) - (wedge_g5[2] * other.e4235), -(wedge_g4[2] * other.e3215) - (wedge_g5[0] * other.e4315), (wedge_g4[1] * other.e4315) + (wedge_g4[2] * other.e4125)) + (other.group0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])), 
            /* e5 */
            -(wedge_g3_xyz[0] * other.e4235) - (wedge_g3_xyz[1] * other.e4315) - (wedge_g3_xyz[2] * other.e4125) - (right_dual_g0[3] * this.e4 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((wedge_g6_xyz[0] * other.e3215) + (wedge_g8[1] * other.e4125), (wedge_g6_xyz[1] * other.e3215) + (wedge_g8[2] * other.e4235), (wedge_g6_xyz[2] * other.e3215) + (wedge_g8[0] * other.e4315), -(wedge_g6_xyz[1] * other.e4315) - (wedge_g6_xyz[2] * other.e4125)) - (other.group0.yzxx * float4(wedge_g8.zxy, wedge_g6_xyz[0])), 
            /* e41, e42, e43 */
            (wedge_g7.zxy * other.group0.yzx) - (wedge_g7.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) - (float3(-(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g9.yzx * other.group0.zxy) - (wedge_g9.zxy * other.group0.yzx)), wedge_g10 * other.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(wedge_g10) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(wedge_g9[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234)) * other.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        float wedge_g0_y = (right_dual_g1 * this.e5) + (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4);
        float4 wedge_g9 = right_dual_g0 * float4(this.scalar);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_dual_g1 * this.scalar * other.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(wedge_g0_y) * other.group0, 
            /* e5 */
            wedge_g0_y * other.e5, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215);
        float4 wedge_g1 = right_dual_g0 * float4(this.scalar);
        float4 wedge_g3 = (float4(other.e3215) * this.group1) - (right_dual_g0 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group1.xyz);
        float3 wedge_g5 = (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((right_dual_g0[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g0[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g0[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (float3(right_dual_g0[3]) * this.group5.xyz) + (this.group4.yzx * right_dual_g0.zxy) - (this.group4.zxy * right_dual_g0.yzx);
        float3 wedge_g8 = (float3(other.e3215) * this.group5.xyz) + (right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx);
        float4 wedge_g9 = float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[2] * this.e125) + (this.e321 * other.e3215)) + (right_dual_g0.yzxy * float4(this.group6.zxy, this.e315)) + float4((float3(other.e3215) * this.group7.xyz), right_dual_g0[0] * this.e235);
        float wedge_g10 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (this.scalar * other.e3215 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(wedge_g4[0] * other.e3215) - (wedge_g5[1] * other.e4125), -(wedge_g4[1] * other.e3215) - (wedge_g5[2] * other.e4235), -(wedge_g4[2] * other.e3215) - (wedge_g5[0] * other.e4315), (wedge_g4[2] * other.e4125) + (wedge_g3[3] * other.e1234)) + (other.group0.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4((float3(other.e1234) * wedge_g3.xyz), wedge_g4[1] * other.e4315), 
            /* e5 */
            -(wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) - (other.group0.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group0.yzx) - (wedge_g7.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) - (float3(wedge_g6[3]) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g9.yzxw * float4(other.group0.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group0.yzx), wedge_g10 * other.e3215), 
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(wedge_g9[3]) * other.group0.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0_y) * other.group0, 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[3] * this.e12345) + (right_dual_g2[3] * this.e5) + (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412);
        float4 wedge_g1 = float4(right_dual_g0[3]) * this.group1;
        float4 wedge_g3 = (float4(right_dual_g0[3]) * this.group3) + (float4(this.scalar) * float4(right_dual_g2.xyz, right_dual_g1[3]));
        float3 wedge_g4 = (float3(right_dual_g0[3]) * this.group4.xyz) + (float3(this.scalar) * right_dual_g0.xyz);
        float3 wedge_g5 = (float3(right_dual_g0[3]) * this.group5.xyz) + (float3(this.scalar) * right_dual_g1.xyz);
        float4 wedge_g6 = float4((right_dual_g0[0] * this.e5) + (right_dual_g0[3] * this.e415), (right_dual_g0[1] * this.e5) + (right_dual_g0[3] * this.e425), (right_dual_g0[2] * this.e5) + (right_dual_g0[3] * this.e435), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((right_dual_g2.xyz * this.group1.www), right_dual_g0[3] * this.e321) - (right_dual_g1.wwwx * this.group1.xyzx);
        float3 wedge_g7 = (float3(right_dual_g0[3]) * this.group7.xyz) + (float3(this.e4) * right_dual_g1.xyz) + (right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx);
        float3 wedge_g8 = (float3(right_dual_g0[3]) * this.group8.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group1.yzx) - (right_dual_g2.yzx * this.group1.zxy);
        float4 wedge_g9 = float4((right_dual_g0[1] * this.e35) + (right_dual_g0[3] * this.e4235) + (right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23), (right_dual_g0[2] * this.e15) + (right_dual_g0[3] * this.e4315) + (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31), (right_dual_g0[0] * this.e25) + (right_dual_g0[3] * this.e4125) + (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (right_dual_g2[2] * this.e12)) + (right_dual_g3 * float4(this.scalar)) + float4((this.group4.yzx * right_dual_g2.zxy), right_dual_g0[3] * this.e3215) - (right_dual_g2.yzxx * float4(this.group4.zxy, this.e23)) - float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g2[1] * this.e31);
        float wedge_g10 = (right_dual_g0[3] * this.e1234) + (right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_dual_g0[3] * this.scalar * other.e12345) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g8[1] * other.e412) + (wedge_g1[0] * other.e12345) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g8[2] * other.e423) + (wedge_g1[1] * other.e12345) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g8[0] * other.e431) + (wedge_g1[2] * other.e12345) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g7[2] * other.e435) - (wedge_g6[0] * other.e423) - (wedge_g6[1] * other.e431) - (wedge_g6[2] * other.e412)) + (float4(wedge_g0_y) * other.group3) + float4((wedge_g7.zxy * other.group2.yzx), wedge_g1[3] * other.e12345) - float4((wedge_g7.yzx * other.group2.zxy), wedge_g7[0] * other.e415) - float4((wedge_g8.zxy * other.group0.yzx), wedge_g7[1] * other.e425), 
            /* e5 */
            (wedge_g0_y * other.e5) + (right_dual_g0[3] * this.e5 * other.e12345) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g9[1] * other.e425) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) - (wedge_g9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g10) * other.group1.xyz) + (wedge_g9.yzx * other.group0.zxy) - (wedge_g9.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (float3(wedge_g10) * other.group2.xyz) + (float3(wedge_g9[3]) * other.group0.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group0.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g9 * float4(other.e12345), 
            /* e1234 */
            wedge_g10 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g0[3] * this.scalar) + (right_dual_g2[3] * this.e1234) + (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43);
        float4 wedge_g1 = right_dual_g3 * float4(this.scalar);
        float4 wedge_g3 = (float4(right_dual_g2[3]) * this.group1) - (right_dual_g3 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * this.group1.xyz);
        float3 wedge_g5 = (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy);
        float4 wedge_g6 = float4((right_dual_g2[3] * this.e41) + (right_dual_g3[3] * this.e15), (right_dual_g2[3] * this.e42) + (right_dual_g3[3] * this.e25), (right_dual_g2[3] * this.e43) + (right_dual_g3[3] * this.e35), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (right_dual_g3.xyzx * float4(this.group3.www, this.e23));
        float3 wedge_g7 = (float3(right_dual_g3[3]) * this.group5.xyz) + (float3(this.scalar) * right_dual_g0.xyz) + (this.group4.yzx * right_dual_g3.zxy) - (this.group4.zxy * right_dual_g3.yzx);
        float3 wedge_g8 = (float3(right_dual_g2[3]) * this.group5.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group3.zxy) - (right_dual_g3.zxy * this.group3.yzx);
        float4 wedge_g9 = (this.group6.zxyw * float4(right_dual_g3.yzx, right_dual_g2[3])) + float4((this.group7.xyz * right_dual_g2.www), right_dual_g3[0] * this.e235) + float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g3[1] * this.e315) + float4((right_dual_g2.xyz * this.group1.www), right_dual_g3[2] * this.e125) - (float4(this.e5) * float4(right_dual_g0.xyz, right_dual_g1[3])) - (this.group1.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((this.group8.xyz * right_dual_g3.www), right_dual_g2[0] * this.e1) - float4((right_dual_g3.zxy * this.group6.yzx), right_dual_g2[2] * this.e3);
        float wedge_g10 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (right_dual_g2[3] * this.scalar * other.e1234) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (other.group3.yzxx * float4(wedge_g5.zxy, wedge_g4[0])) + float4((wedge_g9.zxy * other.group1.yzx), wedge_g4[2] * other.e4125) + float4((wedge_g3.xyz * other.group2.www), wedge_g4[1] * other.e4315) + float4((other.group0.xyz * wedge_g9.www), wedge_g3[3] * other.e1234) - (float4(wedge_g10) * float4(other.group2.xyz, other.e45)) - (wedge_g9.yzxz * float4(other.group1.zxy, other.e43)) - float4((wedge_g4 * other.group3.www), wedge_g9[0] * other.e41) - float4((wedge_g5.yzx * other.group3.zxy), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215), -(wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125)) + (float4(wedge_g0_y) * float4(other.group2.xyz, other.e45)) - (other.group3.yzxx * float4(wedge_g8.zxy, wedge_g6[0])), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group0.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group3.yzx) - (wedge_g7.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group1.xyz) - (float3(wedge_g6[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g9.yzxw * float4(other.group3.zxy, other.e1234)) - float4((wedge_g9.zxy * other.group3.yzx), wedge_g10 * other.e3215), 
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group3.xyz), 
            /* e235, e315, e125 */
            (float3(wedge_g9[3]) * other.group3.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0_y) * other.group3, 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Plane> operator /(Plane lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Plane project_via_origin_onto(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public DipoleInversion project_via_origin_onto(CircleRotor other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return DipoleInversion(
            /* e41, e42, e43 */
            (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public DipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (other.e1234 * this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Plane project_via_origin_onto(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Flector project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Flector project_via_origin_onto(Motor other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4((wedge_g0[2] * other.e315) + (wedge_g0[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g0[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g0[3] * other.e435), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215);
        float4 wedge_g9 = float4(other.group0.yx[0] * -1.0) * this.group0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g9[1] * other.e42) - float4(float3(0.0), wedge_g9[2] * other.e43), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45), 
            /* e15, e25, e35, e45 */
            (float4(((float3(wedge_g9[3]) * other.group6.xyz) + (other.group8.yzx * wedge_g9.zxy)), wedge_g9[2] * other.e435) * float4(1.0, 1.0, 1.0, -1.0)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4(float3(0.0), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, 0.0)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz), 
            /* e235, e315, e125 */
            (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            wedge_g0_y * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Plane project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Sphere project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group0, 
            /* e1234 */
            wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 wedge_g0 = float4(other.e12345 * -1.0) * this.group0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.yzxx * float4(other.group0.zxy, other.e1)) + float4((wedge_g0.zxy * other.group0.yzx * float3(-1.0)), (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4)), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * wedge_g0.www), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group1.xyz), 0.0) + float4((wedge_g0.zxy * other.group2.yzx), 0.0) - float4((wedge_g0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<RoundPoint> operator /(RoundPoint lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g1[1] * this.e3) + (right_dual_g2[0] * this.e4), (right_dual_g1[2] * this.e1) + (right_dual_g2[1] * this.e4), (right_dual_g1[0] * this.e2) + (right_dual_g2[2] * this.e4), -(right_dual_g2[1] * this.e2) - (right_dual_g2[2] * this.e3)) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0]));
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[1] * other.e42), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((right_dual_g2[0] * this.e4) + (other.e423 * this.e5), (right_dual_g2[1] * this.e4) + (other.e431 * this.e5), (right_dual_g2[2] * this.e4) + (other.e412 * this.e5), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = (float4(this.e5) * float4(right_dual_g1.xyz, right_dual_g2[3])) + (this.group0.yzxx * float4(right_dual_g2.zxy, right_dual_g3[0])) + float4((right_dual_g2.yzx * this.group0.zxy * float3(-1.0)), (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[3] * other.e1), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[3] * other.e2), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[3] * other.e3), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[2] * other.e435)) + (wedge_g2.yzxw * float4(other.group0.zxy, other.e5)) - (wedge_g2.zxyx * float4(other.group0.yzx, other.e415)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g2[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public DualNum project_via_origin_onto(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * other.e3215 * this.e4, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group0.xyz);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((wedge_g0 * float3(other.e321)), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (this.group0.wwwx * float4(right_dual_g0.xyz, right_dual_g1[0])) + float4((this.group0.xyz * right_dual_g0.www * float3(-1.0)), (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(other.group0.www, other.e5)) + float4((other.group1.xyz * wedge_g0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4((other.group0.zxy * wedge_g0.yzx * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiFlector project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35)) - float4((wedge_g1.yzx * other.group0.zxy), right_dual_g1[3] * other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiPlane project_via_origin_onto(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e5 */
            right_dual_g0 * other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group0.xyzx);
        float3 wedge_g2 = (float3(this.e5) * right_dual_g1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g2[1] * other.e412) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g2[2] * other.e423) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g2[0] * other.e431) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431) - float4((wedge_g2.zxy * other.group0.yzx), wedge_g1[0] * other.e423), 
            /* e5 */
            -(wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx);
        float4 wedge_g1 = float4((right_dual_g2[0] * this.e4) + (other.e423 * this.e5), (right_dual_g2[1] * this.e4) + (other.e431 * this.e5), (right_dual_g2[2] * this.e4) + (other.e412 * this.e5), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group0.xyzx);
        float3 wedge_g2_xyz = (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group0.yzx) - (right_dual_g2.yzx * this.group0.zxy);
        float4 wedge_g3 = float4(right_dual_g2[3]) * float4(this.group0.xyz, this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            wedge_g0 * float3(other.e12345), 
            /* e415, e425, e435, e321 */
            wedge_g1 * float4(other.e12345), 
            /* e235, e315, e125, e4 */
            float4((wedge_g2_xyz * float3(other.e12345)), (right_dual_g2[3] * other.e12345 * this.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g2_xyz[1] * other.e412) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g3[0] * other.e12345), (wedge_g2_xyz[2] * other.e423) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g3[1] * other.e12345), (wedge_g2_xyz[0] * other.e431) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g3[2] * other.e12345), -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[2] * other.e125)) + (other.group2.yzxw * float4(wedge_g0.zxy, wedge_g3[3])) - (other.group2.zxyx * float4(wedge_g0.yzx, wedge_g1[0])) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g1[1] * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g2[0] * this.e4) + (right_dual_g1[1] * this.e3), (right_dual_g2[1] * this.e4) + (right_dual_g1[2] * this.e1), (right_dual_g2[2] * this.e4) + (right_dual_g1[0] * this.e2), -(right_dual_g2[2] * this.e3) - (right_dual_g1[3] * this.e5)) - (this.group0.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((right_dual_g0 * float3(this.e5)), right_dual_g2[0] * this.e1);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e41), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group0.xyz);
        float4 wedge_g1 = (right_dual_g3.zxyw * this.group0.yzxw) - float4((right_dual_g3.yzx * this.group0.zxy), right_dual_g2[3] * this.e5);
        float4 wedge_g2 = (float4(this.e5, this.e5, this.e5, 1.0) * float4(right_dual_g3.xyz, (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g3.www, right_dual_g0[0]));
        float4 wedge_g3 = float4((right_dual_g1[1] * this.e3) + (right_dual_g2[0] * this.e4), (right_dual_g1[2] * this.e1) + (right_dual_g2[1] * this.e4), (right_dual_g1[0] * this.e2) + (right_dual_g2[2] * this.e4), -(right_dual_g2[1] * this.e2) - (right_dual_g2[2] * this.e3)) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0]));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) + float4((other.group3.xyz * wedge_g3.www), wedge_g0[0] * other.e4235) - (wedge_g3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyx * float4(other.group1.yzx, other.e15)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (wedge_g1.yzxw * other.group3.zxyw) - (wedge_g2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge_g0 * other.group3.www), wedge_g2[0] * other.e4235) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[1] * other.e4315)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public VersorEven project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float wedge_g0_w = right_dual_g0[0] * this.e4;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), wedge_g0_w * other.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), (wedge_g0_w * other.e5) + (right_dual_g0[1] * other.e12345 * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(this.e4) * right_dual_g0.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3] * this.e4) * other.group0 * float4(-1.0), 
            /* e5 */
            (wedge_g0_xyz[0] * other.e15) + (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35) - (right_dual_g0[0] * other.e45 * this.e1) - (right_dual_g0[1] * other.e45 * this.e2) - (right_dual_g0[2] * other.e45 * this.e3) - (right_dual_g0[3] * other.e45 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(this.e4) * right_dual_g1.xyz;
        float3 wedge_g1_xyz = (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy);
        float3 wedge_g2_xyz = (float3(right_dual_g1[3]) * this.group0.xyz) - (float3(this.e5) * right_dual_g1.xyz);
        float wedge_g2_w = right_dual_g0[3] * this.e4;
        float3 wedge_g3_xyz = float3(this.e4) * right_dual_g0.xyz;
        float wedge_g3_w = -(right_dual_g0[0] * this.e1) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3) - (right_dual_g0[3] * this.e5);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2_w) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3_xyz.yzx * other.group1.zxy) - (wedge_g3_xyz.zxy * other.group1.yzx)), wedge_g2_w * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(float3(wedge_g3_w), wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g3_xyz * other.group1.www), wedge_g2_w * other.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g3_w * other.e45) + (wedge_g3_xyz[1] * other.e25) + (wedge_g3_xyz[2] * other.e35) - (wedge_g2_xyz[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group1.yzx), wedge_g3_xyz[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, right_dual_g1[3] * this.e4)) - (other.group1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2_xyz[0])) - (float4(wedge_g2_xyz, wedge_g2_w).wwwy * float4(other.group0.xyz, other.e4315))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(Line other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3);
        float3 wedge_g2 = (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g1_w * other.e415) + (wedge_g0[2] * other.e315), (wedge_g1_w * other.e425) + (wedge_g0[0] * other.e125), (wedge_g1_w * other.e435) + (wedge_g0[1] * other.e235), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g0[0] * other.e415), 
            /* e5 */
            -(wedge_g1_xyz[0] * other.e235) - (wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public VersorEven project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(this.e4) * float4(right_dual_g0.xyz, right_dual_g1[3]);
        float3 wedge_g1_xyz = float3(this.e4) * right_dual_g1.xyz;
        float wedge_g1_w = -(right_dual_g0[0] * this.e1) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3);
        float4 wedge_g2 = float4(((float3(this.e5) * right_dual_g0.xyz) + (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy)), right_dual_g0[3] * this.e5);
        float4 wedge_g3 = float4(right_dual_g0[3]) * this.group0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g1_xyz * float3(other.e12345)) + (float3(wedge_g0[3]) * other.group0.xyz)), wedge_g1_w * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g1_xyz[0] * other.e235) - (wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g3[0] * other.e12345), (wedge_g0[0] * other.e125) + (wedge_g3[1] * other.e12345), (wedge_g0[1] * other.e235) + (wedge_g3[2] * other.e12345), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + (other.group0 * float4(float3(wedge_g1_w), wedge_g3[3])) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) - (other.e4 * this.e5);
        float4 wedge_g1 = float4(right_dual_g0[0]) * this.group0;
        float wedge_g2 = right_dual_g0[0] * this.e5;
        float4 wedge_g3 = (float4(other.e3215) * this.group0) - (right_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group0.xyz);
        float3 wedge_g5 = (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy);
        float4 wedge_g6 = float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) - (this.group0.xyzx * float4(float3(other.e321 * -1.0), other.e415));
        float3 wedge_g7 = (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx);
        float3 wedge_g8 = (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group0.yzx) - (other.group8.yzx * this.group0.zxy);
        float4 wedge_g9 = float4((right_dual_g8[0] * this.e4) + (right_dual_g6[1] * this.e3), (right_dual_g8[1] * this.e4) + (right_dual_g6[2] * this.e1), (right_dual_g8[2] * this.e4) + (right_dual_g6[0] * this.e2), -(right_dual_g8[2] * this.e3) - (right_dual_g6[3] * this.e5)) - (this.group0.yzxy * float4(right_dual_g6.zxy, right_dual_g8[1])) - float4((right_dual_g7 * float3(this.e5)), right_dual_g8[0] * this.e1);
        float wedge_g10 = (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3) + (right_dual_g6[3] * this.e4);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(this.e4) * right_dual_g0.xyz;
        float3 wedge_g1_xyz = (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy);
        float3 wedge_g2 = (float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e5) * right_dual_g0.xyz);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[0] * other.e3215) - (wedge_g1_xyz[1] * other.e4125), -(wedge_g0[1] * other.e3215) - (wedge_g1_xyz[2] * other.e4235), -(wedge_g0[2] * other.e3215) - (wedge_g1_xyz[0] * other.e4315), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) + (other.group0.yzxx * float4(wedge_g1_xyz.zxy, wedge_g0[0])), 
            /* e5 */
            -(wedge_g2[0] * other.e4235) - (wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125) - (right_dual_g0[3] * other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4) - (other.e4 * this.e5);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public RoundPoint project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group0.xyz);
        float4 wedge_g1 = float4((right_dual_g0.zxy * this.group0.yzx), this.e4 * other.e3215) - (right_dual_g0.yzxw * float4(this.group0.zxy, this.e5));
        float3 wedge_g2 = (float3(other.e3215) * this.group0.xyz) - (float3(this.e5) * right_dual_g0.xyz);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(wedge_g0[0] * other.e3215) - (wedge_g1[1] * other.e4125), -(wedge_g0[1] * other.e3215) - (wedge_g1[2] * other.e4235), -(wedge_g0[2] * other.e3215) - (wedge_g1[0] * other.e4315), (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234)) + (other.group0.yzxy * float4(wedge_g1.zxy, wedge_g0[1])) + float4((wedge_g2 * float3(other.e1234)), wedge_g0[0] * other.e4235), 
            /* e5 */
            -(wedge_g2[0] * other.e4235) - (wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125) - (wedge_g1[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g3[0])) + (this.group0.wwwy * float4(right_dual_g1.xyz, right_dual_g3[1])) + float4((right_dual_g0.zxy * this.group0.yzx * float3(-1.0)), (right_dual_g2[3] * this.e5) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4));
        float4 wedge_g1 = float4((right_dual_g0[0] * this.e5) + (right_dual_g2[0] * this.e4), (right_dual_g0[1] * this.e5) + (right_dual_g2[1] * this.e4), (right_dual_g0[2] * this.e5) + (right_dual_g2[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group0.xyzx);
        float4 wedge_g2 = float4(((float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group0.yzx) - (right_dual_g2.yzx * this.group0.zxy)), right_dual_g0[3] * this.e5);
        float4 wedge_g3 = float4(right_dual_g0[3]) * this.group0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g0[3] * other.e1) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g0[3] * other.e2) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g0[3] * other.e3) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + (other.group0.zxyw * float4(wedge_g2.yzx, wedge_g3[3])) + float4((wedge_g3.xyz * other.group0.www), wedge_g0[3] * other.e4) - (other.group0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * this.group0.xyz);
        float4 wedge_g1 = (this.group0.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])) - float4((right_dual_g3.yzx * this.group0.zxy), right_dual_g3[3] * this.e5);
        float4 wedge_g2 = (float4(this.e5, this.e5, this.e5, 1.0) * float4(right_dual_g3.xyz, (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g2.www, right_dual_g0[0]));
        float4 wedge_g3 = float4((right_dual_g1[1] * this.e3) + (right_dual_g2[0] * this.e4), (right_dual_g1[2] * this.e1) + (right_dual_g2[1] * this.e4), (right_dual_g1[0] * this.e2) + (right_dual_g2[2] * this.e4), -(right_dual_g1[3] * this.e5) - (right_dual_g2[2] * this.e3)) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0])) - float4((float3(this.e5) * right_dual_g0.xyz), right_dual_g2[1] * this.e2);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge_g3.www, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g3[0] * other.e41) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - float4((wedge_g3.xyz * other.group3.www), wedge_g2[3] * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Scalar> operator /(Scalar lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz * float3(-1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2[3]) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4(this.scalar) * other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2[3]) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2[3] * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2[3] * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2[3] * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2[3] * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) - float4((wedge_g3.yzx * other.group2.zxy), wedge_g2[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiDualNum project_via_origin_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * other.group0.www * float3(-1.0)), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (other.e321 * other.e321 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(AntiLine other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * other.group0.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e3215) * wedge_g0.xyz)), wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Circle other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_via_origin_onto(CircleRotor other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            wedge_g0 * float3(other.e12345), 
            /* e23, e31, e12, e45 */
            wedge_g1 * float4(other.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * wedge_g2.xyz), (wedge_g2[3] * other.e12345) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Dipole other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz * float3(-1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g2 = float3(this.scalar) * other.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiCircleRotor project_via_origin_onto(DipoleInversion other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz * float3(-1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g3 = float4(this.scalar) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * wedge_g1.xyz) + (wedge_g0.zxy * other.group3.yzx) - (wedge_g0.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[0] * other.e3215) + (wedge_g2[0] * other.e1234), (wedge_g0[1] * other.e3215) + (wedge_g2[1] * other.e1234), (wedge_g0[2] * other.e3215) + (wedge_g2[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(wedge_g1.xyz, wedge_g2[3])) + (other.group3.zxyx * float4(wedge_g2.yzx, wedge_g3[0])) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e1234) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) - float4((wedge_g2.zxy * other.group3.yzx), wedge_g0[0] * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiDualNum project_via_origin_onto(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * float2(this.scalar) * other.group0.xy * float2(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * other.e45 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_via_origin_onto(Flector other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * wedge_g0.www * float3(-1.0)), (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) - (wedge_g0[3] * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public AntiMotor project_via_origin_onto(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * wedge_g0.xyz), (wedge_g0[3] * other.e12345) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g1[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 wedge_g0 = float2(this.scalar) * other.group0.yx * float2(-1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g2 = other.e3215 * this.scalar;
        float4 wedge_g3 = float4(this.scalar) * float4(other.group8.xyz, other.e321 * -1.0);
        float3 wedge_g4 = float3(this.scalar) * other.group7.xyz;
        float3 wedge_g5 = float3(this.scalar) * other.group6.xyz;
        float4 wedge_g6 = float4(this.scalar) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g7 = float3(this.scalar) * other.group4.xyz * float3(-1.0);
        float3 wedge_g8 = float3(this.scalar) * other.group3.xyz * float3(-1.0);
        float4 wedge_g9 = float4(this.scalar) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g10 = other.e4 * this.scalar * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g0[0] * other.e12345) + (wedge_g0[1] * other.scalar) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0[1] * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[1]) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g2 * other.e12345) + (wedge_g0[1] * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0[1]) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g10) * other.group6.xyz) + (float3(wedge_g0[1]) * other.group4.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g0[1]) * other.group5.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0[1]) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0[1]) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0[1]) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0[1]) * other.group9), 
            /* e1234 */
            (wedge_g10 * other.e12345) + (wedge_g0[1] * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Plane other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (other.e4 * other.e5 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Sphere other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e4235) + (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g0[3] * other.e3215) + (this.scalar * other.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge_g3.yzx, wedge_g0[3])) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((wedge_g0.xyz * other.group0.www), wedge_g2[3] * other.e5) + float4((other.group1.xyz * wedge_g2.www), wedge_g3[0] * other.e1) - (other.group0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.e12345) + (wedge_g2[3] * other.e235), (wedge_g1[1] * other.e12345) + (wedge_g2[3] * other.e315), (wedge_g1[2] * other.e12345) + (wedge_g2[3] * other.e125), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (other.group0 * float4(wedge_g3.www, wedge_g1[3])) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 wedge_g0 = float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g2 = float4(this.scalar) * float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g3 = float4(this.scalar) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45) - (wedge_g2[0] * other.e41) - (wedge_g2[1] * other.e42) - (wedge_g2[2] * other.e43)) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) + float4((other.group0.xyz * wedge_g0.www), wedge_g2[3] * other.e1234) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge_g0[3] * other.e23) + (wedge_g2[0] * other.e1234), (wedge_g0[3] * other.e31) + (wedge_g2[1] * other.e1234), (wedge_g0[3] * other.e12) + (wedge_g2[2] * other.e1234), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (wedge_g0 * float4(other.group3.www, other.e45)) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2.yzx * other.group3.zxy) - (wedge_g2.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Sphere> operator /(Sphere lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public AntiDualNum operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public AntiDualNum project_via_origin_onto(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * this.e1234) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float wedge_g0 = other.e3215 * this.e1234;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(wedge_g0) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public Sphere project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e1234 */
            right_dual_g0 * other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public DipoleInversion project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g2[3]) * this.group0;
        float wedge_g1 = right_dual_g2[3] * this.e1234;
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(wedge_g1) * other.group1.xyz) + (other.group0.zxy * wedge_g0.yzx) - (other.group0.yzx * wedge_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (wedge_g0.zxy * other.group2.yzx) - (wedge_g0.yzx * other.group2.zxy)), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public DipoleInversion project_via_origin_onto(DipoleInversion other) {
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234) + (other.e1234 * this.e3215);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float wedge_g1 = right_dual_g0[1] * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), wedge_g1 * other.e5), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public Flector project_via_origin_onto(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(wedge_g0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g0[3]) * this.group0;
        float wedge_g1 = right_dual_g0[3] * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g1) * float4(other.group0.xyz, other.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g1) * other.group1.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (wedge_g0.zxy * other.group1.yzx) - (wedge_g0.yzx * other.group1.zxy)), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234);
        float4 wedge_g9 = float4(right_dual_g0[0]) * this.group0;
        float wedge_g10 = right_dual_g0[0] * this.e1234;
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(float3(0.0), wedge_g10 * other.e45) * float4(0.0, 0.0, 0.0, -1.0)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), 0.0) + float4((other.group5.yzx * wedge_g9.zxy), 0.0) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4(float3(0.0), wedge_g9[1] * other.e42) - float4(float3(0.0), wedge_g9[2] * other.e43) - float4((float3(wedge_g10) * other.group3.xyz), 0.0), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45), 
            /* e15, e25, e35, e45 */
            (float4(((float3(wedge_g9[3]) * other.group6.xyz) + (other.group8.yzx * wedge_g9.zxy)), wedge_g9[2] * other.e435) * float4(1.0, 1.0, 1.0, -1.0)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4(float3(0.0), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (other.group7.zxy * wedge_g9.yzx) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public Plane project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public Sphere project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group0, 
            /* e1234 */
            wedge_g0 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g0[3]) * this.group0;
        float wedge_g1 = right_dual_g0[3] * this.e1234;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.yzxx * float4(other.group0.zxy, other.e1)) + float4((wedge_g0.zxy * other.group0.yzx * float3(-1.0)), (wedge_g1 * other.e5) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4)) + float4((float3(wedge_g1) * other.group1.xyz), wedge_g0[1] * other.e2), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1 * other.e235) + (wedge_g0[3] * other.e423), (wedge_g1 * other.e315) + (wedge_g0[3] * other.e431), (wedge_g1 * other.e125) + (wedge_g0[3] * other.e412), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) - (wedge_g0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (wedge_g0.zxy * other.group2.yzx) - (wedge_g0.yzx * other.group2.zxy)), wedge_g1 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g0 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Sphere {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) + (this.e1234 * other.e3215);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(wedge_g0) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(wedge_g0) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(wedge_g0) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<VersorEven> operator /(VersorEven lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4((right_dual_g1[1] * this.e3) + (right_dual_g2[0] * this.e4), (right_dual_g1[2] * this.e1) + (right_dual_g2[1] * this.e4), (right_dual_g1[0] * this.e2) + (right_dual_g2[2] * this.e4), -(right_dual_g2[1] * this.e2) - (right_dual_g2[2] * this.e3)) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (this.group3.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0]));
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e41)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[1] * other.e42), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((right_dual_g2[0] * this.e4) + (other.e423 * this.e5), (right_dual_g2[1] * this.e4) + (other.e431 * this.e5), (right_dual_g2[2] * this.e4) + (other.e412 * this.e5), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = (float4(this.e5) * float4(right_dual_g1.xyz, right_dual_g2[3])) + (this.group3.yzxx * float4(right_dual_g2.zxy, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((right_dual_g2.yzx * this.group3.zxy), other.e423 * this.e235);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), (wedge_g2[3] * other.e4) - (wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[3] * other.e1), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[3] * other.e2), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[3] * other.e3), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[2] * other.e435)) + (wedge_g2.yzxw * float4(other.group0.zxy, other.e5)) - (wedge_g2.zxyx * float4(other.group0.yzx, other.e415)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g2[1] * other.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_via_origin_onto(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float4(other.e3215) * float4(this.group0.xyz, this.e321)).xyz * float3(other.group0.xx, other.e3215) * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * other.e3215 * this.e4) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (float4(this.e4, this.e4, this.e4, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321))) - (right_dual_g0.wwwx * float4(this.group3.xyz, this.e423));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((wedge_g0.xyz * other.group0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (this.group3.wwwx * float4(right_dual_g0.xyz, right_dual_g1[0])) + float4(float3(0.0), (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (right_dual_g0.wwwx * float4(this.group3.xyz, this.e423));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(wedge_g0[3]) * other.group0, 
            /* e1, e2, e3, e5 */
            (wedge_g0 * float4(other.group0.www, other.e5)) + float4((other.group1.xyz * wedge_g0.www), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiPlane project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group3.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (wedge_g0.zxyx * float4(other.group0.yzx, other.e15)) + float4((other.group0.zxy * wedge_g0.yzx * float3(-1.0)), (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiFlector project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g1 = float4((right_dual_g1[0] * this.e4) + (right_dual_g1[3] * this.e423), (right_dual_g1[1] * this.e4) + (right_dual_g1[3] * this.e431), (right_dual_g1[2] * this.e4) + (right_dual_g1[3] * this.e412), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((right_dual_g0.yzx * this.group3.zxy), right_dual_g1[3] * this.e321) - (this.group3.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0]));
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((wedge_g1.xyz * other.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (wedge_g1.zxyx * float4(other.group0.yzx, other.e15)) + float4(float3(0.0), (wedge_g1[1] * other.e25) + (wedge_g1[2] * other.e35)) - float4((wedge_g1.yzx * other.group0.zxy), right_dual_g1[3] * other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiPlane project_via_origin_onto(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual_g0) * float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx);
        float4 wedge_g1 = float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (right_dual_g1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = float4((right_dual_g1[0] * this.e5) + (other.e125 * this.e2), (right_dual_g1[1] * this.e5) + (other.e235 * this.e3), (right_dual_g1[2] * this.e5) + (other.e315 * this.e1), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), other.e423 * this.e235);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(wedge_g2[3]) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group2.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g2[1] * other.e412), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g2[2] * other.e423), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g2[0] * other.e431), -(wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e315) - float4((other.group0.yzx * wedge_g2.zxy), wedge_g1[0] * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4((right_dual_g1[0] * this.e4) + (right_dual_g2[3] * this.e423), (right_dual_g1[1] * this.e4) + (right_dual_g2[3] * this.e431), (right_dual_g1[2] * this.e4) + (right_dual_g2[3] * this.e412), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((other.group0.yzx * this.group3.zxy), right_dual_g2[3] * this.e12345) - float4((other.group0.zxy * this.group3.yzx), other.e423 * this.e235);
        float4 wedge_g1 = float4((right_dual_g2[0] * this.e4) + (right_dual_g2[3] * this.e415), (right_dual_g2[1] * this.e4) + (right_dual_g2[3] * this.e425), (right_dual_g2[2] * this.e4) + (right_dual_g2[3] * this.e435), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + float4((other.group0.xyz * this.group2.www), right_dual_g2[3] * this.e321) - (right_dual_g1.wwwx * this.group3.xyzx);
        float3 wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group2.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group3.yzx) - (right_dual_g2.yzx * this.group3.zxy);
        float4 wedge_g3 = float4(right_dual_g2[3]) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345, other.e12345, other.e12345, 1.0) * float4(wedge_g2_xyz, -(wedge_g2_xyz[0] * other.e415) - (wedge_g2_xyz[1] * other.e425) - (wedge_g2_xyz[2] * other.e435) - (wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125))) + (other.group2 * float4(wedge_g0.www, right_dual_g2[3] * this.e5)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415) + (wedge_g3[0] * other.e12345), (wedge_g0[0] * other.e125) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425) + (wedge_g3[1] * other.e12345), (wedge_g0[1] * other.e235) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435) + (wedge_g3[2] * other.e12345), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + float4((wedge_g2_xyz.yzx * other.group0.zxy), wedge_g3[3] * other.e12345) - float4((wedge_g2_xyz.zxy * other.group0.yzx), wedge_g1[0] * other.e423) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float4 wedge_g0 = float4((right_dual_g2[0] * this.e4) + (right_dual_g1[1] * this.e3), (right_dual_g2[1] * this.e4) + (right_dual_g1[2] * this.e1), (right_dual_g2[2] * this.e4) + (right_dual_g1[0] * this.e2), -(right_dual_g2[2] * this.e3) - (right_dual_g1[3] * this.e5)) - (this.group3.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])) - float4((right_dual_g0 * this.group2.www), right_dual_g2[0] * this.e1);
        float wedge_g1 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e31) + (wedge_g0[3] * other.e41), (wedge_g0[0] * other.e12) + (wedge_g0[3] * other.e42), (wedge_g0[1] * other.e23) + (wedge_g0[3] * other.e43), -(wedge_g1 * other.e45) - (wedge_g0[2] * other.e43)) - (wedge_g0.yzxy * float4(other.group1.zxy, other.e42)) - float4((float3(wedge_g1) * other.group2.xyz), wedge_g0[0] * other.e41), 
            /* e5 */
            (wedge_g0[0] * other.e15) + (wedge_g0[1] * other.e25) + (wedge_g0[2] * other.e35) + (wedge_g0[3] * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group3.xyz);
        float4 wedge_g1 = (right_dual_g3.zxyw * this.group3.yzxw) - float4((right_dual_g3.yzx * this.group3.zxy), right_dual_g2[3] * this.e5);
        float4 wedge_g2 = (this.group3.xyzx * float4(right_dual_g3.www, right_dual_g0[0])) + float4(float3(0.0), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412)) - float4((right_dual_g3.xyz * this.group2.www), right_dual_g2[3] * this.e321);
        float4 wedge_g3 = (float4(right_dual_g3[3]) * float4(this.group0.xyz, this.e321)) + (right_dual_g3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_dual_g1.yzx * this.group3.zxy), right_dual_g3[0] * this.e235) + float4((right_dual_g2.xyz * this.group3.www), right_dual_g3[1] * this.e315) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0])) - float4((right_dual_g3.zxy * this.group1.yzx), right_dual_g2[2] * this.e3);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g2[3] * other.e45) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) + float4((other.group3.xyz * wedge_g3.www), wedge_g0[0] * other.e4235) - (wedge_g3.xyzx * float4(other.group3.www, other.e41)), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyx * float4(other.group1.yzx, other.e15)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[2] * other.e35) - (wedge_g1.yzxw * other.group3.zxyw) - (wedge_g2.wwwz * float4(other.group2.xyz, other.e4125)) - float4((wedge_g0 * other.group3.www), wedge_g2[0] * other.e4235) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[1] * other.e4315)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float4 wedge_g0 = float4(right_dual_g0.yy, right_dual_g0[1], (right_dual_g0[0] * this.e4) + (right_dual_g0[1] * this.e12345)) * float4(this.group0.xyz, 1.0);
        float4 wedge_g2 = float4(right_dual_g0[1]) * this.group2;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.e12345, (wedge_g0[3] * other.e5) + (wedge_g2[3] * other.e12345)) * float4(wedge_g2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = float3(this.e4) * right_dual_g0.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3] * this.e4) * other.group0 * float4(-1.0), 
            /* e5 */
            (wedge_g0_xyz[0] * other.e15) + (wedge_g0_xyz[1] * other.e25) + (wedge_g0_xyz[2] * other.e35) - (right_dual_g0[0] * other.e45 * this.e1) - (right_dual_g0[1] * other.e45 * this.e2) - (right_dual_g0[2] * other.e45 * this.e3) - (right_dual_g0[3] * other.e45 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(this.e4) * right_dual_g1.xyz;
        float3 wedge_g1_xyz = (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy);
        float4 wedge_g2 = (this.group3 * float4(right_dual_g1.www, right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) - (right_dual_g1.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (right_dual_g1.yzxy * float4(this.group1.zxy, this.e315)) + (right_dual_g1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (right_dual_g1[3] * this.e321) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3) - (right_dual_g0[3] * this.e5)) + float4((right_dual_g0.xyz * this.group3.www), right_dual_g1[0] * this.e235) - float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g0[0] * this.e1);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group1.zxy) - (wedge_g3.zxy * other.group1.yzx)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (other.group1.xyzx * float4(wedge_g3.www, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) - float4((wedge_g3.xyz * other.group1.www), wedge_g2[3] * other.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (wedge_g3[1] * other.e25) + (wedge_g3[2] * other.e35) + (wedge_g3[3] * other.e45) - (wedge_g2[2] * other.e4125)) + float4((wedge_g1_xyz.zxy * other.group1.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, right_dual_g1[3] * this.e4)) - (wedge_g2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(Line other) {
        float3 wedge_g0 = float3(this.e4) * other.group0.xyz;
        float3 wedge_g1_xyz = float3(this.e4) * other.group1.xyz;
        float wedge_g1_w = -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3);
        float4 wedge_g2 = float4((other.e415 * this.e5) + (other.e125 * this.e2), (other.e425 * this.e5) + (other.e235 * this.e3), (other.e435 * this.e5) + (other.e315 * this.e1), -(other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)) - float4((other.group1.yzx * this.group3.zxy), other.e415 * this.e415);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((other.group0.xyz * wedge_g2.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(wedge_g2[3]) * other.group1.xyz), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_w * other.e415) + (wedge_g0[2] * other.e315), (wedge_g1_w * other.e425) + (wedge_g0[0] * other.e125), (wedge_g1_w * other.e435) + (wedge_g0[1] * other.e235), -(wedge_g1_xyz[0] * other.e235) - (wedge_g1_xyz[1] * other.e315) - (wedge_g1_xyz[2] * other.e125) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)) - float4((wedge_g0.yzx * other.group1.zxy), wedge_g2[0] * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (right_dual_g0 * float4(this.group3.www, this.e12345)) + float4(float3(0.0), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g1[3] * this.e4);
        float4 wedge_g1 = (float4(right_dual_g0[3]) * this.group1) + (float4(this.e4, this.e4, this.e4, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e1) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3)));
        float4 wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * right_dual_g0.xyz) + (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy)), right_dual_g0[3] * this.e5);
        float4 wedge_g3 = float4(right_dual_g0[3]) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            wedge_g0 * float4(other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g1.xyz)), wedge_g1[3] * other.e12345), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g3[0] * other.e12345), (wedge_g0[0] * other.e125) + (wedge_g3[1] * other.e12345), (wedge_g0[1] * other.e235) + (wedge_g3[2] * other.e12345), -(wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435)) + (other.group0 * float4(wedge_g1.www, wedge_g3[3])) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[0] * this.e12345) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) - (right_dual_g3_w * this.e321) - (other.e4 * this.e5) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412);
        float4 wedge_g1 = float4(right_dual_g0[0]) * this.group3;
        float wedge_g2 = right_dual_g0[0] * this.e5;
        float4 wedge_g3 = (float4(other.e3215) * this.group3) - (right_dual_g1 * float4(this.e5));
        float3 wedge_g4 = (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group3.xyz);
        float3 wedge_g5 = (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy);
        float4 wedge_g6 = float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) + (float4(right_dual_g0[0]) * this.group1) - (this.group3.xyzx * float4(float3(right_dual_g3_w), other.e415));
        float3 wedge_g7 = (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx);
        float3 wedge_g8 = (float3(right_dual_g0[0]) * this.group2.xyz) + (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy);
        float4 wedge_g9 = (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g1.yzxy * float4(this.group1.zxy, this.e315)) + float4((right_dual_g8 * this.group3.www), right_dual_g1[0] * this.e235) + float4((right_dual_g6.yzx * this.group3.zxy), right_dual_g1[2] * this.e125) - (right_dual_g6.zxyw * float4(this.group3.yzx, this.e5)) - float4((right_dual_g7 * this.group2.www), right_dual_g8[0] * this.e1) - float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g8[1] * this.e2) - float4((this.group2.xyz * right_dual_g1.www), right_dual_g8[2] * this.e3);
        float wedge_g10 = (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3) + (right_dual_g6[3] * this.e4) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = float3(this.e4) * right_dual_g0.xyz;
        float3 wedge_g1_xyz = (right_dual_g0.zxy * this.group3.yzx) - (right_dual_g0.yzx * this.group3.zxy);
        float4 wedge_g2 = float4((this.group3.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) - (right_dual_g0.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (right_dual_g0.yzxx * float4(this.group1.zxy, this.e235)) + (right_dual_g0.wwwy * float4(this.group0.xyz, this.e315)) + float4((right_dual_g0.zxy * this.group1.yzx * float3(-1.0)), (right_dual_g0[2] * this.e125) + (right_dual_g0[3] * this.e321));
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(wedge_g2[3]) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((wedge_g3.yzx * other.group0.zxy) - (wedge_g3.zxy * other.group0.yzx)), wedge_g2[3] * other.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g3.xyz, (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1_xyz.zxy * other.group0.yzx), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) - (float4(other.e3215) * float4(wedge_g0, right_dual_g0[3] * this.e4)) - (other.group0.zxyx * float4(wedge_g1_xyz.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public RoundPoint project_via_origin_onto(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0 = (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4) - (other.e4 * this.e5);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e5 */
            wedge_g0 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group3.xyz);
        float4 wedge_g1 = float4((right_dual_g0.zxy * this.group3.yzx), other.e3215 * this.e4) - (right_dual_g0.yzxw * float4(this.group3.zxy, this.e5));
        float4 wedge_g2 = float4((float3(other.e3215) * this.group3.xyz), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (right_dual_g0.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[2] * this.e125) + (other.e3215 * this.e321)) + (right_dual_g0.yzxx * float4(this.group1.zxy, this.e235)) + float4((float3(other.e3215) * this.group0.xyz), right_dual_g0[1] * this.e315);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group0.zxy, other.e1234)) - (other.group0.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g3.xyz, (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (other.group0.xyzx * float4(wedge_g3.www, wedge_g0[0])), 
            /* e1, e2, e3, e5 */
            float4((wedge_g1[2] * other.e4315) + (wedge_g2[0] * other.e1234), (wedge_g1[0] * other.e4125) + (wedge_g2[1] * other.e1234), (wedge_g1[1] * other.e4235) + (wedge_g2[2] * other.e1234), -(wedge_g2[1] * other.e4315) - (wedge_g2[2] * other.e4125)) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (other.group0.zxyx * float4(wedge_g1.yzx, wedge_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public VersorEven project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = (float4(right_dual_g0[3]) * this.group0) + (this.group3.wwwx * float4(right_dual_g1.xyz, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412)) + float4((right_dual_g0.yzx * this.group3.zxy), right_dual_g2[3] * this.e5) - float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g2[0] * this.e423);
        float4 wedge_g1 = float4((right_dual_g0[0] * this.e5) + (right_dual_g2[0] * this.e4), (right_dual_g0[1] * this.e5) + (right_dual_g2[1] * this.e4), (right_dual_g0[2] * this.e5) + (right_dual_g2[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) + (float4(right_dual_g0[3]) * this.group1) - (right_dual_g1.wwwx * this.group3.xyzx);
        float4 wedge_g2 = float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * right_dual_g1.xyz) + (right_dual_g2.zxy * this.group3.yzx) - (right_dual_g2.yzx * this.group3.zxy)), right_dual_g0[3] * this.e5);
        float4 wedge_g3 = float4(right_dual_g0[3]) * this.group3;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g0.xyz)), wedge_g0[3] * other.e12345), 
            /* e415, e425, e435, e321 */
            (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group1), 
            /* e235, e315, e125, e5 */
            (wedge_g2 * float4(other.e12345)) + (float4(wedge_g0[3]) * other.group2) + float4(float3(0.0), -(wedge_g1[0] * other.e235) - (wedge_g1[1] * other.e315) - (wedge_g1[2] * other.e125) - (wedge_g2[0] * other.e415) - (wedge_g2[1] * other.e425) - (wedge_g2[2] * other.e435)), 
            /* e1, e2, e3, e4 */
            float4((wedge_g0[2] * other.e315) + (wedge_g0[3] * other.e1) + (wedge_g1[0] * other.e321) + (wedge_g1[3] * other.e415), (wedge_g0[0] * other.e125) + (wedge_g0[3] * other.e2) + (wedge_g1[1] * other.e321) + (wedge_g1[3] * other.e425), (wedge_g0[1] * other.e235) + (wedge_g0[3] * other.e3) + (wedge_g1[2] * other.e321) + (wedge_g1[3] * other.e435), -(wedge_g0[0] * other.e415) - (wedge_g0[1] * other.e425) - (wedge_g0[2] * other.e435) - (wedge_g1[2] * other.e412)) + (other.group0.zxyw * float4(wedge_g2.yzx, wedge_g3[3])) + float4((wedge_g3.xyz * other.group0.www), wedge_g0[3] * other.e4) - (other.group0.yzxx * float4(wedge_g2.zxy, wedge_g1[0])) - float4((wedge_g0.yzx * other.group2.zxy), wedge_g1[1] * other.e431)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(ProjectViaOriginOntoInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * this.group3.xyz);
        float4 wedge_g1 = (this.group3.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])) - (right_dual_g3.yzxw * float4(this.group3.zxy, this.e5));
        float4 wedge_g2 = (this.group3.xyzx * float4(right_dual_g2.www, right_dual_g0[0])) + float4(float3(0.0), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321)) - (right_dual_g3.xyzx * float4(this.group2.www, this.e423));
        float4 wedge_g3 = (right_dual_g3.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_dual_g1.yzx * this.group3.zxy), right_dual_g2[3] * this.e321) + float4((right_dual_g2.xyz * this.group3.www), right_dual_g3[0] * this.e235) + float4((this.group0.xyz * right_dual_g2.www), right_dual_g3[1] * this.e315) - (float4(this.e5) * float4(right_dual_g0.xyz, right_dual_g1[3])) - (this.group3.yzxx * float4(right_dual_g1.zxy, right_dual_g2[0])) - float4((right_dual_g3.zxy * this.group1.yzx), right_dual_g2[1] * this.e2) - float4((this.group2.xyz * right_dual_g3.www), right_dual_g2[2] * this.e3);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * wedge_g3.xyz) - (float3(wedge_g2[3]) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g3.yzxw * float4(other.group3.zxy, other.e1234)) - (other.group3.yzxw * float4(wedge_g3.zxy, wedge_g2[3])), 
            /* e235, e315, e125, e4 */
            (other.group3.xyzx * float4(wedge_g3.www, wedge_g0[0])) + float4(float3(0.0), (wedge_g0[1] * other.e4315) + (wedge_g0[2] * other.e4125) + (wedge_g1[3] * other.e1234) - (wedge_g3[0] * other.e41) - (wedge_g3[1] * other.e42) - (wedge_g3[2] * other.e43)) - float4((wedge_g3.xyz * other.group3.www), wedge_g2[3] * other.e45), 
            /* e1, e2, e3, e5 */
            (float4(wedge_g3[3]) * float4(other.group0.xyz, other.e45)) + (wedge_g3.zxyz * float4(other.group1.yzx, other.e35)) + (other.group2.wwwy * float4(wedge_g2.xyz, wedge_g3[1])) + float4((wedge_g1.zxy * other.group3.yzx), wedge_g3[0] * other.e15) - (float4(other.e3215) * float4(wedge_g0, wedge_g1[3])) - (wedge_g2.wwwy * float4(other.group2.xyz, other.e4315)) - (other.group3.zxyx * float4(wedge_g1.yzx, wedge_g2[0])) - float4((wedge_g3.yzx * other.group1.zxy), wedge_g2[2] * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<VersorOdd> operator /(VersorOdd lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_via_origin_onto(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0 = right_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * right_dual_g2.xyz;
        float wedge_g2_w = (right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), (wedge_g2_w * other.scalar) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_via_origin_onto(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * right_dual_g2.xyz;
        float wedge_g2_w = (right_dual_g2[3] * this.scalar) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (right_dual_g3[0] * this.scalar), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (right_dual_g3[1] * this.scalar), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (right_dual_g3[2] * this.scalar), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) + float4((other.group0.yzx * this.group2.zxy), right_dual_g3[3] * this.scalar) - (this.group2.yzxx * float4(other.group0.zxy, right_dual_g1[0])) - float4((right_dual_g2.yzx * this.group0.zxy), right_dual_g1[1] * this.e25);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(wedge_g3[3]) * float4(other.group1.xyz, other.e4)) + (wedge_g3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (wedge_g2_w * other.e5) + (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) - (wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((wedge_g3.yzx * other.group2.zxy), wedge_g2_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_via_origin_onto(AntiDualNum other) {
        float4 wedge_g0 = float4(other.group0.xx, other.e3215, (other.e3215 * this.e1234) + (other.scalar * this.scalar)) * float4(this.group0.xyz, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), wedge_g0[3] * other.scalar), 
            /* e15, e25, e35, e3215 */
            wedge_g0 * float4(other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_via_origin_onto(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(wedge_g1.xyz, right_dual_g0[3] * this.scalar) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_via_origin_onto(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g1 = float4((right_dual_g0[3] * this.e23) + (right_dual_g1[0] * this.scalar), (right_dual_g0[3] * this.e31) + (right_dual_g1[1] * this.scalar), (right_dual_g0[3] * this.e12) + (right_dual_g1[2] * this.scalar), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) + (this.group0.yzxw * float4(right_dual_g0.zxy, right_dual_g1[3])) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((wedge_g1.xyz * other.group0.www * float3(-1.0)), (wedge_g1[0] * other.e1) + (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) - (right_dual_g0[3] * other.e321 * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((wedge_g1.zxy * other.group0.yzx) - (wedge_g1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_via_origin_onto(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_dual_g0 * float3(this.scalar);
        float wedge_g0_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group0.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(wedge_g0_w) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiMotor project_via_origin_onto(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0 * float4(this.scalar)) + (float4(right_dual_g1[3]) * float4(this.group0.xyz, this.e1234)) + float4(float3(0.0), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43));
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0[3]) * other.group0.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(wedge_g0[3]) * other.group1.xyz) + (float3(other.e3215) * wedge_g0.xyz)), wedge_g0[3] * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public Scalar project_via_origin_onto(AntiPlane other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5 * -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0) * float4(other.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g0) * float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_via_origin_onto(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0 = float3(this.scalar) * other.group0.xyz;
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = float3(this.scalar) * other.group2.xyz;
        float wedge_g2_w = -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (other.e431 * this.e35) + (other.e125 * this.e42), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (other.e412 * this.e15) + (other.e235 * this.e43), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (other.e423 * this.e25) + (other.e315 * this.e41), -(right_dual_g1[0] * this.e15) - (right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e235 * this.e23) - float4((other.group2.yzx * this.group0.zxy), other.e315 * this.e31);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(wedge_g2_w) * other.group1.xyz) + (other.group0.zxy * wedge_g3.yzx) - (other.group0.yzx * wedge_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_w * other.e235) + (wedge_g3[3] * other.e423), (wedge_g2_w * other.e315) + (wedge_g3[3] * other.e431), (wedge_g2_w * other.e125) + (wedge_g3[3] * other.e412), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e315) + (wedge_g3[3] * other.e415), (wedge_g3[0] * other.e125) + (wedge_g3[3] * other.e425), (wedge_g3[1] * other.e235) + (wedge_g3[3] * other.e435), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g2_xyz[1] * other.e431) - (wedge_g2_xyz[2] * other.e412) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321)) - float4((other.group2.zxy * wedge_g3.yzx), wedge_g2_xyz[0] * other.e423)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = (float3(right_dual_g2[3]) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g2[3]) * this.group1);
        float4 wedge_g2 = (right_dual_g2 * float4(this.group0.www, this.e1234)) + float4((this.group2.xyz * right_dual_g2.www), -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12));
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (right_dual_g2[3] * this.e4235), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (right_dual_g2[3] * this.e4315), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (right_dual_g2[3] * this.e4125), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) + float4((other.group0.yzx * this.group2.zxy), right_dual_g2[3] * this.e3215) - (this.group2.yzxx * float4(other.group0.zxy, right_dual_g1[0])) - float4((right_dual_g2.yzx * this.group0.zxy), right_dual_g1[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((wedge_g0_xyz[0] * other.e12345) + (wedge_g2[3] * other.e415), (wedge_g0_xyz[1] * other.e12345) + (wedge_g2[3] * other.e425), (wedge_g0_xyz[2] * other.e12345) + (wedge_g2[3] * other.e435), -(wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((other.group0.zxy * wedge_g3.yzx), right_dual_g2[3] * other.e12345 * this.scalar) - float4((other.group0.yzx * wedge_g3.zxy), wedge_g2[0] * other.e423), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.e12345) + (wedge_g2[3] * other.e235), (wedge_g1[1] * other.e12345) + (wedge_g2[3] * other.e315), (wedge_g1[2] * other.e12345) + (wedge_g2[3] * other.e125), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + float4((other.group0.xyz * wedge_g3.www), wedge_g1[3] * other.e12345) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_via_origin_onto(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        float3 wedge_g0 = right_dual_g0 * float3(this.scalar);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float3 wedge_g2_xyz = right_dual_g2 * float3(this.scalar);
        float wedge_g2_w = -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(wedge_g2_w) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group2.xyz), -(wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((right_dual_g0 * this.group0.www), right_dual_g2[3] * this.e3215) + float4((this.group1.xyz * right_dual_g2.www), right_dual_g3[0] * this.e4235) - float4((right_dual_g3.yzx * this.group0.zxy), right_dual_g0[0] * this.e15);
        float4 wedge_g1 = float4((right_dual_g2[3] * this.e15) + (right_dual_g3[3] * this.e41), (right_dual_g2[3] * this.e25) + (right_dual_g3[3] * this.e42), (right_dual_g2[3] * this.e35) + (right_dual_g3[3] * this.e43), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(right_dual_g3[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group2.zxy) - (right_dual_g3.zxy * this.group2.yzx);
        float4 wedge_g3 = float4(this.scalar) * float4(right_dual_g3.xyz, right_dual_g2[3]);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group3.yzxy * float4(wedge_g0.zxy, wedge_g3[1])) + float4(float3(0.0), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[0] * other.e15) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((other.group0.xyz * wedge_g0.www), right_dual_g3[3] * other.e1234 * this.scalar) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) - float4((wedge_g0.yzx * other.group3.zxy), wedge_g2_xyz[0] * other.e41), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[0] * other.e3215), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[1] * other.e3215), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[2] * other.e3215), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (float4(wedge_g0[3]) * other.group1) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        float4 wedge_g0 = float4(right_dual_g0[1]) * this.group0;
        float4 wedge_g2 = float4(right_dual_g0[1]) * this.group2;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.e12345, (wedge_g0[3] * other.e12345) + (wedge_g2[3] * other.e5)) * float4(wedge_g0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[1]) * float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            wedge_g2 * float4(other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * float4(right_dual_g0.yy, right_dual_g0[1], (right_dual_g0[0] * this.scalar) + (right_dual_g0[1] * this.e3215)) * float4(this.group3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_via_origin_onto(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g2_w = -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), wedge_g2_w * other.e45), 
            /* e15, e25, e35, scalar */
            float4((float3(wedge_g2_w) * other.group0.xyz), right_dual_g0[3] * other.e45 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) - (this.group0.zxyx * float4(right_dual_g1.yzx, right_dual_g0[0]));
        float4 wedge_g1 = (this.group0 * float4(right_dual_g1.www, right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) - (right_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(right_dual_g1[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g0.xyz) + (right_dual_g1.yzx * this.group2.zxy) - (right_dual_g1.zxy * this.group2.yzx);
        float3 wedge_g3_xyz = right_dual_g1.xyz * this.group0.www;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group1.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])) + float4(float3(0.0), (wedge_g3_xyz[1] * other.e4315) + (wedge_g3_xyz[2] * other.e4125) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[3] * other.e45)) - (wedge_g0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (wedge_g0 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(wedge_g0[3]) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * other.group1.zxy), 0.0) - float4((wedge_g2_xyz.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor project_via_origin_onto(Line other) {
        float3 wedge_g1_xyz = other.group0.xyz * this.group0.www;
        float wedge_g2_w = -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43);
        float4 wedge_g3 = float4((other.e415 * this.e45) + (other.e125 * this.e42), (other.e425 * this.e45) + (other.e235 * this.e43), (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e415 * this.e15);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(wedge_g2_w) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(wedge_g2_w) * other.group1.xyz), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, scalar */
            float4((wedge_g3[2] * other.e315) + (wedge_g3[3] * other.e415), (wedge_g3[0] * other.e125) + (wedge_g3[3] * other.e425), (wedge_g3[1] * other.e235) + (wedge_g3[3] * other.e435), -(wedge_g1_xyz[0] * other.e415) - (wedge_g1_xyz[1] * other.e425) - (wedge_g1_xyz[2] * other.e435)) - float4((other.group1.zxy * wedge_g3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 wedge_g0 = float4(right_dual_g0[3]) * this.group0;
        float4 wedge_g1 = float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g0.xyz)), right_dual_g0[3] * this.e45);
        float4 wedge_g2 = (float4(right_dual_g0[3]) * this.group2) + (float4(this.scalar, this.scalar, this.scalar, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)));
        float4 wedge_g3 = (right_dual_g0 * float4(this.group1.www, this.e3215)) + float4((right_dual_g1.zxy * this.group0.yzx), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((this.group3.xyz * right_dual_g0.www), right_dual_g1[3] * this.scalar) - float4((right_dual_g1.yzx * this.group0.zxy), right_dual_g0[0] * this.e15);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0 * float4(wedge_g2.www, wedge_g0[3])) + float4(float3(0.0), -(wedge_g0[0] * other.e235) - (wedge_g0[1] * other.e315) - (wedge_g0[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435)) + float4((wedge_g0.xyz * other.group0.www), wedge_g2[3] * other.e5), 
            /* e23, e31, e12, e45 */
            (wedge_g1 * float4(other.e12345)) + float4((other.group1.xyz * wedge_g2.www), -(wedge_g3[0] * other.e415) - (wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group0.xyz) + (float3(other.e12345) * wedge_g2.xyz) + (wedge_g3.zxy * other.group1.yzx) - (wedge_g3.yzx * other.group1.zxy)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float wedge_g0_y = (right_dual_g0[1] * this.scalar) + (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45);
        float4 wedge_g1 = right_dual_g1 * float4(this.scalar);
        float wedge_g2 = other.e3215 * this.scalar;
        float4 wedge_g3 = (right_dual_g3 * float4(this.scalar)) + (float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45));
        float3 wedge_g4 = (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz);
        float3 wedge_g5 = (float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group6.xyz);
        float4 wedge_g6 = float4((right_dual_g6[0] * this.scalar) + (other.e3215 * this.e41), (right_dual_g6[1] * this.scalar) + (other.e3215 * this.e42), (right_dual_g6[2] * this.scalar) + (other.e3215 * this.e43), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((this.group2.xyz * right_dual_g1.www), right_dual_g6[3] * this.scalar) - (right_dual_g1.xyzx * this.group1.wwwx);
        float3 wedge_g7 = (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g1[3]) * this.group1.xyz) + (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy);
        float3 wedge_g8 = (right_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + (right_dual_g1.yzx * this.group2.zxy) - (right_dual_g1.zxy * this.group2.yzx);
        float4 wedge_g9 = float4((right_dual_g3[2] * this.e42) + (right_dual_g3[3] * this.e23) + (right_dual_g9[0] * this.scalar) + (other.e415 * this.e45), (right_dual_g3[0] * this.e43) + (right_dual_g3[3] * this.e31) + (right_dual_g9[1] * this.scalar) + (other.e425 * this.e45), (right_dual_g3[1] * this.e41) + (right_dual_g3[3] * this.e12) + (right_dual_g9[2] * this.scalar) + (other.e435 * this.e45), -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12) - (other.e435 * this.e35)) + (float4(right_dual_g0[0]) * this.group3) + float4((other.group7.yzx * this.group2.zxy), right_dual_g9[3] * this.scalar) - (this.group2.yzxx * float4(other.group7.zxy, other.e415)) - float4((right_dual_g3.yzx * this.group0.zxy), other.e425 * this.e25);
        float wedge_g10 = (right_dual_g0[0] * this.e1234) - (other.e4 * this.scalar) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12);
        return MultiVector(
            /* scalar, e12345 */
            float2((wedge_g0_y * other.scalar) + (wedge_g10 * other.e5) + (wedge_g2 * other.e1234) + (wedge_g1[0] * other.e4235) + (wedge_g1[1] * other.e4315) + (wedge_g1[2] * other.e4125) + (wedge_g1[3] * other.e3215) + (wedge_g9[0] * other.e1) + (wedge_g9[1] * other.e2) + (wedge_g9[2] * other.e3) + (wedge_g9[3] * other.e4) + (right_dual_g0[0] * other.e12345 * this.scalar) - (wedge_g4[0] * other.e235) - (wedge_g4[1] * other.e315) - (wedge_g4[2] * other.e125) - (wedge_g5[0] * other.e415) - (wedge_g5[1] * other.e425) - (wedge_g5[2] * other.e435) - (wedge_g7[0] * other.e15) - (wedge_g7[1] * other.e25) - (wedge_g7[2] * other.e35) - (wedge_g8[0] * other.e41) - (wedge_g8[1] * other.e42) - (wedge_g8[2] * other.e43) - (wedge_g3[0] * other.e423) - (wedge_g3[1] * other.e431) - (wedge_g3[2] * other.e412) - (wedge_g3[3] * other.e321) - (wedge_g6[0] * other.e23) - (wedge_g6[1] * other.e31) - (wedge_g6[2] * other.e12) - (wedge_g6[3] * other.e45), wedge_g0_y * other.e12345), 
            /* e1, e2, e3, e4 */
            float4((wedge_g5[2] * other.e4315) + (wedge_g3[0] * other.e1234) + (wedge_g6[0] * other.e321) + (wedge_g6[3] * other.e415), (wedge_g5[0] * other.e4125) + (wedge_g3[1] * other.e1234) + (wedge_g6[1] * other.e321) + (wedge_g6[3] * other.e425), (wedge_g5[1] * other.e4235) + (wedge_g3[2] * other.e1234) + (wedge_g6[2] * other.e321) + (wedge_g6[3] * other.e435), -(wedge_g10 * other.e45) - (wedge_g7[0] * other.e415) - (wedge_g7[1] * other.e425) - (wedge_g7[2] * other.e435)) + (wedge_g1 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group1) + float4((other.group4.xyz * wedge_g9.www), wedge_g4[0] * other.e4235) + float4((wedge_g7.zxy * other.group8.yzx), wedge_g3[3] * other.e1234) + float4((wedge_g8.yzx * other.group7.zxy), wedge_g4[2] * other.e4125) + float4((other.group5.yzx * wedge_g9.zxy), wedge_g4[1] * other.e4315) - (wedge_g9.yzxx * float4(other.group5.zxy, other.e41)) - float4((wedge_g4 * other.group9.www), wedge_g6[0] * other.e423) - float4((float3(wedge_g10) * other.group3.xyz), wedge_g6[2] * other.e412) - float4((wedge_g5.yzx * other.group9.zxy), wedge_g6[1] * other.e431) - float4((wedge_g7.yzx * other.group8.zxy), wedge_g9[2] * other.e43) - float4((wedge_g8.zxy * other.group7.yzx), wedge_g9[1] * other.e42), 
            /* e5 */
            (wedge_g0_y * other.e5) + (wedge_g2 * other.e12345) + (wedge_g9[0] * other.e15) + (wedge_g9[1] * other.e25) + (wedge_g9[2] * other.e35) + (wedge_g9[3] * other.e45) - (wedge_g8[0] * other.e415) - (wedge_g8[1] * other.e425) - (wedge_g8[2] * other.e435) - (wedge_g3[0] * other.e4235) - (wedge_g3[1] * other.e4315) - (wedge_g3[2] * other.e4125) - (wedge_g3[3] * other.e3215) - (wedge_g6[0] * other.e235) - (wedge_g6[1] * other.e315) - (wedge_g6[2] * other.e125), 
            /* e15, e25, e35, e45 */
            float4((wedge_g8[1] * other.e4125) + (wedge_g6[0] * other.e3215) + (wedge_g9[2] * other.e315) + (wedge_g9[3] * other.e415), (wedge_g8[2] * other.e4235) + (wedge_g6[1] * other.e3215) + (wedge_g9[0] * other.e125) + (wedge_g9[3] * other.e425), (wedge_g8[0] * other.e4315) + (wedge_g6[2] * other.e3215) + (wedge_g9[1] * other.e235) + (wedge_g9[3] * other.e435), -(wedge_g6[0] * other.e4235) - (wedge_g6[1] * other.e4315) - (wedge_g6[2] * other.e4125) - (wedge_g9[2] * other.e435)) + (wedge_g3 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group3) - (wedge_g9.yzxx * float4(other.group8.zxy, other.e415)) - float4((wedge_g8.zxy * other.group9.yzx), wedge_g9[1] * other.e425), 
            /* e41, e42, e43 */
            (wedge_g4 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group4.xyz) + (float3(wedge_g10) * other.group6.xyz) + (float3(other.e1234) * wedge_g6.xyz) + (wedge_g7.zxy * other.group9.yzx) + (other.group7.zxy * wedge_g9.yzx) - (wedge_g7.yzx * other.group9.zxy) - (other.group7.yzx * wedge_g9.zxy), 
            /* e23, e31, e12 */
            (wedge_g5 * float3(other.e12345)) + (wedge_g7 * float3(other.e3215)) + (wedge_g8 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group5.xyz) + (float3(wedge_g10) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group7.xyz) - (float3(wedge_g6[3]) * other.group9.xyz) - (float3(other.e321) * wedge_g9.xyz), 
            /* e415, e425, e435, e321 */
            (wedge_g6 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group6) + float4((wedge_g9.yzx * other.group9.zxy), wedge_g9[3] * other.e1234) - (other.group9.yzxw * float4(wedge_g9.zxy, wedge_g10)), 
            /* e423, e431, e412 */
            (wedge_g7 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group7.xyz) + (float3(other.e1234) * wedge_g9.xyz) - (float3(wedge_g10) * other.group9.xyz), 
            /* e235, e315, e125 */
            (wedge_g8 * float3(other.e12345)) + (float3(wedge_g0_y) * other.group8.xyz) + (float3(wedge_g9[3]) * other.group9.xyz) - (float3(other.e3215) * wedge_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (wedge_g9 * float4(other.e12345)) + (float4(wedge_g0_y) * other.group9), 
            /* e1234 */
            (wedge_g0_y * other.e1234) + (wedge_g10 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0.zxyx * float4(this.group0.yzx, this.e4235)) + float4((right_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234));
        float4 wedge_g1 = float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(right_dual_g0[3]) * this.group1.xyz) + (right_dual_g0.yzx * this.group2.zxy) - (right_dual_g0.zxy * this.group2.yzx);
        float3 wedge_g3_xyz = right_dual_g0.xyz * this.group0.www;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.yzxx * float4(wedge_g0.zxy, wedge_g3_xyz[0])) + float4((wedge_g0.yzx * other.group0.zxy * float3(-1.0)), (wedge_g3_xyz[1] * other.e4315) + (wedge_g3_xyz[2] * other.e4125)), 
            /* e23, e31, e12, e45 */
            (float4(other.e3215, other.e3215, other.e3215, 1.0) * float4(wedge_g0.xyz, -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125))) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * wedge_g1.xyz), 0.0) + float4((wedge_g2_xyz.yzx * other.group0.zxy), 0.0) - float4((wedge_g2_xyz.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public Scalar project_via_origin_onto(RoundPoint other) {
        float4 wedge_g0 = float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4) - (other.e4 * other.e5 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0.zxyx * float4(this.group0.yzx, this.e4235)) + (right_dual_g0.wwwy * float4(this.group1.xyz, this.e4315)) + float4((right_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234));
        float4 wedge_g1 = float4((right_dual_g0[3] * this.e15) + (other.e3215 * this.e41), (right_dual_g0[3] * this.e25) + (other.e3215 * this.e42), (right_dual_g0[3] * this.e35) + (other.e3215 * this.e43), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(other.e3215) * this.group1.xyz) + (right_dual_g0.yzx * this.group2.zxy) - (right_dual_g0.zxy * this.group2.yzx);
        float4 wedge_g3 = right_dual_g0 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.yzxx * float4(wedge_g0.zxy, wedge_g3[0])) + float4((wedge_g0.yzx * other.group0.zxy * float3(-1.0)), (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) + (other.e3215 * other.e1234 * this.scalar)) + float4((float3(other.e1234) * wedge_g1.xyz), wedge_g3[1] * other.e4315), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[0] * other.e3215), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[1] * other.e3215), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[2] * other.e3215), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) - (wedge_g1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group0.zxy) - (wedge_g2_xyz.zxy * other.group0.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float3 wedge_g0_xyz = (float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.scalar) * right_dual_g0.xyz);
        float4 wedge_g1 = (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0[3]) * this.group1);
        float4 wedge_g2 = (right_dual_g2 * float4(this.scalar)) + (float4(right_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43));
        float4 wedge_g3 = float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (right_dual_g3[0] * this.scalar), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (right_dual_g3[1] * this.scalar), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (right_dual_g3[2] * this.scalar), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) + (right_dual_g0.yzxw * float4(this.group2.zxy, this.e3215)) + float4((this.group3.xyz * right_dual_g0.www), right_dual_g3[3] * this.scalar) - (this.group2.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0])) - float4((right_dual_g2.yzx * this.group0.zxy), right_dual_g1[1] * this.e25);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.zxyw * float4(wedge_g3.yzx, right_dual_g0[3] * this.scalar)) + float4(float3(0.0), (wedge_g3[1] * other.e2) + (wedge_g3[2] * other.e3) + (wedge_g3[3] * other.e4) - (wedge_g0_xyz[0] * other.e235) - (wedge_g0_xyz[1] * other.e315) - (wedge_g0_xyz[2] * other.e125) - (wedge_g1[0] * other.e415) - (wedge_g1[1] * other.e425) - (wedge_g1[2] * other.e435) - (wedge_g1[3] * other.e321) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412)) + float4((wedge_g0_xyz * other.group0.www), wedge_g2[3] * other.e5) + float4((other.group1.xyz * wedge_g2.www), wedge_g3[0] * other.e1) - (other.group0.yzxx * float4(wedge_g3.zxy, wedge_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((wedge_g1[0] * other.e12345) + (wedge_g2[3] * other.e235), (wedge_g1[1] * other.e12345) + (wedge_g2[3] * other.e315), (wedge_g1[2] * other.e12345) + (wedge_g2[3] * other.e125), -(wedge_g3[1] * other.e425) - (wedge_g3[2] * other.e435)) + (other.group0 * float4(wedge_g3.www, wedge_g1[3])) - (wedge_g3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g3[3]) * other.group1.xyz) + (float3(other.e12345) * wedge_g2.xyz) + (wedge_g3.zxy * other.group2.yzx) - (wedge_g3.yzx * other.group2.zxy)), wedge_g2[3] * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            wedge_g3 * float4(other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(ProjectViaOriginOntoInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension VersorOdd {
    public VersorOdd project_via_origin_onto(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0 * float4(this.scalar)) + (right_dual_g3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((right_dual_g3.zxy * this.group0.yzx), right_dual_g2[3] * this.e1234) - float4((right_dual_g3.yzx * this.group0.zxy), right_dual_g0[0] * this.e15);
        float4 wedge_g1 = float4((right_dual_g2[3] * this.e41) + (right_dual_g3[3] * this.e15), (right_dual_g2[3] * this.e42) + (right_dual_g3[3] * this.e25), (right_dual_g2[3] * this.e43) + (right_dual_g3[3] * this.e35), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (right_dual_g3.xyzx * this.group1.wwwx);
        float3 wedge_g2_xyz = (float3(right_dual_g2[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g2.xyz) + (right_dual_g3.yzx * this.group2.zxy) - (right_dual_g3.zxy * this.group2.yzx);
        float4 wedge_g3 = right_dual_g3 * float4(this.scalar);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (wedge_g0.zxyw * float4(other.group3.yzx, other.scalar)) + float4(float3(0.0), (wedge_g3[1] * other.e4315) + (wedge_g3[2] * other.e4125) + (wedge_g3[3] * other.e3215) - (wedge_g2_xyz[0] * other.e41) - (wedge_g2_xyz[1] * other.e42) - (wedge_g2_xyz[2] * other.e43) - (wedge_g0[1] * other.e25) - (wedge_g0[2] * other.e35) - (wedge_g1[0] * other.e23) - (wedge_g1[1] * other.e31) - (wedge_g1[2] * other.e12) - (wedge_g1[3] * other.e45)) + float4((wedge_g1.xyz * other.group2.www), wedge_g3[0] * other.e4235) + float4((other.group0.xyz * wedge_g0.www), right_dual_g2[3] * other.e1234 * this.scalar) - (wedge_g0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            float4((wedge_g2_xyz[0] * other.e1234) + (wedge_g0[3] * other.e23), (wedge_g2_xyz[1] * other.e1234) + (wedge_g0[3] * other.e31), (wedge_g2_xyz[2] * other.e1234) + (wedge_g0[3] * other.e12), -(wedge_g1[1] * other.e4315) - (wedge_g1[2] * other.e4125)) + (wedge_g0 * float4(other.group3.www, other.e45)) - (wedge_g1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(wedge_g0[3]) * other.group2.xyz) + (float3(other.e3215) * wedge_g1.xyz) + (wedge_g2_xyz.yzx * other.group3.zxy) - (wedge_g2_xyz.zxy * other.group3.yzx)), wedge_g0[3] * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(wedge_g0[3]) * other.group3
        );
    }
}
